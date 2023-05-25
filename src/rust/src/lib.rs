pub mod conversion; // some random set of oppionated conversion functions, derived from Robj-api
pub mod uobj; // zero-cost abstraction, cherry-pick/override extendr traits
pub use either::*;

use conversion as convert;

use extendr_api::prelude::*;
pub use uobj::*; //bring  Uobj traits into context of macro expansions

#[derive(Clone, Debug)]
struct MyClass {
    pub my_i32: i32,
}

// COMMENT 1: this uobj pattern does not need PR473 to avoid throwing errors and choose result encodings.
// The user can just  redefine `impl<T, E> TryFrom<std::result::Result<T, E>> for Uobj` in uobj.rs

// COMMENT 2: if the user themselves wanted to implement something like Either<left,right>
// they could just add the trait to uobj.rs

#[extendr(dep_inject = "Uobj")] //COMMENT 4 inject zero-cost abstraction to modify any Robj conversion trait
impl MyClass {
    pub fn new() -> MyClass {
        MyClass { my_i32: 42 }
    }
    // COMMENT 5: THIS ONE IS NEW due to (dep_inject =  "Uobj")
    // introduce native usize conversion in extendr, foreign rule is not a problem because user owns Uobj
    // performs implicit conversion error handling, err msg writing, but all customizable via uobj.rs -traits
    pub fn usize_implicit_conversion_implicit_errorhandling(x: usize) {
        println!("this usize is {}", x);
    }

    // COMMENT 6: A user could implement extendr either functionality by adding 15 lines to their uobj.rs
    pub fn show_case_user_either(x: Either<i32, usize>) {
        println!("this usize or String is {:?}", x);
    }

    // via generic trait wrapper pattern, implicit conversion, but explicit error handling and error msg writing.
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

    // manually Robj not involving Uobj at all
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
