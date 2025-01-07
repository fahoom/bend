# bend

Bend is a compiler backend, intended for me to learn more about intermediate representations and compiler optimisations.

## Resources

- [Modern Compiler Implementation in ML](https://www.cs.princeton.edu/~appel/modern/)
- [Simple and Efficient Construction of SSA](https://c9x.me/compile/bib/braun13cc.pdf)

## High-Level Overview

The biggest unit in the program is a `Module`, which contains global variables, type definitions and `Function`s.

A `Function` contains a list of `Instruction`s and `BasicBlock`s.

## Instruction Set

TODO!

## Compiler Processes

### IR Construction

TODO!

### IR Optimisation Passes

<details>
<summary>Passes I've encountered researching</summary>
This is just a list of common optimisation passes I have found. They are not implemented yet.

- Inlining
- Copy Propagation
- Dead Code Elimination
- Global common sub-expression elimination
- Global value numbering
- Loop invariant code motion
- Sparse conditional constant propagation
- Strength reduction
- Tail recursion elimination
- Expression reassocoation
- Partial redundancy elimination
- Loop unrolling
- Scalar replacement
- Vectorisation
</details>

TODO!

### Register Allocation

<details>
<summary>Rough Notes</summary>

Register allocation is just a graph-colouring problem. Need to handle spilling onto stack
splitting ranges where a register is used. Most JIT compilers use a linear scan instead.

</details>

TODO!

### Instruction Selection

TODO!
