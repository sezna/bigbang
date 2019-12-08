//! Implements a WebSocket server that provides updates of the current state of the simulation
//! to all connected clients.

extern crate bigbang;
use bigbang::{Entity, GravTree};
#[macro_use]
extern crate log;

use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Once, RwLock};
use std::thread;

use futures::{
    compat::{Compat, Compat01As03, Compat01As03Sink, Future01CompatExt, Stream01CompatExt},
    future::FutureExt,
    prelude::*,
};
use tokio::runtime::TaskExecutor;
use websocket::message::OwnedMessage;
use websocket::r#async::{Server, TcpStream};
use websocket::server::{r#async::Incoming, upgrade::r#async::Upgrade};

const ENTITY_COUNT: usize = 1000;

fn spawn_future<F>(f: F, executor: &TaskExecutor)
where
    F: FutureExt + 'static + Send + Unpin,
{
    executor.spawn(Compat::new(f.unit_error().map(move |_| Ok(()))));
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
    while let Some(_) = rx.next().await {
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

static INIT: Once = Once::new();

fn init_logging() -> Result<(), Box<dyn Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .level_for("hyper", log::LevelFilter::Info)
        .level_for("mio", log::LevelFilter::Info)
        .level_for("tokio_core", log::LevelFilter::Info)
        .level_for("tokio_reactor", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

/// Starts the WebSocket server and returns a handle that can be used to communicate with it in
/// order to indicate that there is a new update to send to clients.
fn start_ws_server(
    executor: TaskExecutor,
) -> Result<(Arc<RwLock<Vec<f64>>>, multiqueue::BroadcastFutSender<()>), Box<dyn Error>> {
    INIT.call_once(|| init_logging().expect("Failed to initialize logger"));

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
        let entity = Entity::random_entity();
        vec_that_wants_to_be_a_kdtree.push(entity);
    }

    let mut test_tree = GravTree::new(&mut vec_that_wants_to_be_a_kdtree, 0.2);

    let mut i = 0;
    loop {
        println!("time step: {}", i);
        test_tree = test_tree.time_step();

        // Update the state with data about all of the entities
        {
            let mut state_inner = state.write().unwrap();
            state_inner.clear();
            for entity in test_tree.as_vec() {
                state_inner.push(entity.x);
                state_inner.push(entity.y);
                state_inner.push(entity.z);
                state_inner.push(entity.radius);
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

        i += 1;
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
