extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod buckets;
mod drop_example;
mod errors;
mod game;
mod generic_lifetimes;
mod lifetimes;
mod moving;
mod player;
mod vectors;
mod word_counter;

fn main() {
    splitter();
    moving::test();

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
    errors::result_type();

    splitter();
    errors::option_type();

    splitter();
    errors::getting_result();

    splitter();
    lifetimes::lifetimes_simple();

    splitter();
    lifetimes::return_list_example();

    splitter();
    generic_lifetimes::game();

    splitter();
    generic_lifetimes::stemming_words();
}

fn splitter() {
    println!("\n------------------");
}
