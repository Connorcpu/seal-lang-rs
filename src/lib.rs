#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate lazy_static;

extern crate unicode_xid;
extern crate simd;
extern crate num;

pub mod lexer;
#[allow(unused_imports)]
pub mod parser;
pub mod ast;
pub mod vm;
