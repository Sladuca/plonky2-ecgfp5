extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use plonky2::field::extension::quintic::QuinticExtension;
use core::marker::PhantomData;
use std::ops::Div;

use num::{Integer, Zero, bigint::BigUint};

use plonky2::field::extension::{Extendable, FieldExtension};
use plonky2::field::types::{PrimeField, PrimeField64, Field};
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct QuinticExtensionTarget([Target; 5]);

impl QuinticExtensionTarget {
    pub fn new(limbs: [Target; 5]) -> Self {
        Self(limbs)
    }

    pub fn to_target_array(&self)  -> [Target; 5] {
        self.0
    }
}

pub trait CircuitBuilderGfp5<F: RichField + Extendable<5>> {
    fn add_virtual_gfp5_target(&mut self) -> QuinticExtensionTarget;
    fn zero_gfp5(&mut self) -> QuinticExtensionTarget;
    fn one_gfp5(&mut self) -> QuinticExtensionTarget;
    fn neg_gfp5(&mut self, a: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn add_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn sub_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn mul_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn div_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn inverse_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn any_sqrt_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn canonical_sqrt_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;

    fn square_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget;
    fn add_many_gfp5(&mut self, terms: Vec<QuinticExtensionTarget>) -> QuinticExtensionTarget;
    fn dot_product_gfp5(&mut self, a: Vec<QuinticExtensionTarget>, b: Vec<QuinticExtensionTarget>) -> QuinticExtensionTarget;
}

impl<F: RichField + Extendable<D> + Extendable<5>, const D: usize> CircuitBuilderGfp5<F> for CircuitBuilder<F, D> {
    fn add_virtual_gfp5_target(&mut self) -> QuinticExtensionTarget {
        QuinticExtensionTarget::new([
            self.add_virtual_target(),
            self.add_virtual_target(),
            self.add_virtual_target(),
            self.add_virtual_target(),
            self.add_virtual_target(),
        ])
    }

    fn zero_gfp5(&mut self) -> QuinticExtensionTarget {
        QuinticExtensionTarget::new([self.zero(); 5])
    }

    fn one_gfp5(&mut self) -> QuinticExtensionTarget {
        QuinticExtensionTarget::new([self.one(), self.zero(), self.zero(), self.zero(), self.zero()])
    }

    fn neg_gfp5(&mut self, a: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
        QuinticExtensionTarget::new([
            self.neg(a0),
            self.neg(a1),
            self.neg(a2),
            self.neg(a3),
            self.neg(a4),
        ])
    }

    fn add_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget {
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

    fn sub_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget {
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


    fn mul_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let QuinticExtensionTarget([a0, a1, a2, a3, a4]) = a;
        let QuinticExtensionTarget([b0, b1, b2, b3, b4]) = b;

        // c0 ← a0b0 + 3(a1b4 + a2b3 + a3b2 + a4b1)
        // c1 ← a0b1 + a1b0 + 3(a2b4 + a3b3 + a4b2)
        // c2 ← a0b2 + a1b1 + a2b0 + 3(a3b4 + a4b3)
        // c3 ← a0b3 + a1b2 + a2b1 + a3b0 + 3a4b4
        // c4 ← a0b4 + a1b3 + a2b2 + a3b1 + a4b0 
        
        let mut terms = Vec::new();
        terms.push(self.mul(a1, b4));
        terms.push(self.mul(a2, b3));
        terms.push(self.mul(a3, b2));
        terms.push(self.mul(a4, b1));
        let mut c0 = self.add_many(terms);
        c0 = self.mul_const(F::from_canonical_u32(3), c0); 
        c0 = self.add(c0, self.mul(a0, b0));

        let mut terms = Vec::new();
        terms.push(self.mul(a2, b4));
        terms.push(self.mul(a3, b3));
        terms.push(self.mul(a4, b2));
        let mut c1 = self.add_many(terms);
        c1 = self.mul_const(F::from_canonical_u32(3), c1);
        c1 = self.add(c1, self.mul(a1, b0));
        c1 = self.add(c1, self.mul(a0, b1));

        let mut c2 = self.mul(a3, b4);
        c1 = self.add(c1, self.mul(a4, b3));
        c1 = self.mul_const(F::from_canonical_u32(3), c1);
        c1 = self.add(c1, self.mul(a2, b0));
        c1 = self.add(c1, self.mul(a1, b1));
        c1 = self.add(c1, self.mul(a0, b2));

        let mut c3 = self.mul(a4, b4);
        c3 = self.mul_const(F::from_canonical_u32(3), c3);
        c3 = self.add(c3, self.mul(a3, b0));
        c3 = self.add(c3, self.mul(a2, b1));
        c3 = self.add(c3, self.mul(a1, b2));
        c3 = self.add(c3, self.mul(a0, b3));

        let mut c4 = self.mul(a4, b0);
        c4 = self.add(c4, self.mul(a3, b1));
        c4 = self.add(c4, self.mul(a2, b2));
        c4 = self.add(c4, self.mul(a1, b3));
        c4 = self.add(c4, self.mul(a0, b4));

        QuinticExtensionTarget::new([c0, c1, c2, c3, c4])
    }

    fn div_gfp5(&mut self, a: QuinticExtensionTarget, b: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let quotient = self.add_virtual_gfp5_target();
        self.add_simple_generator(QuinticQuotientGenerator::new(a, b, quotient));
        let quotient_times_denominator = self.mul_gfp5(quotient, b);

        for (lhs, rhs) in quotient_times_denominator.to_target_array().into_iter().zip(a.to_target_array().into_iter()) {
            self.connect(lhs, rhs);
        }

        quotient
    }

    fn inverse_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let one = self.one_gfp5();
        let inverse = self.add_virtual_gfp5_target();
        self.add_simple_generator(QuinticQuotientGenerator::new(one, x, inverse));

        let should_be_one = self.mul_gfp5(inverse, x);
        for (lhs, rhs) in should_be_one.to_target_array().into_iter().zip(one.to_target_array().into_iter()) {
            self.connect(lhs, rhs);
        }

        inverse
    }

    fn any_sqrt_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let root_x = self.add_virtual_gfp5_target();
        self.add_simple_generator(QuinticSqrtGenerator::new(x, root_x));
        
        let should_be_x = self.square_gfp5(root_x);
        for (lhs, rhs) in should_be_x.to_target_array().into_iter().zip(x.to_target_array().into_iter()) {
            self.connect(lhs, rhs);
        }

        root_x
    }

    fn canonical_sqrt_gfp5(&mut self, x: QuinticExtensionTarget) -> QuinticExtensionTarget {
        let root_x = self.any_sqrt_gfp5(x);

        let [x0, _, _, _, _] = root_x.to_target_array();
        let bit_decomp = self.split_le_base::<2>(x0, 64);

        let one = self.one();
        self.connect(bit_decomp[0], one);

        root_x
    }

    fn square_gfp5(&mut self, a: QuinticExtensionTarget) -> QuinticExtensionTarget {
        self.mul_gfp5(a, a)
    }

    fn add_many_gfp5(&mut self, terms: Vec<QuinticExtensionTarget>) -> QuinticExtensionTarget {
        let mut sum = self.zero_gfp5();
        for term in terms {
            sum = self.add_gfp5(sum, term);
        }
        sum 
    }

    fn dot_product_gfp5(&mut self, a: Vec<QuinticExtensionTarget>, b: Vec<QuinticExtensionTarget>) -> QuinticExtensionTarget {
        let mut terms = Vec::new();
        for (a, b) in a.into_iter().zip(b.into_iter()) {
            terms.push(self.mul_gfp5(a, b));
        }
        self.add_many_gfp5(terms) 
    }

    
}

#[derive(Debug)]
pub struct QuinticQuotientGenerator {
    numerator: QuinticExtensionTarget,
    denominator: QuinticExtensionTarget,
    quotient: QuinticExtensionTarget,
}

impl QuinticQuotientGenerator {
    pub fn new(numerator: QuinticExtensionTarget, denominator: QuinticExtensionTarget, quotient: QuinticExtensionTarget) -> Self {
        QuinticQuotientGenerator { numerator, denominator, quotient }
    }
}

impl<F: RichField + Extendable<5>> SimpleGenerator<F>
    for QuinticQuotientGenerator 
{
    fn dependencies(&self) -> Vec<Target> {
        let mut deps = self.numerator.to_target_array().to_vec();
        deps.extend(self.denominator.to_target_array());
        deps
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let numerator_limbs = self.numerator.to_target_array().map(|t| witness.get_target(t));
        let numerator = QuinticExtension::<F>::from_basefield_array(numerator_limbs);

        let denominator_limbs = self.denominator.to_target_array().map(|t| witness.get_target(t));
        let denominator = QuinticExtension::<F>::from_basefield_array(denominator_limbs);

        let quotient = numerator / denominator;
        for (lhs, rhs) in self.quotient.to_target_array().into_iter().zip(<QuinticExtension<F> as FieldExtension<5>>::to_basefield_array(&quotient).into_iter()) {
            out_buffer.set_target(lhs, rhs);
        }
    }
}

#[derive(Debug)]
pub struct QuinticSqrtGenerator<F: RichField + Extendable<5>> {
    x: QuinticExtensionTarget,
    root_x: QuinticExtensionTarget,
    gs: Vec<F>
}

impl<F: RichField + Extendable<5>> QuinticSqrtGenerator<F> {
    pub fn new(x: QuinticExtensionTarget, root_x: QuinticExtensionTarget) -> Self {
        let g = F::primitive_root_of_unity(32);
        let gs = (0..32).map(|i| g.exp_power_of_2(i)).collect::<Vec<_>>();
        QuinticSqrtGenerator { x, root_x, gs }
    }
}

impl<F: RichField + Extendable<5>> SimpleGenerator<F>
    for QuinticSqrtGenerator<F>
{
    fn dependencies(&self) -> Vec<Target> {
        self.x.to_target_array().to_vec()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let x_limbs = self.x.to_target_array().map(|t| witness.get_target(t));
        let x = QuinticExtension::<F>::from_basefield_array(x_limbs);

        let root_x = canonical_sqrt_gfp5(x, &self.gs);
        
        for (lhs, rhs) in self.root_x.to_target_array().into_iter().zip(<QuinticExtension<F> as FieldExtension<5>>::to_basefield_array(&root_x).into_iter()) {
            out_buffer.set_target(lhs, rhs);
        }
    }
}

fn canonical_sqrt_gfp5<F: RichField + Extendable<5>>(x: QuinticExtension<F>, gs: &[F]) -> QuinticExtension<F> {
    let root_x = sqrt_gfp5(x, gs);
    let [x0, _, _, _, _] = <QuinticExtension<F> as FieldExtension<5>>::to_basefield_array(&x);
    let x0_u64 = x0.to_canonical_u64();

    if x0_u64 & 1 == 0 {
        -root_x
    } else {
        root_x
    }
}

fn sqrt_gfp5<F: RichField + Extendable<5>>(x: QuinticExtension<F>, gs: &[F]) -> QuinticExtension<F> {
    let p = F::order();
    let mut r = BigUint::zero();
    for _ in 0..4 {
        r = p * r + p;
    }
    r += BigUint::from(1u32);

    let q = BigUint::from(2u32).pow(32) - BigUint::from(1u32);
    let y_ext = x.exp_biguint(&r);
    let y = <QuinticExtension::<F> as FieldExtension<5>>::to_basefield_array(&y_ext)[0];

    let mut u = y.exp_biguint(&(q + BigUint::from(1u32)).div(BigUint::from(2u32)));
    let mut v = y.exp_biguint(&q);

    for i in (1..32).rev() {
        let w = v.exp_power_of_2(i - 1);
        if w == F::ZERO - F::ONE {
            u = gs[32 - i - 1] * u;
            v = gs[32 - i] * v;
        }
    }

    let root_xr = match v {
        F::ZERO | F::ONE => u,
        _ => panic!("not a square")
    };

    let x_2_31 = x.exp_power_of_2(31);
    let x_2_63 = x_2_31.exp_power_of_2(32);

    let denom = (x_2_63 * x).div(x_2_31);
    let num = QuinticExtension::from_basefield_array([root_xr, F::ZERO, F::ZERO, F::ZERO, F::ZERO]);
    num.div(denom)
}
