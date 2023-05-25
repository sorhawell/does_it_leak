pub mod conversion; // set of oppionated conversion functions, derived from Robj-api
pub mod uobj; // zero-cost abstraction, cherry-pick/override extendr traits

use conversion as convert;

use extendr_api::prelude::*;
pub use uobj::*; //bring  Uobj traits into context of macro expansions

#[derive(Clone, Debug)]
struct MyClass {
    pub my_i32: i32,
}

#[extendr(dep_inject = "Uobj")] //zero-cost abstraction, cherry-pick/override extendr traits
impl MyClass {
    pub fn new() -> MyClass {
        MyClass { my_i32: 42 }
    }

    // THIS ONE IS NEW due to (dep_inject =  "Uobj")
    // introduce native usize conversion in extendr, foreign rule is not a problem because user owns Uobj
    pub fn usize_implicit_conversion_implicit_errorhandling(x: usize) -> i32 {
        println!("this usize is {}", x);
        32
    }

    // via generic trait wrapper pattern
    pub fn usize_implicit_conversion_explicit_errorhandling(x: Wrap<Result<usize>>) -> Result<()> {
        println!(
            "this usize is {}",
            x.0.map_err(|err| Uobj::blame(Box::new(err), "x"))? //reusing Uobj blame to point fingers at "x" if failed conversion
        ); // must explicitly handle errors
        Ok(())
    }

    // via wrapper struct pattern
    pub fn usize_explicit_conversion_explicit_errorhandling(x: Uobj) -> Result<()> {
        let x: usize = x
            .try_into()
            .map_err(|err| Uobj::blame(Box::new(err), "x"))?;
        println!("this usize is {}", x);
        Ok(())
    }

    //manually Robj not involving Uobj at all
    pub fn usize_manually(x: Robj) -> Result<()> {
        let x: Result<usize> = convert::robj_to_usize(x).map_err(|err| {
            extendr_api::error::Error::Other(format!(
                "when converting [x] {}",
                extendr_api::error::Error::Other(err),
            ))
        });
        println!("this usize is {}", x?);
        Ok(())
    }
}

extendr_module! {
    mod helloextendr;
    impl MyClass;
}
