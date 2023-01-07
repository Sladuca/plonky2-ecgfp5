// this is mostly copied from https://github.com/pornin/ecgfp5
// and adapted to build atop plonky2 primitives
// note - unlike the original, this is not constant time

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use alloc::vec::Vec;
use plonky2_field::extension::quintic::QuinticExtension;
use plonky2_field::extension::FieldExtension;
use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_field::ops::Square;
use plonky2_field::types::Field;

use crate::curve::base_field::{Half, Legendre, MulSmall, SquareRoot};
use crate::curve::mul_table::*;
use crate::curve::scalar_field::Scalar;

use crate::curve::{GFp, GFp5};

/// A curve point.
#[derive(Clone, Copy, Debug)]
pub struct Point {
    // Internally, we use the (x,u) fractional coordinates: for curve
    // point (x,y), we have (x,u) = (x,x/y) = (X/Z,U/T) (for the neutral
    // N, the u coordinate is 0).
    x: GFp5,
    z: GFp5,
    u: GFp5,
    t: GFp5,
}

// A curve point in affine (x,u) coordinates. This is used internally
// to make "windows" that speed up point multiplications.
// this is also the representation used in-circuit
#[derive(Clone, Copy, Debug)]
pub(crate) struct AffinePoint {
    pub(crate) x: GFp5,
    pub(crate) u: GFp5,
}

impl Point {
    // Curve equation 'a' constant.
    const A: GFp5 = QuinticExtension([GFp::TWO, GFp::ZERO, GFp::ZERO, GFp::ZERO, GFp::ZERO]);

    // Curve equation 'b' constant is equal to B1*z as a u32
    const B1_U32: u32 = 263;

    /* unused
    // Curve equation 'b' constant.
    const B: GFp5 = GFp5([
        GFp::ZERO,
        GFp::from_u64_reduce(Self::B1 as u64),
        GFp::ZERO,
        GFp::ZERO,
        GFp::ZERO,
    ]);
    */

    // 4*b
    const B_MUL4: GFp5 = QuinticExtension([
        GFp::ZERO,
        GoldilocksField(4 * Self::B1_U32 as u64),
        GFp::ZERO,
        GFp::ZERO,
        GFp::ZERO,
    ]);

    /// The neutral point (neutral of the group law).
    pub const NEUTRAL: Self = Self {
        x: GFp5::ZERO,
        z: GFp5::ONE,
        u: GFp5::ZERO,
        t: GFp5::ONE,
    };

    /// The conventional generator (corresponding to encoding w = 4).
    pub const GENERATOR: Self = Self {
        x: QuinticExtension([
            GoldilocksField(12883135586176881569),
            GoldilocksField(4356519642755055268),
            GoldilocksField(5248930565894896907),
            GoldilocksField(2165973894480315022),
            GoldilocksField(2448410071095648785),
        ]),
        z: GFp5::ONE,
        u: QuinticExtension([
            GoldilocksField(13835058052060938241),
            GFp::ZERO,
            GFp::ZERO,
            GFp::ZERO,
            GFp::ZERO,
        ]),
        t: GFp5::ONE,
    };

    /// Encode this point into a field element. Encoding is always
    /// canonical.
    pub fn encode(self) -> GFp5 {
        // Encoded form is the value w = 1/u. GFpor the neutral (u == 0),
        // the encoded form is 0. Since our inversion over GF(p^5) already
        // yields 0 in that case, there is no need for any special code.
        self.t / self.u
    }

    /// Test whether a field element can be decoded into a point.
    /// returns `true` if decoding would work, `false` otherwise.
    pub fn validate(w: GFp5) -> bool {
        // Value w can be decoded if and only if it is zero, or
        // (w^2 - a)^2 - 4*b is a quadratic residue.
        let e = w.square() - Self::A;
        let delta = e.square() - Self::B_MUL4;
        w == GFp5::ZERO || delta.legendre() == GFp::ONE
    }

    /// Attempt to decode a point from a field element
    pub fn decode(w: GFp5) -> Option<Self> {
        // Curve equation is y^2 = x*(x^2 + a*x + b); encoded value
        // is w = y/x. Dividing by x, we get the equation:
        //   x^2 - (w^2 - a)*x + b = 0
        // We solve for x and keep the solution which is not itself a
        // square (if there are solutions, exactly one of them will be
        // a square, and the other will not be a square).

        let e = w.square() - Self::A;
        let delta = e.square() - Self::B_MUL4;
        let r = delta.sqrt();
        let c = r.is_some();
        let r = r.unwrap_or(GFp5::ZERO);

        let x1 = (e + r).half();
        let x2 = (e - r).half();
        let x = if x1.legendre() == GFp::ONE { x1 } else { x2 };

        // If c == 0, then we want to get the neutral here; note that if
        // w == 0, then delta = a^2 - 4*b, which is not a square, and
        // thus we also get c == 0.
        let x = if c { x } else { GFp5::ZERO };
        let z = GFp5::ONE;
        let u = if c { GFp5::ONE } else { GFp5::ZERO };
        let t = if c { w } else { GFp5::ONE };

        // If w == 0 then this is in fact a success.
        if c || w == GFp5::ZERO {
            Some(Self { x, z, u, t })
        } else {
            None
        }
    }

    // General point addition. GFpormulas are complete (no special case).
    fn set_add(&mut self, rhs: &Self) {
        // cost: 10M
        let (x1, z1, u1, t1) = (self.x, self.z, self.u, self.t);
        let (x2, z2, u2, t2) = (rhs.x, rhs.z, rhs.u, rhs.t);

        let t1 = x1 * x2;
        let t2 = z1 * z2;
        let t3 = u1 * u2;
        let t4 = t1 * t2;
        let t5 = (x1 + z1) * (x2 + z2) - t1 - t2;
        let t6 = (u1 + t1) * (u2 + t2) - t3 - t4;
        let t7 = t1 + t2.mul_small_k1(Self::B1_U32);
        let t8 = t4 * t7;
        let t9 = t3 * (t5.mul_small_k1(2 * Self::B1_U32) + t7.double());
        let t10 = (t4 + t3.double()) * (t5 + t7);

        self.x = (t10 - t8).mul_small_k1(Self::B1_U32);
        self.z = t8 - t9;
        self.u = t6 * (t2.mul_small_k1(Self::B1_U32) - t1);
        self.t = t8 + t9;
    }

    // Add a point in affine coordinates to this one.
    fn set_add_affine(&mut self, rhs: &AffinePoint) {
        // cost: 8M
        let (x1, z1, u1, t1) = (self.x, self.z, self.u, self.t);
        let (x2, u2) = (rhs.x, rhs.u);

        let t1 = x1 * x2;
        let t2 = z1;
        let t3 = u1 * u2;
        let t4 = t1;
        let t5 = x1 + x2 * z1;
        let t6 = u1 + u2 * t1;
        let t7 = t1 + t2.mul_small_k1(Self::B1_U32);
        let t8 = t4 * t7;
        let t9 = t3 * (t5.mul_small_k1(2 * Self::B1_U32) + t7.double());
        let t10 = (t4 + t3.double()) * (t5 + t7);

        self.x = (t10 - t8).mul_small_k1(Self::B1_U32);
        self.u = t6 * (t2.mul_small_k1(Self::B1_U32) - t1);
        self.z = t8 - t9;
        self.t = t8 + t9;
    }

    // Subtract a point in affine coordinates from this one.
    fn set_sub_affine(&mut self, rhs: &AffinePoint) {
        self.set_add_affine(&AffinePoint {
            x: rhs.x,
            u: -rhs.u,
        })
    }

    fn set_neg(&mut self) {
        self.u = -self.u;
    }

    fn set_sub(&mut self, rhs: &Self) {
        self.set_add(&rhs.neg())
    }

    /// Specialized point doubling function (faster than using general
    /// addition on the point and itself).
    pub fn double(self) -> Self {
        let mut r = self;
        r.set_double();
        r
    }

    fn set_double(&mut self) {
        // cost: 4M+5S
        let (x, z, u, t) = (self.x, self.z, self.u, self.t);

        let t1 = z * t;
        let t2 = t1 * t;
        let x1 = t2.square();
        let z1 = t1 * u;
        let t3 = u.square();
        let w1 = t2 - (x + z).double() * t3;
        let t4 = z1.square();

        self.x = t4.mul_small_k1(4 * Self::B1_U32);
        self.z = w1.square();
        self.u = (w1 + z1).square() - t4 - self.z;
        self.t = x1.double() - t4 * GFp5::from_canonical_u64(4) - self.z;
    }

    /// Multiply this point by 2^n (i.e. n successive doublings). This is
    /// faster than calling the double() function n times.
    pub fn mdouble(self, n: u32) -> Self {
        let mut r = self;
        r.set_mdouble(n);
        r
    }

    fn set_mdouble(&mut self, n: u32) {
        // Handle corner cases (0 or 1 double).
        if n == 0 {
            return;
        }
        if n == 1 {
            self.set_double();
            return;
        }

        // cost: n*(2M+5S) + 2M+1S
        let (x0, z0, u0, t0) = (self.x, self.z, self.u, self.t);
        let mut t1 = z0 * t0;
        let mut t2 = t1 * t0;
        let x1 = t2.square();
        let z1 = t1 * u0;
        let mut t3 = u0.square();
        let mut w1 = t2 - (x0 + z0).double() * t3;
        let mut t4 = w1.square();
        let mut t5 = z1.square();
        let mut x = t5.square().mul_small_k1(16 * Self::B1_U32);
        let mut w = x1.double() - t5.mul_small(4) - t4;
        let mut z = (w1 + z1).square() - t4 - t5;

        for _ in 2..n {
            t1 = z.square();
            t2 = t1.square();
            t3 = w.square();
            t4 = t3.square();
            t5 = (w + z).square() - t1 - t3;
            z = t5 * ((x + t1).double() - t3);
            x = (t2 * t4).mul_small_k1(16 * Self::B1_U32);
            w = -t4 - t2.mul_small_kn01(4, 4 * Self::B1_U32);
        }

        t1 = w.square();
        t2 = z.square();
        t3 = (w + z).square() - t1 - t2;
        w1 = t1 - (x + t2).double();
        self.x = t3.square().mul_small_k1(Self::B1_U32);
        self.z = w1.square();
        self.u = t3 * w1;
        self.t = t1.double() * (t1 - t2.double()) - self.z;
    }

    /// Return `true` if this point is the neutral, `false` otherwise.
    pub fn is_neutral(self) -> bool {
        self.u == GFp5::ZERO
    }

    /// Compare this point with another
    /// return `true` if they're equal`, `false` otherwise
    pub fn equals(self, rhs: Self) -> bool {
        self.u * rhs.t == rhs.u * self.t
    }

    // Convert points to affine coordinates.
    fn batch_to_affine(src: &[Self]) -> Vec<AffinePoint> {
        // We use a trick due to Montgomery: to compute the inverse of
        // x and of y, a single inversion suffices, with:
        //    1/x = y*(1/(x*y))
        //    1/y = x*(1/(x*y))
        // This extends to the case of inverting n values, with a total
        // cost of 1 inversion and 3*(n-1) multiplications.
        match src.len() {
            0 => Vec::new(),
            1 => {
                let p = src[0];
                let m1 = (p.z * p.t).try_inverse().unwrap_or(GFp5::ZERO);
                let res = AffinePoint {
                    x: p.x * p.t * m1,
                    u: p.u * p.z * m1,
                };

                vec![res]
            }
            n => {
                let mut res = Vec::with_capacity(n);
                // Compute product of all values to invert, and invert it.
                // We also use the x and u coordinates of the points in the
                // destination slice to keep track of the partial products.
                let mut m = src[0].z * src[0].t;
                for i in 1..n {
                    let x = m;
                    m *= src[i].z;
                    let u = m;
                    m *= src[i].t;

                    res.push(AffinePoint { x, u })
                }

                m = m.try_inverse().unwrap_or(GFp5::ZERO);

                // Propagate back inverses.
                for i in (1..n).rev() {
                    res[i].u = src[i].u * res[i].u * m;
                    m *= src[i].t;
                    res[i].x = src[i].x * res[i].x * m;
                    m *= src[i].z;
                }
                res[0].u = src[0].u * src[0].z * m;
                m *= src[0].t;
                res[0].x = src[0].x * m;

                res
            }
        }
    }

    // Optimal window size should be 4 or 5 bits, depending on target
    // architecture. On an Intel i5-8259U ("Coffee Lake" core), a 5-bit
    // window seems very slightly better.
    const WINDOW: usize = 5;
    const WIN_SIZE: usize = 1 << ((Self::WINDOW - 1) as i32);

    fn make_window_affine(self) -> Vec<AffinePoint> {
        let mut tmp = [Self::NEUTRAL; Self::WIN_SIZE];
        tmp[0] = self;
        for i in 1..Self::WIN_SIZE {
            if (i & 1) == 0 {
                tmp[i] = self.add(tmp[i - 1]);
            } else {
                tmp[i] = tmp[i >> 1].double();
            }
        }

        let win = Self::batch_to_affine(&tmp);
        win
    }

    // Multiply this point by a scalar.
    fn set_mul(&mut self, s: &Scalar) {
        // Make a window with affine points.
        let win = self.make_window_affine();
        let mut digits = [0; (319 + Self::WINDOW) / Self::WINDOW];
        s.recode_signed(&mut digits, Self::WINDOW as i32);

        *self = AffinePoint::lookup(&win, digits[digits.len() - 1]).to_point();

        for &digit in digits.iter().rev() {
            self.set_mdouble(Self::WINDOW as u32);
            *self += AffinePoint::lookup(&win, digit);
        }
    }

    /// Multiply the conventional generator by a scalar.
    /// This function is faster than using the multiplication operator
    /// on the generator point.
    pub fn mulgen(s: Scalar) -> Self {
        // Precomputed tables are for j*(2^(80*i))*G, for i = 0 to 3
        // and j = 1 to 16, i.e. 5-bit windows.
        let mut digits = [0; 64];
        s.recode_signed(&mut digits, 5);

        let mut p = AffinePoint::lookup(&G0, digits[7]).to_point();
        p += AffinePoint::lookup(&G40, digits[15]);
        p += AffinePoint::lookup(&G80, digits[23]);
        p += AffinePoint::lookup(&G120, digits[31]);
        p += AffinePoint::lookup(&G160, digits[39]);
        p += AffinePoint::lookup(&G200, digits[47]);
        p += AffinePoint::lookup(&G240, digits[55]);
        p += AffinePoint::lookup(&G280, digits[63]);
        for i in (0..7).rev() {
            p.set_mdouble(5);
            p += AffinePoint::lookup(&G0, digits[i]);
            p += AffinePoint::lookup(&G40, digits[i + 8]);
            p += AffinePoint::lookup(&G80, digits[i + 16]);
            p += AffinePoint::lookup(&G120, digits[i + 24]);
            p += AffinePoint::lookup(&G160, digits[i + 32]);
            p += AffinePoint::lookup(&G200, digits[i + 40]);
            p += AffinePoint::lookup(&G240, digits[i + 48]);
            p += AffinePoint::lookup(&G280, digits[i + 56]);
        }
        p
    }

    fn make_window_5(self) -> [Self; 16] {
        let mut win = [Self::NEUTRAL; 16];
        win[0] = self;
        for i in 1..win.len() {
            if (i & 1) == 0 {
                win[i] = self.add(win[i - 1]);
            } else {
                win[i] = win[i >> 1].double();
            }
        }
        win
    }

    fn lookup_vartime(win: &[Self], k: i32) -> Self {
        if k > 0 {
            return win[(k - 1) as usize];
        } else if k == 0 {
            return Self::NEUTRAL;
        } else {
            return -win[(-k - 1) as usize];
        }
    }

    /// Given scalars s and k, and point R, verify whether s*G + k*Q = R
    /// (with G being the curve conventional generator, and Q this instance).
    /// This is the main operation in Schnorr signature verification.
    /// WARNING: this function is not constant-time; use only on
    /// public data.
    pub fn verify_muladd_vartime(self, s: Scalar, k: Scalar, r: Self) -> bool {
        // We use a method by Antipa et al (SAC 2005), following the
        // description in: https://eprint.iacr.org/2020/454
        // We split k into two (signed) integers c0 and c1 such
        // that k = c0/c1 mod n; the integers c0 and c1 fit on 161 bits
        // each (including the signed bit). The verification is then:
        //    (s*c1)*G + c0*Q - c1*R = 0
        // We split s*c1 into two 160-bit halves, and use the precomputed
        // tables for G; thus, all scalars fit on 160 bits (+sign).
        //
        // Since formulas for multiple doublings favour long runs of
        // doublings, we do not use a wNAF representation; instead, we
        // make regular 5-bit (signed) windows.
        //
        // We use fractional coordinates for the Q and R windows; it is
        // not worth it converting them to affine.

        // Compute c0 and c1.
        let (c0, c1) = k.lagrange();

        // Compute t <- s*c1.
        let t = s * c1.to_scalar_vartime();

        // Recode multipliers.
        let mut tt = [0i32; 64];
        t.recode_signed(&mut tt, 5);
        let tt0 = &tt[..32];
        let tt1 = &tt[32..];
        let ss0 = c0.recode_signed_5();
        let ss1 = c1.recode_signed_5();

        // Make windows for this point (Q) and for -R.
        let winQ = self.make_window_5();
        let winR = (-r).make_window_5();

        let mut p = Self::lookup_vartime(&winQ, ss0[32]);
        if ss1[32] != 0 {
            p += Self::lookup_vartime(&winR, ss1[32]);
        }
        for i in (0..32).rev() {
            p.set_mdouble(5);
            if tt0[i] != 0 {
                p += AffinePoint::lookup_vartime(&G0, tt0[i]);
            }
            if tt1[i] != 0 {
                p += AffinePoint::lookup_vartime(&G160, tt1[i]);
            }
            if ss0[i] != 0 {
                p += Self::lookup_vartime(&winQ, ss0[i]);
            }
            if ss1[i] != 0 {
                p += Self::lookup_vartime(&winR, ss1[i]);
            }
        }

        !p.is_neutral()
    }
}

impl AffinePoint {
    const NEUTRAL: Self = Self {
        x: GFp5::ZERO,
        u: GFp5::ZERO,
    };

    fn to_point(self) -> Point {
        let Self { x, u } = self;
        Point {
            x,
            z: GFp5::ONE,
            u,
            t: GFp5::ONE,
        }
    }

    fn set_neg(&mut self) {
        self.u = -self.u;
    }

    // Lookup a point in a window. The win[] slice must contain values
    // i*P for i = 1 to n (win[0] contains P, win[1] contains 2*P, and
    // so on). Index value k is an integer in the -n to n range; returned
    // point is k*P.
    fn set_lookup(&mut self, win: &[Self], k: i32) {
        // sign = 0xFFFFFFFF if k < 0, 0x00000000 otherwise
        let sign = (k >> 31) as u32;
        // ka = abs(k)
        let ka = ((k as u32) ^ sign).wrapping_sub(sign);
        // km1 = ka - 1
        let km1 = ka.wrapping_sub(1);

        let mut x = GFp5::ZERO;
        let mut u = GFp5::ZERO;
        for i in 0..win.len() {
            let m = km1.wrapping_sub(i as u32);
            let c = (((m | m.wrapping_neg()) >> 31) as u64).wrapping_sub(1);
            x = if c == 0 { x } else { win[i].x };
            u = if c == 0 { u } else { win[i].u };
        }

        // If k < 0, then we must negate the point.
        let c = (sign as u64) | ((sign as u64) << 32);
        self.x = x;

        if c != 0 {
            self.u = -u
        }
    }

    fn lookup(win: &[Self], k: i32) -> Self {
        let mut r = Self::NEUTRAL;
        r.set_lookup(win, k);
        r
    }

    // Same as lookup(), except this implementation is variable-time.
    fn lookup_vartime(win: &[Self], k: i32) -> Self {
        if k > 0 {
            return win[(k - 1) as usize];
        } else if k == 0 {
            return Self::NEUTRAL;
        } else {
            return -win[(-k - 1) as usize];
        }
    }
}

// We implement all the needed traits to allow use of the arithmetic
// operators on points. We support all combinations of operands
// either as Point structures, or pointers to Point structures. Some
// operations with AffinePoint structures are also implemented.

impl Add<Point> for Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: Point) -> Point {
        let mut r = self;
        r.set_add(&other);
        r
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: &Point) -> Point {
        let mut r = self;
        r.set_add(other);
        r
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: Point) -> Point {
        let mut r = *self;
        r.set_add(&other);
        r
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: &Point) -> Point {
        let mut r = *self;
        r.set_add(other);
        r
    }
}

impl Add<AffinePoint> for Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: AffinePoint) -> Point {
        let mut r = self;
        r.set_add_affine(&other);
        r
    }
}

impl Add<&AffinePoint> for Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: &AffinePoint) -> Point {
        let mut r = self;
        r.set_add_affine(other);
        r
    }
}

impl Add<AffinePoint> for &Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: AffinePoint) -> Point {
        let mut r = *self;
        r.set_add_affine(&other);
        r
    }
}

impl Add<&AffinePoint> for &Point {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: &AffinePoint) -> Point {
        let mut r = *self;
        r.set_add_affine(other);
        r
    }
}

impl Add<Point> for AffinePoint {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: Point) -> Point {
        let mut r = other;
        r.set_add_affine(&self);
        r
    }
}

impl Add<&Point> for AffinePoint {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: &Point) -> Point {
        let mut r = *other;
        r.set_add_affine(&self);
        r
    }
}

impl Add<Point> for &AffinePoint {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: Point) -> Point {
        let mut r = other;
        r.set_add_affine(self);
        r
    }
}

impl Add<&Point> for &AffinePoint {
    type Output = Point;

    #[inline(always)]
    fn add(self, other: &Point) -> Point {
        let mut r = *other;
        r.set_add_affine(self);
        r
    }
}

impl AddAssign<Point> for Point {
    #[inline(always)]
    fn add_assign(&mut self, other: Point) {
        self.set_add(&other);
    }
}

impl AddAssign<&Point> for Point {
    #[inline(always)]
    fn add_assign(&mut self, other: &Point) {
        self.set_add(other);
    }
}

impl AddAssign<AffinePoint> for Point {
    #[inline(always)]
    fn add_assign(&mut self, other: AffinePoint) {
        self.set_add_affine(&other);
    }
}

impl AddAssign<&AffinePoint> for Point {
    #[inline(always)]
    fn add_assign(&mut self, other: &AffinePoint) {
        self.set_add_affine(other);
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: Point) -> Point {
        let mut r = self;
        r.set_sub(&other);
        r
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: &Point) -> Point {
        let mut r = self;
        r.set_sub(other);
        r
    }
}

impl Sub<Point> for &Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: Point) -> Point {
        let mut r = *self;
        r.set_sub(&other);
        r
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: &Point) -> Point {
        let mut r = *self;
        r.set_sub(other);
        r
    }
}

impl Sub<AffinePoint> for Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: AffinePoint) -> Point {
        let mut r = self;
        r.set_sub_affine(&other);
        r
    }
}

impl Sub<&AffinePoint> for Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: &AffinePoint) -> Point {
        let mut r = self;
        r.set_sub_affine(other);
        r
    }
}

impl Sub<AffinePoint> for &Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: AffinePoint) -> Point {
        let mut r = *self;
        r.set_sub_affine(&other);
        r
    }
}

impl Sub<&AffinePoint> for &Point {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: &AffinePoint) -> Point {
        let mut r = *self;
        r.set_sub_affine(other);
        r
    }
}

impl Sub<Point> for AffinePoint {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: Point) -> Point {
        let mut r = other;
        r.set_sub_affine(&self);
        r
    }
}

impl Sub<&Point> for AffinePoint {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: &Point) -> Point {
        let mut r = *other;
        r.set_sub_affine(&self);
        r
    }
}

impl Sub<Point> for &AffinePoint {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: Point) -> Point {
        let mut r = other;
        r.set_sub_affine(self);
        r
    }
}

impl Sub<&Point> for &AffinePoint {
    type Output = Point;

    #[inline(always)]
    fn sub(self, other: &Point) -> Point {
        let mut r = *other;
        r.set_sub_affine(self);
        r
    }
}

impl SubAssign<Point> for Point {
    #[inline(always)]
    fn sub_assign(&mut self, other: Point) {
        self.set_sub(&other);
    }
}

impl SubAssign<&Point> for Point {
    #[inline(always)]
    fn sub_assign(&mut self, other: &Point) {
        self.set_sub(other);
    }
}

impl SubAssign<AffinePoint> for Point {
    #[inline(always)]
    fn sub_assign(&mut self, other: AffinePoint) {
        self.set_sub_affine(&other);
    }
}

impl SubAssign<&AffinePoint> for Point {
    #[inline(always)]
    fn sub_assign(&mut self, other: &AffinePoint) {
        self.set_sub_affine(other);
    }
}

impl Neg for Point {
    type Output = Point;

    #[inline(always)]
    fn neg(self) -> Point {
        let mut r = self;
        r.set_neg();
        r
    }
}

impl Neg for &Point {
    type Output = Point;

    #[inline(always)]
    fn neg(self) -> Point {
        let mut r = *self;
        r.set_neg();
        r
    }
}

impl Neg for AffinePoint {
    type Output = AffinePoint;

    #[inline(always)]
    fn neg(self) -> AffinePoint {
        let mut r = self;
        r.set_neg();
        r
    }
}

impl Neg for &AffinePoint {
    type Output = AffinePoint;

    #[inline(always)]
    fn neg(self) -> AffinePoint {
        let mut r = *self;
        r.set_neg();
        r
    }
}

impl Mul<Scalar> for Point {
    type Output = Point;

    #[inline(always)]
    fn mul(self, other: Scalar) -> Point {
        let mut r = self;
        r.set_mul(&other);
        r
    }
}

impl Mul<&Scalar> for Point {
    type Output = Point;

    #[inline(always)]
    fn mul(self, other: &Scalar) -> Point {
        let mut r = self;
        r.set_mul(other);
        r
    }
}

impl Mul<Scalar> for &Point {
    type Output = Point;

    #[inline(always)]
    fn mul(self, other: Scalar) -> Point {
        let mut r = *self;
        r.set_mul(&other);
        r
    }
}

impl Mul<&Scalar> for &Point {
    type Output = Point;

    #[inline(always)]
    fn mul(self, other: &Scalar) -> Point {
        let mut r = *self;
        r.set_mul(other);
        r
    }
}

impl MulAssign<Scalar> for Point {
    #[inline(always)]
    fn mul_assign(&mut self, other: Scalar) {
        self.set_mul(&other);
    }
}

impl MulAssign<&Scalar> for Point {
    #[inline(always)]
    fn mul_assign(&mut self, other: &Scalar) {
        self.set_mul(other);
    }
}

impl Mul<Point> for Scalar {
    type Output = Point;

    #[inline(always)]
    fn mul(self, other: Point) -> Point {
        let mut r = other;
        r.set_mul(&self);
        r
    }
}

impl Mul<&Point> for Scalar {
    type Output = Point;

    #[inline(always)]
    fn mul(self, other: &Point) -> Point {
        let mut r = *other;
        r.set_mul(&self);
        r
    }
}

impl Mul<Point> for &Scalar {
    type Output = Point;

    #[inline(always)]
    fn mul(self, other: Point) -> Point {
        let mut r = other;
        r.set_mul(self);
        r
    }
}

impl Mul<&Point> for &Scalar {
    type Output = Point;

    #[inline(always)]
    fn mul(self, other: &Point) -> Point {
        let mut r = *other;
        r.set_mul(self);
        r
    }
}
