const WIDTH: usize = 30;
const HEIGHT: usize = 15;

fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ch = if y == 0 || y == HEIGHT - 1 {
                // Верхня та нижня межа
                '*'
            } else if x == 0 || x == WIDTH - 1 {
                // Бокові межі
                '*'
            } else if x == y * (WIDTH - 1) / (HEIGHT - 1) {
                // Діагональ з верхнього лівого до нижнього правого
                '*'
            } else if x == (HEIGHT - 1 - y) * (WIDTH - 1) / (HEIGHT - 1) {
                // Діагональ з верхнього правого до нижнього лівого
                '*'
            } else {
                ' '
            };
            output.push(ch);
        }
        output.push('\n');
    }

    print!("{}", output);
}
