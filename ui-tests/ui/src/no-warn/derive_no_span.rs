//! Without a span no warning will be printed.

#[derive(derive::DeprecatedNoSpan)]
struct Test {

}

fn main() {
    let _ = Test { };
}
