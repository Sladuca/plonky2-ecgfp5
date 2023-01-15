use plonky2_field::{extension::quintic::QuinticExtension, goldilocks_field::GoldilocksField};

pub(crate) type GFp5 = QuinticExtension<GoldilocksField>;
pub(crate) type GFp = GoldilocksField;

pub mod base_field;
pub mod curve;
pub(crate) mod mul_table;
pub mod scalar_field;

#[cfg(test)]
pub mod test_utils;
