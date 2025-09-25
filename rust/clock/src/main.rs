use clock::Clock;

fn main() {
    dbg!(Clock::new(2, 20).add_minutes(-3000) == Clock::new(2, 20).add_minutes(-3000));
}
