use alloc::vec::Vec;
use plonky2_field::extension::quintic::QuinticExtension;

use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartitionWitness, Witness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_field::extension::{Extendable, FieldExtension};
use plonky2_field::types::Field;

use crate::curve::base_field::SquareRoot;
use crate::curve::{GFp, GFp5};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct QuinticExtensionTarget([Target; 5]);

impl QuinticExtensionTarget {
    pub fn new(limbs: [Target; 5]) -> Self {
        Self(limbs)
    }

    pub fn to_target_array(&self) -> [Target; 5] {
        self.0
    }
}

pub trait CircuitBuilderGFp5<F: RichField + Extendable<5>> {
    fn add_virtual_quintic_ext_target(&mut self) -> QuinticExtensionTarget;
    fn connect_quintic_ext(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget);
    fn register_quintic_ext_public_input(&mut self, a: QuinticExtensionTarget);

    fn zero_quintic_ext(&mut self) -> QuinticExtensionTarget;
    fn one_quintic_ext(&mut self) -> QuinticExtensionTarget;
    fn constant_quintic_ext(&mut self, c: GFp5) -> QuinticExtensionTarget;
    fn select_quintic_ext(
        &mut self,
        cond: BoolTarget,
        a: QuinticExtensionTarget,
        b: QuinticExtensionTarget,
    ) -> QuinticExtensionTarget;
    fn random_access_quintic_ext(
        &mut self,
        access_index: Target,
        v: &[QuinticExtensionTarget],
    ) -> QuinticExtensionTarget;
    fn is_equal_quintic_ext(
        &mut self,
        a: QuinticExtensionTarget,
        b: QuinticExtensionTarget,
    ) -> BoolTarget;

    fn neg_quintic_ext(&mut self, a: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn add_quintic_ext(
        &mut self,
        a: QuinticExtensionTarget,
        b: QuinticExtensionTarget,
    ) -> QuinticExtensionTarget;
    fn add_const_quintic_ext(
        &mut self,
        a: QuinticExtensionTarget,
        c: GFp5,
    ) -> QuinticExtensionTarget;
    fn sub_quintic_ext(
        &mut self,
        a: QuinticExtensionTarget,
        b: QuinticExtensionTarget,
    ) -> QuinticExtensionTarget;
    fn mul_quintic_ext(
        &mut self,
        a: QuinticExtensionTarget,
        b: QuinticExtensionTarget,
    ) -> QuinticExtensionTarget;
    fn mul_const_quintic_ext(
        &mut self,
        c: GFp5,
        a: QuinticExtensionTarget,
    ) -> QuinticExtensionTarget;

    fn div_quintic_ext(
        &mut self,
        a: QuinticExtensionTarget,
        b: QuinticExtensionTarget,
    ) -> QuinticExtensionTarget;
    fn div_const_quintic_ext(
        &mut self,
        a: QuinticExtensionTarget,
        c: GFp5,
    ) -> QuinticExtensionTarget;
    fn inverse_quintic_ext(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn any_sqrt_quintic_ext(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn try_any_sqrt_quintic_ext(
        &mut self,
        x: QuinticExtensionTarget,
    ) -> (QuinticExtensionTarget, BoolTarget);
    fn try_canonical_sqrt_quintic_ext(
        &mut self,
        x: QuinticExtensionTarget,
    ) -> (QuinticExtensionTarget, BoolTarget);
    fn canonical_sqrt_quintic_ext(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;

    fn sgn0_quintic_ext(&mut self, x: QuinticExtensionTarget) -> BoolTarget;
    fn legendre_sym_quintic_ext(&mut self, x: QuinticExtensionTarget) -> Target;
    fn frob_quintic_ext(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn frob2_quintic_ext(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;

    fn square_quintic_ext(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn add_many_quintic_ext(
        &mut self,
        terms: Vec<QuinticExtensionTarget>,
    ) -> QuinticExtensionTarget;
    fn dot_product_quintic_ext(
        &mut self,
        a: Vec<QuinticExtensionTarget>,
        b: Vec<QuinticExtensionTarget>,
    ) -> QuinticExtensionTarget;
}

pub trait PartialWitnessQuinticExt<F: RichField + Extendable<5>>: Witness<F> {
    fn get_quintic_ext_target(&self, target: QuinticExtensionTarget) -> QuinticExtension<F>;

    fn get_quintic_ext_targets(
        &self,
        targets: &[QuinticExtensionTarget],
    ) -> Vec<QuinticExtension<F>> {
        targets
            .iter()
            .map(|&t| self.get_quintic_ext_target(t))
            .collect()
    }

    fn set_quintic_ext_target(
        &mut self,
        target: QuinticExtensionTarget,
        value: QuinticExtension<F>,
    );

    fn set_quintic_ext_targets(
        &mut self,
        targets: &[QuinticExtensionTarget],
        values: &[QuinticExtension<F>],
    ) {
        for (&t, &v) in targets.iter().zip(values.iter()) {
            self.set_quintic_ext_target(t, v);
        }
    }
}

impl<W: Witness<F>, F: RichField + Extendable<5>> PartialWitnessQuinticExt<F> for W {
    fn get_quintic_ext_target(&self, target: QuinticExtensionTarget) -> QuinticExtension<F> {
        let QuinticExtensionTarget([t0, t1, t2, t3, t4]) = target;
        QuinticExtension([
            self.get_target(t0),
            self.get_target(t1),
            self.get_target(t2),
            self.get_target(t3),
            self.get_target(t4),
        ])
    }

    fn set_quintic_ext_target(
        &mut self,
        target: QuinticExtensionTarget,
        value: QuinticExtension<F>,
    ) {
        let QuinticExtensionTarget([t0, t1, t2, t3, t4]) = target;
        let [v0, v1, v2, v3, v4] = value.0;

        self.set_target(t0, v0);
        self.set_target(t1, v1);
        self.set_target(t2, v2);
        self.set_target(t3, v3);
        self.set_target(t4, v4);
    }

}

macro_rules! impl_circuit_builder_for_extension_degree {
    ($degree:literal) => {
        impl CircuitBuilderGFp5<GFp> for CircuitBuilder<GFp, $degree> {
            fn add_virtual_quintic_ext_target(&mut self) -> QuinticExtensionTarget {
                QuinticExtensionTarget::new([
                    self.add_virtual_target(),
                    self.add_virtual_target(),
                    self.add_virtual_target(),
                    self.add_virtual_target(),
                    self.add_virtual_target(),
                ])
            }

            fn connect_quintic_ext(
                &mut self,
                a: QuinticExtensionTarget,
                b: QuinticExtensionTarget,
            ) {
                for (lhs, rhs) in a
                    .to_target_array()
                    .into_iter()
                    .zip(b.to_target_array().into_iter())
                {
                    self.connect(lhs, rhs);
                }
            }

            fn register_quintic_ext_public_input(&mut self, a: QuinticExtensionTarget) {
                for t in a.to_target_array().into_iter() {
                    self.register_public_input(t);
                }
            }

            fn zero_quintic_ext(&mut self) -> QuinticExtensionTarget {
                QuinticExtensionTarget::new([self.zero(); 5])
            }

            fn one_quintic_ext(&mut self) -> QuinticExtensionTarget {
                QuinticExtensionTarget::new([
                    self.one(),
                    self.zero(),
                    self.zero(),
                    self.zero(),
                    self.zero(),
                ])
            }

            fn constant_quintic_ext(&mut self, c: GFp5) -> QuinticExtensionTarget {
                let QuinticExtension([c0, c1, c2, c3, c4]) = c;
                QuinticExtensionTarget::new([
                    self.constant(c0),
                    self.constant(c1),
                    self.constant(c2),
                    self.constant(c3),
                    self.constant(c4),
                ])
            }

            fn select_quintic_ext(
                &mut self,
                cond: BoolTarget,
                a: QuinticExtensionTarget,
                b: QuinticExtensionTarget,
            ) -> QuinticExtensionTarget {
                let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
                let QuinticExtensionTarget([b0, b1, b2, b3, b4]) = b;
                QuinticExtensionTarget::new([
                    self.select(cond, a0, b0),
                    self.select(cond, a1, b1),
                    self.select(cond, a2, b2),
                    self.select(cond, a3, b3),
                    self.select(cond, a4, b4),
                ])
            }

            fn random_access_quintic_ext(
                &mut self,
                access_index: Target,
                v: &[QuinticExtensionTarget],
            ) -> QuinticExtensionTarget {
                let mut a0s = Vec::new();
                let mut a1s = Vec::new();
                let mut a2s = Vec::new();
                let mut a3s = Vec::new();
                let mut a4s = Vec::new();
                for &QuinticExtensionTarget([a0, a1, a2, a3, a4]) in v {
                    a0s.push(a0);
                    a1s.push(a1);
                    a2s.push(a2);
                    a3s.push(a3);
                    a4s.push(a4);
                }

                QuinticExtensionTarget([
                    self.random_access(access_index, a0s),
                    self.random_access(access_index, a1s),
                    self.random_access(access_index, a2s),
                    self.random_access(access_index, a3s),
                    self.random_access(access_index, a4s),
                ])
            }

            fn is_equal_quintic_ext(
                &mut self,
                a: QuinticExtensionTarget,
                b: QuinticExtensionTarget,
            ) -> BoolTarget {
                let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
                let QuinticExtensionTarget([b0, b1, b2, b3, b4]) = b;

                let terms = vec![
                    self.is_equal(a0, b0).target,
                    self.is_equal(a1, b1).target,
                    self.is_equal(a2, b2).target,
                    self.is_equal(a3, b3).target,
                    self.is_equal(a4, b4).target,
                ];

                let prod = self.mul_many(terms);
                BoolTarget::new_unsafe(prod)
            }

            fn neg_quintic_ext(&mut self, a: QuinticExtensionTarget) -> QuinticExtensionTarget {
                let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
                QuinticExtensionTarget::new([
                    self.neg(a0),
                    self.neg(a1),
                    self.neg(a2),
                    self.neg(a3),
                    self.neg(a4),
                ])
            }

            fn add_quintic_ext(
                &mut self,
                a: QuinticExtensionTarget,
                b: QuinticExtensionTarget,
            ) -> QuinticExtensionTarget {
                let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
                let QuinticExtensionTarget([b0, b1, b2, b3, b4]) = b;
                QuinticExtensionTarget::new([
                    self.add(a0, b0),
                    self.add(a1, b1),
                    self.add(a2, b2),
                    self.add(a3, b3),
                    self.add(a4, b4),
                ])
            }

            fn add_const_quintic_ext(
                &mut self,
                a: QuinticExtensionTarget,
                c: GFp5,
            ) -> QuinticExtensionTarget {
                let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
                let QuinticExtension([c0, c1, c2, c3, c4]) = c;
                QuinticExtensionTarget::new([
                    self.add_const(a0, c0),
                    self.add_const(a1, c1),
                    self.add_const(a2, c2),
                    self.add_const(a3, c3),
                    self.add_const(a4, c4),
                ])
            }

            fn sub_quintic_ext(
                &mut self,
                a: QuinticExtensionTarget,
                b: QuinticExtensionTarget,
            ) -> QuinticExtensionTarget {
                let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
                let QuinticExtensionTarget([b0, b1, b2, b3, b4]) = b;
                QuinticExtensionTarget::new([
                    self.sub(a0, b0),
                    self.sub(a1, b1),
                    self.sub(a2, b2),
                    self.sub(a3, b3),
                    self.sub(a4, b4),
                ])
            }

            fn mul_quintic_ext(
                &mut self,
                a: QuinticExtensionTarget,
                b: QuinticExtensionTarget,
            ) -> QuinticExtensionTarget {
                let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
                let QuinticExtensionTarget([b0, b1, b2, b3, b4]) = b;

                // c0 ← a0b0 + 3(a1b4 + a2b3 + a3b2 + a4b1)
                // c1 ← a0b1 + a1b0 + 3(a2b4 + a3b3 + a4b2)
                // c2 ← a0b2 + a1b1 + a2b0 + 3(a3b4 + a4b3)
                // c3 ← a0b3 + a1b2 + a2b1 + a3b0 + 3a4b4
                // c4 ← a0b4 + a1b3 + a2b2 + a3b1 + a4b0

                let mut c0 = self.mul(a4, b1);
                c0 = self.mul_add(a3, b2, c0);
                c0 = self.mul_add(a2, b3, c0);
                c0 = self.mul_add(a1, b4, c0);
                c0 = self.mul_const(GFp::from_canonical_u64(3), c0);
                c0 = self.mul_add(a0, b0, c0);

                let mut c1 = self.mul(a4, b2);
                c1 = self.mul_add(a3, b3, c1);
                c1 = self.mul_add(a2, b4, c1);
                c1 = self.mul_const(GFp::from_canonical_u64(3), c1);
                c1 = self.mul_add(a1, b0, c1);
                c1 = self.mul_add(a0, b1, c1);

                let mut c2 = self.mul(a4, b3);
                c2 = self.mul_add(a3, b4, c2);
                c2 = self.mul_const(GFp::from_canonical_u64(3), c2);
                c2 = self.mul_add(a2, b0, c2);
                c2 = self.mul_add(a1, b1, c2);
                c2 = self.mul_add(a0, b2, c2);

                let mut c3 = self.mul(a4, b4);
                c3 = self.mul_const(GFp::from_canonical_u64(3), c3);
                c3 = self.mul_add(a3, b0, c3);
                c3 = self.mul_add(a2, b1, c3);
                c3 = self.mul_add(a1, b2, c3);
                c3 = self.mul_add(a0, b3, c3);

                let mut c4 = self.mul(a4, b0);
                c4 = self.mul_add(a3, b1, c4);
                c4 = self.mul_add(a2, b2, c4);
                c4 = self.mul_add(a1, b3, c4);
                c4 = self.mul_add(a0, b4, c4);

                QuinticExtensionTarget::new([c0, c1, c2, c3, c4])
            }

            // TODO optimize
            fn mul_const_quintic_ext(
                &mut self,
                c: GFp5,
                a: QuinticExtensionTarget,
            ) -> QuinticExtensionTarget {
                let c = self.constant_quintic_ext(c);
                self.mul_quintic_ext(c, a)
            }

            /// returns `a / b` is `b` is nonzero, `0` otherwise
            fn div_quintic_ext(
                &mut self,
                a: QuinticExtensionTarget,
                b: QuinticExtensionTarget,
            ) -> QuinticExtensionTarget {
                let zero = self.zero_quintic_ext();
                let quotient = self.add_virtual_quintic_ext_target();
                self.add_simple_generator(QuinticQuotientGenerator::new(a, b, quotient));

                let quotient_times_denominator = self.mul_quintic_ext(quotient, b);
                let is_zero = self.is_equal_quintic_ext(quotient_times_denominator, zero);
                let quotient_check = self.is_equal_quintic_ext(quotient_times_denominator, a);
                let is_valid = self.and(is_zero, quotient_check);
                self.assert_bool(is_valid);

                quotient
            }

            /// returns `a / b` is `b` is nonzero, `0` otherwise
            fn div_const_quintic_ext(
                &mut self,
                num: QuinticExtensionTarget,
                denom: GFp5,
            ) -> QuinticExtensionTarget {
                let denom = self.constant_quintic_ext(denom);
                self.div_quintic_ext(num, denom)
            }

            fn inverse_quintic_ext(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget {
                let one = self.one_quintic_ext();
                let inverse = self.add_virtual_quintic_ext_target();
                self.add_simple_generator(QuinticQuotientGenerator::new(one, x, inverse));

                let should_be_one = self.mul_quintic_ext(inverse, x);
                self.connect_quintic_ext(should_be_one, one);

                inverse
            }

            fn any_sqrt_quintic_ext(
                &mut self,
                x: QuinticExtensionTarget,
            ) -> QuinticExtensionTarget {
                let (root_x, _) = self.try_any_sqrt_quintic_ext(x);
                root_x
            }

            fn try_any_sqrt_quintic_ext(
                &mut self,
                x: QuinticExtensionTarget,
            ) -> (QuinticExtensionTarget, BoolTarget) {
                let zero = self.zero_quintic_ext();
                let root_x = self.add_virtual_quintic_ext_target();
                let is_sqrt = self.add_virtual_bool_target_unsafe();
                self.add_simple_generator(QuinticSqrtGenerator::new(x, root_x, is_sqrt));

                let should_be_x_or_zero = self.square_quintic_ext(root_x);
                let x_or_zero = self.select_quintic_ext(is_sqrt, x, zero);
                self.connect_quintic_ext(should_be_x_or_zero, x_or_zero);

                (root_x, is_sqrt)
            }

            /// returns true or false indicating a notion of "sign" for quintic_ext.
            /// This is used to canonicalize the square root
            /// This is an implementation of the function sgn0 from the IRTF's hash-to-curve document
            /// https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-hash-to-curve-07#name-the-sgn0-function
            fn sgn0_quintic_ext(&mut self, x: QuinticExtensionTarget) -> BoolTarget {
                let one = self.one();
                let zero = self.zero();

                let mut sign = self.constant_bool(false);
                let mut is_zero = self.constant_bool(true);
                for limb in x.to_target_array() {
                    let bit_decomp = self.split_le_base::<2>(limb, 64);

                    // sign_i = x_i mod 2
                    // is_zero_i = x_i == 0
                    // SAFETY: targets from bit_decomp guaranteed to contain values of 0 or 1
                    let sign_i = BoolTarget::new_unsafe(self.sub(one, bit_decomp[0]));
                    let is_zero_i = self.is_equal(limb, zero);

                    // sign = sign || (is_zero && sign_i)
                    // is_zero = is_zero && is_zero_i

                    // x or y = x + y - xy
                    let is_zero_and_sign_i = self.and(is_zero_i, sign_i);
                    let sign_and_is_zero_and_sign_i = self.and(sign, is_zero_and_sign_i);
                    let tmp = self.mul_const_add(
                        -GFp::ONE,
                        sign_and_is_zero_and_sign_i.target,
                        sign.target,
                    );
                    sign = BoolTarget::new_unsafe(self.add(tmp, is_zero_and_sign_i.target));
                    is_zero = self.and(is_zero, is_zero_i);
                }

                sign
            }

            fn legendre_sym_quintic_ext(&mut self, x: QuinticExtensionTarget) -> Target {
                // compute x^r where r = p^4 + p^3 + p^2 + p + 1
                let frob1 = self.frob_quintic_ext(x);
                let frob2 = self.frob2_quintic_ext(x);
                let frob1_times_frob2 = self.mul_quintic_ext(frob1, frob2);
                let frob2_frob1_times_frob2 = self.frob_quintic_ext(frob1_times_frob2);
                let x_to_r_minus_1 =
                    self.mul_quintic_ext(frob1_times_frob2, frob2_frob1_times_frob2);
                let x_to_r_quintic = self.mul_quintic_ext(x_to_r_minus_1, x);

                // x^r guaranteed to be in base field
                let QuinticExtensionTarget([y, _, _, _, _]) = x_to_r_quintic;

                let y31 = self.exp_power_of_2(y, 31);
                let y63 = self.exp_power_of_2(y31, 32);

                self.div(y63, y31)
            }

            fn frob_quintic_ext(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget {
                let frob_coeff_1 = GFp::from_canonical_u64(1041288259238279555);
                let frob_coeff_2 = GFp::from_canonical_u64(15820824984080659046);
                let frob_coeff_3 = GFp::from_canonical_u64(211587555138949697);
                let frob_coeff_4 = GFp::from_canonical_u64(1373043270956696022);

                let QuinticExtensionTarget([c0, mut c1, mut c2, mut c3, mut c4]) = x;

                c1 = self.mul_const(frob_coeff_1, c1);
                c2 = self.mul_const(frob_coeff_2, c2);
                c3 = self.mul_const(frob_coeff_3, c3);
                c4 = self.mul_const(frob_coeff_4, c4);

                QuinticExtensionTarget([c0, c1, c2, c3, c4])
            }

            fn frob2_quintic_ext(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget {
                let frob2_coeff_1 = GFp::from_canonical_u64(15820824984080659046);
                let frob2_coeff_2 = GFp::from_canonical_u64(1373043270956696022);
                let frob2_coeff_3 = GFp::from_canonical_u64(1041288259238279555);
                let frob2_coeff_4 = GFp::from_canonical_u64(211587555138949697);

                let QuinticExtensionTarget([c0, mut c1, mut c2, mut c3, mut c4]) = x;

                c1 = self.mul_const(frob2_coeff_1, c1);
                c2 = self.mul_const(frob2_coeff_2, c2);
                c3 = self.mul_const(frob2_coeff_3, c3);
                c4 = self.mul_const(frob2_coeff_4, c4);

                QuinticExtensionTarget([c0, c1, c2, c3, c4])
            }

            // returns the sqrt(x) such that `sgn0(sqrt(x)) == false`
            fn canonical_sqrt_quintic_ext(
                &mut self,
                x: QuinticExtensionTarget,
            ) -> QuinticExtensionTarget {
                let root_x = self.any_sqrt_quintic_ext(x);
                let neg_root_x = self.neg_quintic_ext(root_x);

                let sign = self.sgn0_quintic_ext(root_x);
                self.select_quintic_ext(sign, neg_root_x, root_x)
            }

            fn try_canonical_sqrt_quintic_ext(
                &mut self,
                x: QuinticExtensionTarget,
            ) -> (QuinticExtensionTarget, BoolTarget) {
                let (root_x, is_sqrt) = self.try_any_sqrt_quintic_ext(x);
                let neg_root_x = self.neg_quintic_ext(root_x);

                let sign = self.sgn0_quintic_ext(root_x);
                let canonical_root_x = self.select_quintic_ext(sign, neg_root_x, root_x);

                (canonical_root_x, is_sqrt)
            }

            // TODO optimize
            fn square_quintic_ext(&mut self, a: QuinticExtensionTarget) -> QuinticExtensionTarget {
                self.mul_quintic_ext(a, a)
            }

            fn add_many_quintic_ext(
                &mut self,
                terms: Vec<QuinticExtensionTarget>,
            ) -> QuinticExtensionTarget {
                let mut sum = self.zero_quintic_ext();
                for term in terms {
                    sum = self.add_quintic_ext(sum, term);
                }
                sum
            }

            fn dot_product_quintic_ext(
                &mut self,
                a: Vec<QuinticExtensionTarget>,
                b: Vec<QuinticExtensionTarget>,
            ) -> QuinticExtensionTarget {
                let mut terms = Vec::new();
                for (a, b) in a.into_iter().zip(b.into_iter()) {
                    terms.push(self.mul_quintic_ext(a, b));
                }
                self.add_many_quintic_ext(terms)
            }
        }
    };
}

impl_circuit_builder_for_extension_degree!(1);
impl_circuit_builder_for_extension_degree!(2);
impl_circuit_builder_for_extension_degree!(4);
impl_circuit_builder_for_extension_degree!(5);

#[derive(Debug)]
pub struct QuinticQuotientGenerator {
    numerator: QuinticExtensionTarget,
    denominator: QuinticExtensionTarget,
    quotient: QuinticExtensionTarget,
}

impl QuinticQuotientGenerator {
    pub fn new(
        numerator: QuinticExtensionTarget,
        denominator: QuinticExtensionTarget,
        quotient: QuinticExtensionTarget,
    ) -> Self {
        QuinticQuotientGenerator {
            numerator,
            denominator,
            quotient,
        }
    }
}

impl<F: RichField + Extendable<5>> SimpleGenerator<F> for QuinticQuotientGenerator {
    fn dependencies(&self) -> Vec<Target> {
        let mut deps = self.numerator.to_target_array().to_vec();
        deps.extend(self.denominator.to_target_array());
        deps
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let numerator_limbs = self
            .numerator
            .to_target_array()
            .map(|t| witness.get_target(t));
        let numerator = QuinticExtension::<F>::from_basefield_array(numerator_limbs);

        let denominator_limbs = self
            .denominator
            .to_target_array()
            .map(|t| witness.get_target(t));
        let denominator = QuinticExtension::<F>::from_basefield_array(denominator_limbs);

        let quotient = if denominator == QuinticExtension::<F>::ZERO {
            QuinticExtension::<F>::ZERO
        } else {
            numerator / denominator
        };
        for (lhs, rhs) in self.quotient.to_target_array().into_iter().zip(
            <QuinticExtension<F> as FieldExtension<5>>::to_basefield_array(&quotient).into_iter(),
        ) {
            out_buffer.set_target(lhs, rhs);
        }
    }
}

#[derive(Debug)]
pub struct QuinticSqrtGenerator {
    x: QuinticExtensionTarget,
    root_x: QuinticExtensionTarget,
    is_sqrt: BoolTarget,
}

impl QuinticSqrtGenerator {
    pub fn new(
        x: QuinticExtensionTarget,
        root_x: QuinticExtensionTarget,
        is_sqrt: BoolTarget,
    ) -> Self {
        QuinticSqrtGenerator { x, root_x, is_sqrt }
    }
}

impl SimpleGenerator<GFp> for QuinticSqrtGenerator {
    fn dependencies(&self) -> Vec<Target> {
        self.x.to_target_array().to_vec()
    }

    fn run_once(&self, witness: &PartitionWitness<GFp>, out_buffer: &mut GeneratedValues<GFp>) {
        let x_limbs = self.x.to_target_array().map(|t| witness.get_target(t));
        let x = QuinticExtension::<GFp>::from_basefield_array(x_limbs);

        match x.canonical_sqrt() {
            Some(root_x) => {
                for (lhs, rhs) in self
                    .root_x
                    .to_target_array()
                    .into_iter()
                    .zip(<GFp5 as FieldExtension<5>>::to_basefield_array(&root_x).into_iter())
                {
                    out_buffer.set_target(lhs, rhs);
                }
                out_buffer.set_target(self.is_sqrt.target, GFp::ONE);
            }
            None => {
                for limb in self.root_x.to_target_array().into_iter() {
                    out_buffer.set_target(limb, GFp::ZERO);
                }
                out_buffer.set_target(self.is_sqrt.target, GFp::ZERO);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use plonky2::field::types::{Field, Sample};
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
    use rand::thread_rng;

    use super::*;
    use crate::curve::test_utils::{gfp5_random_non_square, gfp5_random_sgn0_eq_0};

    #[test]
    fn test_add() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x_expected = GFp5::sample(&mut rng);
        let y_expected = GFp5::sample(&mut rng);
        let z_expected = x_expected + y_expected;

        let x = builder.constant_quintic_ext(x_expected);
        let y = builder.constant_quintic_ext(y_expected);
        let z = builder.add_quintic_ext(x, y);
        builder.register_quintic_ext_public_input(z);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_quintic_ext_target(z, z_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_mul() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x_expected = GFp5::sample(&mut rng);
        let y_expected = GFp5::sample(&mut rng);
        let z_expected = x_expected * y_expected;

        let x = builder.constant_quintic_ext(x_expected);
        let y = builder.constant_quintic_ext(y_expected);
        let z = builder.mul_quintic_ext(x, y);
        builder.register_quintic_ext_public_input(z);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_quintic_ext_target(z, z_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_sub() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x_expected = GFp5::sample(&mut rng);
        let y_expected = GFp5::sample(&mut rng);
        let z_expected = x_expected - y_expected;

        let x = builder.constant_quintic_ext(x_expected);
        let y = builder.constant_quintic_ext(y_expected);
        let z = builder.sub_quintic_ext(x, y);
        builder.register_quintic_ext_public_input(z);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_quintic_ext_target(z, z_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_div() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x_expected = GFp5::sample(&mut rng);
        let y_expected = GFp5::sample(&mut rng);
        let z_expected = x_expected / y_expected;

        let x = builder.constant_quintic_ext(x_expected);
        let y = builder.constant_quintic_ext(y_expected);
        let z = builder.div_quintic_ext(x, y);
        builder.register_quintic_ext_public_input(z);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_quintic_ext_target(z, z_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_inverse_quintic_ext() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x_expected = GFp5::sample(&mut rng);
        let x_inv_expected = x_expected.inverse();

        let x = builder.constant_quintic_ext(x_expected);
        let x_inv = builder.inverse_quintic_ext(x);
        builder.register_quintic_ext_public_input(x_inv);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_quintic_ext_target(x_inv, x_inv_expected);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_any_sqrt_quintic_ext() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = GFp5::sample(&mut rng);
        let square_expected = x * x;

        builder.constant_quintic_ext(square_expected);

        let circuit = builder.build::<C>();

        let pw = PartialWitness::new();
        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }
    #[test]

    fn test_canonical_sqrt_quintic_ext() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let x = gfp5_random_sgn0_eq_0();
        let square_expected = x * x;

        let square = builder.constant_quintic_ext(square_expected);
        let sqrt = builder.canonical_sqrt_quintic_ext(square);
        builder.register_quintic_ext_public_input(sqrt);

        let circuit = builder.build::<C>();

        let mut pw = PartialWitness::new();
        pw.set_quintic_ext_target(sqrt, x);

        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }

    #[test]
    fn test_try_any_sqrt_quintic_ext() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let mut rng = thread_rng();

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let zero = builder.zero_quintic_ext();
        let true_target = builder.constant_bool(true);
        let false_target = builder.constant_bool(false);

        let x = GFp5::sample(&mut rng);
        let square_expected = x * x;

        let square = builder.constant_quintic_ext(square_expected);
        let (_, is_square) = builder.try_any_sqrt_quintic_ext(square);
        builder.connect(true_target.target, is_square.target);

        let non_square = gfp5_random_non_square();
        let non_square = builder.constant_quintic_ext(non_square);
        let (should_be_zero, is_square) = builder.try_any_sqrt_quintic_ext(non_square);
        builder.connect(false_target.target, is_square.target);
        builder.connect_quintic_ext(should_be_zero, zero);

        let circuit = builder.build::<C>();

        let pw = PartialWitness::new();
        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }
    #[test]

    fn test_try_canonical_sqrt_quintic_ext() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let zero = builder.zero_quintic_ext();
        let true_target = builder.constant_bool(true);
        let false_target = builder.constant_bool(false);

        let x = gfp5_random_sgn0_eq_0();
        let square_expected = x * x;

        let square = builder.constant_quintic_ext(square_expected);
        let (_, is_square) = builder.try_canonical_sqrt_quintic_ext(square);
        builder.connect(true_target.target, is_square.target);

        let non_square = gfp5_random_non_square();
        let non_square = builder.constant_quintic_ext(non_square);
        let (should_be_zero, is_square) = builder.try_canonical_sqrt_quintic_ext(non_square);
        builder.connect(false_target.target, is_square.target);
        builder.connect_quintic_ext(should_be_zero, zero);

        let circuit = builder.build::<C>();

        let pw = PartialWitness::new();
        let proof = circuit.prove(pw)?;
        circuit.verify(proof)
    }
}
