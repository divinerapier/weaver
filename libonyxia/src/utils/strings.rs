pub fn trim_suffix<'a>(s: &'a str, sub: &str) -> Option<&'a str> {
    if !s.ends_with(sub) {
        return None;
    }
    return Some(&s[0..s.len() - sub.len()]);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_string_trim() {
        assert_eq!(super::trim_suffix("1234567", "234567"), Some("1"))
    }
}
