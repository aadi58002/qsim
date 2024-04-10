#![allow(unused)]

#![feature(get_many_mut)]

mod qbit;
mod qcircuit;
mod qgates;

use qbit::structs::Qbit;
use qcircuit::register::QRegister;

fn main() {
    let mut count_00 = 0;
    let mut count_01 = 0;
    let mut count_10 = 0;
    let mut count_11 = 0;

    for _ in 0..10000{
        let mut qreg = QRegister::new(2);
        qreg.h(0);
        qreg.cx(0,1);
        qreg.cx(1,0);
        let output = qreg.measure();
        match (output[1],output[0]) {
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
