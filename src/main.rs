mod dice;

mod simulations {
    pub mod save_ace;
    pub mod orphans;
}

static GAMES: u64 = 100000000;
static THREADS: u64 = 32;

fn main() {
    // simulations::save_ace::simulation();
    simulations::orphans::simulation();
}
