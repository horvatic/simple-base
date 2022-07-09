mod network;

fn main() -> std::io::Result<()> {
    network::connection::listen()
}