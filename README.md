# QUADPROG

## Working in Progress. Not usable yet.

This library contains routines for solving quadratic programming problems for the Rust programming language.

quadprog is a porting of a [R](http://www.r-project.org) package:
[quadprog](http://cran.r-project.org/web/packages/quadprog/), implemented in
Fortran.

It implements the dual method of Goldfarb and Idnani (1982, 1983) for solving
quadratic programming problems of the form: 

`min(d T b + 1=2b T Db) with AT b >= b0`

## References

D. Goldfarb and A. Idnani (1982). Dual and Primal-Dual Methods for Solving
Strictly Convex Quadratic Programs. In J. P. Hennart (ed.), Numerical Analysis,
Springer-Verlag, Berlin, pages 226–239.

D. Goldfarb and A. Idnani (1983). A numerically stable dual method for solving
strictly convex quadratic programs. Mathematical Programming, 27, 1–33.

## Installation

This is a `Working In Progress` project, intending `alpha` quality, because it is my first project in Rust.

See also [node-quadprog](https://github.com/albertosantini/node-quadprog).

## Notes

**To maintain a one-to-one porting with the Fortran implementation, the array
index starts from 1 and not from zero. Please, be aware and give a look at the
examples in the test folder**.
