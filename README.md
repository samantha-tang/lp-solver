# lp-solver

A basic linear program solver in Rust.

## Usage

This program assumes your problem is of the form,\
$$\min_{x\in\mathbb{R_{\geqslant 0}}} c^{\intercal}x \\ \text{s.t. } Ax\geq b$$\:
where $A\in M_{m\times n}(R)$, $b\in\mathbb{R}^m$ and $c\in\mathbb{R}^n$ for $m,n\in\mathbb{N}$.
