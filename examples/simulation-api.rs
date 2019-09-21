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
extern crate persistent;
extern crate rand;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bigbang;
extern crate iron_cors;
extern crate serde_json;
extern crate staticfile;

use bigbang::{AsEntity, Entity};
use iron::prelude::*;
use iron_cors::CorsMiddleware;
use router::Router;
use staticfile::Static;

use chrono::{DateTime, Utc};

use iron::typemap::Key;
use iron::{status, Request, Response};
use mount::Mount;
use persistent::State;

const TIME_STEP: f64 = 0.001;

#[derive(Clone, Serialize, Deserialize)]
struct MyEntity {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    color: String,
    is_colliding: bool
}

impl AsEntity for MyEntity {
    fn as_entity(&self) -> Entity {
        return Entity {
            x: self.x,
            y: self.y,
            z: 0.0,
            vx: self.vx,
            vy: self.vy,
            vz: 0.0,
            radius: self.radius,
            mass: self.radius,
            is_colliding: self.is_colliding
        };
    }

    fn apply_velocity(&self, simulation_results: ((f64, f64, f64), bool), time_step: f64) -> Self {
        let ((vx, vy, _vz), is_colliding) = simulation_results;
        MyEntity {
            vx,
            vy,
            x: self.x + (vx * time_step),
            y: self.y + (vy * time_step),
            radius: self.radius,
            color: if is_colliding { String::from("red") } else { String::from("blue") },
            is_colliding
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
            radius: rand::random::<f64>(),
            color: String::from("blue"),
            is_colliding: false
        }
    }
}

struct SimulationState {
    entities: Vec<MyEntity>,
    last_time_ran: DateTime<Utc>,
    last_response: String,
}

impl Key for SimulationState {
    type Value = SimulationState;
}

fn main() {
    let mut starter_entities: Vec<MyEntity> = (0..4).map(|_| MyEntity::random_entity()).collect();
    let mut big_boi = MyEntity::random_entity();
    big_boi.x = 2f64;
    big_boi.y = 10f64;
    big_boi.radius = 0.8f64;
    big_boi.vx = 2f64;
    starter_entities.push(big_boi);
    let mut big_boi_2 = MyEntity::random_entity();
    big_boi_2.x = 18f64;
    big_boi_2.vx = -2f64;
    big_boi_2.y = 10f64;
    big_boi_2.radius = 0.2f64;
    starter_entities.push(big_boi_2);
    let mut big_boi_3 = MyEntity::random_entity();
    big_boi_3.x = 4f64;
    big_boi_3.vx = 2f64;
    big_boi_3.y = 2f64;
    big_boi_3.vy = 1f64;
    big_boi_3.radius = 0.3f64;
    starter_entities.push(big_boi_3);
    let sim_state = SimulationState {
        entities: starter_entities.clone(),
        last_time_ran: Utc::now(),
        last_response: String::from("initializing"),
    };
    let store: State<SimulationState> = State::one(sim_state);
    println!("initializing simulation...");
    let _gravtree = bigbang::GravTree::new(&mut starter_entities, TIME_STEP);
    let mut router = Router::new();
    router.get("/", move |r: &mut Request| simulation(r), "home");

    let mut chain = Chain::new(router);
    chain.link_before(store);

    let cors_middleware = CorsMiddleware::with_allow_any();
    chain.link_around(cors_middleware);

    // Find the path of the JS visualization file to serve.
    let project_directory = env!("CARGO_MANIFEST_DIR");
    println!("project dir is {}", project_directory);
    let files_path = format!("{}{}", project_directory, "/examples/visualize.html");
    let mut mount = Mount::new();
    mount
        .mount("/api", chain)
        .mount("/", Static::new(files_path));

    println!("Browse to http://localhost:4001 to heat up your computer.");
    Iron::new(mount)
        .http("localhost:4001")
        .expect("unable to mount server");
}

fn simulation(r: &mut Request) -> IronResult<Response> {
    let state = r
        .get::<State<SimulationState>>()
        .expect("failed to load sim state");
    if Utc::now()
        .signed_duration_since(state.read().unwrap().last_time_ran)
        .num_milliseconds()
        < 30
    {
        return Ok(Response::with((
            status::Ok,
            state.read().unwrap().last_response.clone(),
        )));
    }
    let mut simulation_vec = state.read().unwrap().entities.clone();
    let grav_tree = bigbang::GravTree::new(&mut simulation_vec, TIME_STEP);

    let mut new_vec = grav_tree.time_step().as_vec();

    // bounce off the walls if they're exceeding the boundaries
    for e in new_vec.iter_mut() {
        if e.x - e.radius <= 0.1f64 {
            e.vx = e.vx * -1.0;
            e.x = 0.1f64 + e.radius;
        }
        else if e.x + e.radius >= 19.9f64 {
            e.vx = e.vx * -1.0;
            e.x = 19.9f64 - e.radius;
        }


        if e.y - e.radius < 0.01f64
        {
            e.vy = e.vy * -1.0;
            e.y = 0.01f64 + e.radius;
        }
        else if e.y + e.radius > 19.9f64 {
            e.vy = e.vy * -1.0;
            e.y = 19.9f64 - e.radius;
        }
    }

    let resp_data = serde_json::to_string(&new_vec).unwrap();

    *state.write().unwrap() = SimulationState {
        entities: new_vec,
        last_time_ran: Utc::now(),
        last_response: resp_data.clone(),
    };
    return Ok(Response::with((status::Ok, resp_data)));
}
