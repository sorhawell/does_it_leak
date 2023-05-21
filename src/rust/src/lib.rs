// use extendr_api::prelude::*;
// use std::result::Result;
// //IS1////////////
// /// @export
// //#[extendr]
// fn implicit_string(x: String) -> String {
//     x.len().to_string()
// }
#![feature(prelude_import)]
#![feature(core_panic)]
#[prelude_import]
//use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use extendr_api::prelude::*;
use std::result::Result;
/// @export
fn implicit_string(x: String) -> String {
    x.len().to_string()
}
#[no_mangle]
#[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn wrap__implicit_string(x: extendr_api::SEXP) -> extendr_api::SEXP {
    let res_res = unsafe {
        use extendr_api::robj::*;
        let _x_robj = extendr_api::new_owned(x);
        std::panic::catch_unwind(|| -> std::result::Result<Robj, extendr_api::Error> {
            Ok(extendr_api::Robj::from(implicit_string(
                <String>::from_robj(&_x_robj)?,
            )))
        })
    };
    match res_res {
        Ok(Ok(zz)) => {
            use extendr_api::robj::*;
            return unsafe { zz.get() };
        }
        Ok(Err(extendr_err)) => {
            let err_string = extendr_err.to_string();
            drop(extendr_err);
            extendr_api::throw_r_error(&err_string);
        }
        Err(unwind_err) => {
            drop(unwind_err);
            let err_string = {
                let res = std::fmt::format(format_args!(
                    "user function panicked: {0}",
                    "implicit_string panicked.\0"
                ));
                res
            };
            extendr_api::handle_panic(err_string.as_str(), || {
                ::core::panicking::panic("explicit panic")
            });
            ::core::panicking::panic_fmt(format_args!(
                "internal error: entered unreachable code: {0}",
                format_args!("internal extendr error, this should never happened")
            ));
        }
        _ => ::core::panicking::panic_fmt(format_args!(
            "internal error: entered unreachable code: {0}",
            format_args!("internal extendr error, this should never happened")
        )),
    }
}
#[allow(non_snake_case)]
fn meta__implicit_string(metadata: &mut Vec<extendr_api::metadata::Func>) {
    let mut args = <[_]>::into_vec(
        //#[rustc_box]
        Box::new([extendr_api::metadata::Arg {
            name: "x",
            arg_type: "String",
            default: None,
        }]),
    );
    metadata.push(extendr_api::metadata::Func {
        doc: " @export",
        rust_name: "implicit_string",
        r_name: "implicit_string",
        mod_name: "implicit_string",
        args: args,
        return_type: "String",
        func_ptr: wrap__implicit_string as *const u8,
        hidden: false,
    })
}
// #[no_mangle]
// #[allow(non_snake_case)]
// pub fn get_helloextendr_metadata() -> extendr_api::metadata::Metadata {
//     let mut functions = Vec::new();
//     let mut impls = Vec::new();
//     meta__implicit_string(&mut functions);
//     functions.push(extendr_api::metadata::Func {
//         doc: "Metadata access function.",
//         rust_name: "get_helloextendr_metadata",
//         mod_name: "get_helloextendr_metadata",
//         r_name: "get_helloextendr_metadata",
//         args: Vec::new(),
//         return_type: "Metadata",
//         func_ptr: wrap__get_helloextendr_metadata as *const u8,
//         hidden: true,
//     });
//     functions.push(extendr_api::metadata::Func {
//         doc: "Wrapper generator.",
//         rust_name: "make_helloextendr_wrappers",
//         mod_name: "make_helloextendr_wrappers",
//         r_name: "make_helloextendr_wrappers",
//         args: <[_]>::into_vec(
//             #[rustc_box]
//             ::alloc::boxed::Box::new([
//                 extendr_api::metadata::Arg {
//                     name: "use_symbols",
//                     arg_type: "bool",
//                     default: None,
//                 },
//                 extendr_api::metadata::Arg {
//                     name: "package_name",
//                     arg_type: "&str",
//                     default: None,
//                 },
//             ]),
//         ),
//         return_type: "String",
//         func_ptr: wrap__make_helloextendr_wrappers as *const u8,
//         hidden: true,
//     });
//     extendr_api::metadata::Metadata {
//         name: "helloextendr",
//         functions,
//         impls,
//     }
// }
// #[no_mangle]
// #[allow(non_snake_case)]
// pub extern "C" fn wrap__get_helloextendr_metadata() -> extendr_api::SEXP {
//     use extendr_api::GetSexp;
//     unsafe { extendr_api::Robj::from(get_helloextendr_metadata()).get() }
// }
// #[no_mangle]
// #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
// pub extern "C" fn wrap__make_helloextendr_wrappers(
//     use_symbols_sexp: extendr_api::SEXP,
//     package_name_sexp: extendr_api::SEXP,
// ) -> extendr_api::SEXP {
//     unsafe {
//         use extendr_api::robj::*;
//         use extendr_api::GetSexp;
//         let robj = new_owned(use_symbols_sexp);
//         let use_symbols: bool = <bool>::from_robj(&robj).unwrap();
//         let robj = new_owned(package_name_sexp);
//         let package_name: &str = <&str>::from_robj(&robj).unwrap();
//         extendr_api::Robj::from(
//             get_helloextendr_metadata()
//                 .make_r_wrappers(use_symbols, package_name)
//                 .unwrap(),
//         )
//         .get()
//     }
// }
// #[no_mangle]
// #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
// pub extern "C" fn R_init_helloextendr_extendr(info: *mut extendr_api::DllInfo) {
//     unsafe { extendr_api::register_call_methods(info, get_helloextendr_metadata()) };
// }

// //IS2//
// /// @export
// #[extendr]
// fn implicit_strings(x: Strings) -> String {
//     x.len().to_string()
// }

// //ID1
// /// @export
// #[extendr]
// fn implicit_double(x: f64) -> String {
//     x.to_string()
// }

// //ID2
// /// @export
// #[extendr]
// fn implicit_doubles(x: Doubles) -> String {
//     x.len().to_string()
// }

// //IS1_t
// /// @export
// #[extendr(use_try_from = true)]
// fn try_implicit_string(x: String) -> String {
//     x.len().to_string()
// }

// //IS2_t
// /// @export
// #[extendr(use_try_from = true)]
// fn try_implicit_strings(x: Strings) -> String {
//     x.len().to_string()
// }

// //ID1_t/
// /// @export
// #[extendr(use_try_from = true)]
// fn try_implicit_double(x: f64) -> String {
//     x.to_string()
// }

// //ID2_t/
// /// @export
// #[extendr(use_try_from = true)]
// fn try_implicit_doubles(x: Doubles) -> String {
//     x.len().to_string()
// }

// //US1
// /// @export
// #[extendr]
// fn unwrap_string(x: Robj) -> String {
//     let x: String = x.try_into().unwrap();
//     x
// }

// //US2
// /// @export
// #[extendr]
// fn unwrap_strings(x: Robj) -> String {
//     let x = x.as_string_vector().ok_or("ERROR").unwrap();
//     x.len().to_string()
// }

// //UD1
// /// @export
// #[extendr]
// fn unwrap_double(x: Robj) -> String {
//     let x: f64 = x.try_into().map_err(|_| "ERROR").unwrap();
//     x.to_string()
// }

// //UD2
// /// @export
// #[extendr]
// fn unwrap_doubles(x: Robj) -> String {
//     x.as_real_vector().ok_or("ERROR").unwrap().len().to_string()
// }

// //ES1
// /// @export
// #[extendr]
// fn error_string(x: Robj) -> String {
//     let x: String = x.try_into().unwrap_or_else(|_| "ERROR".to_string());
//     x
// }

// //ES2
// /// @export
// #[extendr]
// fn error_strings(x: Robj) -> String {
//     x.as_string_vector()
//         .map(|x| x.len().to_string())
//         .unwrap_or_else(|| "ERROR".to_string())
// }

// //ED1
// /// @export
// #[extendr]
// fn error_double(x: Robj) -> String {
//     x.try_into()
//         .map(|x: f64| x.to_string())
//         .unwrap_or_else(|_| "ERROR".to_string())
// }

// //ED2
// /// @export
// #[extendr]
// fn error_doubles(x: Robj) -> String {
//     x.as_real_vector()
//         .map(|x| x.len().to_string())
//         .unwrap_or_else(|| "ERROR".to_string())
// }

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn implicit_string;
    // fn implicit_strings;
    // fn implicit_double;
    // fn implicit_doubles;
    // fn try_implicit_string;
    // fn try_implicit_strings;
    // fn try_implicit_double;
    // fn try_implicit_doubles;
    // fn unwrap_string;
    // fn unwrap_strings;
    // fn unwrap_double;
    // fn unwrap_doubles;
    // fn error_string;
    // fn error_strings;
    // fn error_double;
    // fn error_doubles;
}
