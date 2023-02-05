use num::BigUint;
use plonky2::{hash::hash_types::RichField, plonk::circuit_builder::CircuitBuilder, iop::witness::{PartialWitness, WitnessWrite, Witness}};
use plonky2_ecdsa::gadgets::biguint::BigUintTarget;
use plonky2_field::{extension::Extendable, types::{Field, PrimeField}};

use crate::curve::scalar_field::Scalar;


pub trait CircuitBuilderScalar<F: RichField + Extendable<D>, const D: usize> {
	fn register_scalar_public_input(&mut self, scalar: &BigUintTarget);
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderScalar<F, D> for CircuitBuilder<F, D> {
	fn register_scalar_public_input(&mut self, target: &BigUintTarget) {
		for limb in target.limbs.iter() {
			self.register_public_input(limb.0);
		}
	}
}

pub trait PartialWitnessScalar<F: RichField> {
	fn set_scalar_target(&mut self, target: &BigUintTarget, value: Scalar);
	fn get_scalar_target(&mut self, target: &BigUintTarget) -> Scalar;
}

impl<F: RichField> PartialWitnessScalar<F> for PartialWitness<F> {
	fn set_scalar_target(&mut self, target: &BigUintTarget, value: Scalar) {
		let value = value.to_canonical_biguint();
		for (&limb, limb_value) in target.limbs.iter().zip(value.to_u32_digits()) {
			self.set_target(limb.0, F::from_canonical_u32(limb_value));
		}
	}

	fn get_scalar_target(&mut self, target: &BigUintTarget) -> Scalar {
		let mut limbs = Vec::new();
		for limb in target.limbs.iter() {
			limbs.push(self.get_target(limb.0).to_canonical_u64() as u32);
		}

		let as_biguint = BigUint::from_slice(&limbs);
		Scalar::from_noncanonical_biguint(as_biguint)
	}
}

