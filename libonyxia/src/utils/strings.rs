use crate::error::{Error, Result};

pub fn trim_suffix<'a>(s: &'a str, sub: &str) -> Result<&'a str> {
    if !s.ends_with(sub) {
        Err(Error::naive(format!("{} is not ending with {}", s, sub)))
    } else {
        Ok(&s[0..s.len() - sub.len()])
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_string_trim() {
        assert_eq!(super::trim_suffix("1234567", "234567").unwrap(), "1")
    }
}
