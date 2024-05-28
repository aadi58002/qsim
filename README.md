# Intro
Qsim is a self learning project for better understanding of quantum computer.

# Example
Basic circuit example
```rust
#![allow(unused)]
#![feature(get_many_mut)]

mod algo;
mod qbit;
mod qcircuit;
mod qgates;

use algo::grover::GroverState;
use qbit::structs::Qbit;
use qcircuit::register::QRegister;
use std::collections::HashMap;

fn main() {
    let mut freq_map: HashMap<Vec<bool>, usize> = HashMap::new();
    let mut qc_main = qcircuit::register::QRegister::new(5);
    for _ in 0..10000 {
        let mut qc = qc_main.clone();
        qc.h(0);
        qc.h(1);
        qc.h(1);
        qc.h(2);
        qc.cx(2, 3);
        qc.cx(2, 4);

        *freq_map.entry(qc.measure()).or_insert(1) += 1;
    }
    dbg!(freq_map);

}
```

# Example 2 (Grover's Algo)

```rust
#![allow(unused)]
#![feature(get_many_mut)]

mod algo;
mod qbit;
mod qcircuit;
mod qgates;

use algo::grover::GroverState;
use qbit::structs::Qbit;
use qcircuit::register::QRegister;
use std::collections::HashMap;

fn main() {
    let bool_vec = vec![
        [false, false, false],
        [false, false, true],
        [false, true, false],
        [true, false, false],
        [false, true, true],
        [true, false, true],
        [true, true, false],
        [true, true, true],
    ];
    
    let mut freq_map = HashMap::new();
    for val in bool_vec {
        freq_map.insert(val.to_vec(),0);
    }
    
    let grov = GroverState::new(vec![[true, true, true], [true, false, false]]);
    for _ in 0..10000 {
        freq_map.entry(grov.collapse()).and_modify(|val| *val += 1);
    }
    dbg!(freq_map);
}
```
