use ::simple_base::engine;

fn main() {
    let addr = "0.0.0.0:8080";

    engine::runner::run(addr)
}