use core::ops::Range;
use alloc::vec::Vec;

use plonky2::{plonk::{circuit_data::CircuitConfig, vars::{EvaluationVars, EvaluationVarsBase, EvaluationTargets}, circuit_builder::CircuitBuilder}, hash::hash_types::RichField, gates::{gate::Gate, util::StridedConstraintConsumer}, iop::{ext_target::ExtensionTarget, target::Target, generator::{SimpleGenerator, GeneratedValues, WitnessGenerator}, witness::{PartitionWitness, Witness, WitnessWrite}}};
use plonky2_field::{extension::{Extendable}, types::Field};

/// A gate which can perform a weighted multiply-add, i.e. `result = c0 x y + c1 z`. If the config
/// supports enough routed wires, it can support several such operations in one gate.
#[derive(Debug, Clone)]
pub struct ArithmeticGFp5Gate {
    /// Number of arithmetic operations performed by an arithmetic gate.
    pub num_ops: usize,
}

// extension degree of the GFp5
const DEGREE: usize = 5;

// need 20 wires per operation
// each element needs 5 wires
// each operation needs 4 elements (multiplicand 0, multiplicand 1, addend, output)
const WIRES_PER_OP: usize = 4 * DEGREE;

impl ArithmeticGFp5Gate {
    pub fn new_from_config(config: &CircuitConfig) -> Self {
        Self {
            num_ops: Self::num_ops(config),
        }
    }

    /// Determine the maximum number of operations that can fit in one gate for the given config.
    pub(crate) fn num_ops(config: &CircuitConfig) -> usize {
        config.num_routed_wires / WIRES_PER_OP
    }

    pub fn wires_ith_multiplicand_0(i: usize) -> Range<usize> {
        4 * DEGREE * i..4 * DEGREE * i + DEGREE
    }
    pub fn wires_ith_multiplicand_1(i: usize) -> Range<usize> {
        4 * DEGREE * i + DEGREE..4 * DEGREE * i + 2 * DEGREE
    }
    pub fn wires_ith_addend(i: usize) -> Range<usize> {
        4 * DEGREE * i + 2 * DEGREE..4 * DEGREE * i + 3 * DEGREE
    }
    pub fn wires_ith_output(i: usize) -> Range<usize> {
        4 * DEGREE * i + 3 * DEGREE..4 * DEGREE * i + 4 * DEGREE
    }
}

impl<F: RichField + Extendable<D>, const D: usize> Gate<F, D> for ArithmeticGFp5Gate {
    fn id(&self) -> String {
        format!("{self:?}")
    }

    fn eval_unfiltered(&self, vars: EvaluationVars<F, D>) -> Vec<F::Extension> {
        let const_0_limbs = vars.local_constants[0..DEGREE].try_into().unwrap();
        let const_1_limbs = vars.local_constants[DEGREE..2*DEGREE].try_into().unwrap();

        let mut constraints = Vec::new();
        for i in 0..self.num_ops {
            let multiplicand_0_limbs: [F::Extension; 5] = vars.local_wires[Self::wires_ith_multiplicand_0(i)].try_into().unwrap();
			let multiplicand_1_limbs: [F::Extension; 5] = vars.local_wires[Self::wires_ith_multiplicand_1(i)].try_into().unwrap();
			let addend_limbs: [F::Extension; 5] = vars.local_wires[Self::wires_ith_addend(i)].try_into().unwrap();
			let output_limbs: [F::Extension; 5] = vars.local_wires[Self::wires_ith_output(i)].try_into().unwrap();

			let xy_limbs = gfp5_mul_limbwise(multiplicand_0_limbs, multiplicand_1_limbs);
			let axy_limbs = gfp5_mul_limbwise(const_0_limbs, xy_limbs);
			let bz_limbs = gfp5_mul_limbwise(const_1_limbs, addend_limbs);

			for (output_limb, (axy_limb, bz_limb)) in output_limbs.into_iter().zip(axy_limbs.into_iter().zip(bz_limbs)) {
				constraints.push(output_limb - (axy_limb + bz_limb));
			}
        }

        constraints
    }

    fn eval_unfiltered_base_one(
        &self,
        vars: EvaluationVarsBase<F>,
        mut yield_constr: StridedConstraintConsumer<F>,
    ) {
        let const_0_limbs = vars.local_constants.view(0..DEGREE).try_into().unwrap();
        let const_1_limbs = vars.local_constants.view(DEGREE..2*DEGREE).try_into().unwrap();

        for i in 0..self.num_ops {
			let multiplicand_0_limbs = vars.local_wires.view(Self::wires_ith_multiplicand_0(i)).try_into().unwrap();
			let multiplicand_1_limbs = vars.local_wires.view(Self::wires_ith_multiplicand_1(i)).try_into().unwrap();
			let addend_limbs = vars.local_wires.view(Self::wires_ith_addend(i)).try_into().unwrap();
			let output_limbs: [F; 5] = vars.local_wires.view(Self::wires_ith_output(i)).try_into().unwrap();

			let xy_limbs = gfp5_mul_limbwise(multiplicand_0_limbs, multiplicand_1_limbs);
			let axy_limbs = gfp5_mul_limbwise(const_0_limbs, xy_limbs);
			let bz_limbs = gfp5_mul_limbwise(const_1_limbs, addend_limbs);

			for (output_limb, (axy_limb, bz_limb)) in output_limbs.into_iter().zip(axy_limbs.into_iter().zip(bz_limbs)) {
				yield_constr.one(output_limb - (axy_limb + bz_limb));
			}
        }
    }

    fn eval_unfiltered_circuit(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        vars: EvaluationTargets<D>,
    ) -> Vec<ExtensionTarget<D>> {
        let const_0_limbs: [ExtensionTarget<D>; 5] = vars.local_constants[0..DEGREE].try_into().unwrap();
        let const_1_limbs: [ExtensionTarget<D>; 5] = vars.local_constants[DEGREE..2*DEGREE].try_into().unwrap();

        let mut constraints = Vec::new();
        for i in 0..self.num_ops {
            let multiplicand_0_limbs: [ExtensionTarget<D>; 5] = vars.local_wires[Self::wires_ith_multiplicand_0(i)].try_into().unwrap();
			let multiplicand_1_limbs: [ExtensionTarget<D>; 5] = vars.local_wires[Self::wires_ith_multiplicand_1(i)].try_into().unwrap();
			let addend_limbs: [ExtensionTarget<D>; 5] = vars.local_wires[Self::wires_ith_addend(i)].try_into().unwrap();
			let output_limbs: [ExtensionTarget<D>; 5] = vars.local_wires[Self::wires_ith_output(i)].try_into().unwrap();

			let xy_limbs = gfp5_mul_limbwise_circuit_lifted(builder, multiplicand_0_limbs, multiplicand_1_limbs);
			let axy_limbs = gfp5_mul_limbwise_circuit_lifted(builder, const_0_limbs, xy_limbs);
			let bz_limbs = gfp5_mul_limbwise_circuit_lifted(builder, const_1_limbs, addend_limbs);
			
			for (output_limb, (axy_limb, bz_limb)) in output_limbs.into_iter().zip(axy_limbs.into_iter().zip(bz_limbs)) {
				let computed_output_limb = builder.add_extension(axy_limb, bz_limb);
				let diff = builder.sub_extension(output_limb, computed_output_limb);
				constraints.push(diff);
			}
        }

        constraints
    }

    fn generators(&self, row: usize, local_constants: &[F]) -> Vec<Box<dyn WitnessGenerator<F>>> {
        (0..self.num_ops)
            .map(|op_idx| {
                let g: Box<dyn WitnessGenerator<F>> = Box::new(
                    ArithmeticGFp5Generator {
                        row,
                        const_0_limbs: local_constants[0..DEGREE].try_into().unwrap(),
                        const_1_limbs: local_constants[DEGREE..2*DEGREE].try_into().unwrap(),
                        op_idx,
                    }
                    .adapter(),
                );
                g
            })
            .collect()
    }

    fn num_wires(&self) -> usize {
        self.num_ops * WIRES_PER_OP
    }

    fn num_constants(&self) -> usize {
        2 * DEGREE
    }

    fn degree(&self) -> usize {
        3
    }

    fn num_constraints(&self) -> usize {
        self.num_ops * DEGREE
    }
}

#[derive(Clone, Debug)]
pub struct ArithmeticGFp5Generator<F: RichField + Extendable<D>, const D: usize> {
	row: usize,
	const_0_limbs: [F; 5],
	const_1_limbs: [F; 5],
	op_idx: usize,
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F>
    for ArithmeticGFp5Generator<F, D>
{
    fn dependencies(&self) -> Vec<Target> {
		ArithmeticGFp5Gate::wires_ith_multiplicand_0(self.op_idx)
			.chain(ArithmeticGFp5Gate::wires_ith_multiplicand_1(self.op_idx))
			.chain(ArithmeticGFp5Gate::wires_ith_addend(self.op_idx))
			.map(|wire| Target::wire(self.row, wire))
			.collect()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let get_wire = |wire: usize| -> F { witness.get_target(Target::wire(self.row, wire)) };
		
		let multiplicand_0_limbs: [F; 5] = ArithmeticGFp5Gate::wires_ith_multiplicand_0(self.op_idx).map(|wire| get_wire(wire)).collect::<Vec<_>>().try_into().unwrap();
		let multiplicand_1_limbs: [F; 5] = ArithmeticGFp5Gate::wires_ith_multiplicand_1(self.op_idx).map(|wire| get_wire(wire)).collect::<Vec<_>>().try_into().unwrap();
		let addend_limbs: [F; 5] = ArithmeticGFp5Gate::wires_ith_addend(self.op_idx).map(|wire| get_wire(wire)).collect::<Vec<_>>().try_into().unwrap();

        let output_limbs = ArithmeticGFp5Gate::wires_ith_output(self.op_idx).map(|wire| Target::wire(self.row, wire));

		let xy_limbs = gfp5_mul_limbwise(multiplicand_0_limbs, multiplicand_1_limbs);
		let axy_limbs = gfp5_mul_limbwise(self.const_0_limbs, xy_limbs);
		let bz_limbs = gfp5_mul_limbwise(self.const_1_limbs, addend_limbs);

		for (output_limb, (axy_limb, bz_limb)) in output_limbs.into_iter().zip(axy_limbs.into_iter().zip(bz_limbs)) {
			out_buffer.set_target(output_limb, axy_limb + bz_limb)
		}
    }
}


fn gfp5_mul_limbwise<F: Field>(a: [F; 5], b: [F; 5]) -> [F; 5] {
	let [a0, a1, a2, a3, a4] = a;
	let [b0, b1, b2, b3, b4] = b;

	// c0 ← a0b0 + 3(a1b4 + a2b3 + a3b2 + a4b1)
	// c1 ← a0b1 + a1b0 + 3(a2b4 + a3b3 + a4b2)
	// c2 ← a0b2 + a1b1 + a2b0 + 3(a3b4 + a4b3)
	// c3 ← a0b3 + a1b2 + a2b1 + a3b0 + 3a4b4
	// c4 ← a0b4 + a1b3 + a2b2 + a3b1 + a4b0

	let three = F::from_canonical_u16(3);

	let c0 = a0 * b0 + three * (a1 * b4 + a2 * b3 + a3 * b2 + a4 * b1);
	let c1 = a0 * b1 + a1 * b0 + three * (a2 * b4 + a3 * b3 + a4 * b2);
	let c2 = a0 * b2 + a1 * b1 + a2 * b0 + three * (a3 * b4 + a4 * b3);
	let c3 = a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0 + three * a4 * b4;
	let c4 = a0 * b4 + a1 * b3 + a2 * b2 + a3 * b1 + a4 * b0;

	[c0, c1, c2, c3, c4]
}

fn gfp5_mul_limbwise_circuit_lifted<F: RichField + Extendable<D>, const D: usize>(builder: &mut CircuitBuilder<F, D>, a: [ExtensionTarget<D>; 5], b: [ExtensionTarget<D>; 5]) -> [ExtensionTarget<D>; 5] {
	let [a0, a1, a2, a3, a4] = a;
	let [b0, b1, b2, b3, b4] = b;

	// c0 ← a0b0 + 3(a1b4 + a2b3 + a3b2 + a4b1)
	// c1 ← a0b1 + a1b0 + 3(a2b4 + a3b3 + a4b2)
	// c2 ← a0b2 + a1b1 + a2b0 + 3(a3b4 + a4b3)
	// c3 ← a0b3 + a1b2 + a2b1 + a3b0 + 3a4b4

	let mut c0 = builder.mul_extension(a4, b1);
	c0 = builder.mul_add_extension(a3, b2, c0);
	c0 = builder.mul_add_extension(a2, b3, c0);
	c0 = builder.mul_add_extension(a1, b4, c0);
	c0 = builder.mul_const_extension(F::from_canonical_u64(3), c0);
	c0 = builder.mul_add_extension(a0, b0, c0);

	let mut c1 = builder.mul_extension(a4, b2);
	c1 = builder.mul_add_extension(a3, b3, c1);
	c1 = builder.mul_add_extension(a2, b4, c1);
	c1 = builder.mul_const_extension(F::from_canonical_u64(3), c1);
	c1 = builder.mul_add_extension(a1, b0, c1);
	c1 = builder.mul_add_extension(a0, b1, c1);

	let mut c2 = builder.mul_extension(a4, b3);
	c2 = builder.mul_add_extension(a3, b4, c2);
	c2 = builder.mul_const_extension(F::from_canonical_u64(3), c2);
	c2 = builder.mul_add_extension(a2, b0, c2);
	c2 = builder.mul_add_extension(a1, b1, c2);
	c2 = builder.mul_add_extension(a0, b2, c2);

	let mut c3 = builder.mul_extension(a4, b4);
	c3 = builder.mul_const_extension(F::from_canonical_u64(3), c3);
	c3 = builder.mul_add_extension(a3, b0, c3);
	c3 = builder.mul_add_extension(a2, b1, c3);
	c3 = builder.mul_add_extension(a1, b2, c3);
	c3 = builder.mul_add_extension(a0, b3, c3);

	let mut c4 = builder.mul_extension(a4, b0);
	c4 = builder.mul_add_extension(a3, b1, c4);
	c4 = builder.mul_add_extension(a2, b2, c4);
	c4 = builder.mul_add_extension(a1, b3, c4);
	c4 = builder.mul_add_extension(a0, b4, c4);

	[c0, c1, c2, c3, c4]
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
	use super::*;

    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::gates::gate_testing::{test_eval_fns, test_low_degree};
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

    #[test]
    fn low_degree() {
        let gate = ArithmeticGFp5Gate::new_from_config(&CircuitConfig::standard_recursion_config());
        test_low_degree::<GoldilocksField, _, 4>(gate);
	}

    #[test]
    fn eval_fns() -> Result<()> {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        let gate = ArithmeticGFp5Gate::new_from_config(&CircuitConfig::standard_recursion_config());
        test_eval_fns::<F, C, _, D>(gate)
    }
}