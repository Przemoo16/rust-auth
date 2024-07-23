use once_cell::sync::Lazy;
use regex::Regex;

// Regex from the specs: https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").expect("Wrong regex pattern")
});

pub fn is_valid_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

#[cfg(test)]
mod tests {
    use super::is_valid_email;

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("test@email.com"));
        assert!(!is_valid_email("test"));
    }
}
