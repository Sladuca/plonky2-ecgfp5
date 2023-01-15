use plonky2_field::{extension::quintic::QuinticExtension, types::Sample};
use rand::thread_rng;
use super::{GFp5, GFp, base_field::{SquareRoot, Sgn0}};

pub fn gfp5_random_non_square() -> GFp5 {
    let mut rng = thread_rng();
    loop {
        let attempt = QuinticExtension::<GFp>::sample(&mut rng);
        if let None = attempt.sqrt() {
            return attempt;
        }
    }
}

pub fn gfp5_random_sgn0_eq_0() -> GFp5 {
    let mut rng = thread_rng();
    loop {
        let attempt = QuinticExtension::<GFp>::sample(&mut rng);
        if false == attempt.sgn0() {
            return attempt;
        }
    }
}
