mod server;
mod router;
mod handler;

use server::Server;

fn main() {
    let server = Server::new("0.0.0.0:3000");
    server.run();
}
