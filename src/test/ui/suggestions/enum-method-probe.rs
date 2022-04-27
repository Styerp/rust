// run-rustfix
#![allow(unused)]

struct Foo;

impl Foo {
    fn get(&self) -> u8 {
        42
    }
}

fn test_result_in_result() -> Result<(), ()> {
    let res: Result<_, ()> = Ok(Foo);
    res.get();
    //~^ ERROR no method named `get` found for enum `Result` in the current scope
    //~| HELP use the `?` operator
    Ok(())
}

fn test_result_in_plain() {
    let res: Result<_, ()> = Ok(Foo);
    res.get();
    //~^ ERROR no method named `get` found for enum `Result` in the current scope
    //~| HELP consider using `Result::expect` to unwrap the `Foo` value, panicking if the value is an `Err`
}

fn test_option_in_option() -> Option<()> {
    let res: Option<_> = Some(Foo);
    res.get();
    //~^ ERROR no method named `get` found for enum `Option` in the current scope
    //~| HELP use the `?` operator
    Some(())
}

fn test_option_in_plain() {
    let res: Option<_> = Some(Foo);
    res.get();
    //~^ ERROR no method named `get` found for enum `Option` in the current scope
    //~| HELP consider using `Option::expect` to unwrap the `Foo` value, panicking if the value is `None`
}

fn main() {}
