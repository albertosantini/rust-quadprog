/// Answer struct
#[derive(Debug)]
pub struct Answer {
    pub solution: Vec<f64>,
    pub lagrangian: Vec<f64>,
    pub value: f64,
    pub unconstrained_solution: Vec<f64>,
    pub iterations: Vec<f64>,
    pub iact: i32,
    pub message: String,
}


/// # solve_qp
///
/// ## Call quadratic programming optimization
///
/// JS signature
/// ```function solveQP(Dmat, dvec, Amat, bvec = [], meq = 0, factorized = [0, 0])```
pub fn solve_qp() -> Answer {
    Answer {
        solution: vec![1.0, 2.0, 3.0],
        lagrangian: vec![4.0, 5.0, 6.0],
        value: 42.0,
        unconstrained_solution: vec![7.0, 8.0, 9.0],
        iterations: vec![10.0, 11.0, 12.0],
        iact: 11,
        message: "Hello, world!".to_string(),
    }
}
