# lp-solver

A basic linear program solver in Rust.


## Usage

This program assumes your problem is of the form,\
$$\min_{x\in\mathbb{R_{\geqslant 0}}} c^{\intercal}x \\ \text{s.t. } Ax\geq b$$\:
where $A\in M_{m\times n}(R)$, $b\in\mathbb{R}^m$ and $c\in\mathbb{R}^n$ for $m,n\in\mathbb{N}$.


## Basic Documentation

This program operates around `Problem` objects, which represent a linear
minimisation model with an objective function, a constraint set, and variables.

To build a model, use the following `Problem` methods:

```
::new()
```
Initialises an empty `Problem` instance.

```
.set_objective(c: Vec<T>, objtype: ObjType)
```
Sets the problem objective vector to `c`, where `objtype` can be `ObjType::Min`
or `ObjType::Max`. In the latter case the program converts to an equivalent
minimisation problem.

```
.add_var(vartype: VarType)
```
Adds variable of type `vartype`. This can be either `VarType::Binary`,
`VarType::Continuous`, or `VarType::Integer`.

```
.assert()
```
Asserts whether the current model's objective and constraint matrices match
in dimensions. Recall that the coefficient matrix $A$ must be of size
$m\times n$ for some $m,n\in\mathbb{N}$, $b$ must be of length $m$, and an
objective must be of size $n$.

---

To build the constraint set, either build (flattened) coefficient matrix
`ConstraintSet.A` and vector `ConstraintSet.b` directly, or use the following
`ConstraintSet` method to add individual constraints:

```
.add_constraint(A_i: Vec<T>, b_i: T, relation: Relation)
```
Adds a single constraint $A_ix\geq b_i$, $A_ix\leq b_i$, or $A_ix = b_i$ for
relation `Ge`, `Le`, or `Eq` respectively. In the latter two cases the
constraint converts to one or two equivalent $\geq$ constraints.
