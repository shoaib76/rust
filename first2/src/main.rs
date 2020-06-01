//golbal = 6666;
fn main() {
    let course = "shoaib muneer";
    print!("Hello, world!");
    let course = course.len();
    println!("{}",course);
    let a = ["hhhhhh","777"];
    for i in a.iter() {
        println!("hello  {}",i);
    }
    let x: u8 = 234;
    let y: f32 = 78.1;
    let z: f32 = x as f32 + y as f32;
        println!("{}",z);

    //const ABC: u16  = 666;
    //const ABC2: u16 = 444;
    let  mut b = 66 as u8;
             b = b + 77;
   // b = b + 1;
    let cha: u8 = b'a';
    let mut taple: (u8,&str,char,u8) = (44,"adsd",'r',7);
    let (c,s,ch,b1) = (47 as u16,"6",'A' as u16,67.4 as u8);

     //let mut spaces = "   ";
     //let mut spaces = spaces.len();

    let guess: u8 = "42".parse().expect("Not a number!");

    println!("{}{} {} {}",guess,b,ch,b1);


    let a = [1, 2, 3, 4, 5];
    let index = 4;

    let element = a[index];

    println!("The value of element is: {}", element);

    let condition = false;

    let number = if condition { 5 } else { 6.7 as u8};

    println!("The value of number is: {}", number);


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is {:?}", result);



    let mut s3 = String::from("s:AAAA &String");
   //{let  s4 = &mut s3 ;
     //      println!("{}",s4);
       //    s4.push_str("strtr");
         //  println!("{}",s3)
  // }
    //s3 = "string: &str";

             let ind = first_word(&s3);
    //         println!("{}",ind);
             s3.clear();
            // let ind = first_word(&s3);
             println!("{}",ind);
//fn change(s5: &mut String){
  //  s5.push_str("string: &str");

 // let s = String::from("hello world");

  //let hello = &s[3..];
  //let world = &s[2..];
  //println!("{}    {}",hello,world);

            //let mut s4 = String::from("hello world");
            let mut s4 = "helllll ";

            let word1 = first_word1(&s4);

           //s4.clear(); // error!

  println!("the first word is: {}", word1);




}

fn first_word1(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}




  fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("this i{} and item{} and string {}",i,item,s);
        if item == b' ' {
            return i;
        }
    }

    s.len()














}
