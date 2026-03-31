use regex::{Captures, Regex};
use std::sync::OnceLock;

static EMAIL_RE: OnceLock<Regex> = OnceLock::new();
static IP_RE: OnceLock<Regex> = OnceLock::new();
static AWS_RE: OnceLock<Regex> = OnceLock::new();
static JWT_RE: OnceLock<Regex> = OnceLock::new();
static STRIPE_RE: OnceLock<Regex> = OnceLock::new();
static CC_RE: OnceLock<Regex> = OnceLock::new();

/// Engine responsible for detecting and masking Personally Identifiable Information and Secrets.
pub struct PiiMasker;

impl PiiMasker {
    /// Validates a potential credit card number using the Luhn Algorithm.
    fn is_valid_luhn(cc: &str) -> bool {
        let digits: Vec<u32> = cc.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() < 13 || digits.len() > 19 {
            return false;
        }

        let mut sum = 0;
        let mut alternate = false;

        for &digit in digits.iter().rev() {
            let mut n = digit;
            if alternate {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }
            sum += n;
            alternate = !alternate;
        }
        sum % 10 == 0
    }

    /// Scans the provided text and redacts sensitive information.
    pub fn mask(text: &str) -> String {
        let email_re = EMAIL_RE
            .get_or_init(|| Regex::new(r"(?i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b").unwrap());
        let ip_re = IP_RE.get_or_init(|| Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap());
        let aws_re = AWS_RE.get_or_init(|| Regex::new(r"\bAKIA[0-9A-Z]{16}\b").unwrap());
        let jwt_re = JWT_RE.get_or_init(|| {
            Regex::new(r"\beyJ[A-Za-z0-9-_=]+\.[A-Za-z0-9-_=]+\.?[A-Za-z0-9-_.+/=]*\b").unwrap()
        });
        let stripe_re = STRIPE_RE
            .get_or_init(|| Regex::new(r"\b(sk_live_|pk_live_)[0-9a-zA-Z]{24,99}\b").unwrap());
        let cc_re = CC_RE.get_or_init(|| Regex::new(r"\b(?:\d[ -]*?){13,19}\b").unwrap());

        let mut sanitized = email_re.replace_all(text, "[EMAIL REDACTED]").to_string();
        sanitized = ip_re.replace_all(&sanitized, "[IP REDACTED]").to_string();
        sanitized = aws_re
            .replace_all(&sanitized, "[AWS KEY REDACTED]")
            .to_string();
        sanitized = jwt_re.replace_all(&sanitized, "[JWT REDACTED]").to_string();
        sanitized = stripe_re
            .replace_all(&sanitized, "[STRIPE TOKEN REDACTED]")
            .to_string();
        sanitized = cc_re
            .replace_all(&sanitized, |caps: &Captures| {
                let matched = &caps[0];
                let clean_num: String = matched.chars().filter(|c| c.is_ascii_digit()).collect();
                if Self::is_valid_luhn(&clean_num) {
                    "[CREDIT CARD REDACTED]".to_string()
                } else {
                    matched.to_string()
                }
            })
            .to_string();

        sanitized
    }
}
