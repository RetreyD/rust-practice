fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let a = 56;
    let b = 98;

    println!("GCD of {} and {} is {}", a, b, gcd(a, b));
}
