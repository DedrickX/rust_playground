mod buckets;
mod game;
mod player;
mod vectors;
mod word_counter;

fn main() {
    buckets::test();
    vectors::test();
    player::test();
    word_counter::test();
    game::test();
}
