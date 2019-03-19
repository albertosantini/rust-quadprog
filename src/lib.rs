#![feature(test)]

extern crate test;

pub mod quadprog;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(42.0, quadprog::solve_qp().value);
    }

    #[bench]
    fn bench_solve_qp(b: &mut Bencher) {
        b.iter(|| quadprog::solve_qp());
    }
}
