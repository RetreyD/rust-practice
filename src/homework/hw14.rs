fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }
    
    let prev = gray(n - 1);
    let mut result = Vec::with_capacity(2usize.pow(n as u32));
    
    // Додаємо 0 спереду для попереднього коду
    for code in &prev {
        let mut s = String::from("0");
        s.push_str(code);
        result.push(s);
    }
    // Додаємо 1 спереду для перевернутого попереднього коду
    for code in prev.iter().rev() {
        let mut s = String::from("1");
        s.push_str(code);
        result.push(s);
    }
    
    result
}

#[test]
fn test() {
   let test_data =
       [
           (0, vec![""]),
           (1, vec!["0", "1"]),
           (2, vec!["00", "01", "11", "10"]),
           (3, vec!["000", "001", "011", "010", "110", "111", "101", "100"]),
           (4, vec![
               "0000", "0001", "0011", "0010",
               "0110", "0111", "0101", "0100",
               "1100", "1101", "1111", "1110",
               "1010", "1011", "1001", "1000"
           ]),
       ];

   for (n, out) in test_data {
       assert_eq!(gray(n), out);
   }
}
