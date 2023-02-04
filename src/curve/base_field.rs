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

        let xr_ext = *self * frob1_times_frob2 * frob2_frob1_times_frob2;
        let xr: GFp = <GFp5 as FieldExtension<5>>::to_basefield_array(&xr_ext)[0];

        let xr_31 = xr.exp_power_of_2(31);
        let xr_63 = xr_31.exp_power_of_2(32);

        // only way `xr_31` can be zero is if `xr` is zero, in which case `self` is zero, in which case we want to return zero.
        let xr_31_inv_or_zero = xr_31.inverse_or_zero();
        xr_63 * xr_31_inv_or_zero
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

pub trait InverseOrZero: Sized {
    fn inverse_or_zero(&self) -> Self;
}

impl InverseOrZero for GFp {
    fn inverse_or_zero(&self) -> Self {
        self.try_inverse().unwrap_or(GFp::ZERO)
    }
}

impl InverseOrZero for GFp5 {
    fn inverse_or_zero(&self) -> Self {
        self.try_inverse().unwrap_or(GFp5::ZERO)
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
                Some(-root_x)
            } else {
                Some(root_x)
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

    g.sqrt().map(|s| e.inverse_or_zero() * s.into())
}

#[cfg(test)]
mod tests {
    use plonky2_field::types::Sample;
    use rand::thread_rng;

    use crate::curve::test_utils::gfp5_random_non_square;

    use super::*;

    #[test]
    fn test_legendre() {
        // test zero
        assert_eq!(GFp::ZERO, GFp5::ZERO.legendre());

        // test non-squares
        for _ in 0..32 {
            let x = gfp5_random_non_square();
            let legendre_sym = x.legendre();

            assert_eq!(legendre_sym, -GFp::ONE);
        }

        // test squares
        for _ in 0..32 {
            let x = GFp5::sample(&mut thread_rng());
            let square = x * x;
            let legendre_sym = square.legendre();

            assert_eq!(legendre_sym, GFp::ONE);
        }

        // test zero
        let x = GFp5::ZERO;
        let square = x * x;
        let legendre_sym = square.legendre();
        assert_eq!(legendre_sym, GFp::ZERO);
    }

    #[test]
    fn test_sqrt_quintic_ext_outside_circuit() {
        let mut rng = thread_rng();

        for _ in 0..30 {
            let x = GFp5::sample(&mut rng);
            let square = x * x;
            let sqrt = square.sqrt().unwrap();

            assert_eq!(sqrt * sqrt, square);
        }
    }

    #[test]
    fn test_canonical_sqrt_quintic_ext_outside_circuit() {
        let mut rng = thread_rng();

        for _ in 0..30 {
            let x = GFp5::sample(&mut rng);
            let square = x * x;
            let sqrt = square.canonical_sqrt().unwrap();

            assert_eq!(sqrt * sqrt, square);
            assert!(!sqrt.sgn0())
        }
    }
}
