use extendr_api::prelude::*;

//IS1
/// @export
#[extendr]
fn implicit_string(x: String) -> String {
    x.len().to_string()
}

//IS2
/// @export
#[extendr]
fn implicit_strings(x: Strings) -> String {
    x.len().to_string()
}

//ID1
/// @export
#[extendr]
fn implicit_double(x: f64) -> String {
    x.to_string()
}

//ID2
/// @export
#[extendr]
fn implicit_doubles(x: Doubles) -> String {
    x.len().to_string()
}

//IS1_t
/// @export
#[extendr(use_try_from = true)]
fn try_implicit_string(x: String) -> String {
    x.len().to_string()
}

//IS2_t
/// @export
#[extendr(use_try_from = true)]
fn try_implicit_strings(x: Strings) -> String {
    x.len().to_string()
}

//ID1_t
/// @export
#[extendr(use_try_from = true)]
fn try_implicit_double(x: f64) -> String {
    x.to_string()
}

//ID2_t
/// @export
#[extendr(use_try_from = true)]
fn try_implicit_doubles(x: Doubles) -> String {
    x.len().to_string()
}

//US1
/// @export
#[extendr]
fn unwrap_string(x: Robj) -> String {
    let x: String = x.try_into().map_err(|_| "ERROR").unwrap();
    x
}

//US2
/// @export
#[extendr]
fn unwrap_strings(x: Robj) -> String {
    let x = x.as_string_vector().ok_or("ERROR").unwrap();
    x.len().to_string()
}

//UD1
/// @export
#[extendr]
fn unwrap_double(x: Robj) -> String {
    let x: f64 = x.try_into().map_err(|_| "ERROR").unwrap();
    x.to_string()
}

//UD2
/// @export
#[extendr]
fn unwrap_doubles(x: Robj) -> String {
    x.as_real_vector().ok_or("ERROR").unwrap().len().to_string()
}

//ES1
/// @export
#[extendr]
fn error_string(x: Robj) -> String {
    let x: String = x.try_into().unwrap_or_else(|_| "ERROR".to_string());
    x
}

//ES2
/// @export
#[extendr]
fn error_strings(x: Robj) -> String {
    x.as_string_vector()
        .map(|x| x.len().to_string())
        .unwrap_or_else(|| "ERROR".to_string())
}

//ED1
/// @export
#[extendr]
fn error_double(x: Robj) -> String {
    x.try_into()
        .map(|x: f64| x.to_string())
        .unwrap_or_else(|_| "ERROR".to_string())
}

//ED2
/// @export
#[extendr]
fn error_doubles(x: Robj) -> String {
    x.as_real_vector()
        .map(|x| x.len().to_string())
        .unwrap_or_else(|| "ERROR".to_string())
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn implicit_string;
    fn implicit_strings;
    fn implicit_double;
    fn implicit_doubles;
    fn try_implicit_string;
    fn try_implicit_strings;
    fn try_implicit_double;
    fn try_implicit_doubles;
    fn unwrap_string;
    fn unwrap_strings;
    fn unwrap_double;
    fn unwrap_doubles;
    fn error_string;
    fn error_strings;
    fn error_double;
    fn error_doubles;
}
