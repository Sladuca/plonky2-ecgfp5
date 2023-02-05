// toy example of a circuit that checks a schnorr signatuse

use log::Level;
use plonky2::{plonk::{config::{Hasher, PoseidonGoldilocksConfig, GenericConfig, GenericHashOut}, circuit_data::{CircuitConfig, CircuitData}, circuit_builder::CircuitBuilder, prover::prove}, iop::{witness::PartialWitness, target::Target}, util::timing::{self, TimingTree}, hash::{hash_types::HashOut, hashing::{SPONGE_WIDTH, hash_n_to_hash_no_pad, hash_n_to_m_no_pad}, poseidon::PoseidonPermutation}};
use plonky2_ecdsa::gadgets::{nonnative::CircuitBuilderNonNative, biguint::WitnessBigUint};
use plonky2_ecgfp5::{curve::{scalar_field::Scalar, curve::Point}, gadgets::{curve::{CircuitBuilderEcGFp5, PartialWitnessCurve}, scalar_field::CircuitBuilderScalar, base_field::{CircuitBuilderGFp5, QuinticExtensionTarget, PartialWitnessQuinticExt}}};
use plonky2_field::{types::{Field, Sample, PrimeField}, extension::quintic::QuinticExtension};
use rand::{thread_rng, Rng};
use env_logger::{try_init_from_env, Env, DEFAULT_FILTER_ENV};


const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

pub const SPONGE_RATE: usize = 8;


// we define a hash function whose digest is 5 GFp5 elems
// note: this doesn't apply any padding, so this is vulnerable to length extension attacks
fn sig_hash(message: &[F]) -> [F; 5] {
	let mut res = [F::ZERO; 5];
	let out = hash_n_to_m_no_pad::<F, PoseidonPermutation>(message, 5);
	res.copy_from_slice(&out[..5]);

	res
}

fn sig_hash_circuit(builder: &mut CircuitBuilder<F, D>, message: &[Target]) -> [Target; 5] {
	let mut state = [(); SPONGE_WIDTH].map(|_| builder.zero());

    // Absorb all input chunks.
    for input_chunk in message.chunks(SPONGE_RATE) {
        state[..input_chunk.len()].copy_from_slice(input_chunk);
        state = builder.permute::<<PoseidonGoldilocksConfig as GenericConfig<D>>::Hasher>(state);
    }

    // Squeeze until we have the desired number of outputs.
	[
		state[0],
		state[1],
		state[2],
		state[3],
		state[4],
	]
}

pub fn main() {
	init_logger();
	let mut rng = thread_rng();


	// 0. generate keypair
	let sk = Scalar::sample(&mut rng);
	let pk = Point::GENERATOR * sk;


	// 1. message digest & encoding

	let message_bytes = b"I'm going to be the king of pirates!";
	let message_elems = message_bytes.map(|b| F::from_canonical_u8(b));
	let m = sig_hash(&message_elems);

	// 2. sample random k
	
	let k  = Scalar::sample(&mut rng);

	// 3. compute R = k*G
	let r = Point::GENERATOR * k;

	// 4. e = H(R || m)
	let mut preimage = r.encode().0.to_vec();
	preimage.extend(m);
	let e_elems = sig_hash(&preimage);
	let e = Scalar::from_gfp5(QuinticExtension(e_elems));

	// 5. s = k - e*sk
	// signature = (s, e)
	let s = k - e * sk;

	// 6. verify signature in circuit

	let config = CircuitConfig::standard_recursion_config();
	let mut builder = CircuitBuilder::<F, D>::new(config);

	let m = builder.constant_quintic_ext(QuinticExtension(m));
	let s = builder.constant_nonnative::<Scalar>(s);
	let e = builder.constant_nonnative::<Scalar>(e);

	// r_v = s*G + e*pk

	let g_to_s = builder.curve_scalar_mul_const(Point::GENERATOR, &s);
	let pk_to_e = builder.curve_scalar_mul_const(pk, &e);
	let r_v = builder.curve_add(g_to_s, pk_to_e);

	// e_v = H(R || m)

	let mut preimage = builder.curve_encode_to_quintic_ext(r_v).0.to_vec();
	preimage.extend(&m.0);
	let e_v_ext = QuinticExtensionTarget(
		sig_hash_circuit(&mut builder, &preimage)
	);
	let e_v = builder.encode_quintic_ext_as_scalar(e_v_ext);

	// check e_v == e
	builder.connect_nonnative(&e, &e_v);

	// build circuit
	let circuit = builder.build::<C>();
	let CircuitData { prover_only, common, verifier_only: _ } = &circuit;

	let pw = PartialWitness::new();
	let mut timing =  TimingTree::new("prove", Level::Debug);
	let proof = prove(prover_only, common, pw, &mut timing).expect("prover failed");
	timing.print();

	circuit.verify(proof).expect("verifier failed");
}

fn init_logger() {
	let _ = try_init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "debug"));
}
