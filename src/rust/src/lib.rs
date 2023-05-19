#![feature(prelude_import)]
#![feature(rustc_attrs)]
use extendr_api::prelude::*;

pub fn safe_handle_panic<F, R>(err_str: &str, f: F) -> std::result::Result<R, String>
where
    F: FnOnce() -> R,
    F: std::panic::UnwindSafe,
{
    std::panic::catch_unwind(f).map_err(|err| format!("any is {:?}", err))
}

//uses implicit conversion
//#[extendr]
fn string_to_string(x: String) -> String {
    x.len().to_string()
}

#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use extendr_api::prelude::*;

#[no_mangle]
#[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn wrap__string_to_string(x: extendr_api::SEXP) -> extendr_api::SEXP {
    let z = unsafe {
        use extendr_api::robj::*;
        let _x_robj = extendr_api::new_owned(x);

        //safe_handle_panics is only a last guard against user induced panics!.
        let result = safe_handle_panic(
            "sry your user function panicked, not extendr.\0",
            || -> std::result::Result<Robj, Robj> {
                //convert user param1
                let param1_result = <String>::from_robj(&_x_robj).map_err(|err| err.into_robj());

                // run user func, but bubble all params first
                let user_f_out = string_to_string(param1_result?);

                //user_f_out could be a result ... vanilla extendr will cause a panic on Err then
                let user_out_after_conversion = extendr_api::Robj::from(user_f_out);

                //hurray user function ran without errors, this is the final value
                Ok(user_out_after_conversion)
            },
        );
        result
    };
    //drop(x);
    let zz = extendr_api::handle_panic("stuff happended", || {
        z.expect("user error/conv").expect("user panic")
    });
    unsafe { zz.get() }
}

#[allow(non_snake_case)]
fn meta__string_to_string(metadata: &mut Vec<extendr_api::metadata::Func>) {
    let mut args = <[_]>::into_vec(Box::new([extendr_api::metadata::Arg {
        name: "x",
        arg_type: "String",
        default: None,
    }]));
    metadata.push(extendr_api::metadata::Func {
        doc: "",
        rust_name: "string_to_string",
        r_name: "string_to_string",
        mod_name: "string_to_string",
        args: args,
        return_type: "String",
        func_ptr: wrap__string_to_string as *const u8,
        hidden: false,
    })
}

extendr_module! {
    mod helloextendr;
    fn string_to_string;

}
