#![feature(test)]

extern crate test;
use test::Bencher;

extern crate duckling;
use duckling::*;

extern crate ecs_bench;

use ecs_bench::pos_vel::{Position, Velocity, N_POS_VEL, N_POS};

#[derive(Debug, Copy, Clone)]
struct PosComp(Position);
#[derive(Debug, Copy, Clone)]
struct VelComp(Velocity);

fn build() -> World<u32> {
    let mut w = World::new();
    {

        for e in 0..N_POS_VEL {
            w.create_entity(PosComp(Position { x: 0.0, y: 0.0 }), VelComp(Velocity { dx: 0.0, dy: 0.0 }));
        }
        for e in N_POS_VEL..(N_POS_VEL + N_POS) {
            w.create_entity(PosComp(Position { x: 0.0, y: 0.0 }), ());
        }
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
        world.run2(|p: &PosComp, v: &VelComp, events: _, writer: _| {
            let p = Position { x: p.0.x + v.0.dx, y: p.0.y + v.0.dy};
            (PosComp(p), *v)
        });
        world.finish();
    });
}
