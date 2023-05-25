use extendr_api::prelude::*;

use crate::{conversion as convert, MyClass};

// simple wrapper and all from-into conversions
#[derive(Debug)]
pub struct Uobj(pub Robj);
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
impl From<&Uobj> for &Robj {
    fn from(x: &Uobj) -> Self {
        let y: &Robj = unsafe { std::mem::transmute(x) }; // it is a tupple struct
        y
    }
}

// Comment a Uobj currently must implemnet the method blame. It is used to write-up error messages
// which will mention what param had a failed conversion.
impl Uobj {
    pub fn blame(err: Box<dyn std::fmt::Display>, arg: &str) -> String {
        format!("when converting [{}] {}", arg, err.to_string())
    }
}


impl From<i32> for Uobj {
    fn from(x: i32) -> Self {
        Robj::from(x).into()
    }
}


impl From<()> for Uobj {
    fn from(x: ()) -> Self {
        Robj::from(x).into()
    }
}

impl From<MyClass> for Uobj {
    fn from(x: MyClass) -> Self {
        Robj::from(x).into()
    }
}

// The user can define how any result should be return to R
// impl<T, E> From<std::result::Result<T, E>> for Uobj
// where
//     T: Into<Robj>,
//     E: std::fmt::Display,
// {
//     fn from(res: std::result::Result<T, E>) -> Self {
//         match res {
//             Ok(x) => List::from_names_and_values(&["ok", "err"], &[x.into(), NULL.into()]),
//             Err(x) => {
//                 let err_robj: Robj = x.to_string().into();
//                 if err_robj.is_null() {
//                     panic!("Internal error: result_list not allowed to return NULL as err-value")
//                 }
//                 List::from_names_and_values(&["ok", "err"], &[NULL.into(), err_robj])
//             }
//         }
//         //can only imagine this would ever fail due memory allcation error, but then panicking is the right choice
//         .expect("Internal error: failed to create an R list")
//         .set_class(&["extendr_result"])
//         .expect("Internal error: failed to set class")
//         .into()
//     }
// }

//The user can define how any result should be return to R
impl<T, E> TryFrom<std::result::Result<T, E>> for Uobj
where
    T: Into<Uobj>,
    E: Into<Box<dyn std::error::Error>>,
{
    type Error = Box<dyn std::error::Error>;
    fn try_from(
        res: std::result::Result<T, E>,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        match res {
            Ok(ok) => Ok(ok.into()),
            Err(err) => Err(err.into()),
        }
    }
}

// generic wrap conversions, derived from Uobj because it is already there
pub struct Wrap<T>(pub T);
impl<T> From<Uobj> for Wrap<std::result::Result<T, T::Error>>
where
    T: crate::uobj::TryFrom<Uobj>,
{
    fn from(uobj: Uobj) -> Self {
        Wrap(uobj.try_into())
    }
}

// example reuse Robj-trait for conversion
impl TryFrom<&Uobj> for &str {
    type Error = Error;
    fn try_from(uobj: &Uobj) -> extendr_api::Result<Self> {
        Self::try_from(&uobj.0)
    }
}

// example derive conversion from Robj-api
impl TryFrom<Uobj> for String {
    type Error = extendr_api::error::Error;
    fn try_from(x: Uobj) -> std::result::Result<Self, Error> {
        rprintln!("hello converter world!!");
        Ok(x.0.as_str().unwrap().to_string())
    }
}

// example derive a conversion from another Uobj-trait
impl TryFrom<&Uobj> for String {
    type Error = Error;
    fn try_from(uobj: &Uobj) -> Result<Self> {
        <&str>::try_from(uobj).map(|s| s.to_string())
    }
}

// example implement conversion for a rust native type
impl TryFrom<Uobj> for usize {
    type Error = Error;
    fn try_from(uobj: Uobj) -> Result<Self> {
        Ok(convert::robj_to_usize(uobj.0)?) //? to convert error-type.
    }
}

use crate::{Error, Robj};
use either::*;

impl<'a, L, R> TryFrom<&'a Uobj> for Either<L, R>
where
    L: TryFrom<&'a Uobj, Error = Error>,
    R: TryFrom<&'a Uobj, Error = Error>,
{
    type Error = Error;

    /// Returns the first type that matches the provided `Uobj`, starting from
    /// `L`-type, and if that fails, then the `R`-type is converted.
    fn try_from(value: &'a Uobj) -> Result<Self> {
        match L::try_from(value) {
            Ok(left) => Ok(Left(left)),
            Err(left_err) => match R::try_from(value) {
                Ok(right) => Ok(Right(right)),
                Err(right_err) => Err(Error::Other(format!(
                    "either error {:?} and {:?}",
                    right_err, left_err
                ))),
            },
        }
    }
}

impl<L, R> TryFrom<Uobj> for Either<L, R>
where
    for<'a> Either<L, R>: TryFrom<&'a Uobj, Error = Error>,
{
    type Error = Error;

    /// Returns the first type that matches the provided `Uobj`, starting from
    /// `L`-type, and if that fails, then the `R`-type is converted.
    fn try_from(value: Uobj) -> Result<Self> {
        (&value).try_into()
    }
}


impl<L, R> From<Either<L, R>> for Uobj
where
    Uobj: From<L> + From<R>,
{
    fn from(value: Either<L, R>) -> Self {
        match value {
            Left(left) => Uobj::from(left),
            Right(right) => Uobj::from(right),
        }
    }
}
