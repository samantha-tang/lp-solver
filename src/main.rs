mod simplex;
pub mod problem;

use problem::{Problem, ConstraintSet, Variable, Matrix, VarType};


fn main() {
    let constraints = ConstraintSet {
        A: Matrix { m: 4, n: 2, data: vec![4,1,1,1,-1,0,0,-1] },
        b: vec![8,4,0,0],
    };
    let vars = vec![
            Variable { vartype: VarType::Continuous, value: None },
            Variable { vartype: VarType::Continuous, value: None },
        ];
    let problem = Problem { c: vec![2,1], constraints, vars };

    println!("{:?}", problem.assert());
}
