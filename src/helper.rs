use std::ops::{Index, IndexMut};


/// flattened matrix struct
pub struct Matrix<T> {
    pub m: usize,
    pub n: usize,
    pub data: Vec<T>,
}

/// read matrix entries with matrix[index1][index2]
impl<T> Index<usize> for Matrix<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

/// mutate matrix entry
impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// optimisation problem struct (currently assumes minimisation objective)
pub struct Problem<T> {
    // constraints
    pub A: Matrix<T>,
    pub b: Vec<T>,
    // objective function
    pub c: Vec<T>,
    pub vars: Vec<Variable<T>>,
}


pub enum VarType {
    Binary,
    Continuous,
}


pub struct Variable<T> {
    pub vartype: VarType,
    pub value: Option<T>,
}


impl<T> Problem<T> {
    pub fn assert(&self) -> Result<(), String> {
        if self.A.m == self.b.len() || self.A.n == self.c.len() {
            Err(String::from("Inputs are not the right relative dimensions."))
        } else {
            Ok(())
        }
    }
}
