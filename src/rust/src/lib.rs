pub mod conversion; // set of oppionated conversion functions, derived from Robj-api
pub mod uobj; // zero-cost abstraction, cherry-pick/override extendr traits

use conversion as convert;

use extendr_api::prelude::*;
pub use uobj::*; //bring  Uobj traits into context of macro expansions

#[derive(Clone, Debug)]
struct MyClass {
    pub s: i32,
}
////
#[extendr(dep_inject = "Uobj")] //zero-cost abstraction, cherry-pick/override extendr traits
impl MyClass {
    pub fn new() -> MyClass {
        MyClass { s: 42 }
    }

    // THIS ONE IS NEW due to (dep_inject =  "Uobj")/
    // extend an extendr conversion to print "Hello Converter World" during conversions
    // pub fn show(s: String) {
    //     println!("{}", s);
    // }

    // THIS ONE IS NEW due to (dep_inject =  "Uobj")
    // introduce native usize conversion in extendr, foreign rule is not a problem because user owns Uobj
    pub fn usize_implicit_conversion_implicit_errorhandling(x: usize) -> i32 {
        println!("this usize is {}", x);
        32
    }

    // via generic trait wrapper pattern
    pub fn usize_implicit_conversion_explicit_errorhandling(x: Wrap<Result<usize>>) -> Result<()> {
        println!("this usize is {}", x.0?); // must explicitly handle errors
        Ok(())
    }

    //     // via wrapper struct pattern
    //     pub fn usize_explicit_conversion_explicit_errorhandling(x: Uobj) -> Result<()> {
    //         let x: usize = x.try_into()?;
    //         println!("this usize is {}", x);
    //         Ok(())
    //     }

    //     //manually Robj not involving Uobj
    //     pub fn usize_manually(s: Robj) -> Result<()> {
    //         let s: Result<usize> =
    //             convert::robj_to_usize(s).map_err(|err| extendr_api::error::Error::Other(err));
    //         println!("this usize is {}", s?);
    //         Ok(())
    //     }
}

extendr_module! {
    mod helloextendr;
    impl MyClass;
}
