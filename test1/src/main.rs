fn test(number: &u8) -> u8 {
      number+1
}

fn main() {
    let mut number = 0;
    for i in 1..4 {
        println!("{} before",number);
        number =  test(&number);
        println!("{}",number);
    }
}
