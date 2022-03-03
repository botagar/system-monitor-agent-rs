#![feature(decl_macro)]

extern crate procstat;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod api;
mod system;

use std::{sync::{Arc,RwLock}, thread, time::{Duration, Instant}};

use machine_uid;

use procstat::ProcStat;

use crate::system::System;

fn main() {
    println!("Starting agent on port xxxx");
    eprintln!("Port not defined");

    let initial_state = initialise_system_state();

    let system_state: Arc<RwLock<System>> = Arc::new(RwLock::new(initial_state));
    let quick_loop_system_state = system_state.clone();

    thread::spawn(|| {
        quick_loop(1, quick_loop_system_state);
    });

    api::start(system_state);
}

fn initialise_system_state() -> System {
    let mut system_init = System::default();
    system_init.uid = machine_uid::get().unwrap();

    system_init
}

fn quick_loop(loop_period_sec: usize, system_state: Arc<RwLock<System>>) {
    struct CpuTimes {
        pub total: f64,
        pub idle: f64,
    }

    system_state.write().unwrap().cpu.name = "A CPU".to_string();

    let mut cpu_times_prev: Option<CpuTimes> = None;

    loop {
        let start_loop = Instant::now();

        match cpu_times_prev {
            Some(ref cpu_time) => {
                let proc_stat_now = ProcStat::read();

                let current_cpu_time_total = proc_stat_now.cpu.total() as f64;
                let current_cpu_time_idle = proc_stat_now.cpu.idle as f64;

                let delta_cpu_time_total = current_cpu_time_total - cpu_time.total;
                let delta_cpu_time_idle = current_cpu_time_idle - cpu_time.idle;

                let percent_busy = (1.0 - (delta_cpu_time_idle / delta_cpu_time_total)) * 100.0;
                
                system_state.write().unwrap().cpu.load = percent_busy;

                cpu_times_prev = Option::Some(CpuTimes {
                    total: current_cpu_time_total,
                    idle: current_cpu_time_idle,
                });

                // println!("CPU Busy: {}", percent_busy);
            }
            None => {
                let proc_stat_now = ProcStat::read();
                let cpu_now = proc_stat_now.cpu;

                cpu_times_prev = Option::Some(CpuTimes {
                    total: cpu_now.total() as f64,
                    idle: cpu_now.idle as f64,
                });
            },
        }


        let end_loop = Instant::now();
        
        let sleep_period = Duration::from_secs(loop_period_sec as u64) - end_loop.duration_since(start_loop);
        thread::sleep(sleep_period.clamp(Duration::from_millis(50), Duration::from_secs(loop_period_sec as u64)));
    }
}
