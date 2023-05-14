use extendr_api::prelude::*;

#[derive(Clone)]
struct MyString {
    pub s: String,
}

struct Uobj(Robj);
impl From<Robj> for Uobj {
    fn from(x: Robj) -> Self {
        Uobj(x)
    }
}
impl From<Uobj> for Robj {
    fn from(x: Uobj) -> Self {
        x.0
    }
}

#[extendr(r_class_name = "BobbyJoeString", r_super_class_name = "AmericanString")]
impl MyString {
    pub fn new() -> MyString {
        MyString { s: "im new".into() }
    }
    pub fn print(&self) {
        println!("mystring is {}", &self.s);
    }

    pub fn show(ms: &MyString, s: String) -> std::result::Result<MyString, String> {
        ms.print();
        println!("{}", s);
        Ok(ms.clone())
    }
}

// impl<'a> extendr_api::FromRobj<'a> for BobbyJoeString {
//     fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
//         if robj.check_external_ptr_type::<BobbyJoeString>() {
//             #[allow(clippy::transmute_ptr_to_ref)]
//             Ok(unsafe { std::mem::transmute(robj.external_ptr_addr::<BobbyJoeString>()) })
//         } else {
//             Err(concat!("expected ", ""))
//         }
//     }
// }

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    impl BobbyJoeString;
}
