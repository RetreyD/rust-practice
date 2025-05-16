fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    // Нормалізація зсуву в межах довжини рядка
    let n = ((n % len) + len) % len;

    let (left, right) = s.split_at((len - n) as usize);
    format!("{}{}", right, left)
}
