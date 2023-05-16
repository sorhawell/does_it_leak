use extendr_api::prelude::*;

#[derive(Clone)]
struct MyString {
    pub s: i32,
}

pub struct Uobj(Robj);
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

impl From<&Robj> for &Uobj {
    fn from(x: &Robj) -> Self {
        let y: &Uobj = unsafe { std::mem::transmute(x) }; // it is a tupple struct
        y
    }
}

impl TryFrom<Uobj> for String {
    type Error = extendr_api::error::Error;
    fn try_from(x: Uobj) -> std::result::Result<Self, Error> {
        Ok(x.0.as_str().unwrap().to_string())
    }
}

impl TryFrom<&Uobj> for &str {
    type Error = Error;
    fn try_from(robj: &Uobj) -> extendr_api::Result<Self> {
        let robj = &robj.0; //SOREN comment unpack
        if robj.is_na() {
            return Err(Error::MustNotBeNA(robj.clone()));
        }
        match robj.len() {
            0 => Err(Error::ExpectedNonZeroLength(robj.clone())),
            1 => {
                if let Some(s) = robj.as_str() {
                    Ok(s)
                } else {
                    Err(Error::ExpectedString(robj.clone()))
                }
            }
            _ => Err(Error::ExpectedScalar(robj.clone())),
        }
    }
}

//converted from extendr try_from_robj.rs
impl TryFrom<&Uobj> for String {
    type Error = Error;
    fn try_from(uobj: &Uobj) -> Result<Self> {
        <&str>::try_from(uobj).map(|s| s.to_string())
    }
}

// fn user_func(robj: Robj) -> Robj {
//     robj
// }

#[extendr(r_class_name = "BobbyJoeString", r_super_class_name = "AmericanString")]
impl MyString {
    pub fn new() -> MyString {
        MyString { s: 42 }
    }
    pub fn print(&self) {
        println!("mystring is {}", &self.s);
    }

    pub fn show(s: String) -> () {
        println!("{}", s);
    }
}

pub trait FromUobj: Sized {
    // Convert an incomming Robj from R into a value or an error.
    fn from_uobj(_robj: Uobj) -> std::result::Result<Self, String> {
        Err("unable to convert value from R object".into())
    }
}

// impl FromUobj for MyString {
//     fn from_uobj(uobj: Uobj) -> std::result::Result<Self, String> {
//         let robj = &uobj.0;
//         MyString::from_robj(robj)
//     }
// }

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    impl BobbyJoeString;
}
