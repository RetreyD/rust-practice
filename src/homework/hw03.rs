const WIDTH: usize = 21;
const HEIGHT: usize = 11;

fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ch = if y == 0 || y == HEIGHT - 1 {
                // верхня та нижня межа
                '*'
            } else if x == 0 || x == WIDTH - 1 {
                // бокові межі
                '*'
            } else if x == y || x == WIDTH - 1 - y {
                // діагоналі конверта
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
