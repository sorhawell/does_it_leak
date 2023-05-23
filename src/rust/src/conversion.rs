const R_MAX_INTEGERISH: f64 = 4503599627370496.0;
const R_MIN_INTEGERISH: f64 = -4503599627370496.0;
const USIZE_MAX_INTO_F64: f64 = usize::MAX as f64;
const U32_MAX_INTO_F64: f64 = u32::MAX as f64;
pub const BIT64_NA_ECODING: i64 = -9223372036854775808i64;

const MSG_INTEGERISH_MAX: &'static str =
    "exceeds double->integer unambigious conversion bound of 2^52 = 4503599627370496.0";
const MSG_INTEGERISH_MIN: &'static str =
    "exceeds double->integer unambigious conversion bound of -(2^52)= -4503599627370496.0";
const MSG_NAN: &'static str = "the value cannot be NaN";
const MSG_NO_LESS_ONE: &'static str = "cannot be less than one";

pub fn try_f64_into_usize_no_zero(x: f64) -> std::result::Result<usize, String> {
    match x {
        _ if x.is_nan() => Err(MSG_NAN.to_string()),
        _ if x < 1.0 => Err(format!("the value {} {}", x, MSG_NO_LESS_ONE)),
        _ if x > R_MAX_INTEGERISH => Err(format!("the value {} {}", x, MSG_INTEGERISH_MAX)),
        _ if x > USIZE_MAX_INTO_F64 => Err(format!(
            "the value {} cannot exceed usize::MAX {}",
            x,
            usize::MAX
        )),
        _ => Ok(x as usize),
    }
}

pub fn try_f64_into_usize(x: f64) -> std::result::Result<usize, String> {
    match x {
        _ if x.is_nan() => Err(MSG_NAN.to_string()),
        _ if x < 0.0 => Err(format!("the value {} cannot be less than zero", x)),
        _ if x > R_MAX_INTEGERISH => Err(format!("the value {} {}", x, MSG_INTEGERISH_MAX)),
        _ if x > USIZE_MAX_INTO_F64 => Err(format!(
            "the value {} cannot exceed usize::MAX {}",
            x,
            usize::MAX
        )),
        _ => Ok(x as usize),
    }
}

pub fn try_f64_into_u64(x: f64) -> std::result::Result<u64, String> {
    match x {
        _ if x.is_nan() => Err(MSG_NAN.to_string()),
        _ if x < 0.0 => Err(format!("the value {} cannot be less than zero", x)),
        _ if x > R_MAX_INTEGERISH => Err(format!("the value {} {}", x, MSG_INTEGERISH_MAX)),
        _ => Ok(x as u64),
    }
}

pub fn try_f64_into_i64(x: f64) -> std::result::Result<i64, String> {
    match x {
        _ if x.is_nan() => Err(MSG_NAN.to_string()),
        _ if x < R_MIN_INTEGERISH => Err(format!("the value {} {}", x, MSG_INTEGERISH_MIN)),
        _ if x > R_MAX_INTEGERISH => Err(format!("the value {} {}", x, MSG_INTEGERISH_MAX)),
        _ => Ok(x as i64),
    }
}

pub fn try_f64_into_u32(x: f64) -> std::result::Result<u32, String> {
    match x {
        _ if x.is_nan() => Err(MSG_NAN.to_string()),
        _ if x < 0.0 => Err(format!("the value {} cannot be less than zero", x)),
        _ if x > U32_MAX_INTO_F64 => Err(format!(
            "the value {} cannot exceed u32::MAX {}",
            x,
            u32::MAX
        )),
        _ => Ok(x as u32),
    }
}

pub fn try_i64_into_u64(x: i64) -> std::result::Result<u64, String> {
    match x {
        _ if x < 0 => Err(format!("the value {} cannot be less than zero", x)),
        _ => Ok(x as u64),
    }
}

pub fn try_i64_into_usize(x: i64) -> std::result::Result<usize, String> {
    match x {
        _ if x < 0 => Err(format!("the value {} cannot be less than zero", x)),
        _ => Ok(x as usize),
    }
}

pub fn try_i64_into_u32(x: i64) -> std::result::Result<u32, String> {
    match x {
        _ if x < 0 => Err(format!("the value {} cannot be less than zero", x)),
        _ if x > u32::MAX as i64 => Err("exceeds u32 max value".to_string()),
        _ => Ok(x as u32),
    }
}

pub fn try_i64_into_u8(x: i64) -> std::result::Result<u8, String> {
    match x {
        _ if x < 0 => Err(format!("the value {} cannot be less than zero", x)),
        _ if x > u8::MAX as i64 => Err("exceeds u8 max value".to_string()),
        _ => Ok(x as u8),
    }
}

pub fn robj_to_char(robj: extendr_api::Robj) -> std::result::Result<char, String> {
    let robj = unpack_r_result_list(robj)?;
    let mut fchar_iter = if let Some(char_str) = robj.as_str() {
        char_str.chars()
    } else {
        "".chars()
    };
    match (fchar_iter.next(), fchar_iter.next()) {
        (Some(x), None) => Ok(x),
        (_, _) => Err(format!("is not a single char string, but {:?}", robj)),
    }
}

pub fn robj_to_string(robj: extendr_api::Robj) -> std::result::Result<String, String> {
    let robj = unpack_r_result_list(robj)?;
    use extendr_api::Length;
    match (robj.as_str(), robj.len()) {
        (Some(x), 1) => Ok(x.to_string()),
        (_, _) => Err(format!("is not a single string, but {:?}", robj)),
    }
}

pub fn robj_to_str<'a>(robj: extendr_api::Robj) -> std::result::Result<&'a str, String> {
    let robj = unpack_r_result_list(robj)?;
    use extendr_api::Length;
    match (robj.as_str(), robj.len()) {
        (Some(x), 1) => Ok(x),
        (_, _) => Err(format!("is not a single string, but {:?}", robj)),
    }
}

pub fn robj_to_usize(robj: extendr_api::Robj) -> std::result::Result<usize, String> {
    let robj = unpack_r_result_list(robj)?;
    use extendr_api::*;
    if robj.rtype() == Rtype::Strings && robj.len() == 1 {
        let us = robj
            .as_str()
            .unwrap_or("empty string")
            .parse::<usize>()
            .map_err(|err| format!("failed parsing {:?} to usize", err));
        return us;
    }

    match (robj.rtype(), robj.len()) {
        (Rtype::Strings, 1) => {
            let us = robj
                .as_str()
                .unwrap_or("empty string")
                .parse::<usize>()
                .map_err(|err| format!("failed parsing {:?} to usize", err));
            return us;
        }
        (Rtype::Doubles, 1) if robj.inherits("integer64") => {
            let usize_result = robj_to_i64(robj).and_then(try_i64_into_usize);
            return usize_result;
        }
        (Rtype::Doubles, 1) => robj.as_real(),
        (Rtype::Integers, 1) => robj.as_integer().map(|i| i as f64),
        (_, _) => None,
    }
    .ok_or_else(|| {
        format!(
            "is not a scalar integer or double as required, but {:?}",
            robj
        )
    })
    .and_then(|float| try_f64_into_usize(float))
}

pub fn robj_to_i64(robj: extendr_api::Robj) -> std::result::Result<i64, String> {
    let robj = unpack_r_result_list(robj)?;
    use extendr_api::*;
    match (robj.rtype(), robj.len()) {
        (Rtype::Strings, 1) => {
            let us = robj
                .as_str()
                .unwrap_or("empty string")
                .parse::<i64>()
                .map_err(|err| format!("failed parsing {:?} to usize", err));
            return us;
        }
        //specialized integer64 conversion
        (Rtype::Doubles, 1) if robj.inherits("integer64") => {
            let res = robj
                .as_real()
                .ok_or_else(|| format!("integer64 conversion failed for, but {:?}", robj))
                .and_then(|x| {
                    let x = unsafe { std::mem::transmute::<f64, i64>(x) };
                    if x == BIT64_NA_ECODING {
                        Err("scalar arguments do not support integer64 NA value".to_string())
                    } else {
                        Ok(x)
                    }
                });

            return res;
        }
        //from R doubles or integers
        (Rtype::Doubles, 1) => robj.as_real(),
        (Rtype::Integers, 1) => robj.as_integer().map(|i| i as f64),
        (_, _) => None,
    }
    .ok_or_else(|| format!("not a scalar integer or double as required, but {:?}", robj))
    .and_then(|float| try_f64_into_i64(float))
}

pub fn robj_to_u64(robj: extendr_api::Robj) -> std::result::Result<u64, String> {
    let robj = unpack_r_result_list(robj)?;
    use extendr_api::*;
    match (robj.rtype(), robj.len()) {
        (Rtype::Strings, 1) => return robj_to_usize(robj).map(|x| x as u64),
        //specialized integer64 conversion
        (Rtype::Doubles, 1) if robj.inherits("integer64") => {
            let usize_result = robj_to_i64(robj).and_then(try_i64_into_u64);
            return usize_result;
        }
        (Rtype::Doubles, 1) => robj.as_real(),
        (Rtype::Integers, 1) => robj.as_integer().map(|i| i as f64),
        (_, _) => None,
    }
    .ok_or_else(|| {
        format!(
            "is not a scalar integer or double as required, but {:?}",
            robj
        )
    })
    .and_then(|float| try_f64_into_u64(float))
}

pub fn robj_to_u32(robj: extendr_api::Robj) -> std::result::Result<u32, String> {
    let robj = unpack_r_result_list(robj)?;
    use extendr_api::*;
    match (robj.rtype(), robj.len()) {
        (Rtype::Strings, 1) => return robj_to_i64(robj).and_then(try_i64_into_u32),
        (Rtype::Doubles, 1) if robj.inherits("integer64") => {
            let usize_result = robj_to_i64(robj).and_then(try_i64_into_u32);
            return usize_result;
        }
        (Rtype::Doubles, 1) => robj.as_real(),
        (Rtype::Integers, 1) => robj.as_integer().map(|i| i as f64),
        (_, _) => None,
    }
    .ok_or_else(|| {
        format!(
            "is not a scalar integer or double as required, but {:?}",
            robj
        )
    })
    .and_then(|float| try_f64_into_u32(float))
}

pub fn robj_to_u8(robj: extendr_api::Robj) -> std::result::Result<u8, String> {
    let robj = unpack_r_result_list(robj)?;
    robj_to_i64(robj.clone()).and_then(try_i64_into_u8)
}

pub fn robj_to_bool(robj: extendr_api::Robj) -> std::result::Result<bool, String> {
    let robj = unpack_r_result_list(robj)?;
    use extendr_api::*;
    match (robj.rtype(), robj.len()) {
        (Rtype::Logicals, 1) => robj.as_bool(),
        (_, _) => None,
    }
    .ok_or_else(|| format!("is not a single bool as required, but {:?}", robj))
}

pub fn robj_to_binary_vec(robj: extendr_api::Robj) -> std::result::Result<Vec<u8>, String> {
    let robj = unpack_r_result_list(robj)?;
    let binary_vec: Vec<u8> = robj
        .as_raw_slice()
        .ok_or_else(|| format!("is not an R raw as required, but {:?}", robj))?
        .iter()
        .map(|byte| *byte)
        .collect();
    Ok(binary_vec)
}

// used in r-polars to intecept incoming Robj encoded errors
// very useful to not throw error imediately, but wait for context
pub fn unpack_r_result_list(
    robj: extendr_api::Robj,
) -> std::result::Result<extendr_api::Robj, String> {
    use extendr_api::*;
    if robj.inherits("extendr_result") {
        let l = robj.as_list().unwrap();
        let ok = l.elt(0).unwrap();
        let err = l.elt(1).unwrap();
        match (ok.rtype(), err.rtype()) {
            (Rtype::Null, Rtype::Null) => Ok(ok),
            (Rtype::Null, _) => {
                if let Some(err_msg) = err.as_str() {
                    Err(err_msg.to_string())
                } else {
                    Err(format!("{:?}", err))
                }
            }
            (_, Rtype::Null) => Ok(ok),
            (_, _) => unreachable!("Internal error: failed to unpack r_result_list"),
        }
    } else {
        Ok(robj)
    }
}
