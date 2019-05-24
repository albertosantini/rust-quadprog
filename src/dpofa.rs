/// dpofa factors a double precision symmetric positive definite matrix.
///
/// **reference** https://www.physics.utoronto.ca/~esalesde/mc3ddox/dpofa_8f_source.html
///
/// **on entry**
///
/// - **a**     double precision(lda, n)
///             the symmetric matrix to be factored.  only the
///             diagonal and upper triangle are used.
/// - **lda**   integer
///             the leading dimension of the array  a.
/// - **n**     integer
///             the order of the matrix  a.
///
/// **return**
///
/// - **a**     an upper triangular matrix  r  so that  a = trans(r)r
///             where  trans(r)  is the transpose.
///             the strict lower triangle is unaltered.
///             if  info .ne. 0 , the factorization is not complete.
/// - **info**  integer
///
///     - = 0 for normal return.
///     - = k signals an error condition.  the leading minor
///           of order  k  is not positive definite.

pub fn dpofa(a: &mut Vec<f64>, lda: usize, n: usize) -> usize {
    assert_eq!(a.len(), lda * n + 1);

    let mut jm1: usize;
    let mut t: f64;
    let mut s: f64;
    let mut info = 0;

    for j in 1..=n {
        info = j;
        s = 0.0;
        jm1 = j - 1;

        if jm1 < 1 {
            s = a[j * j] - s;
        } else {
            for k in 1..=jm1 {
                // t = a[k][j] - ddot(k - 1, a[1][k], 1, a[1][j], 1);
                t = a[k * j];
                for i in 1..k {
                    t -= a[i * j] * a[i * k];
                }
                t /= a[k * k];
                a[k * j] = t;
                s += t * t;
            }
            s = a[j * j] - s;
        }

        if s <= 0.0 {
            break;
        }

        a[j * j] = f64::sqrt(s);
        info = 0;
    }

    info
}
