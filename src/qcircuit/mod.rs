use crate::qbit::structs::Qbit;

#[derive(Debug, Clone)]
pub struct QCircuit {
    qbits: Vec<Qbit>,
}
