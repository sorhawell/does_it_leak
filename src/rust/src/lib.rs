use extendr_api::prelude::*;

#[extendr]
fn implicit_string(x: String) -> String {
    x.len().to_string()
}

extendr_module! {
    mod helloextendr;
    fn implicit_string;

}
