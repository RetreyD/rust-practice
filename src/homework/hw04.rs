const HEIGHT: usize = 11;
const WIDTH: usize = 21;

fn main() {
    let mut output = String::new();

    let mid = HEIGHT / 2;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let dx = if y <= mid {
                mid - y
            } else {
                y - mid
            };
            if x == dx || x == WIDTH - 1 - dx {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
