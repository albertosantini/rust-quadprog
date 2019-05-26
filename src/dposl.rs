#![allow(clippy::many_single_char_names)]

/// dposl
///
/// **on entry**
///
/// - **a**     double precision(lda, n)
///
/// - **lda**   integer
///             the leading dimension of the array  a.
/// - **n**     integer
///             the order of the matrix  a.
/// - **b**     double precision(lda, n)

pub fn dpori(a: &mut Vec<f64>, _lda: usize, n: usize, b: &mut Vec<f64>) {
    let mut k: usize;
    let mut t: f64;

    for k in 1..=n {
        // t = ddot(k - 1, a[1][k], 1, b[1], 1);
        t = 0.0;
        for i in 1..k {
            t += a[i * k] * b[i];
        }

        b[k] = (b[k] - t) / a[k * k];
    }

    for kb in 1..=n {
        k = n + 1 - kb;
        b[k] /= a[k * k];
        t = - b[k];

        // daxpy(k, t, a[1][k], 1, a[1][j], 1);
        for i in 1..=k {
            b[i] += t * a[i * k];
        }
    }
}
