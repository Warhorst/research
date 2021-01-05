use greetings::Greetings;
use greetings_derive::Greetings;

#[derive(Greetings)]
struct NiceGuy;

fn main() {
    let guy = NiceGuy;
    guy.greet()
}
