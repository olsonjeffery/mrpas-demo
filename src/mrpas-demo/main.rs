// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "mrpas-demo#0.1",
       uuid = "1190bd55-d4bb-43fb-8f5d-b68d8449f7ff")];
#[crate_id="mrpas-demo#0.1"];
#[desc = "MRPAS demo"];
#[license = "3-Clause BSD"];

extern mod extra;
use extra::time;

pub mod map;
pub mod mrpas;

fn main() {
    let mut map = map::Map::example();
    let focus = (35,9);
    let max_radius = 10;
    let mut start_angle_buf = [0f64, ..1028];
    let mut end_angle_buf = [0f64, ..1028];
    let mut run_times = ~[];
    let num_of_runs = 101u64;
    let mut ctr = 0;
    while ctr < num_of_runs {
        map.reset();
        let before_time = time::precise_time_ns();
        mrpas::compute(&mut map, focus, max_radius,
                       start_angle_buf.mut_slice_from(0),
                       end_angle_buf.mut_slice_from(0));
        let after_time = time::precise_time_ns();
        run_times.push(after_time - before_time);
        ctr += 1;
    }
    run_times.sort();
    let median = run_times[50];
    let mean = {
        let mut res = 0;
        for i in run_times.iter() {
            res += *i;
        }
        res / num_of_runs
    };
    map.draw_to_stdout((45, 20), focus);
    println!("Over {:?} runs, mean: {:?} nsecs, median: {:?} nsecs", num_of_runs, mean, median);
}