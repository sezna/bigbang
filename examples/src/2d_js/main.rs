/// To get some things out of the way, I know this is not ideal rust. This is under-baked,
/// and an improvisational sketch to demonstrate bigbang's core gravitational tree.
/// I had three goals:
///  * the simulation doesn't progress when there are no visitors (wasted effort)
///  * the simulation, when it is progressing, progresses at a constant rate
///  * some basic caching
///
/// A global lazy static which contains a tuple of when the last simulation ran and
/// what I returned when that ran achieves caching and the auto-shutdown ability.
/// When a new request comes in, if the current time is less than x seconds
/// from when I last advanced the simulation, just return what I did last time.
/// Otherwise, progress the simulation.
///
/// An additional lazy static contains the contents of the entire simulation as global
/// mutable state. Sigh. To be fair, this was written in about 30 minutes.
extern crate chrono;
extern crate iron;
extern crate mount;
extern crate rand;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate bigbang;
extern crate iron_cors;
extern crate serde_json;
extern crate staticfile;

use bigbang::{
    collisions::soft_body, AsEntity, CalculateCollisions, Entity, GravTree, Responsive,
    SimulationResult,
};
use iron::prelude::*;
use iron_cors::CorsMiddleware;
use router::Router;
use staticfile::Static;

use chrono::{DateTime, Utc};

use iron::typemap::Key;
use iron::{status, Request, Response};
use mount::Mount;
use std::sync::RwLock;

const TIME_STEP: f64 = 0.0000002;
const THETA: f64 = 0.2;
const MAX_ENTITIES: i32 = 3;

struct State {
    state: SimulationState,
}

struct SimulationState {
    grav_tree: GravTree<MyEntity>,
    last_time_ran: DateTime<Utc>,
    last_response: String,
}

lazy_static! {
    static ref STATE: RwLock<State> = RwLock::new(State {
        state: SimulationState {
            grav_tree: GravTree::new(
                &Vec::new(),
                TIME_STEP,
                MAX_ENTITIES,
                THETA,
                CalculateCollisions::Yes
            ),
            last_time_ran: Utc::now(),
            last_response: String::new(),
        }
    });
}

#[derive(Clone, Serialize, Deserialize)]
struct MyEntity {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    color: String,
}
impl AsEntity for MyEntity {
    fn as_entity(&self) -> Entity {
        Entity {
            x: self.x,
            y: self.y,
            z: 0.,
            vx: self.vx,
            vy: self.vy,
            vz: 0.,
            radius: self.radius,
            mass: self.radius,
        }
    }
}

impl Responsive for MyEntity {
    fn respond(&self, simulation_result: SimulationResult<MyEntity>, time_step: f64) -> Self {
        let (ax, ay) = if !simulation_result.collisions.is_empty() {
            // If there were some collisions, perform collision calculations instead of gravitational onees.
            let mut ax = 0.;
            let mut ay = 0.;
            for other in &simulation_result.collisions {
                let (collision_ax, collision_ay, _az) = soft_body(self, other, 200000f64);
                ax += collision_ax;
                ay += collision_ay;
            }
            (ax, ay)
        } else {
            // Otherwise, use gravtiational acceleration.
            let (ax, ay, _) = simulation_result.gravitational_acceleration;
            (ax, ay)
        };

        let (mut vx, mut vy) = (self.vx, self.vy);

        // Add the acceleration to the velocity, scaled to the time step
        vx += ax * time_step;
        vy += ay * time_step;
        let (mut x, mut y) = (self.x, self.y);

        // Perform bounds checking on the borders
        if x - self.radius <= 0.1f64 {
            vx *= -0.3;
            x = 0.1f64 + self.radius;
        } else if x + self.radius >= 19.9f64 {
            vx *= -0.3;
            x = 19.9f64 - self.radius;
        }
        if y - self.radius < 0.01f64 {
            vy *= -0.3;
            y = 0.01f64 + self.radius;
        } else if y + self.radius > 19.9f64 {
            vy *= -0.3;
            y = 19.9f64 - self.radius;
        }

        MyEntity {
            vx,
            vy,
            x: x + vx,
            y: y + vy,
            radius: self.radius,
            color: if !simulation_result.collisions.is_empty() {
                "red"
            } else {
                "blue"
            }
            .to_string(),
        }
    }
}

impl Key for MyEntity {
    type Value = MyEntity;
}

impl MyEntity {
    pub fn random_entity() -> MyEntity {
        MyEntity {
            vx: 0f64,
            vy: 0f64,
            x: rand::random::<f64>() * 20f64,
            y: rand::random::<f64>() * 20f64,
            radius: rand::random::<f64>() / 10f64,
            color: String::from("blue"),
        }
    }
}

fn main() {
    let mut starter_entities: Vec<MyEntity> = (0..20).map(|_| MyEntity::random_entity()).collect();
    let mut big_boi = MyEntity::random_entity();
    big_boi.x = 10f64;
    big_boi.y = 10f64;
    big_boi.radius = 1f64;
    starter_entities.push(big_boi);
    let mut big_boi_2 = MyEntity::random_entity();
    big_boi_2.x = 7f64;
    big_boi_2.y = 7f64;
    big_boi_2.radius = 1f64;
    big_boi_2.color = "green".to_string();
    starter_entities.push(big_boi_2);
    let grav_tree = bigbang::GravTree::new(
        &mut starter_entities,
        TIME_STEP,
        MAX_ENTITIES,
        THETA,
        CalculateCollisions::No,
    );

    println!("initializing simulation...");
    {
        STATE.write().unwrap().state = SimulationState {
            grav_tree,
            last_time_ran: Utc::now(),
            last_response: String::new(),
        };
    }
    let mut router = Router::new();
    router.get("/", move |_r: &mut Request| simulation(), "home");

    let mut chain = Chain::new(router);
    let cors_middleware = CorsMiddleware::with_allow_any();
    chain.link_around(cors_middleware);

    // Find the path of the JS visualization file to serve.
    let project_directory = env!("CARGO_MANIFEST_DIR");
    println!("project dir is {}", project_directory);
    let files_path = format!("{}{}", project_directory, "/src/2d_js/visualize.html");
    println!("Serving {}", files_path);
    let mut mount = Mount::new();
    mount
        .mount("/api", chain)
        .mount("/", Static::new(files_path));

    println!("Browse to http://localhost:4001 to heat up your computer.");
    Iron::new(mount)
        .http("localhost:4001")
        .expect("unable to mount server");
}

fn simulation() -> IronResult<Response> {
    {
        if Utc::now()
            .signed_duration_since(STATE.read().unwrap().state.last_time_ran)
            .num_milliseconds()
            < 30
        {
            return Ok(Response::with((
                status::Ok,
                STATE.read().unwrap().state.last_response.clone(),
            )));
        }
    }
    let grav_tree = STATE.read().unwrap().state.grav_tree.time_step();

    let resp_data = serde_json::to_string(&grav_tree.as_vec()).unwrap();

    STATE.write().unwrap().state = SimulationState {
        grav_tree,
        last_time_ran: Utc::now(),
        last_response: resp_data.clone(),
    };
    Ok(Response::with((status::Ok, resp_data)))
}
