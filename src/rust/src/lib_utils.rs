use extendr_api::prelude::*;
use rust_gcatcirc_lib::code;

/// Returns a new [rust_gcatcirc_lib::code::CircCode]
///
/// Establishes all used tuple lengths and stores them into `tuple_length`. It also collects the `alphabet`.
///
/// # Arguments
/// * `code` a set of words
pub(crate) fn new_code_from_vec(code: Vec<String>) -> code::CircCode {
    match code::CircCode::new_from_vec(code) {
        Ok(code) => return code,
        Err(e) => {
            rprintln!("Code is not correct: {}", e);
            R!(stop("Code is not correct")).unwrap();
            return code::CircCode::default()
        },
    }
}
