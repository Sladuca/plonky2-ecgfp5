use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use plonky2_ecgfp5::curve::scalar_field::Scalar;
use plonky2_field::{types::{Sample, Field}, ops::Square};

pub fn bench_scalar(c: &mut Criterion) {
    c.bench_function(
        "add",
        |b| {
            b.iter_batched(
                || (Scalar::rand(), Scalar::rand()),
                |(x, y)| {
                    black_box(x + y)
                },
                BatchSize::SmallInput
            )
        }
    );

    c.bench_function(
        "mul",
        |b| {
            b.iter_batched(
                || (Scalar::rand(), Scalar::rand()),
                |(x, y)| {
                    black_box(x * y)
                },
                BatchSize::SmallInput
            )
        }
    );

    c.bench_function(
        "square",
        |b| {
            b.iter_batched(
                || Scalar::rand(),
                |x| {
                    black_box(x.square())
                },
                BatchSize::SmallInput
            )
        }
    );

    c.bench_function(
        "try_inverse",
        |b| {
            b.iter_batched(
                || Scalar::rand(),
                |x| {
                    black_box(x.try_inverse());
                },
                BatchSize::SmallInput
            )
        }
    );

    c.bench_function(
        "batch_multiplicative_inverse-tiny",
        |b| {
            b.iter_batched(
                || (0..2).into_iter().map(|_| Scalar::rand()).collect::<Vec<_>>(),
                |x| Scalar::batch_multiplicative_inverse(&x),
                BatchSize::SmallInput,
            )
        },
    );

    c.bench_function(
        "batch_multiplicative_inverse-small",
        |b| {
            b.iter_batched(
                || (0..4).into_iter().map(|_| Scalar::rand()).collect::<Vec<_>>(),
                |x| Scalar::batch_multiplicative_inverse(&x),
                BatchSize::SmallInput,
            )
        },
    );

    c.bench_function(
        "batch_multiplicative_inverse-medium",
        |b| {
            b.iter_batched(
                || (0..16).into_iter().map(|_| Scalar::rand()).collect::<Vec<_>>(),
                |x| Scalar::batch_multiplicative_inverse(&x),
                BatchSize::SmallInput,
            )
        },
    );

    c.bench_function(
        "batch_multiplicative_inverse-large",
        |b| {
            b.iter_batched(
                || (0..256).into_iter().map(|_| Scalar::rand()).collect::<Vec<_>>(),
                |x| Scalar::batch_multiplicative_inverse(&x),
                BatchSize::LargeInput,
            )
        },
    );

    c.bench_function(
        "batch_multiplicative_inverse-huge",
        |b| {
            b.iter_batched(
                || (0..65536).into_iter().map(|_| Scalar::rand()).collect::<Vec<_>>(),
                |x| Scalar::batch_multiplicative_inverse(&x),
                BatchSize::LargeInput,
            )
        },
    );
}

criterion_group!(benches, bench_scalar);
criterion_main!(benches);