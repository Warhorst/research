use futures::executor::block_on;

mod dance_example;

fn main() {
    block_on(dance_example::run())
}
