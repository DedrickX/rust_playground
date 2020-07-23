extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod buckets;
mod drop_example;
mod errors;
mod game;
mod player;
mod vectors;
mod word_counter;

fn main() {
    splitter();
    buckets::test();

    splitter();
    vectors::test();

    splitter();
    player::test();

    splitter();
    word_counter::test();

    splitter();
    game::test();

    splitter();
    drop_example::test();

    splitter();
    errors::test_result();

    splitter();
    errors::test_option();

    splitter();
    errors::getting_result();
}

fn splitter() {
    println!("\n------------------");
}
