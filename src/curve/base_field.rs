use plonky2::hash::hash_types::RichField;
use plonky2_field::{
    extension::{quintic::QuinticExtension, Extendable, FieldExtension, Frobenius},
    ops::Square,
    types::{Field, PrimeField},
};

use super::{GFp, GFp5};

pub trait Legendre<F: Field> {
    fn legendre(&self) -> F;
}

impl Legendre<GFp> for QuinticExtension<GFp> {
    fn legendre(&self) -> GFp {
        let frob1 = self.frobenius();
        let frob2 = frob1.frobenius();

        let frob1_times_frob2 = frob1 * frob2;
        let frob2_frob1_times_frob2 = frob1_times_frob2.repeated_frobenius(2);

        let xr_ext = frob1_times_frob2 * frob2_frob1_times_frob2;
        let xr: GFp = <GFp5 as FieldExtension<5>>::to_basefield_array(&xr_ext)[0];

        let xr_31 = xr.exp_power_of_2(31);
        let xr_63 = xr_31.exp_power_of_2(32);
        xr_63 / xr_31
    }
}

pub trait SquareRoot: Sized {
    fn sqrt(&self) -> Option<Self>;
    fn canonical_sqrt(&self) -> Option<Self>;
}

impl SquareRoot for QuinticExtension<GFp> {
    fn sqrt(&self) -> Option<Self> {
        sqrt_quintic_ext_goldilocks(*self)
    }

    fn canonical_sqrt(&self) -> Option<Self> {
        canonical_sqrt_quintic_ext_goldilocks(*self)
    }
}

pub trait Half {
    fn half(&self) -> Self;
}

impl Half for QuinticExtension<GFp> {
    fn half(&self) -> Self {
        let QuinticExtension([a0, a1, a2, a3, a4]) = *self;
        QuinticExtension([
            a0 / GFp::TWO,
            a1 / GFp::TWO,
            a2 / GFp::TWO,
            a3 / GFp::TWO,
            a4 / GFp::TWO,
        ])
    }
}

pub trait Sgn0 {
    fn sgn0(&self) -> bool;
}

impl Sgn0 for QuinticExtension<GFp> {
    fn sgn0(&self) -> bool {
        quintic_ext_sgn0(*self)
    }
}

pub(crate) trait MulSmall {
    // multiply by a small integer (less than 2^31).
    fn mul_small(&self, rhs: u32) -> Self;

    // multiply by a extension field element of the form xz, where `x` is less than `2^29`
    fn mul_small_k1(&self, rhs: u32) -> Self;

    // For two small integers v0 and v1 (both lower than 2^28),
    // multiply this value by a extension field element of the form `z*v1 - v0`.
    fn mul_small_kn01(&self, v0: u32, v1: u32) -> Self;
}

impl MulSmall for QuinticExtension<GFp> {
    fn mul_small(&self, rhs: u32) -> Self {
        let rhs = GFp::from_canonical_u32(rhs);
        let QuinticExtension([a0, a1, a2, a3, a4]) = *self;
        QuinticExtension([a0 * rhs, a1 * rhs, a2 * rhs, a3 * rhs, a4 * rhs])
    }

    fn mul_small_k1(&self, rhs: u32) -> Self {
        let rhs = GFp::from_canonical_u32(rhs);
        let QuinticExtension([a0, a1, a2, a3, a4]) = *self;

        let d0 = a4 * rhs * GFp::from_canonical_u32(3);
        let d1 = a0 * rhs;
        let d2 = a1 * rhs;
        let d3 = a2 * rhs;
        let d4 = a3 * rhs;

        QuinticExtension([d0, d1, d2, d3, d4])
    }

    fn mul_small_kn01(&self, v0: u32, v1: u32) -> Self {
        let v0 = GFp::from_canonical_u32(v0);
        let v1 = GFp::from_canonical_u32(v1);
        let QuinticExtension([a0, a1, a2, a3, a4]) = *self;

        let d0 = a4 * v1 * GFp::from_canonical_u32(3) - a0 * v0;
        let d1 = a0 * v1 - a1 * v0;
        let d2 = a1 * v1 - a2 * v0;
        let d3 = a2 * v1 - a3 * v0;
        let d4 = a3 * v1 - a4 * v0;

        QuinticExtension([d0, d1, d2, d3, d4])
    }
}

/// returns true or false indicating a notion of "sign" for quintic_ext.
/// This is used to canonicalize the square root
/// This is an implementation of the function sgn0 from the IRTF's hash-to-curve document
/// https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-hash-to-curve-07#name-the-sgn0-function
pub(crate) fn quintic_ext_sgn0<F: RichField + Extendable<5>>(x: QuinticExtension<F>) -> bool {
    let mut sign = false;
    let mut zero = true;
    for &limb in x.0.iter() {
        let sign_i = limb.to_canonical_u64() & 1 == 0;
        let zero_i = limb == F::ZERO;
        sign = sign || (zero && sign_i);
        zero = zero && zero_i;
    }
    return sign;
}

// returns the "canoncal" square root of x, if it exists
// the "canonical" square root is the one such that `sgn0(sqrt(x)) == true`
pub(crate) fn canonical_sqrt_quintic_ext_goldilocks(x: GFp5) -> Option<GFp5> {
    match sqrt_quintic_ext_goldilocks(x) {
        Some(root_x) => {
            if quintic_ext_sgn0(root_x) {
                Some(root_x)
            } else {
                Some(-root_x)
            }
        }
        None => None,
    }
}

/// returns `Some(sqrt(x))` if `x` is a square in the field, and `None` otherwise
/// basically copied from here: https://github.com/pornin/ecquintic_ext/blob/ce059c6d1e1662db437aecbf3db6bb67fe63c716/python/ecGFp5.py#L879
pub(crate) fn sqrt_quintic_ext_goldilocks(x: GFp5) -> Option<GFp5> {
    let v = x.exp_power_of_2(31);
    let d = x * v.exp_power_of_2(32) * v.try_inverse().unwrap_or(GFp5::ZERO);
    let e = (d * d.repeated_frobenius(2)).frobenius();
    let f = e.square();

    let [x0, x1, x2, x3, x4] = x.0;
    let [f0, f1, f2, f3, f4] = f.0;
    let g = x0 * f0 + GFp::from_canonical_u64(3) * (x1 * f4 + x2 * f3 + x3 * f2 + x4 * f1);

    g.sqrt()
        .map(|s| e.try_inverse().unwrap_or(GFp5::ZERO) * s.into())
}
