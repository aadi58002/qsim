use crate::qbit::structs::QuantumTransformMatrix;
use num::complex::{Complex, ComplexFloat};
use std::f64::consts::SQRT_2;

pub const PAULI_X_GATE: QuantumTransformMatrix<2> =
    QuantumTransformMatrix::<2>::new([(0.0, 0.0), (1.0, 0.0), (1.0, 0.0), (0.0, 0.0)]);

pub const PAULI_Y_GATE: QuantumTransformMatrix<2> =
    QuantumTransformMatrix::<2>::new([(0.0, 0.0), (0.0, -1.0), (0.0, 1.0), (0.0, 0.0)]);

pub const PAULI_Z_GATE: QuantumTransformMatrix<2> =
    QuantumTransformMatrix::<2>::new([(1.0, 0.0), (0.0, 0.0), (0.0, 0.0), (-1.0, 0.0)]);

pub const HADAMARD_GATE: QuantumTransformMatrix<2> = QuantumTransformMatrix::<2>::new([
    (1_f64 / SQRT_2, 0.0),
    (1_f64 / SQRT_2, 0.0),
    (1_f64 / SQRT_2, 0.0),
    (-1_f64 / SQRT_2, 0.0),
]);

pub const SQRT_X_GATE: QuantumTransformMatrix<2> =
    QuantumTransformMatrix::<2>::new([(0.5, 0.5), (0.5, -0.5), (0.5, -0.5), (0.5, 0.5)]);

pub const SQRT_Z_GATE: QuantumTransformMatrix<2> =
    QuantumTransformMatrix::<2>::new([(1.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 1.0)]);

pub const CONTROLLED_NOT: QuantumTransformMatrix<4> = QuantumTransformMatrix::<4>::new([
    (1.0, 0.0),
    (0.0, 0.0),
    (0.0, 0.0),
    (0.0, 0.0),

    (0.0, 0.0),
    (1.0, 0.0),
    (0.0, 0.0),
    (0.0, 0.0),

    (0.0, 0.0),
    (0.0, 0.0),
    (0.0, 0.0),
    (1.0, 0.0),

    (0.0, 0.0),
    (0.0, 0.0),
    (1.0, 0.0),
    (0.0, 0.0),
]);
