use std::time;

pub const CPU_CLOCK_HZ: u128 = 4_194_304;
pub const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;

fn play() {
    let time = time::Instant::now();
    let mut elapsed = 0;
    loop {
        let e = time.elapsed().as_nanos();
        for _ in 0..(e - elapsed) / M_CYCLE_NANOS {
            // TODO: run emulate_cycle for the components

            elapsed += M_CYCLE_NANOS;
        }
    }
}
