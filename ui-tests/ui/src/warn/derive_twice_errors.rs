#![allow(dead_code)]

#[derive(derive::Deprecated)]
struct Test;

// Will error since the derive macro re-uses the name `test` twice.
#[derive(derive::Deprecated)]
struct Test2;

fn main() {
}
