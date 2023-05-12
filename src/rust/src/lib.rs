use extendr_api::prelude::*;

struct MyString {
    pub s: String,
}

#[extendr(r_class_name = "BobbyJoeString")]
impl MyString {
    pub fn new() -> MyString {
        MyString { s: "im new".into() }
    }
    pub fn print(&self) {
        println!("mystring is {}", &self.s);
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    impl BobbyJoeString;
}
