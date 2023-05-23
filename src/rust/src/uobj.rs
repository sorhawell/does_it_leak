use extendr_api::prelude::*;

use crate::conversion as convert;

// simple wrapper and all from-into conversions
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

impl Uobj {
    pub fn blame(err: Box<dyn std::fmt::Display>, arg: &str) -> String {
        format!("when converting [{}] {}", arg, err.to_string())
    }
}

// impl<T, E> From<std::result::Result<T, E>> for Uobj
// where
//     T: Into<Robj>,
//     E: Into<Robj>,
// {
//     fn from(res: std::result::Result<T, E>) -> Self {
//         match res {
//             Ok(x) => List::from_names_and_values(&["ok", "err"], &[x.into(), NULL.into()]),
//             Err(x) => {
//                 let err_robj = x.into();
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
