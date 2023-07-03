use crate::utils::error::{ProxyError, ProxyResult};
use regex::Regex;

pub(crate) fn validate_product(name: &str) -> ProxyResult<()> {
    let regex = Regex::new(r"^\w[\w\.\-]*$").unwrap();
    if regex.is_match(name) {
        Ok(())
    } else {
        Err(ProxyError::ProductValidation(name.to_string()))
    }
}
