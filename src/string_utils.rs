pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn is_palindrome(s: &str) -> bool {
    let rev: String = s.chars().rev().collect();
    rev == s
}
