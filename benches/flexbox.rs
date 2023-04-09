//! This file includes benchmarks for very large, pseudo-randomly generated trees
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use taffy::prelude::*;
use taffy::style::Style;

mod helpers;
use helpers::{BuildTreeExt, FixedStyleGenerator, RandomStyleGenerator, TaffyTreeBuilder};

#[cfg(feature = "yoga_benchmark")]
use helpers::yoga_helpers;
#[cfg(feature = "yoga_benchmark")]
use slotmap::SlotMap;
#[cfg(feature = "yoga_benchmark")]
use yoga_helpers::{yg, YogaTreeBuilder};

/// A deep tree that matches the shape and styling that yoga use on their benchmarks
fn build_flat_hierarchy<TreeBuilder: BuildTreeExt<RandomStyleGenerator>>(
    target_node_count: u32,
) -> (TreeBuilder::Tree, TreeBuilder::Node) {
    let tree_builder = TreeBuilder::new(RandomStyleGenerator);
    tree_builder.build_flat_hierarchy(target_node_count)
}

/// A deep tree that matches the shape and styling that yoga use on their benchmarks
fn build_deep_hierarchy<TreeBuilder: BuildTreeExt<RandomStyleGenerator>>(
    node_count: u32,
    branching_factor: u32,
) -> (TreeBuilder::Tree, TreeBuilder::Node) {
    let tree_builder = TreeBuilder::new(RandomStyleGenerator);
    tree_builder.build_deep_hierarchy(node_count, branching_factor)
}

/// A deep tree that matches the shape and styling that yoga use on their benchmarks
fn build_huge_nested_hierarchy<TreeBuilder: BuildTreeExt<FixedStyleGenerator>>(
    node_count: u32,
    branching_factor: u32,
) -> (TreeBuilder::Tree, TreeBuilder::Node) {
    let style = Style { size: points(10.0), flex_grow: 1.0, ..Default::default() };
    let tree_builder = TreeBuilder::new(FixedStyleGenerator(style));
    tree_builder.build_deep_hierarchy(node_count, branching_factor)
}

fn taffy_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("yoga 'huge nested'");
    for node_count in [1_000u32, 10_000, 100_000].iter() {
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(BenchmarkId::new("Yoga", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_huge_nested_hierarchy::<YogaTreeBuilder<_, _>>(node_count, 10),
                |(mut tree, root)| {
                    tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("Taffy", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_huge_nested_hierarchy::<TaffyTreeBuilder<_, _>>(node_count, 10),
                |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();

    // Decrease sample size, because the tasks take longer
    let mut group = c.benchmark_group("big trees (wide)");
    group.sample_size(10);
    for node_count in [1_000u32, 10_000, 100_000].iter() {
        #[cfg(feature = "yoga_benchmark")]
        let benchmark_id = BenchmarkId::new(format!("Yoga (2-level hierarchy)"), node_count);
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter_batched(
                || build_flat_hierarchy::<YogaTreeBuilder<_, _>>(node_count),
                |(mut tree, root)| {
                    tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        let benchmark_id = BenchmarkId::new(format!("Taffy (2-level hierarchy)"), node_count);
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter_batched(
                || build_flat_hierarchy::<TaffyTreeBuilder<_, _>>(node_count),
                |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();

    // Decrease sample size, because the tasks take longer
    let mut group = c.benchmark_group("big trees (deep)");
    group.sample_size(10);
    let benches = [(4000, "(12-level hierarchy)"), (10_000, "(14-level hierarchy)"), (100_000, "(17-level hierarchy)")];
    for (node_count, label) in benches.iter() {
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(BenchmarkId::new(format!("Yoga {label}"), node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_deep_hierarchy::<YogaTreeBuilder<_, _>>(node_count, 2),
                |(mut tree, root)| {
                    tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new(format!("Taffy {label}"), node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_deep_hierarchy::<TaffyTreeBuilder<_, _>>(node_count, 2),
                |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();

    let mut group = c.benchmark_group("super deep (1000-level hierarchy)");
    group.sample_size(10);
    for node_count in [1000u32].iter() {
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(BenchmarkId::new("Yoga", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_deep_hierarchy::<YogaTreeBuilder<_, _>>(node_count, 2),
                |(mut tree, root)| {
                    tree[root].calculate_layout(f32::INFINITY, f32::INFINITY, yg::Direction::LTR);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("Taffy", node_count), node_count, |b, &node_count| {
            b.iter_batched(
                || build_deep_hierarchy::<TaffyTreeBuilder<_, _>>(node_count, 2),
                |(mut taffy, root)| taffy.compute_layout(root, Size::MAX_CONTENT).unwrap(),
                criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
