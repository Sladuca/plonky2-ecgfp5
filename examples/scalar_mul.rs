use log::Level;
use plonky2::{plonk::{config::{PoseidonGoldilocksConfig, GenericConfig}, circuit_data::{CircuitConfig, CircuitData}, circuit_builder::CircuitBuilder, prover::prove}, iop::witness::PartialWitness, util::timing::TimingTree};
use plonky2_ecdsa::gadgets::nonnative::CircuitBuilderNonNative;
use plonky2_ecgfp5::{curve::{scalar_field::Scalar, curve::Point}, gadgets::curve::{CircuitBuilderEcGFp5, PartialWitnessCurve}};
use plonky2_field::types::Sample;
use rand::thread_rng;
use env_logger::{try_init_from_env, Env, DEFAULT_FILTER_ENV};


const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

pub fn main() {
	init_logger();
	let mut rng = thread_rng();

	// scalar mul
	println!("testing_scalar_mul...");

	let config = CircuitConfig::standard_recursion_config();
	let mut builder = CircuitBuilder::<F, D>::new(config);

	let p = Point::sample(&mut rng);
	let s = Scalar::sample(&mut rng);
	let prod_expected = p * s;

	let p = builder.curve_constant(p.to_weierstrass());
	let s = builder.constant_nonnative(s);

	let prod = builder.curve_scalar_mul(p, &s);
	builder.register_curve_public_input(prod);
	
	builder.print_gate_counts(0);
	let circuit = builder.build::<C>();

	let mut pw = PartialWitness::new();
	pw.set_curve_target(prod, prod_expected.to_weierstrass());

	let CircuitData { prover_only, common, verifier_only: _ } = &circuit;

	let mut timing =  TimingTree::new("prove", Level::Debug);
	let proof = prove(prover_only, common, pw, &mut timing).expect("prover failed");
	timing.print();

	circuit.verify(proof).expect("verifier failed");


	// scalar mul const
	println!("testing scalar_mul_const...");

	let config = CircuitConfig::standard_recursion_config();
	let mut builder = CircuitBuilder::<F, D>::new(config);

	let p = Point::sample(&mut rng);
	let s = Scalar::sample(&mut rng);
	let prod_expected = p * s;

	let s = builder.constant_nonnative(s);

	let prod = builder.curve_scalar_mul_const(p, &s);
	builder.register_curve_public_input(prod);
	
	builder.print_gate_counts(0);
	let circuit = builder.build::<C>();

	let mut pw = PartialWitness::new();
	pw.set_curve_target(prod, prod_expected.to_weierstrass());

	let CircuitData { prover_only, common, verifier_only: _ } = &circuit;

	let mut timing =  TimingTree::new("prove", Level::Debug);
	let proof = prove(prover_only, common, pw, &mut timing).expect("prover failed");
	timing.print();

	circuit.verify(proof).expect("verifier failed");
}

fn init_logger() {
	let _ = try_init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "debug"));
}
