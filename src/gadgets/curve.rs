use crate::curve::scalar_field::Scalar;
use crate::curve::{curve::AffinePointWithFlag, GFp, GFp5};
use crate::gadgets::base_field::{CircuitBuilderGFp5, QuinticExtensionTarget};
use num::{BigUint, FromPrimitive, Zero};
use plonky2::field::extension::FieldExtension;
use plonky2::field::types::{Field, Field64};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use plonky2::iop::target::Target;
use plonky2::iop::witness::Witness;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_ecdsa::gadgets::biguint::CircuitBuilderBiguint;
use plonky2_ecdsa::gadgets::nonnative::{CircuitBuilderNonNative, NonNativeTarget};
use plonky2_field::extension::Extendable;
use plonky2_field::extension::quintic::QuinticExtension;
use plonky2_field::goldilocks_field::GoldilocksField;

use super::base_field::PartialWitnessQuinticExt;

pub fn scalar_field_order() -> BigUint {
    let mut res = BigUint::from_u128(25 * 5 * 163 * 769 * 1059871).unwrap();
    res *= BigUint::from_u128(253243826720162431254857814100127).unwrap();

    let big_factor_limbs = [
        198400523, 053184002, 814403536, 918162724, 916343842, 520561,
    ];
    let big_factor =
        big_factor_limbs
            .into_iter()
            .rev()
            .enumerate()
            .fold(BigUint::zero(), |acc, (i, limb)| {
                (BigUint::from_u64(100_000_000).unwrap() * acc) + BigUint::from_i32(limb).unwrap()
            });
    res * big_factor
}

const THREE: GFp5 = QuinticExtension([
    GoldilocksField(3),
    GFp::ZERO,
    GFp::ZERO,
    GFp::ZERO,
    GFp::ZERO,
]);

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct CurveTarget(([QuinticExtensionTarget; 2], BoolTarget));

pub trait CircuitBuilderEcGFp5 {
    fn add_virtual_curve_target(&mut self) -> CurveTarget;
    fn register_curve_public_input(&mut self, point: CurveTarget);
    fn curve_constant(&mut self, point: AffinePointWithFlag) -> CurveTarget;
    fn curve_zero(&mut self) -> CurveTarget;
    fn curve_generator(&mut self) -> CurveTarget;
    fn curve_select(&mut self, cond: BoolTarget, a: CurveTarget, b: CurveTarget) -> CurveTarget;
    fn curve_random_access(&mut self, access_index: Target, v: &[CurveTarget]) -> CurveTarget;

    fn curve_add(&mut self, a: CurveTarget, b: CurveTarget) -> CurveTarget;
    fn curve_double(&mut self, a: CurveTarget) -> CurveTarget;
    fn curve_scalar_mul(&mut self, a: CurveTarget, scalar: NonNativeTarget<Scalar>) -> CurveTarget;

    fn curve_encode_to_quintic_ext(&mut self, a: CurveTarget) -> QuinticExtensionTarget;
    fn curve_decode_from_quintic_ext(&mut self, w: QuinticExtensionTarget) -> CurveTarget;
}

macro_rules! impl_circuit_builder_for_extension_degree {
    ($degree:literal) => {
        impl CircuitBuilderEcGFp5 for CircuitBuilder<GFp, $degree> {
            fn add_virtual_curve_target(&mut self) -> CurveTarget {
                let x = self.add_virtual_quintic_ext_target();
                let y = self.add_virtual_quintic_ext_target();
                let is_inf = self.add_virtual_bool_target_safe();
                CurveTarget(([x, y], is_inf))
            }

            fn register_curve_public_input(&mut self, point: CurveTarget) {
                let CurveTarget(([x, y], is_inf)) = point;
                self.register_quintic_ext_public_input(x);
                self.register_quintic_ext_public_input(y);
                self.register_public_input(is_inf.target);
            }

            fn curve_constant(&mut self, point: AffinePointWithFlag) -> CurveTarget {
                let AffinePointWithFlag { x, y, is_inf } = point;

                let x = self.constant_quintic_ext(x);
                let y = self.constant_quintic_ext(y);
                let is_inf = self.constant_bool(is_inf);
                CurveTarget(([x, y], is_inf))
            }

            fn curve_zero(&mut self) -> CurveTarget {
                self.curve_constant(AffinePointWithFlag::NEUTRAL)
            }

            fn curve_generator(&mut self) -> CurveTarget {
                self.curve_constant(AffinePointWithFlag::GENERATOR)
            }

            fn curve_select(
                &mut self,
                cond: BoolTarget,
                a: CurveTarget,
                b: CurveTarget,
            ) -> CurveTarget {
                let CurveTarget(([ax, ay], a_is_inf)) = a;
                let CurveTarget(([bx, by], b_is_inf)) = b;
                CurveTarget((
                    [
                        self.select_quintic_ext(cond, ax, bx),
                        self.select_quintic_ext(cond, ay, by),
                    ],
                    BoolTarget::new_unsafe(self.select(cond, a_is_inf.target, b_is_inf.target)),
                ))
            }

            fn curve_random_access(
                &mut self,
                access_index: Target,
                v: &[CurveTarget],
            ) -> CurveTarget {
                let mut xs = Vec::new();
                let mut ys = Vec::new();
                let mut is_infs = Vec::new();
                for &CurveTarget(([x, y], is_inf)) in v {
                    xs.push(x);
                    ys.push(y);
                    is_infs.push(is_inf.target);
                }

                CurveTarget((
                    [
                        self.random_access_quintic_ext(access_index, &xs),
                        self.random_access_quintic_ext(access_index, &ys),
                    ],
                    BoolTarget::new_unsafe(self.random_access(access_index, is_infs)),
                ))
            }

            fn curve_add(&mut self, a: CurveTarget, b: CurveTarget) -> CurveTarget {
                let CurveTarget(([x1, y1], a_is_inf)) = a;
                let CurveTarget(([x2, y2], b_is_inf)) = b;

                // note: paper has a typo. sx == 1 when x1 != x2, not when x1 == x2
                let mut sx = self.is_equal_quintic_ext(x1, x2);
                sx = self.not(sx);

                let sy = self.is_equal_quintic_ext(y1, y2);

                let lambda_0_if_sx_1 = self.sub_quintic_ext(y2, y1);
                let mut lambda_0_if_sx_0 = self.square_quintic_ext(x1);
                lambda_0_if_sx_0 = self.mul_const_quintic_ext(THREE, lambda_0_if_sx_0);
                lambda_0_if_sx_0 =
                    self.add_const_quintic_ext(lambda_0_if_sx_0, AffinePointWithFlag::A);

                let lambda_1_if_sx_1 = self.sub_quintic_ext(x2, x1);
                let lambda_1_if_sx_0 = self.mul_const_quintic_ext(GFp5::TWO, y1);

                let lambda_0 = self.select_quintic_ext(sx, lambda_0_if_sx_1, lambda_0_if_sx_0);
                let lambda_1 = self.select_quintic_ext(sx, lambda_1_if_sx_1, lambda_1_if_sx_0);
                let lambda = self.div_quintic_ext(lambda_0, lambda_1);

                let mut x3 = self.square_quintic_ext(lambda);
                x3 = self.sub_quintic_ext(x3, x1);
                x3 = self.sub_quintic_ext(x3, x2);

                let mut y3 = self.sub_quintic_ext(x1, x3);
                y3 = self.mul_quintic_ext(lambda, y3);
                y3 = self.sub_quintic_ext(y3, y1);

                let c_is_inf = self.and(sx, sy);
                let c = CurveTarget(([x3, y3], c_is_inf));

                let sel = self.curve_select(a_is_inf, b, c);
                self.curve_select(b_is_inf, a, sel)
            }

            fn curve_double(&mut self, a: CurveTarget) -> CurveTarget {
                let CurveTarget(([x, y], is_inf)) = a;

                let mut lambda_0 = self.square_quintic_ext(x);
                lambda_0 = self.mul_const_quintic_ext(THREE, lambda_0);
                lambda_0 = self.add_const_quintic_ext(lambda_0, AffinePointWithFlag::A);
                let lambda_1 = self.mul_const_quintic_ext(GFp5::TWO, y);

                let lambda = self.div_quintic_ext(lambda_0, lambda_1);

                let mut x2 = self.square_quintic_ext(lambda);
                let two_x = self.mul_const_quintic_ext(GFp5::TWO, x);
                x2 = self.sub_quintic_ext(x2, two_x);

                let mut y2 = self.sub_quintic_ext(x, x2);
                y2 = self.mul_quintic_ext(lambda, y2);
                y2 = self.sub_quintic_ext(y2, y);

                CurveTarget(([x2, y2], is_inf))
            }

            /// a: the point to multiply by
            /// b: little-endian bit representation of the scalar (i.e. least-significant first)
            fn curve_scalar_mul(
                &mut self,
                a: CurveTarget,
                scalar: NonNativeTarget<Scalar>,
            ) -> CurveTarget {
                const WINDOW_BITS: usize = 4;
                let zero = self.zero();
                let one = self.one();
                let n = self.constant_biguint(&scalar_field_order());
                let max_digit = self.constant(GFp::from_canonical_u64(1 << WINDOW_BITS));

                let scalar_bigint = self.nonnative_to_canonical_biguint(&scalar);
                let reduced_scalar = self.rem_biguint(&scalar_bigint, &n);
                let scalar = self.add_biguint(&reduced_scalar, &n);
                let scalar_bits = scalar
                    .limbs
                    .into_iter()
                    .flat_map(|limb| self.split_le(limb.0, 32))
                    .collect::<Vec<_>>();

                let mut signs = Vec::new();
                let mut digits = Vec::new();
                let mut carry = self.constant_bool(false);
                for chunk in scalar_bits.chunks_exact(WINDOW_BITS) {
                    let terms = (0..WINDOW_BITS - 1)
                        .map(|i| self.mul_const(GFp::from_canonical_u32(1 << i), chunk[i].target))
                        .collect::<Vec<_>>();
                    let lower_bits = self.add_many(terms);

                    let mut d_if_lte_8 = self.mul_const_add(
                        GFp::from_canonical_u32(1 << (WINDOW_BITS - 1)),
                        chunk[WINDOW_BITS - 1].target,
                        lower_bits,
                    );
                    d_if_lte_8 = self.add(d_if_lte_8, carry.target);

                    let mut d_if_gt_8 = self.sub(d_if_lte_8, max_digit);
                    d_if_gt_8 = self.neg(d_if_gt_8);

                    let lower_bits_zero = self.is_equal(lower_bits, zero);
                    let lower_bits_not_zero =
                        BoolTarget::new_unsafe(self.sub(one, lower_bits_zero.target));

                    // chunk on its own > 8
                    let chunk_gt_8 = self.and(chunk[WINDOW_BITS - 1], lower_bits_not_zero);
                    // chunk plus carry > 8
                    let chunk_plus_carry_gt_8 = self.and(chunk[WINDOW_BITS - 1], carry);

                    // chunk_gt_8 OR chunk_plus_carry_gt_8
                    let gt_8 = self.mul(chunk_gt_8.target, chunk_plus_carry_gt_8.target);
                    let gt_8 = self.mul_const(-GFp::ONE, gt_8);
                    let gt_8 = BoolTarget::new_unsafe(self.add_many([
                        gt_8,
                        chunk_gt_8.target,
                        chunk_plus_carry_gt_8.target,
                    ]));

                    carry = BoolTarget::new_unsafe(self.select(gt_8, one, zero));
                    digits.push(self.select(gt_8, d_if_gt_8, d_if_lte_8));
                    signs.push(gt_8);
                }

                let mut window = vec![a];
                for _ in 0..((1 << (WINDOW_BITS - 1)) - 1) {
                    let prev = window.last().unwrap();
                    window.push(self.curve_add(*prev, a));
                }

                let mut q = self.curve_zero();
                for (sign, magnitude) in signs.into_iter().zip(digits) {
                    for _ in 0..WINDOW_BITS {
                        q = self.curve_double(q);
                    }

                    let CurveTarget(([x, mut y], is_inf)) =
                        self.curve_random_access(magnitude, &window);
                    let neg_y = self.neg_quintic_ext(y);
                    y = self.select_quintic_ext(sign, neg_y, y);
                    q = self.curve_add(q, CurveTarget(([x, y], is_inf)));
                }

                q
            }

            // TODO: optimize to use base field when we know it's in the base field
            fn curve_encode_to_quintic_ext(&mut self, a: CurveTarget) -> QuinticExtensionTarget {
                let CurveTarget(([x, y], is_inf)) = a;
                let adiv3 = self.constant_quintic_ext(GFp5::TWO / GFp5::from_canonical_u16(3));
                let denom = self.sub_quintic_ext(adiv3, x);
                let w = self.div_quintic_ext(y, denom);

                let zero = self.zero_quintic_ext();
                self.select_quintic_ext(is_inf, zero, w)
            }

            // TODO: optimize to use base field when we know it's in the base field
            fn curve_decode_from_quintic_ext(&mut self, w: QuinticExtensionTarget) -> CurveTarget {
                let one = self.one();
                let zero_quintic_ext = self.zero_quintic_ext();
                let a = self.constant_quintic_ext(GFp5::TWO);
                let b = self.constant_quintic_ext(GFp5::from_basefield_array([
                    GFp::ZERO,
                    GFp::from_canonical_u32(263),
                    GFp::ZERO,
                    GFp::ZERO,
                    GFp::ZERO,
                ]));
                let bmul4 = self.mul_const_quintic_ext(GFp5::from_canonical_u32(4), b);

                let mut e = self.square_quintic_ext(w);
                e = self.sub_quintic_ext(e, a);

                let mut delta = self.square_quintic_ext(e);
                delta = self.sub_quintic_ext(delta, bmul4);

                let (r, is_sqrt) = self.try_canonical_sqrt_quintic_ext(delta);

                let mut x1 = self.add_quintic_ext(e, r);
                x1 = self.div_const_quintic_ext(x1, GFp5::TWO);

                let mut x2 = self.sub_quintic_ext(e, r);
                x2 = self.div_const_quintic_ext(x2, GFp5::TWO);

                let legendre_x1 = self.legendre_sym_quintic_ext(x1);
                let legendre_is_one = self.is_equal(legendre_x1, one);
                let x = self.select_quintic_ext(legendre_is_one, x2, x1);

                let negw = self.neg_quintic_ext(w);
                let y = self.mul_quintic_ext(negw, x);

                // is infinity if w is zero or delta is not a square
                let w_is_zero = self.is_equal_quintic_ext(w, zero_quintic_ext);
                let both = self.mul(w_is_zero.target, is_sqrt.target);
                let mut is_inf = self.add(w_is_zero.target, is_sqrt.target);
                is_inf = self.sub(is_inf, both);

                let x = self.add_const_quintic_ext(x, GFp5::TWO / GFp5::from_canonical_u16(3));
                CurveTarget(([x, y], BoolTarget::new_unsafe(is_inf)))
            }
        }
    };
}

impl_circuit_builder_for_extension_degree!(1);
impl_circuit_builder_for_extension_degree!(2);
impl_circuit_builder_for_extension_degree!(4);
impl_circuit_builder_for_extension_degree!(5);


pub trait PartialWitnessCurve<F: RichField + Extendable<5>>: Witness<F> {
    fn get_curve_target(&self, target: CurveTarget) -> AffinePointWithFlag;
    fn get_curve_targets(
        &self,
        targets: &[CurveTarget],
    ) -> Vec<AffinePointWithFlag> {
        targets.iter().map(|&t| self.get_curve_target(t)).collect()
    }
    
    fn set_curve_target(
        &mut self,
        target: CurveTarget,
        value: AffinePointWithFlag,
    );

    fn set_curve_targets(
        &mut self,
        targets: &[CurveTarget],
        values: &[AffinePointWithFlag],
    ) {
        for (&t, &v) in targets.iter().zip(values.iter()) {
            self.set_curve_target(t, v);
        }
    }
}

impl<W: PartialWitnessQuinticExt<GFp>> PartialWitnessCurve<GFp> for W {
    fn get_curve_target(&self, target: CurveTarget) -> AffinePointWithFlag {
        let CurveTarget(([x, y], is_inf)) = target;
        let x = self.get_quintic_ext_target(x);
        let y = self.get_quintic_ext_target(y);
        let is_inf = self.get_bool_target(is_inf);
        AffinePointWithFlag { x, y, is_inf }
    }

    fn set_curve_target(
        &mut self,
        target: CurveTarget,
        value: AffinePointWithFlag,
    ) {
        let CurveTarget(([x, y], is_inf)) = target;
        self.set_quintic_ext_target(x, value.x);
        self.set_quintic_ext_target(y, value.y);
        self.set_bool_target(is_inf, value.is_inf);
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use plonky2::{field::types::{Field, Sample}, plonk::{config::{PoseidonGoldilocksConfig, GenericConfig}, circuit_data::CircuitConfig}, iop::witness::PartialWitness};
    use rand::{thread_rng, Rng};

    use crate::curve::curve::Point;

    use super::*;

    fn random_point<R: Rng>(rng: &mut R) -> Point {
        let scalar = Scalar::sample(rng);
        Point::GENERATOR * scalar
    }

    #[test]
    fn test_curve_add() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let p1 = random_point(&mut rng);
        let p2 = random_point(&mut rng);
        let p3_expected = p1 + p2;

        let p1 = builder.curve_constant(p1.to_affine_with_flag());
        let p2 = builder.curve_constant(p2.to_affine_with_flag());
        let p3 = builder.curve_add(p1, p2);
        builder.register_curve_public_input(p3);
        
        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_curve_target(p3, p3_expected.to_affine_with_flag());

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_curve_double() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let p1 = random_point(&mut rng);
        let p2_expected = p1.double();

        let p1 = builder.curve_constant(p1.to_affine_with_flag());
        let p2 = builder.curve_double(p1);
        builder.register_curve_public_input(p2);
        
        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_curve_target(p2, p2_expected.to_affine_with_flag());

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_curve_scalar_mul() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let p = random_point(&mut rng);
        let s = Scalar::sample(&mut rng);
        let prod_expected = p * s;

        let p = builder.curve_constant(p.to_affine_with_flag());
        let s = builder.constant_nonnative(s);

        let prod = builder.curve_scalar_mul(p, s);
        builder.register_curve_public_input(prod);
        
        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_curve_target(prod, prod_expected.to_affine_with_flag());

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }
}