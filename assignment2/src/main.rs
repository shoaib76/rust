fn main() {
    let int: u8 = 4;
    let flo: f32 = 2.1;
    let mut name = String::from("shoaib");
        println!("{}",int);
        println!("{}",flo);
        println!("{}",name);
    let square = fan(int,flo,&mut name);
        println!("{}",name);
        println!("{}",square);
}

fn fan(int1: u8,fl: f32,name1: &mut String) -> f32 {
    name1.push_str(" muneer");
    for i in 0..int1{
        println!("{}",i);
    };
    fl * fl


}