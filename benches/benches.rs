use bigbang::{collisions::soft_body, AsEntity, Entity, GravTree, SimulationResult};
use std::time;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
#[derive(Clone)]
struct MyEntity {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    radius: f64,
}
impl AsEntity for MyEntity {
    fn as_entity(&self) -> Entity {
        return Entity {
            x: self.x,
            y: self.y,
            z: self.z,
            vx: self.vx,
            vy: self.vy,
            vz: self.vz,
            radius: self.radius,
            mass: if self.radius < 1. { 0.5 } else { 105. },
        };
    }

    fn respond(&self, simulation_result: SimulationResult<MyEntity>, time_step: f64) -> Self {
        let (mut ax, mut ay, mut az) = simulation_result.gravitational_acceleration;
        let (x, y, z) = (self.x, self.y, self.z);
        let (mut vx, mut vy, mut vz) = (self.vx, self.vy, self.vz);
        // calculate the collisions
        for other in &simulation_result.collisions {
            let (collision_ax, collision_ay, collision_az) = soft_body(self, other, 20f64);
            ax += collision_ax;
            ay += collision_ay;
            az += collision_az;
        }
        vx += ax * time_step;
        vy += ay * time_step;
        vz += az * time_step;
        MyEntity {
            vx,
            vy,
            vz,
            x: x + vx,
            y: y + vy,
            z: z + vz,
            radius: self.radius,
        }
    }
}

impl MyEntity {
    pub fn new_entity(x: f64, y: f64, z: f64, radius: f64) -> MyEntity {
        MyEntity {
            vx: 0f64,
            vy: 0f64,
            vz: 0f64,
            x,
            y,
            z,
            radius,
        }
    }
}

fn initialize_data(number_of_particles: usize) -> Vec<MyEntity> {
    let mut vec: Vec<MyEntity> = Vec::new();
    for x in 0..number_of_particles {
        let entity = MyEntity::new_entity(x as f64, x as f64, x as f64, 10.);
        vec.push(entity);
    }
    vec
}

fn initialize_tree(number_of_particles: usize, theta: f64) -> GravTree<MyEntity> {
    let mut data = initialize_data(number_of_particles);
    GravTree::new(&mut data, theta)
}

// Theta isn't used in tree construction so it isn't varied in the benches
fn tree_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree construction");
    group.bench_function("tree construction n=125", |b| {
        b.iter_batched(
            || initialize_data(125),
            |mut data| GravTree::new(&mut data, 0.2),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("tree construction n=2000", |b| {
        b.iter_batched(
            || initialize_data(2000),
            |mut data| GravTree::new(&mut data, 0.2),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("tree construction n=20_000", |b| {
        b.iter_batched(
            || initialize_data(20_000),
            |mut data| GravTree::new(&mut data, 0.2),
            BatchSize::SmallInput,
        )
    });
}

// Benching time stepping with a low number of entities - 125

fn time_step_0125(c: &mut Criterion) {
    let mut group = c.benchmark_group("time step: n=125");

    group.bench_function("time_step n=125 theta=0.2", |b| {
        b.iter_batched(
            || initialize_tree(125, 0.2),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("time_step n=125 theta=0.3", |b| {
        b.iter_batched(
            || initialize_tree(125, 0.3),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("time_step n=125 theta=0.4", |b| {
        b.iter_batched(
            || initialize_tree(125, 0.4),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("time_step n=125 theta=0.5", |b| {
        b.iter_batched(
            || initialize_tree(125, 0.5),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
}

// Benching time stepping with a medium number of entities - 2000

fn time_step_2000(c: &mut Criterion) {
    let mut group = c.benchmark_group("time step: n=2000");
    group.measurement_time(time::Duration::new(35, 0));
    group.bench_function("time_step n=2000 theta=0.2", |b| {
        b.iter_batched(
            || initialize_tree(2000, 0.2),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("time_step n=2000 theta=0.3", |b| {
        b.iter_batched(
            || initialize_tree(2000, 0.3),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("time_step n=2000 theta=0.4", |b| {
        b.iter_batched(
            || initialize_tree(2000, 0.4),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("time_step n=2000 theta=0.5", |b| {
        b.iter_batched(
            || initialize_tree(2000, 0.5),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
}

// Benching time stepping with a medium high number of entities - 20_000
fn time_step_20000(c: &mut Criterion) {
    let mut group = c.benchmark_group("time step: n=20_000");
    group.measurement_time(time::Duration::new(35, 0));
    group.bench_function("time_step n=20_000 theta=0.2", |b| {
        b.iter_batched(
            || initialize_tree(20_000, 0.2),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("time_step n=20_000 theta=0.3", |b| {
        b.iter_batched(
            || initialize_tree(20_000, 0.3),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("time_step n=20_000 theta=0.4", |b| {
        b.iter_batched(
            || initialize_tree(20_000, 0.4),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
    group.bench_function("time_step n=20_000 theta=0.5", |b| {
        b.iter_batched(
            || initialize_tree(20_000, 0.5),
            |data| data.time_step(),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    tree_construction,
    time_step_0125,
    time_step_2000,
    time_step_20000,
);
criterion_main!(benches);
