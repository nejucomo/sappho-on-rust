======
Sappho
======

Every K months along a Poisson distribution, I begin a new implementation
of a mythical hobby programming language I call Sappho.  This is the
implementation of the day: an interpreter written in Rust.

Status
======

**Status:** a primordial prototype. I'm creating unit tests of the parser
matching input source code against expected parse results.

Testing Goals
=============

There are several goals with respect to automated testing of
sappho-on-rust, in priority order:

1. Use standalone test-vector files, so that it's easy for other implementations to test against the same vectors.
2. Don't Repeat Yourself - write test cases once without having to maintain multiple mappings of the cases.
3. Have separate fine-grained test cases so that failures are very specific and easy to identify.

In rust, each test case must be a separately defined function. We can achieve 1 by using the `include_str` macro. We can achieve 3 by using macros to generate test cases. However, there's no way to loop on test vector directories at macro-expansion time to generate cases automatically from directory contents. Therefore, we can't achieve 2 with that approach.

So far we've been sacrificing 2 by naming each test case file redundantly in macro calls. We intend to transition to sacrifice 3 in favor of 2 by using the `include_dir` crate, and having large "coarse" tests that loop over cases within a single test.

TODO
====

#. Try using `impl Trait` and removing `parser()` calls.
#. Introduce query/proc 𝜆.
#. Create type abstraction over AST to reify Sappho mutation categories into the rust type system.

