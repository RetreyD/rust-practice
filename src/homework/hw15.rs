use std::collections::HashSet;

fn solve_cryptarithm() {
    let digits = 1..=8;

    let mut solutions = Vec::new();

    // Перебираємо усі можливі цифри для літер
    for &m in digits.clone().collect::<Vec<_>>().iter() {
    for &u in digits.clone().collect::<Vec<_>>().iter() {
    for &x in digits.clone().collect::<Vec<_>>().iter() {
    for &a in digits.clone().collect::<Vec<_>>().iter() {
    for &s in digits.clone().collect::<Vec<_>>().iter() {
    for &l in digits.clone().collect::<Vec<_>>().iter() {
    for &o in digits.clone().collect::<Vec<_>>().iter() {
    for &n in digits.clone().collect::<Vec<_>>().iter() {

        let letters = [m,u,x,a,s,l,o,n];
        let unique: HashSet<_> = letters.iter().cloned().collect();
        if unique.len() != letters.len() {
            continue; // Цифри мають бути різні
        }

        let num1 = m*1000 + u*100 + x*10 + a;
        let num2 = a;
        let product = s*1000 + l*100 + o*10 + n;

        if num1 * num2 == product {
            solutions.push((m,u,x,a,s,l,o,n));
        }
    }}}}}}}

    println!("Знайдено {} рішень:", solutions.len());
    for (m,u,x,a,s,l,o,n) in solutions {
        println!("  {}{}{}{} * {} = {}{}{}{}", m,u,x,a,a,s,l,o,n);
        println!("  {:4}", m*1000 + u*100 + x*10 + a);
        println!("* {:1}", a);
        println!("-------");
        println!("{:4}", s*1000 + l*100 + o*10 + n);
        println!();
    }
}

fn main() {
    solve_cryptarithm();
}
