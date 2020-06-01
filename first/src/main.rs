
use std::io;


#[derive(Debug)]
      struct Book {
        name: String,
        aut : String,
        price: u16,
        ave : bool }
#[derive(Debug)]
      struct Rec {w:u16,h:u16}

        impl Rec {
  fn abc(size : u16) {
    println!("this is abc {}",size);
  } 
  fn abc2(u : u16){
    println!("this is abc2 {}",u);
  }
}

   

fn main() {
   // let mut name1 = String::from("7");
   let book5 = Rec::abc(65);
   book5;
   book5;
   //println!("this call after fun {:?}",book5);
   //book5.abc2(7);
   //let  name1 = 65;
   //let  name2 = name1; 
    //println!("Guess the number!{} {}",name2,name1);
    //let mut temp = 100;
   // println!("Please input your guess.");
    //name1 = 777777;
     
   //  let mut book1 = Book {
     //  name : String::from("book2"),
    //   aut : String::from("shoaib"),
     //  price : 654,
    //   ave : true
   //  };
   
   //  let book2 = Book {
   //   aut : String::from("book3"),
  //    name : String::from("muneer"),
  //    price : 644,
   //   ave : true
   // };
  // book1.name.push_str(" sssss");
  //main1(book1);
  // println!("{:#?}",book1);
   //println!("{:?}",book2);
    //let mut guess = String::new();

   //io::stdin().read_line(&mut name1)
    //.expect("Failed to read line");
   //let name2: u8 = name1.trim().parse().expect("msg: str");
  // println!("You guessed: {:p} {:p}",&name1, &name2);
  // let mut s1 = String::from("shoaib");
  // let a = &s1;
  // let b = &s1;
   //println!("{}",a);
  // {let mut d = &mut s1;
    //main1(&mut d);
    //d.push_str("string: &str");
    //main1(&mut d);
    //println!("{}",d); 
  //}
  //{ let mut c = &mut s1;
    //main1(&mut c);
    //c.push_str("string: &str");
  // println!("{}",c) 
   //}
   //println!("{}",d);
let  int = "ddd";
  main1(int);
  println!("{:p}",&int);
  let int1 = 1;
  let int2 = 5;
for i in int1..int2{
  println!("{}",i);}
 let mut int8 = 8; 
let int7 = {for int6 in 1..7 {
   println!("{}",int6)
  //int8 = int6;
 }
 int8
};
{
println!("this is code {}",int7)
} 

//println!("{:?}",int4);

}
fn main1(name: &str){
 // name.name.push_str("sssss22");
 //name = "4";
  println!("{:p}",&name);
}
//use rand::Rng;
//use std::cmp::Ordering;
//use std::io;

//fn main1(name: mut String) {
    // --snip--
    //println!("Guess the number!");
    //let mut bbb = String::from("hghghg");
    //let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);
    //loop {

    
  //  let bbb = 's';
    //println!("Please input your guess.");

    //let mut guess = String::new();

    //io::stdin()
      //  .read_line(&mut guess)
       // .expect("Failed to read line");

    //let guess: u32 = guess.trim().parse().expect("Please type a number!");
    //println!("You guessed: {}", guess);

    //match guess.cmp(&secret_number) {
      //  Ordering::Less => println!("Too small!"),
       // Ordering::Greater => println!("Too big!"),
        //Ordering::Equal => { println!("You win!");
        //break;

        
        //name.push_str("string: &str");
       // println!("{}",name);
        
      //   }
           
      // }

