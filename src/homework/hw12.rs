/// Порахувати мінімальну кількість перенесень, щоб усі кораблі мали однакову кількість грузу.
/// Кожен переніс — це зміна вантажу між сусідніми кораблями.
/// Повертає `usize` — кількість операцій.
pub fn count_permutation(shipments: &Vec<u32>) -> usize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();
    
    // Якщо вантаж не можна розділити порівну
    if total as usize % n != 0 {
        panic!("Вантаж не можна рівномірно розподілити");
    }

    let average = total / n as u32;
    let mut moves = 0;
    let mut balance = 0;

    for &load in shipments {
        balance += (load as i32 - average as i32);
        moves += balance.abs() as usize;
    }

    moves
}

/// Перевірка: чи можливо розподілити груз рівномірно
pub fn can_distribute_equally(shipments: &Vec<u32>) -> bool {
    let total: u32 = shipments.iter().sum();
    total as usize % shipments.len() == 0
}

/// Альтернативна сигнатура функції, якщо треба враховувати випадки, коли розподіл неможливий:
pub fn count_permutation_safe(shipments: &Vec<u32>) -> Option<usize> {
    if !can_distribute_equally(shipments) {
        return None;
    }
    Some(count_permutation(shipments))
}

/// Генерація вектору, який можна рівномірно розподілити
use rand::Rng;

pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(1..10);
    let mut shipments = vec![avg; n];

    // Вносимо випадкові зміни, зберігаючи загальну суму
    for _ in 0..(n / 2) {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        if i != j && shipments[i] > 1 {
            let delta = rng.gen_range(1..=shipments[i]);
            shipments[i] -= delta;
            shipments[j] += delta;
        }
    }

    shipments
}

/// Приклади з поясненням
pub fn example_cases() {
    let example1 = vec![8, 2, 2, 4, 4];
    println!("Example 1: {:?}, moves: {}", example1, count_permutation(&example1));

    let example2 = vec![9, 3, 7, 2, 9];
    println!("Example 2: {:?}, moves: {}", example2, count_permutation(&example2));
}
