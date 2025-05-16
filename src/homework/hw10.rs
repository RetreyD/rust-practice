fn is_palindrome(x: u32) -> bool {
    let mut n = x;
    let mut rev = 0;

    while n > 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }

    x == rev
}
