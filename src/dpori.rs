/// dpori
///
/// **on entry**
///
/// - **a**     double precision(lda, n)
///
/// - **lda**   integer
///             the leading dimension of the array  a.
/// - **n**     integer
///             the order of the matrix  a.
///

pub fn dpori(a: &mut Vec<f64>, _lda: usize, n: usize) {
    let mut kp1: usize;
    let mut t: f64;

    for k in 1..=n {
        a[k * k] = 1.0 / a[k * k];
        t = -a[k * k];

        // dscal(k - 1, t, a[1][k], 1);
        for i in 1..k {
            a[i * k] *= t;
        }

        kp1 = k + 1;
        if n < kp1 {
            break;
        }
        for j in kp1..=n {
            t = a[k * j];
            a[k * j] = 0.0;

            // daxpy(k, t, a[1][k], 1, a[1][j], 1);
            for i in 1..=k {
                a[i * j] += t * a[i * k];
            }
        }
    }
}
