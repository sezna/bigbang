//! Implements a WebSocket server that provides updates of the current state of the simulation
//! to all connected clients.

extern crate bigbang;
extern crate rand;
use bigbang::{collisions::soft_body, AsEntity, GravTree, Responsive, SimulationResult};
#[macro_use]
extern crate log;

use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::thread;

use futures::{
    compat::{Compat, Compat01As03, Compat01As03Sink, Future01CompatExt, Stream01CompatExt},
    future::FutureExt,
    prelude::*,
};

use rand::prelude::*;
use tokio::runtime::TaskExecutor;
use websocket::message::OwnedMessage;
use websocket::r#async::{Server, TcpStream};
use websocket::server::{r#async::Incoming, upgrade::r#async::Upgrade};
const ENTITY_COUNT: usize = 200;
const MAX_ENTITIES: i32 = 3;
const THETA: f64 = 0.2;
const TIME_STEP: f64 = 0.00001;

fn spawn_future<F>(f: F, executor: &TaskExecutor)
where
    F: FutureExt + 'static + Send + Unpin,
{
    executor.spawn(Compat::new(f.unit_error().map(move |_| Ok(()))));
}

#[derive(Clone, PartialEq, AsEntity)]
struct Entity {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    radius: f64,
    mass: f64,
}

impl Responsive for Entity {
    fn respond(&self, simulation_result: SimulationResult<Entity>, time_step: f64) -> Self {
        let (ax, ay, az) = if !simulation_result.collisions.is_empty() {
            // If there were some collisions, perform collision calculations instead of gravitational onees.
            let mut ax = 0.;
            let mut ay = 0.;
            let mut az = 0.;
            for other in &simulation_result.collisions {
                let (collision_ax, collision_ay, collision_az) = soft_body(self, other, 200000f64);
                ax += collision_ax;
                ay += collision_ay;
                az += collision_az
            }
            (ax, ay, az)
        } else {
            // Otherwise, use gravtiational acceleration.
            let (ax, ay, az) = simulation_result.gravitational_acceleration;
            (ax, ay, az)
        };

        let (mut vx, mut vy, mut vz) = (self.vx, self.vy, self.vz);

        // Add the acceleration to the velocity, scaled to the time step
        vx += ax * time_step;
        vy += ay * time_step;
        vz += az * time_step;

        Entity {
            vx,
            vy,
            vz,
            x: self.x + vx,
            y: self.y + vy,
            z: self.z + vz,
            radius: self.radius,
            mass: self.mass,
        }
    }
}

async fn handle_connection(
    executor: TaskExecutor,
    upgrade: Upgrade<TcpStream>,
    socket_addr: SocketAddr,
    state: Arc<RwLock<Vec<f64>>>,
    rx: multiqueue::BroadcastFutReceiver<()>,
) {
    debug!("Handling WS connection...");
    if !upgrade.protocols().iter().any(|s| s == "rust-websocket") {
        warn!(
            "Invalid protocol received from WS client; doesn't include \"rust-websocket\": {:?}",
            upgrade.protocols()
        );
        spawn_future(upgrade.reject().compat(), &executor);
        return;
    }

    let client: websocket::client::r#async::Client<TcpStream> = match upgrade
        .use_protocol("rust-websocket")
        .accept()
        .compat()
        .await
    {
        Ok((client, _headers)) => client,
        Err(err) => {
            error!("Error accepting client connection: {:?}", err);
            return;
        }
    };

    info!("New client connected; sending welcome message...");

    let mut compat_client = Compat01As03Sink::new(client);
    match compat_client
        .send(websocket::message::Message::text("Connected").into())
        .await
    {
        Ok(()) => (),
        Err(err) => {
            error!("Error sending welcome message to client: {:?}", err);
            return;
        }
    };

    let (mut sink, _stream) = compat_client.split();

    // Listen for notification events and trigger each client to send the current simulation state
    let mut rx = rx.compat();
    while (rx.next().await).is_some() {
        debug!(
            "New server tick!  Sending update to connected client on address {}...",
            socket_addr
        );

        trace!("Reading state buffer...");
        let msg = {
            let state_inner = &*state.read().unwrap();
            let mut state_clone: Vec<u8> = unsafe { std::mem::transmute(state_inner.clone()) };
            unsafe { state_clone.set_len(state_inner.len() * std::mem::size_of::<f64>()) };

            OwnedMessage::Binary(state_clone)
        };
        trace!("Releasing read lock on state buffer.");
        trace!("Sending update message to client {}", socket_addr);

        match sink.send(msg).await {
            Ok(_) => debug!("Successfully sent server update to client {}", socket_addr),
            Err(err) => error!("Error sending server update to client: {:?}", err),
        };
    }
}

async fn server_logic(
    executor: TaskExecutor,
    incoming: Incoming<TcpStream>,
    state: Arc<RwLock<Vec<f64>>>,
    rx: multiqueue::BroadcastFutReceiver<()>,
) -> Result<(), ()> {
    let mut incoming = incoming.compat();
    while let Some(res) = incoming.next().await {
        match res {
            Ok((upgrade, socket_addr)) => {
                info!("Got a new WS connection from {}", socket_addr);

                executor.spawn(
                    handle_connection(
                        executor.clone(),
                        upgrade,
                        socket_addr,
                        Arc::clone(&state),
                        rx.add_stream(),
                    )
                    .unit_error()
                    .boxed()
                    .compat(),
                );
            }
            Err(err) => {
                error!("Error in WS server: {:?}", err);
            }
        }
    }

    Ok(())
}

/// Starts the WebSocket server and returns a handle that can be used to communicate with it in
/// order to indicate that there is a new update to send to clients.
fn start_ws_server(
    executor: TaskExecutor,
) -> Result<(Arc<RwLock<Vec<f64>>>, multiqueue::BroadcastFutSender<()>), Box<dyn Error>> {
    //INIT.call_once(|| init_logging().expect("Failed to initialize logger"));

    let (tx, rx) = multiqueue::broadcast_fut_queue(1);

    let state = Arc::new(RwLock::new(Vec::new()));
    let state_clone = Arc::clone(&state);

    thread::spawn(move || {
        let ws_server = Server::bind("0.0.0.0:3355", &tokio::reactor::Handle::default())
            .expect("Failed to initialize WS server");

        // Initialize the loop that handles incoming client connections, sends them welcome
        // messages, and stores them in the set of active connections.
        let server_loop_v03_future =
            server_logic(executor.clone(), ws_server.incoming(), state, rx.clone());

        spawn_future(server_loop_v03_future.boxed(), &executor);

        info!("Initializing main sink...");
        let sink_fut = Compat01As03::new(rx).fold((), |_acc, _| future::ready(()));
        spawn_future(sink_fut.boxed(), &executor);
    });

    Ok((state_clone, tx))
}

#[allow(unreachable_code)]
async fn run(executor: TaskExecutor) {
    let (state, tx) = match start_ws_server(executor) {
        Ok((state, tx)) => (state, tx),
        Err(err) => panic!("Failed to initialize WebSocket server: {:?}", err),
    };
    let mut tx = Compat01As03Sink::new(tx);

    let mut vec_that_wants_to_be_a_kdtree: Vec<Entity> = Vec::new();
    for _ in 0..ENTITY_COUNT {
        let mass = rand::thread_rng().gen_range(0.1, 2.5);
        let entity = Entity {
            vx: 0.0, //rand::thread_rng().gen_range(-190., 90.),
            vy: 0.0, //rand::thread_rng().gen_range(-190., 90.),
            vz: 0.0, //rand::thread_rng().gen_range(-190., 90.),
            x: rand::thread_rng().gen_range(-190., 90.),
            y: rand::thread_rng().gen_range(-190., 90.),
            z: rand::thread_rng().gen_range(-190., 90.),
            radius: mass,
            mass,
        };
        vec_that_wants_to_be_a_kdtree.push(entity);
    }

    // create one large entity in the middle
    vec_that_wants_to_be_a_kdtree.push(Entity {
        vx: 0.,
        vy: 0.,
        vz: 0.,
        x: rand::thread_rng().gen_range(-190., 90.),
        y: rand::thread_rng().gen_range(-190., 90.),
        z: rand::thread_rng().gen_range(-190., 90.),
        radius: 25.,
        mass: 250.,
    });

    let mut test_tree = GravTree::new(
        &mut vec_that_wants_to_be_a_kdtree,
        TIME_STEP,
        MAX_ENTITIES,
        THETA,
        bigbang::CalculateCollisions::Yes,
    );

    loop {
        test_tree = test_tree.time_step();
        let mut entities = test_tree.as_vec();
        // Update the state with data about all of the entities
        {
            let mut state_inner = state.write().unwrap();
            state_inner.clear();
            // TODO this needs to be made into a proper iter later, instead of a vec and reconstruction
            for e in entities.iter_mut() {
                state_inner.push(e.x);
                state_inner.push(e.y);
                state_inner.push(e.z);
                state_inner.push(e.radius);
            }
        }
        trace!("Buffer state updated");

        // Notify all connected WS clients that a new update is available
        trace!("before broadcast send");
        // Send a message for all connected clients + one for the main sink

        match tx.send(()).await {
            Ok(_) => {
                debug!("Successfully sent tick notification message to all connected clients",)
            }
            Err(err) => error!("Error sending notification message to clients: {:?}", err),
        }

        // Delay::new(Duration::from_millis(30)).await.unwrap();
    }
}

pub fn main() {
    let mut runtime = tokio::runtime::Builder::new().build().unwrap();
    let executor = runtime.executor();
    let main_future = run(executor);

    runtime
        .block_on(
            main_future
                .map(|_| -> Result<(), ()> { Ok(()) })
                .boxed()
                .compat(),
        )
        .expect("Server loop exited");
}
