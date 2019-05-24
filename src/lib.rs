#![feature(test)]

extern crate test;

pub mod quadprog;
pub mod dpofa;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn assert_approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < std::f64::EPSILON
    }

    #[test]
    fn it_works_solve_qp() {
        assert_approx_eq(42.0, quadprog::solve_qp().value);
    }

    #[test]
    fn it_works_dpofa() {
        let a = &mut vec![0.0, 2.0, 0.0, 0.0, 2.0];
        let info = dpofa::dpofa(a, 2, 2);

        assert_eq!(0, info);
        assert_approx_eq(0.0, a[0]);
        assert_approx_eq(std::f64::consts::SQRT_2, a[1]);
        assert_approx_eq(0.0, a[2]);
        assert_approx_eq(0.0, a[3]);
        assert_approx_eq(std::f64::consts::SQRT_2, a[4]);
    }

    #[bench]
    fn bench_solve_qp(b: &mut Bencher) {
        b.iter(quadprog::solve_qp);
    }

    #[bench]
    fn bench_dpofa(b: &mut Bencher) {
        let a = &mut vec![0.0, 2.0, 0.0, 0.0, 2.0];

        b.iter(|| dpofa::dpofa(a, 2, 2));
    }
}
