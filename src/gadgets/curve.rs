use crate::curve::scalar_field::Scalar;
use crate::curve::{curve::{Point, WeierstrassPoint}, GFp, GFp5};
use crate::gadgets::base_field::{CircuitBuilderGFp5, QuinticExtensionTarget};
use plonky2::field::types::Field; 
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::BoolTarget;
use plonky2::iop::target::Target;
use plonky2::iop::witness::Witness;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_ecdsa::gadgets::nonnative::NonNativeTarget;
use plonky2_ecdsa::gadgets::split_nonnative::CircuitBuilderSplit;
use plonky2_field::extension::Extendable;
use plonky2_field::extension::quintic::QuinticExtension;
use plonky2_field::goldilocks_field::GoldilocksField;

use super::base_field::PartialWitnessQuinticExt;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct CurveTarget(([QuinticExtensionTarget; 2], BoolTarget));

pub trait CircuitBuilderEcGFp5 {
    fn add_virtual_curve_target(&mut self) -> CurveTarget;
    fn register_curve_public_input(&mut self, point: CurveTarget);
    fn curve_constant(&mut self, point: WeierstrassPoint) -> CurveTarget;
    fn curve_zero(&mut self) -> CurveTarget;
    fn curve_generator(&mut self) -> CurveTarget;

    fn curve_eq(&mut self, a: CurveTarget, b: CurveTarget) -> BoolTarget;
    fn curve_select(&mut self, cond: BoolTarget, a: CurveTarget, b: CurveTarget) -> CurveTarget;
    fn curve_random_access(&mut self, access_index: Target, v: &[CurveTarget]) -> CurveTarget;

    fn curve_add(&mut self, a: CurveTarget, b: CurveTarget) -> CurveTarget;
    fn curve_add_spec(&mut self, a: CurveTarget, b: CurveTarget) -> CurveTarget;
    fn curve_double(&mut self, a: CurveTarget) -> CurveTarget;

    fn precompute_window(&mut self, a: CurveTarget, window_bits: usize) -> Vec<CurveTarget>;
    fn curve_scalar_mul(&mut self, a: CurveTarget, scalar: &NonNativeTarget<Scalar>) -> CurveTarget;

    fn precompute_window_const(&mut self, point: Point, window_bits: usize) -> Vec<CurveTarget>;
    fn curve_scalar_mul_const(&mut self, point: Point, scalar: &NonNativeTarget<Scalar>) -> CurveTarget;

    fn curve_encode_to_quintic_ext(&mut self, a: CurveTarget) -> QuinticExtensionTarget;
    fn curve_decode_from_quintic_ext(&mut self, w: QuinticExtensionTarget) -> CurveTarget;

    fn curve_muladd_2(&mut self, a: CurveTarget, b: CurveTarget, scalar_a: &NonNativeTarget<Scalar>, scalar_b: &NonNativeTarget<Scalar>) -> CurveTarget;
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

            fn curve_constant(&mut self, point: WeierstrassPoint) -> CurveTarget {
                let WeierstrassPoint { x, y, is_inf } = point;

                let x = self.constant_quintic_ext(x);
                let y = self.constant_quintic_ext(y);
                let is_inf = self.constant_bool(is_inf);
                CurveTarget(([x, y], is_inf))
            }

            fn curve_zero(&mut self) -> CurveTarget {
                self.curve_constant(WeierstrassPoint::NEUTRAL)
            }

            fn curve_generator(&mut self) -> CurveTarget {
                self.curve_constant(WeierstrassPoint::GENERATOR)
            }

            fn curve_eq(
                &mut self,
                a: CurveTarget,
                b: CurveTarget,
            ) -> BoolTarget {
                let CurveTarget(([ax, ay], a_is_inf)) = a;
                let CurveTarget(([bx, by], b_is_inf)) = b;

                let both_inf = self.and(a_is_inf, b_is_inf);

                let x_eq = self.is_equal_quintic_ext(ax, bx);
                let y_eq = self.is_equal_quintic_ext(ay, by);
                let both_eq = self.and(x_eq, y_eq);

                self.or(both_inf, both_eq)
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
                let x_same = self.is_equal_quintic_ext(x1, x2);
                let mut y_diff = self.is_equal_quintic_ext(y1, y2);
                y_diff = self.not(y_diff);

                let lambda_0_if_x_not_same = self.sub_quintic_ext(y2, y1);

                let mut lambda_0_if_x_same = self.square_quintic_ext(x1);
                lambda_0_if_x_same = self.triple_quintic_ext(lambda_0_if_x_same);
                lambda_0_if_x_same =
                    self.add_const_quintic_ext(lambda_0_if_x_same, WeierstrassPoint::A);

                let lambda_1_if_x_not_same = self.sub_quintic_ext(x2, x1);
                let lambda_1_if_x_same = self.double_quintic_ext(y1);

                let lambda_0 = self.select_quintic_ext(x_same, lambda_0_if_x_same, lambda_0_if_x_not_same);
                let lambda_1 = self.select_quintic_ext(x_same, lambda_1_if_x_same, lambda_1_if_x_not_same);
                let lambda = self.div_or_zero_quintic_ext(lambda_0, lambda_1);

                let mut x3 = self.square_quintic_ext(lambda);
                x3 = self.sub_quintic_ext(x3, x1);
                x3 = self.sub_quintic_ext(x3, x2);

                let mut y3 = self.sub_quintic_ext(x1, x3);
                y3 = self.mul_quintic_ext(lambda, y3);
                y3 = self.sub_quintic_ext(y3, y1);

                let c_is_inf = self.and(x_same, y_diff);
                let c = CurveTarget(([x3, y3], c_is_inf));

                let sel = self.curve_select(a_is_inf, b, c);
                self.curve_select(b_is_inf, a, sel)
            }

            fn curve_add_spec(&mut self, a: CurveTarget, b: CurveTarget) -> CurveTarget {
                let CurveTarget(([x1, y1], _)) = a;
                let CurveTarget(([x2, y2], _)) = b;

                let lambda_0 = self.sub_quintic_ext(y2, y1);
                let lambda_1 = self.sub_quintic_ext(x2, x1);
                let lambda = self.div_or_zero_quintic_ext(lambda_0, lambda_1);

                let mut x3 = self.square_quintic_ext(lambda);
                x3 = self.sub_quintic_ext(x3, x1);
                x3 = self.sub_quintic_ext(x3, x2);

                let mut y3 = self.sub_quintic_ext(x1, x3);
                y3 = self.mul_quintic_ext(lambda, y3);
                y3 = self.sub_quintic_ext(y3, y1);

                CurveTarget(([x3, y3], BoolTarget::new_unsafe(self.zero())))
            }

            fn curve_double(&mut self, a: CurveTarget) -> CurveTarget {
                let CurveTarget(([x, y], is_inf)) = a;

                let mut lambda_0 = self.square_quintic_ext(x);
                lambda_0 = self.triple_quintic_ext(lambda_0);
                lambda_0 = self.add_const_quintic_ext(lambda_0, WeierstrassPoint::A);
                let lambda_1 = self.double_quintic_ext(y);

                let lambda = self.div_or_zero_quintic_ext(lambda_0, lambda_1);

                let mut x2 = self.square_quintic_ext(lambda);
                let two_x = self.double_quintic_ext(x);
                x2 = self.sub_quintic_ext(x2, two_x);

                let mut y2 = self.sub_quintic_ext(x, x2);
                y2 = self.mul_quintic_ext(lambda, y2);
                y2 = self.sub_quintic_ext(y2, y);

                CurveTarget(([x2, y2], is_inf))
            }

            fn precompute_window(&mut self, a: CurveTarget, window_bits: usize) -> Vec<CurveTarget> {
                debug_assert!(window_bits > 1);
                let mut multiples = vec![self.curve_zero()];
                multiples.push(a);
                multiples.push(self.curve_double(a));
                
                for _ in 3..(1 << window_bits) {
                    multiples.push(
                        self.curve_add(multiples.last().unwrap().clone(), a)
                    );
                }

                multiples
            }

            // TODO optimize
            fn curve_scalar_mul(
                &mut self,
                a: CurveTarget,
                scalar: &NonNativeTarget<Scalar>,
            ) -> CurveTarget {
                let window = self.precompute_window(a, 4);
                let four_bit_limbs = self.split_nonnative_to_4_bit_limbs(&scalar);

                let num_limbs = four_bit_limbs.len();
                let mut res = self.curve_random_access(four_bit_limbs[num_limbs - 1], &window);
                for limb in four_bit_limbs.into_iter().rev().skip(1) {
                    for _ in 0..4 {
                        res = self.curve_double(res);
                    }

                    let addend = self.curve_random_access(limb, &window);
                    res = self.curve_add(res, addend);
                }

                res
            }


            fn precompute_window_const(&mut self, point: Point, window_bits: usize) -> Vec<CurveTarget> {
                let mut curr = point;
                let mut multiples = vec![self.curve_zero()];

                for _ in 1..(1 << window_bits) {
                    multiples.push(self.curve_constant(curr.to_weierstrass()));
                    curr += point;
                }

                multiples
            }

            fn curve_scalar_mul_const(&mut self, point: Point, scalar: &NonNativeTarget<Scalar>, ) -> CurveTarget {
                let window = self.precompute_window_const(point, 4);
                let four_bit_limbs = self.split_nonnative_to_4_bit_limbs(&scalar);

                let num_limbs = four_bit_limbs.len();
                let mut res = self.curve_random_access(four_bit_limbs[num_limbs - 1], &window);
                for limb in four_bit_limbs.into_iter().rev().skip(1) {
                    for _ in 0..4 {
                        res = self.curve_double(res);
                    }

                    let addend = self.curve_random_access(limb, &window);
                    res = self.curve_add(res, addend);
                }

                res
            }

            // TODO: optimize to use base field when we know it's in the base field
            fn curve_encode_to_quintic_ext(&mut self, a: CurveTarget) -> QuinticExtensionTarget {
                let CurveTarget(([x, y], is_inf)) = a;
                let adiv3 = self.constant_quintic_ext(GFp5::TWO / GFp5::from_canonical_u16(3));
                let denom = self.sub_quintic_ext(adiv3, x);
                let w = self.div_or_zero_quintic_ext(y, denom);

                let zero = self.zero_quintic_ext();
                self.select_quintic_ext(is_inf, zero, w)
            }

            // TODO: optimize to use base field when we know it's in the base field
            fn curve_decode_from_quintic_ext(&mut self, w: QuinticExtensionTarget) -> CurveTarget {
                let one = self.one();
                let zero_quintic_ext = self.zero_quintic_ext();
                let a = self.constant_quintic_ext(Point::A);
                let bmul4 = self.constant_quintic_ext(Point::B_MUL4);

                let mut e = self.square_quintic_ext(w);
                e = self.sub_quintic_ext(e, a);

                let mut delta = self.square_quintic_ext(e);
                delta = self.sub_quintic_ext(delta, bmul4);

                let (r, delta_is_sqrt) = self.try_any_sqrt_quintic_ext(delta);

                // if delta is not a sqrt, then w must be zero. otherwise, it's not a valid point encoding
                // we check this by asserting that delta_is_sqrt OR w == 0.
                let w_is_zero = self.is_equal_quintic_ext(w, zero_quintic_ext);
                let delta_is_sqrt_or_w_is_zero = self.or(delta_is_sqrt, w_is_zero);
                self.assert_bool(delta_is_sqrt_or_w_is_zero);

                let mut x1 = self.add_quintic_ext(e, r);
                x1 = self.div_const_quintic_ext(x1, GFp5::TWO);

                let mut x2 = self.sub_quintic_ext(e, r);
                x2 = self.div_const_quintic_ext(x2, GFp5::TWO);

                let legendre_x1 = self.legendre_sym_quintic_ext(x1);
                let legendre_is_one = self.is_equal(legendre_x1, one);
                let x = self.select_quintic_ext(legendre_is_one, x1, x2);

                let negw = self.neg_quintic_ext(w);
                let y = self.mul_quintic_ext(negw, x);

                let x = self.add_const_quintic_ext(x, Point::A / GFp5::from_canonical_u16(3));
                // since we checked above that w is zero if delta is not a sqrt, we can just set is_inf to delta_is_not_sqrt
                let is_inf = self.not(delta_is_sqrt);
                CurveTarget(([x, y], is_inf))
            }

            fn curve_muladd_2(&mut self, a: CurveTarget, b: CurveTarget, scalar_a: &NonNativeTarget<Scalar>, scalar_b: &NonNativeTarget<Scalar>) -> CurveTarget {
                let a_window = self.precompute_window(a, 4);
                let a_four_bit_limbs = self.split_nonnative_to_4_bit_limbs(&scalar_a);

                let b_window = self.precompute_window(b, 4);
                let b_four_bit_limbs = self.split_nonnative_to_4_bit_limbs(&scalar_b);

                debug_assert!(a_four_bit_limbs.len() == b_four_bit_limbs.len());

                let num_limbs = a_four_bit_limbs.len();
                let a_start = self.curve_random_access(a_four_bit_limbs[num_limbs - 1], &a_window);
                let b_start = self.curve_random_access(b_four_bit_limbs[num_limbs - 1], &b_window);
                let mut res = self.curve_add(a_start, b_start);

                for (a_limb, b_limb) in a_four_bit_limbs.into_iter().zip(b_four_bit_limbs).rev().skip(1) {
                    for _ in 0..4 {
                        res = self.curve_double(res);
                    }

                    let a_addend = self.curve_random_access(a_limb, &a_window);
                    let b_addend = self.curve_random_access(b_limb, &b_window);
                    let addend = self.curve_add(a_addend, b_addend);
                    res = self.curve_add(res, addend);
                }

                res
            }
        }
    };
}

impl_circuit_builder_for_extension_degree!(1);
impl_circuit_builder_for_extension_degree!(2);
impl_circuit_builder_for_extension_degree!(4);
impl_circuit_builder_for_extension_degree!(5);


pub trait PartialWitnessCurve<F: RichField + Extendable<5>>: Witness<F> {
    fn get_curve_target(&self, target: CurveTarget) -> WeierstrassPoint;
    fn get_curve_targets(
        &self,
        targets: &[CurveTarget],
    ) -> Vec<WeierstrassPoint> {
        targets.iter().map(|&t| self.get_curve_target(t)).collect()
    }
    
    fn set_curve_target(
        &mut self,
        target: CurveTarget,
        value: WeierstrassPoint,
    );

    fn set_curve_targets(
        &mut self,
        targets: &[CurveTarget],
        values: &[WeierstrassPoint],
    ) {
        for (&t, &v) in targets.iter().zip(values.iter()) {
            self.set_curve_target(t, v);
        }
    }
}

impl<W: PartialWitnessQuinticExt<GFp>> PartialWitnessCurve<GFp> for W {
    fn get_curve_target(&self, target: CurveTarget) -> WeierstrassPoint {
        let CurveTarget(([x, y], is_inf)) = target;
        let x = self.get_quintic_ext_target(x);
        let y = self.get_quintic_ext_target(y);
        let is_inf = self.get_bool_target(is_inf);
        WeierstrassPoint { x, y, is_inf }
    }

    fn set_curve_target(
        &mut self,
        target: CurveTarget,
        value: WeierstrassPoint,
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
    use plonky2::{field::types::Sample, plonk::{config::{PoseidonGoldilocksConfig, GenericConfig}, circuit_data::CircuitConfig}, iop::witness::PartialWitness};
    use plonky2_ecdsa::gadgets::nonnative::CircuitBuilderNonNative;
    use rand::thread_rng;

    use crate::curve::curve::Point;

    use super::*;

    #[test]
    fn test_curve_add() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let p1 = Point::sample(&mut rng);
        let p2 = Point::sample(&mut rng);
        let p3_expected = p1 + p2;

        let p1 = builder.curve_constant(p1.to_weierstrass());
        let p2 = builder.curve_constant(p2.to_weierstrass());
        let p3 = builder.curve_add(p1, p2);
        builder.register_curve_public_input(p3);
        
        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_curve_target(p3, p3_expected.to_weierstrass());

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

        let p1 = Point::sample(&mut rng);
        let p2_expected = p1.double();

        let p1 = builder.curve_constant(p1.to_weierstrass());
        let p2 = builder.curve_double(p1);
        builder.register_curve_public_input(p2);
        
        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_curve_target(p2, p2_expected.to_weierstrass());

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

        let p = Point::sample(&mut rng);
        let s = Scalar::sample(&mut rng);
        let prod_expected = p * s;

        let p = builder.curve_constant(p.to_weierstrass());
        let s = builder.constant_nonnative(s);

        let prod = builder.curve_scalar_mul(p, &s);
        builder.register_curve_public_input(prod);
        
        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_curve_target(prod, prod_expected.to_weierstrass());

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_curve_scalar_mul_const() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let p = Point::sample(&mut rng);
        let s = Scalar::sample(&mut rng);
        let prod_expected = p * s;

        let s = builder.constant_nonnative(s);

        let prod = builder.curve_scalar_mul_const(p, &s);
        builder.register_curve_public_input(prod);
        
        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_curve_target(prod, prod_expected.to_weierstrass());

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_curve_encode() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let p = Point::sample(&mut rng);
        let w_expected = p.encode();
        
        let p = builder.curve_constant(p.to_weierstrass());
        let w = builder.curve_encode_to_quintic_ext(p);
        builder.register_quintic_ext_public_input(w);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_quintic_ext_target(w, w_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_curve_decode() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let p_expected = Point::sample(&mut rng);
        let w = p_expected.encode();

        let w = builder.constant_quintic_ext(w);
        let p = builder.curve_decode_from_quintic_ext(w);
        builder.register_curve_public_input(p);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_curve_target(p, p_expected.to_weierstrass());

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_curve_muladd_2() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let p1 = Point::sample(&mut rng);
        let p2 = Point::sample(&mut rng);
        let s1 = Scalar::sample(&mut rng);
        let s2 = Scalar::sample(&mut rng);
        let prod_expected = p1 * s1 + p2 * s2;

        let p1 = builder.curve_constant(p1.to_weierstrass());
        let s1 = builder.constant_nonnative(s1);

        let p2 = builder.curve_constant(p2.to_weierstrass());
        let s2 = builder.constant_nonnative(s2);

        let prod = builder.curve_muladd_2(p1, p2, &s1, &s2);
        builder.register_curve_public_input(prod);
        
        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_curve_target(prod, prod_expected.to_weierstrass());

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }
}