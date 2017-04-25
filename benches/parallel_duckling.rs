#![feature(test)]

extern crate test;
use test::Bencher;

extern crate duckling;

extern crate ecs_bench;

use duckling::*;

use ecs_bench::parallel::{R, W1, W2, N};

#[derive(Debug, Copy, Clone)]
struct RComp(R);

#[derive(Debug, Copy, Clone)]
struct W1Comp(W1);

fn build() -> World<()> {
    let mut w = World::new();

    // setup entities


    for e in 0..N {
        w.create_entity(RComp(R { x: 0.0 }), W1Comp(W1 { x: 1.0 }));
    }
    w
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut world = build();

    b.iter(|| {
        world.run2(|w1: &W1Comp, r: &RComp, events: _, writer: _| {
            (W1Comp(W1{ x: w1.0.x + r.0.x}), *r)
        });
        world.finish();
    });
}
