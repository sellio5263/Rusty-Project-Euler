pub fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}
