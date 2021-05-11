#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();

    let message = "hello from rust!!";

    js! {
        console.log(@{message});
    }
}
