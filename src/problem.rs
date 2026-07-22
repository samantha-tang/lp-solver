/// optimisation problem struct
pub struct Problem<T> {
    pub c: Vec<T>, //objective
    pub constraints: ConstraintSet<T>,
    pub vars: Vec<Variable<T>>,
}

impl<T: Clone + std::ops::Neg<Output = T>> Problem<T> {
    pub fn new() -> Self {
        Self {
            c: vec![],
            constraints: ConstraintSet::new(),
            vars: vec![],
        }
    }
    pub fn assert(&self) -> Result<(), String> {
        if self.constraints.A.m != self.constraints.b.len() || self.constraints.A.n != self.c.len() {
            Err(String::from("Inputs A, b, c are not the right relative dimensions."))
        } else if self.vars.len() != self.constraints.A.n {
            Err(String::from("Coefficient matrix A should have {self.vars.len} columns."))
        } else {
            Ok(())
        }
    }
    pub fn set_objective(&mut self, c: Vec<T>, objtype: ObjType) {
        match objtype {
            ObjType::Min => {
                self.c = c;
            },           
            ObjType::Max => {
                self.c = c.iter().cloned().map(|x| -x).collect()
            },
        }
    }
    pub fn add_var(&mut self, vartype: VarType) {
        self.vars.push(Variable::new(vartype));
    }
}

// constraint set, interpreted as Ax >= b
#[allow(non_snake_case)]
pub struct ConstraintSet<T> { 
    pub A: Matrix<T>,
    pub b: Vec<T>,
}

impl<T: Clone + std::ops::Neg<Output = T>> ConstraintSet<T> {
    fn new() -> Self {
        Self { A: Matrix::new(), b: vec![] }
    }
    fn push_row(&mut self, a_row: Vec<T>, b_val: T) {
        self.A.data.extend(a_row);
        self.A.m += 1;
        self.b.push(b_val);
    }
    fn negate_row(a_row: &[T], b_val: &T) -> (Vec<T>, T) {
        (a_row.iter().cloned().map(|x| -x).collect(), -b_val.clone())
    }
    pub fn add_constraint(&mut self, a_i: Vec<T>, b_i: T, relation: Relation) {
        match relation {
            Relation::Ge => self.push_row(a_i, b_i),
            Relation::Le => {
                let (neg_a, neg_b) = Self::negate_row(&a_i, &b_i);
                self.push_row(neg_a, neg_b);
            },
            Relation::Eq => {
                let (neg_a, neg_b) = Self::negate_row(&a_i, &b_i);
                self.push_row(a_i, b_i);
                self.push_row(neg_a, neg_b);
            },
        }
    }
}

pub struct Variable<T> {
    pub vartype: VarType,
    pub value: Option<T>,
}

impl<T> Variable<T> {
    fn new(vartype: VarType) -> Self {
        Self { vartype, value: None }
    }
}

/// flattened matrix struct
pub struct Matrix<T> {
    pub m: usize,
    pub n: usize,
    pub data: Vec<T>,
}

impl<T> Matrix<T> {
    fn new() -> Self {
        Self { m: 0, n: 0, data: vec![] }
    }
}

pub enum ObjType {
    Min,
    Max,
}

pub enum Relation {
    Ge,
    Le,
    Eq,
}

pub enum VarType {
    Binary,
    Continuous,
    Integer,
}
