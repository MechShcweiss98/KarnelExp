extern crate rand;

// use self::rand::Rng;
// use spin::mutex::Mutex;
use spin::Mutex;
use rand::Rng;

extern "C" {
    fn nanosecond_timer_c() -> u64;
}

pub const MIN_RAND: u8 = 0;
pub const MAX_RAND: u8 = 100;

unsafe fn crate_rng() -> rand::rngs::StdRng{
    rand::rngs::StdRng::seed_from_u64(nanosecond_timer_c())
}

lazy_static! {
    // static ref RNG: Mutex<StdRng> = Mutex::new(rand::JitterRng::new_with_timer(|| unsafe { nanosecond_timer_c() }));
    static ref RNG: Mutex<rand::rngs::StdRng> = Mutex::new(unsafe {crate_rng()});
    pub static ref CONFIG: Mutex<ExperimentConfig> = Mutex::new(experimentConfig::new());
}

#[derive(Default)]
pub struct ExperimentConfig {
    pub chance: u8,
}

impl ExperimentConfig {
    pub fn new() -> Self {
        let chance = RNG.lock().gen_range(MIN_RAND..=MAX_RAND);
        ExperimentConfig { chance }
    }
}

#[no_mangle]
pub extern "C" fn sample() -> u8 {
    let sampled = RNG.lock().gen_range(MIN_RAND..=MAX_RAND);
    if sampled < CONFIG.lock().chance {
        panic!("Boom!");
    } else {
        sampled
    }
}

#[no_mangle]
pub extern "C" fn set_chance(chance: u8) {
    if chance >= MIN_RAND && chance <= MAX_RAND {
        CONFIG.lock().chance = chance
    }
}

#[no_mangle]
pub extern "C" fn get_chance() -> u8 {
    CONFIG.lock().chance
}
