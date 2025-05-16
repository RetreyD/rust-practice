fn draw_tree(levels: usize) {
    let mut output = String::new();
    let max_width = levels * 2 + 1;

    (0..levels).for_each(|level| {
        let height = level + 2; // кожен рівень має більше рядків
        (0..height).for_each(|line| {
            let stars = 1 + 2 * line;
            let padding = (max_width - stars) / 2;
            let row = " ".repeat(padding) + &"*".repeat(stars) + "\n";
            output.push_str(&row);
        });
    });

    // Стовбур
    let trunk_width = if levels % 2 == 0 { 2 } else { 1 };
    let trunk_height = levels;
    let padding = (max_width - trunk_width) / 2;
    (0..trunk_height).for_each(|_| {
        let row = " ".repeat(padding) + &"*".repeat(trunk_width) + "\n";
        output.push_str(&row);
    });

    print!("{}", output);
}

fn main() {
    let levels = 4; // змінюй кількість трикутників тут
    draw_tree(levels);
}
