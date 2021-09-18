use greetings::Greetings;
use greetings_derive::Greetings;
use attribute::*;

#[derive(Greetings)]
struct NiceGuy;

fn main() {
    do_stuff(42)
}

#[awesome_attribute("foo", bar)]
struct Foo {
}

#[debug]
fn do_stuff() {

}