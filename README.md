![docs.rs](https://docs.rs/bigbang/badge.svg)
![crates.io](https://img.shields.io/crates/v/bigbang.svg)
[![Build Status](https://travis-ci.org/sezna/bigbang.svg?branch=master)](https://travis-ci.org/sezna/bigbang)
_Have you used this project in your work? I'd love to hear about it and work with you. Email me at [alex@alex-hansen.com](mailto:alex@alex-hansen.com)._


## This project is under major construction -- check out the "collisions" branch for my progress. It is being converted into a much more physically accurate soft-body simulation with efficient collision calculation.


# About the project
This is a project in re-implementing a c++ particle simulation in Rust for speed comparison purposes. I originally created this tree at Trinity University with Dr. Mark Lewis around 2015. Rust changed a lot in the following years, and so I re-wrote it in 2019. The second time I wrote it, I actually read the Rust book and attempted best practices :)


![example in 3d](./3d_example.gif)
_3d websocket-based simulation is available in the examples directory and was provided by [Casey Primozic](https://cprimozic.net/)._

# What exactly does it do?
It constructs a k-d tree of 3 dimensions and optimally calculates the gravitational force all of the entities are exerting on each other. It then calls `apply_acceleration()` on each individual entity with the acceleration value it calculated.

It optimizes gravitational calculation by treating entire nodes of 3d space as one giant entity, avoiding a lot of calculation. 

# Getting started with bigbang
## Implementing the `AsEntity` trait
#### (or just using the provided `Entity` struct)
In order to use your arbitrary type inside this tree, your struct must be `AsEntity + Clone + Send + Sync`. I'd like to eventually get rid of the `Clone` requirement, but currently the tree works in an immutable way where each time step an entirely new tree is constructed with the gravitational acceleration applied to it. This makes parallelism easier to reason about and safer, and requires `Clone`. `Send` and `Sync` are required for the parallelism. 

The real meat and potatoes you must implement is the trait `AsEntity`. To do so, you must provide a way to represent your struct as a gravitational entity, and a way in which it responds to an acceleration force. This looks like:

```rust
fn as_entity(&self) -> Entity;
fn apply_acceleration(&self, acceleration: (f64, f64, f64), time_step: f64) -> Self;
```

`as_entity` must take your struct and return it as a gravitational entity consisting of a velocity vector, a position vector, a radius, and a mass:
```
use bigbang::Entity;
struct Entity {
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
    pub mass: f64,
}
```

`apply_acceleration(accel: (f64, f64, f64), time_step: f64) -> Self` takes a three-tuple of `f64` values representing acceleration on the x, y, and z axes, and a coefficient for how long a single unit of time is in this simulation. You must return a new `Self` which has responded to this acceleration (typically just adding it to your velocity).

Here is what those implementations look like for `Entity` itself:
```rust
use bigbang::{ AsEntity, Entity };
impl AsEntity for Entity {
    fn as_entity(&self) -> Entity {
        return self.clone();
    }
    fn apply_acceleration(&self, acceleration: (f64, f64, f64), time_step: f64) -> Self {
        let (vx, vy, vz) = (
            self.vx + acceleration.0 * time_step,
            self.vy + acceleration.1 * time_step,
            self.vz + acceleration.2 * time_step,
        );
        Entity {
            vx,
            vy,
            vz,
            x: self.x + (vx * time_step),
            y: self.y + (vy * time_step),
            z: self.z + (vz * time_step),
            radius: self.radius,
            mass: self.mass,
        }
    }
}
```

If you have no custom fields to keep track of on your struct and just want to simulate raw particles, you can use `bigbang::Entity` directly, as is done in `examples/sample_simulation.rs`. 

## Starting the Simulation
Now that you have a compliant type with sufficient trait implementations, you may construct a vector with the starting positions for all of these entities. Pass a mutable reference to that vector and a _time\_step_ coefficent into `GravTree::new()` and you'll be off to the races:
```rust
use bigbang::{ GravTree, AsEntity };

struct MyEntity { ... }

impl AsEntity for MyEntity { ...}

let mut my_fun_vec:Vec<MyEntity> = vec![entity1, entity2, entity3];
let grav_tree = GravTree::new(&my_fun_vec, 0.2);

```

The _time\_step_ coefficient is later passed into `apply_acceleration()`. It can be used to effectively control the granularity of the simulation, i.e. how much each simulation frame actually impacts the movement of the entities. A smaller _time\_step_ will result in a more granular, more precise tree. For my general research purposes, I've found `0.2` to be a good starting number. For something like a video game or real-time simulation, you may wish to up that number quite a lot. How you choose to implement this coefficient is ultimately up to you in your `apply_acceleration()` function, though.

In order to advance the simulation, call `grav_tree.time_step()`. Given enough particles, this will probably heat up your computer. It will also eat all of your threads. 

See the examples directory for a minimalist working example.

## Saving output and loading from files

`bigbang` supports both saving to data files and loading from them. Be warned, when saving to a data file, it does not currently save out the `time_step` value. You must provide that again when you load from a file.

The reason for this is because the output is compliant with visualization software like [SwiftViz](https://github.com/MarkCLewis/SwiftVis2).
# C/C++ Interface
If you are hoping to use this with C or C++, I have provided FFI functionality. I have tested it on a small scale. I would love to work with you to test it on a larger scale and help you set it up. Contact me at [alex@alex-hansen.com](mailto:alex@alex-hansen.com) if you'd like help setting this up in C/C++. 
