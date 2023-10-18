use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench(c: &mut Criterion) {
    dotenvy::dotenv().ok();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let schema = rt.block_on(api_interface::create_schema()).unwrap();
    let size = 100;
    let query = |method: &str, count: u16| {
        format!(
            "
                   query {{
                       {method}(first: {count}) {{
                       edges{{
                         cursor
                         node{{
                           id,
                           name,
                           subCategories,
                           imageUrl
                         }}
                       }},
                       pageInfo {{
                         hasNextPage,
                         hasPreviousPage
                       }}
                     }}
                   }}
                "
        )
    };

    c.bench_with_input(BenchmarkId::new("categories", size), &size, |b, &s| {
        b.to_async(&rt)
            .iter(|| schema.execute(query("categories", s)));
    });

    c.bench_with_input(BenchmarkId::new("subCategories", size), &size, |b, &s| {
        b.to_async(&rt)
            .iter(|| schema.execute(query("subCategories", s)));
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
