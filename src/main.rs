#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#![feature(collections)]

/* I'm not sure why this is necessary to silence "naming is uncertain
 * with container conventions" warnings. Hypothesis is that peg parser
 * expansion uses some unstable API.
 */
#![feature(core)]

extern crate collections;

mod ast;
