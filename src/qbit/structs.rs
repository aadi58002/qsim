use color_eyre::Result;
use nalgebra::SMatrix;
use num::complex::{Complex, ComplexFloat};
use rand::{distributions::WeightedIndex, prelude::*};

use crate::qgates::clifford::*;

type QMatrix<const R: usize, const C: usize> = SMatrix<Complex<f64>, R, C>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Qbit {
    pub state: QMatrix<2, 1>,
    pub value: Option<bool>,
}

impl Qbit {
    pub fn new(bit: bool) -> Self {
        Self {
            state: SMatrix::<Complex<f64>, 2, 1>::new(
                Complex::new(!bit as u8 as f64, 0.0),
                Complex::new(bit as u8 as f64, 0.0),
            ),
            value: None,
        }
    }

    pub fn measure(&mut self) -> bool {
        if self.value.is_none() {
            let x_squared = self.state.x.re().powi(2);
            let y_squared = self.state.y.re().powi(2);

            let weights = [x_squared, y_squared];
            let weighted_index = WeightedIndex::new(&weights).unwrap();
            let mut rng = thread_rng();

            self.value = Some(weighted_index.sample(&mut rng) != 0);
            self.state = Qbit::new(self.value.unwrap()).state;
        }
        self.value.unwrap()
    }

    pub fn h(&mut self) {
        HADAMARD_GATE.apply(self);
    }
    pub fn px(&mut self) {
        PAULI_X_GATE.apply(self);
    }
    pub fn py(&mut self) {
        PAULI_Y_GATE.apply(self);
    }
    pub fn pz(&mut self) {
        PAULI_Z_GATE.apply(self);
    }
    pub fn cx(&mut self, target_qbit: &mut Qbit) {
        CONTROLLED_NOT.apply(self, target_qbit);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct QuantumTransformMatrix<const T: usize>(QMatrix<T, T>);

impl QuantumTransformMatrix<2> {
    pub const fn new(values: [(f64, f64); 4]) -> Self {
        Self(QMatrix::<2, 2>::new(
            Complex::new(values[0].0, values[0].1),
            Complex::new(values[1].0, values[1].1),
            Complex::new(values[2].0, values[2].1),
            Complex::new(values[3].0, values[3].1),
        ))
    }

    pub fn apply(self, qbit: &mut Qbit) {
        qbit.state = self.0 * qbit.state;
        qbit.value = None;
    }
}

impl QuantumTransformMatrix<4> {
    pub const fn new(values: [(f64, f64); 16]) -> Self {
        Self(QMatrix::<4, 4>::new(
            Complex::new(values[0].0, values[0].1),
            Complex::new(values[1].0, values[1].1),
            Complex::new(values[2].0, values[2].1),
            Complex::new(values[3].0, values[3].1),
            Complex::new(values[4].0, values[4].1),
            Complex::new(values[5].0, values[5].1),
            Complex::new(values[6].0, values[6].1),
            Complex::new(values[7].0, values[7].1),
            Complex::new(values[8].0, values[8].1),
            Complex::new(values[9].0, values[9].1),
            Complex::new(values[10].0, values[10].1),
            Complex::new(values[11].0, values[11].1),
            Complex::new(values[12].0, values[12].1),
            Complex::new(values[13].0, values[13].1),
            Complex::new(values[14].0, values[14].1),
            Complex::new(values[15].0, values[15].1),
        ))
    }

    pub fn apply(self, control_qbit: &mut Qbit, target_qbit: &mut Qbit) {
        if (control_qbit.measure()) {
            let changed_bit = self.0
                * QMatrix::<4, 1>::new(
                    control_qbit.state.x,
                    control_qbit.state.y,
                    target_qbit.state.x,
                    target_qbit.state.y,
                );
            control_qbit.state.x = changed_bit.x;
            control_qbit.state.y = changed_bit.y;
            target_qbit.state.x = changed_bit.z;
            target_qbit.state.y = changed_bit.w;
        }
        target_qbit.value = None;
    }
}
