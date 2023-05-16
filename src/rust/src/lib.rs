use extendr_api::prelude::*;
use extendr_api::Result as EResult;
use std::fmt::Debug;
use std::result::Result;

// -0- some random opinionated conversion
fn usize_bound_check<T>(x: T) -> Result<T, String>
where
    T: PartialOrd,
    T: Default,
    T: Debug,
{
    if x < T::default() {
        Err(format!("usize cannot be smaller than 0 it was ({:?})", x))
    } else {
        Ok(x)
    }
}
fn inner_converter_usize(x: Robj) -> Result<usize, String> {
    match x.rtype() {
        _ if x.len() != 1 => Err("error input length must equal to one".into()),
        Rtype::Doubles => {
            usize_bound_check(x.as_real().expect("as matched")).map(|val| val as usize)
        }
        Rtype::Integers => {
            usize_bound_check(x.as_integer().expect("as matched")).map(|val| val as usize)
        }
        _ => Err("hey im just a simple converter!!".into()),
    }
}

// -1- simple Wrap<T> which has TryFrom
struct Wrap<T>(T);
impl TryFrom<Robj> for Wrap<usize> {
    type Error = extendr_api::Error;
    fn try_from(x: Robj) -> EResult<Wrap<usize>> {
        inner_converter_usize(x)
            .map_err(|str| Error::Other(str.into()))
            .map(|val| Wrap(val))
    }
}

//this will still throw error with mem leaking, due to unwrap_or_throw_error() in extendr macros
#[extendr(use_try_from = true)]
fn conversion_extendr_error_throw(x: Wrap<usize>) -> Result<f64, String> {
    let my_usize: usize = x.0;
    Ok(my_usize as f64)
}

// -2- error redirected from extendr-error handling, the conversion has to be non-fallible, delay the Error
// implement Wrap for a Result
impl From<Robj> for Wrap<Result<usize, String>> {
    fn from(x: Robj) -> Self {
        Wrap(inner_converter_usize(x).map_err(|str| str.to_string()))
    }
}

//this is ok, but not very ergonomic
#[extendr(use_try_from = true)]
fn conversion_manual_error_handler(x: Wrap<Result<usize, String>>) -> Result<f64, String> {
    let my_usize: usize = x.0? + 42; //catch any conversion error and bubble it to output
    Ok(my_usize as f64)
}

// -3- same as 2, but more abbreviated, also showcase use of exotic Errors with implicit conversion
struct WR<T, E>(Result<T, E>);
impl From<Robj> for WR<usize, MyExoticError> {
    fn from(x: Robj) -> Self {
        WR(inner_converter_usize(x).map_err(|str| str.to_string().into())) //converts .into() MyExoticError
    }
}
//implicit conversions between to Error-types
struct MyExoticError(String);
impl From<String> for MyExoticError {
    fn from(x: String) -> Self {
        MyExoticError(x)
    }
}
impl From<MyExoticError> for String {
    fn from(x: MyExoticError) -> Self {
        x.0
    }
}

//use two Error types and perform implicit conversions
#[extendr(use_try_from = true)]
fn conversion_manual_error_handler_short(x: WR<usize, MyExoticError>) -> Result<f64, String> {
    let my_usize: usize = x.0? + 42; //COMMENT ? here uses implicitly `From<MyExoticError> for String`
    Ok(my_usize as f64)
}

// -4- fixed error type to make the wrapper more ergonomic
struct WRS<T>(Result<T, MyExoticError>);
impl From<Robj> for WRS<usize> {
    fn from(x: Robj) -> Self {
        WRS(inner_converter_usize(x).map_err(|str| str.to_string().into()))
    }
}

//this is quite short, but err context handling is missing
#[extendr(use_try_from = true)]
fn conversion_manual_error_handler_shorter(x: WRS<usize>) -> Result<f64, String> {
    let my_usize: usize = x.0? + 42;
    Ok(my_usize as f64)
}

// -5- add some error context methods
impl<T> WRS<T> {
    pub fn blame_param(self, param_name: &str) -> Self {
        Self(self.0.map_err(|err_msg| {
            format!("for param [{}] an error because {}", param_name, err_msg.0).into()
        }))
    }
    pub fn add_context(self, context: &str) -> Self {
        Self(
            self.0
                .map_err(|err_msg| format!("in {}: {}", context, err_msg.0).into()),
        )
    }
}

//blame the param x
#[extendr(use_try_from = true)]
fn usize_add_42_blame(x: WRS<usize>) -> Result<f64, String> {
    let my_usize: usize = x.blame_param("x").0? + 42;
    Ok(my_usize as f64)
}

//add context, I prefer to not add final context on rust-side, because two different public function may call
//the same underlying private rust function. It is better to add the final context in specific public function
#[extendr(use_try_from = true)]
fn usize_add_42_context_blame(x: WRS<usize>) -> Result<f64, String> {
    let my_usize: usize = x.blame_param("x").add_context("usize_add_42").0? + 42;
    Ok(my_usize as f64)
}

// -6- use a little macro snippet to not mention the param name x twice (nice if a very long param name)
macro_rules! blame {
    ($param:ident) => {
        $param.blame_param(stringify!($param))
    };
}
#[extendr(use_try_from = true)]
fn usize_add_42_blame_macro(x: WRS<usize>) -> Result<f64, String> {
    let my_usize: usize = blame!(x).0? + 42;
    Ok(my_usize as f64)
}

// -7- use a little macro snippet to not mention the param name x twice (nice if a very long param name)
impl From<MyExoticError> for Robj {
    fn from(x: MyExoticError) -> Self {
        x.0.into() // this .into() relies on that my extendr branch has impl From<String> for Robj implemented also
    }
}
#[extendr(use_try_from = true)]
fn usize_add_42_blame_macro_exotic_error(x: WRS<usize>) -> Result<f64, MyExoticError> {
    let my_usize: usize = blame!(x).0? + 42;
    Ok(my_usize as f64)
}

// -8- add superclass tagging struct
struct SuperClassMe<T>(T, &'static str);
impl<T> From<SuperClassMe<T>> for Robj
where
    T: IntoRobj,
{
    //this conversion could be done with fewer string allocations
    fn from(x: SuperClassMe<T>) -> Self {
        let sc = x.1;
        let robj = x.0.into_robj();
        let mut v: Vec<_> = robj.class().expect("hey why you no class?!").collect();
        v.push(sc.into());
        robj.set_class(v).expect("oups");
        robj
    }
}

struct MyStruct(usize);

#[extendr]
impl MyStruct {
    fn print(&self) {
        rprintln!("imma MyStruct with value = {}", self.0);
    }
}

#[extendr(use_try_from = true)]
fn usize_add_42_superclass(x: WRS<usize>) -> Result<SuperClassMe<MyStruct>, MyExoticError> {
    let my_usize: usize = blame!(x).0? + 42;
    Ok(SuperClassMe(MyStruct(my_usize), "my_super_class"))
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn conversion_extendr_error_throw;
    fn conversion_manual_error_handler_short;
    fn conversion_manual_error_handler_shorter;
    fn usize_add_42_blame;
    fn usize_add_42_context_blame;
    fn usize_add_42_blame_macro;
    fn usize_add_42_blame_macro_exotic_error;
    fn usize_add_42_superclass;
    impl MyStruct;

}
