use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use plonky2_ecgfp5::curve::scalar_field::Scalar;
use plonky2_ecgfp5::curve::curve::Point;
use plonky2_field::types::Sample;

pub fn bench_curve(c: &mut Criterion) {
    c.bench_function(
        "add",
        |b| {
            b.iter_batched(
                || (Point::rand(), Point::rand()),
                |(a, b)| {
					black_box(a + b);
                },
                BatchSize::SmallInput
            )
        }
    );

    c.bench_function(
        "double",
        |b| {
            b.iter_batched(
                || Point::rand(),
                |point| {
					black_box(point.double());
                },
                BatchSize::SmallInput
            )
        }
    );

    c.bench_function(
        "scalar_mul",
        |b| {
            b.iter_batched(
                || (Point::rand(), Scalar::rand()),
                |(point, scalar)| {
					black_box(point * scalar);
                },
                BatchSize::SmallInput
            )
        }
    );


    c.bench_function(
        "mulgen",
        |b| {
            b.iter_batched(
                || Scalar::rand(),
                |scalar| {
					black_box(Point::mulgen(scalar));
				},
				BatchSize::SmallInput
			)
		}
    );
}

criterion_group!(benches, bench_curve);
criterion_main!(benches);