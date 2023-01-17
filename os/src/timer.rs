
use riscv::register::time;

use crate::{config::CLOCK_FREQ, sbi::set_timer};

// 1秒分成100个时间片
const TICKS_PER_SEC: usize = 100;
// 1微妙=1秒
const MICRO_PER_SEC: usize = 1_000_000;


pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}


pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ/MICRO_PER_SEC)
}

pub fn get_time() -> usize {
    time::read()
}