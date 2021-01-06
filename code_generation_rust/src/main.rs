use greetings::Greetings;
use greetings_derive::Greetings;
use attribute::awesome_attribute;

#[derive(Greetings)]
struct NiceGuy;

fn main() {
    let guy = NiceGuy;
    guy.greet()
}

#[awesome_attribute("foo", bar)]
struct Foo {
}