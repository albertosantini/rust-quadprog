#![feature(test)]

extern crate test;

pub mod quadprog;
pub mod dpofa;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works_solve_qp() {
        assert_eq!(42.0, quadprog::solve_qp().value);
    }

    #[test]
    fn it_works_dpofa() {
        assert_eq!(0, dpofa::dpofa(&vec![1.0, 2.0, 3.0], 1, 3));
    }

    #[bench]
    fn bench_solve_qp(b: &mut Bencher) {
        b.iter(|| quadprog::solve_qp());
    }

    #[bench]
    fn bench_dpofa(b: &mut Bencher) {
        b.iter(|| dpofa::dpofa(&vec![1.0, 2.0, 3.0], 1, 3));
    }
}
