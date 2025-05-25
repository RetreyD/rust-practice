fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;

    permute(&mut digits.map(|x| x as u32).to_vec(), 0, &mut |perm| {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        if muxa * a == slon {
            println!("  {}{}{}{}", m, u, x, a);
            println!("x       {}", a);
            println!("  ------");
            println!("    {}{}{}{}", s, l, o, n);
            println!();
            count += 1;
        }
    });

    println!("Знайдено рішень: {}", count);
}

fn permute<F>(arr: &mut Vec<u32>, start: usize, f: &mut F)
where
    F: FnMut(&Vec<u32>),
{
    if start == arr.len() {
        f(arr);
        return;
    }

    for i in start..arr.len() {
        arr.swap(start, i);
        permute(arr, start + 1, f);
        arr.swap(start, i);
    }
}
