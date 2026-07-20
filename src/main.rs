mod simplex;
pub mod helper;

use helper::{Problem, Matrix, Variable, VarType}


fn main() {
    let problem = Problem {
        A: Matrix { m: 4, n: 2, data: vec![4,1,1,1,-1,0,0,-1] },
        b: vec![8,4,0,0],
        c: vec![2,1],
        vars: vec![Variable { vartype: VarType::Continuous, value: None }],
    };

    problem.assert();
}
