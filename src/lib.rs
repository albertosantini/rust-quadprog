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
        let a = &mut vec![0.0, 2.0, 0.0, 0.0, 2.0];
        let info = dpofa::dpofa(a, 2, 2);

        assert_eq!(0, info);
        assert_eq!(0.0, a[0]);
        assert_eq!(1.4142135623730951, a[1]);
        assert_eq!(0.0, a[2]);
        assert_eq!(0.0, a[3]);
        assert_eq!(1.4142135623730951, a[4]);
    }

    #[bench]
    fn bench_solve_qp(b: &mut Bencher) {
        b.iter(|| quadprog::solve_qp());
    }

    #[bench]
    fn bench_dpofa(b: &mut Bencher) {
        let a = &mut vec![0.0, 2.0, 0.0, 0.0, 2.0];

        b.iter(|| dpofa::dpofa(a, 2, 2));
    }
}
