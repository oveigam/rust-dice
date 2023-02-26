mod dice;

mod simulations {
    pub mod save_ace;
}

static GAMES: u64 = 1000000;
static THREADS: u64 = 32;

fn main() {
    simulations::save_ace::simulation();
}
