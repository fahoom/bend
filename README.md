# bend

Bend is a compiler backend, intended for me to learn more about intermediate representations and compiler optimisations.

## High-Level Overview

The biggest unit in the program is a `Module`, which contains global variables, type definitions and `Function`s.

A `Function` contains a list of `Instruction`s and `BasicBlock`s.

## Data Types

The IR will initially just work on `i64`s (2's complement signed integers), and aggregates of these. Further down the line, it would be worth adding support for other power of two integers
and floating point values.

I am a huge fan of the idea of opaque pointer types, so I may implement these because I think they are very cool.

## Instruction Set

Binary Operations: `add`, `sub`, `mul`, `div`...; Some of these operations may produce multiple results. It would be worth looking at integrating tuples into the IR.

Terminators: `ret`, `branch`...

Memory: `load`, `store`, `alloca` ...; No GEP. I don't like the idea of GEP.

## IR Construction

To construct our IR, we will use Braun's algorithm [Simple and Efficient Construction of SSA](https://c9x.me/compile/bib/braun13cc.pdf), allowing us to produce our IR straight from an
AST. Construction should be done through an `IRBuilder` interface.

## IR Optimisation Passes

Some key optimisation passes I should implement:

- Dead Code Elimnation
- Scalar Replacement of Aggregates
- Loop Invariant Code Motion
- Common Subexpression Elimination
- Constant Propogation

## Register Allocation

## Instruction Selection

