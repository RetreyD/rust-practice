use rand::Rng;

/// Генерує рандомний вектор довжиною `n` зі значеннями [10..=99]
pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

/// Знаходить індекс пари з найменшою сумою у векторі
pub fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    if data.len() < 2 {
        panic!("Має бути хоча б дві позиції");
    }

    let mut min_index = 0;
    let mut min_sum = data[0] + data[1];

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, min_sum)
}

/// Форматований вивід результату з підкресленням найменшої пари
pub fn print_result(data: &[i32]) {
    let (min_index, min_sum) = min_adjacent_sum(data);

    // Вивід індексів
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // Вивід даних
    print!("data:   ");
    for num in data {
        print!("{:>3},", num);
    }
    println!();

    // Вивід стрілки на мінімальну пару
    print!("indexes:");
    for i in 0..data.len() {
        if i == min_index {
            print!("\\__ ");
        } else if i == min_index + 1 {
            print!("__/ ");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_index],
        data[min_index + 1],
        min_sum,
        min_index,
        min_index + 1
    );
}

/// Приклад запуску функцій
pub fn run_example() {
    let data = gen_random_vector(20);
    print_result(&data);
}
