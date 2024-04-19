use tokio::runtime::Runtime;
use tokio::signal;
use tokio::sync::oneshot;
use sandcore_world::world::World;
use crate::api;

mod client;

pub fn run() {
    let world = World::new();

    let (shutdown_sender, shutdown_receiver) = oneshot::channel();

    let server = std::thread::spawn(move ||{run_server(shutdown_sender)});
    let world = std::thread::spawn(move ||{run_world(shutdown_receiver, world)});

    server.join().unwrap();
    world.join().unwrap();
}


fn run_server(shutdown_sender: oneshot::Sender<bool>) {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, api::routes()).with_graceful_shutdown(shutdown_signal()).await.unwrap();
        shutdown_sender.send(true).unwrap();
    });
}

fn run_world(mut shutdown_receiver: oneshot::Receiver<bool>, mut world: World) {
    loop {
        world.update();
        if let Ok(_) = shutdown_receiver.try_recv() { break }
    }
}

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
}