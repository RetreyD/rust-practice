fn main() {
    let height: usize = 11;
    let mid = height / 2;

    for y in 0..height {
        let dx = if y <= mid {
            mid - y
        } else {
            y - mid
        };
        let stars = height - 2 * dx;
        let spaces = dx;

        // Друкуємо пробіли
        for _ in 0..spaces {
            print!(" ");
        }

        // Друкуємо зірочки
        for _ in 0..stars {
            print!("*");
        }

        println!();
    }
}
