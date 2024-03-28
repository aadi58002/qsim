#![allow(unused)]

mod qbit;
mod qcircuit;
mod qgates;

use qbit::structs::Qbit;

fn main() {
    let mut count_00 = 0;
    let mut count_01 = 0;
    let mut count_10 = 0;
    let mut count_11 = 0;

    for _ in 0..10000 {
        let mut q = Qbit::new(false);
        let mut tq = Qbit::new(false);
        q.h();
        // dbg!(q,tq);
        q.cx(&mut tq);
        // dbg!(q,tq);
        tq.cx(&mut q);
        // dbg!(q,tq);
        match (q.measure(), tq.measure()) {
            (true, true) => count_11 += 1,
            (true, false) => count_10 += 1,
            (false, true) => count_01 += 1,
            (false, false) => count_00 += 1,
        }
    }

    println!(
        "count_00: {}, count_01: {}, count_10: {}, count_11: {}",
        count_00, count_01, count_10, count_11
    );
}
