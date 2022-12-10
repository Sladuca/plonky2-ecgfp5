use plonky2::field::extension::quintic::QuinticExtension;
use plonky2::field::ops::Square;
use crate::base_field::{QuinticExtensionTarget, CircuitBuilderQuinticExt};
use plonky2::field::extension::{Extendable, FieldExtension};
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::{BoolTarget, Target};
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;

fn curve_a<F: RichField + Extendable<5>>() -> QuinticExtension<F> {
	let a = QuinticExtension::<F>::from_canonical_u16(2);
	let b = QuinticExtension::<F>::from_canonical_u16(263);
	let three = QuinticExtension::<F>::from_canonical_u16(3);

	(three * b - a.square()) / three
}


#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct CurveTarget(([QuinticExtensionTarget; 2], BoolTarget));

pub trait CircuitBuilderCurve<F: RichField + Extendable<5>> {
	fn add_virtual_curve_target(&mut self) -> CurveTarget;
	fn curve_constant(&mut self, x: QuinticExtension<F>, y: QuinticExtension<F>, is_inf: bool) -> CurveTarget;
	fn curve_select(&mut self, cond: BoolTarget, a: CurveTarget, b: CurveTarget) -> CurveTarget;
	
	fn curve_add(&mut self, a: CurveTarget, b: CurveTarget) -> CurveTarget;
	fn curve_double(&mut self, a: CurveTarget) -> CurveTarget;
	fn curve_scalar_mul(&mut self, a: CurveTarget, scalar_bits: &[BoolTarget]) -> CurveTarget;

	fn curve_encode_to_quintic_ext(&mut self, a: CurveTarget) -> QuinticExtensionTarget;
	fn curve_decode_from_quintic_ext(&mut self, a: QuinticExtensionTarget) -> CurveTarget;
}

impl<F: RichField + Extendable<5> + Extendable<D>, const D: usize> CircuitBuilderCurve<F> for CircuitBuilder<F, D> {
	fn add_virtual_curve_target(&mut self) -> CurveTarget {
		let x = self.add_virtual_quintic_ext_target();
		let y = self.add_virtual_quintic_ext_target();
		let is_inf = self.add_virtual_bool_target_safe();
		CurveTarget(([x, y], is_inf))
	}

	fn curve_constant(&mut self, x: QuinticExtension<F>, y: QuinticExtension<F>, is_inf: bool) -> CurveTarget {
		let x = self.constant_quintic_ext(x);
		let y = self.constant_quintic_ext(y);
		let is_inf = self.constant_bool(is_inf);
		CurveTarget(([x, y], is_inf))
	}

	fn curve_select(&mut self, cond: BoolTarget, a: CurveTarget, b: CurveTarget) -> CurveTarget {
		let CurveTarget(([ax, ay], a_is_inf)) = a;
		let CurveTarget(([bx, by], b_is_inf)) = b;
		CurveTarget((
			[
				self.select_quintic_ext(cond, ax, bx),
				self.select_quintic_ext(cond, ay, by)
			],
			BoolTarget::new_unsafe(
				self.select(cond, a_is_inf.target, b_is_inf.target)
			)
		))
	}

	fn curve_add(&mut self, a: CurveTarget, b: CurveTarget) -> CurveTarget {
		let CurveTarget(([x1, y1], a_is_inf)) = a;
		let CurveTarget(([x2, y2], b_is_inf)) = b;
		let three = QuinticExtension::<F>::from_canonical_u16(3);

		let sx = self.is_equal_quintic_ext(x1, x2);
		let sy = self.is_equal_quintic_ext(y1, y2);

		let lambda_0_if_sx_0 = self.sub_quintic_ext(y2, y1);
		let mut lambda_0_if_sx_1 = self.square_quintic_ext(x1);
		lambda_0_if_sx_0 = self.mul_const_quintic_ext(three, lambda_0_if_sx_0);
		lambda_0_if_sx_0 = self.add_const_quintic_ext(lambda_0_if_sx_0, curve_a());

		let lambda_1_if_sx_0 = self.add_quintic_ext(y1, y1);
		let mut lambda_1_if_sx_1 = self.sub_quintic_ext(x2, x1);

		// note: paper has a typo. select opposite what the paper says
		let lambda_0 = self.select_quintic_ext(sx, lambda_0_if_sx_0, lambda_0_if_sx_1);
		let lambda_1 = self.select_quintic_ext(sy, lambda_1_if_sx_0, lambda_1_if_sx_1);
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

	// TODO: optimize
	fn curve_double(&mut self, a: CurveTarget) -> CurveTarget {
		self.curve_add(a, a)
	}
}