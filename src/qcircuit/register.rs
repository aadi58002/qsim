use std::ops::Index;

use crate::qbit::structs::Qbit;
use crate::qgates::clifford::*;

#[derive(Debug, Clone)]
pub struct QRegister {
    qbits: Vec<Qbit>,
}

impl Index<usize> for QRegister {
    type Output = Qbit;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.qbits[i]
    }
}

impl QRegister {
    pub fn new(qbit_num: usize) -> Self {
        QRegister {
            qbits: vec![Qbit::new(false); qbit_num],
        }
    }

    pub fn measure(self) -> Vec<bool>{
        let mut output = Vec::with_capacity(self.qbits.len());
        for mut val in self.qbits{
            output.push(val.measure());
        }
        output
    }

    pub fn h(&mut self,target_qbit: usize) {
        HADAMARD_GATE.apply(&mut self.qbits[target_qbit]);
    }
    pub fn px(&mut self,target_qbit: usize) {
        PAULI_X_GATE.apply(&mut self.qbits[target_qbit]);
    }
    pub fn py(&mut self,target_qbit: usize) {
        PAULI_Y_GATE.apply(&mut self.qbits[target_qbit]);
    }
    pub fn pz(&mut self,target_qbit: usize) {
        PAULI_Z_GATE.apply(&mut self.qbits[target_qbit]);
    }
    pub fn cx(&mut self,control_qbit: usize,target_qbit: usize) {
        assert_ne!(target_qbit,control_qbit);
        let [control,target] = self.qbits.get_many_mut([control_qbit,target_qbit]).unwrap();
        CONTROLLED_NOT.apply(control,target);
    }
}
