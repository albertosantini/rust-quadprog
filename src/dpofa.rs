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

// fn dot_cblas(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
// function dpofa(a, lda, n, info) {
pub fn dpofa(a: &Vec<f64>, lda: usize, n: usize) -> i32 {
    assert_eq!(a.len(), lda * n);

    let info = 0;

    info
}
