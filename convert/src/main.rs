fn main() {
    let age = 66;
    let numtostr = age.to_string();
        println!("{:?}{:?}",age,numtostr);
        age1(age);

}

fn age1(age: u8){
    if age == 66{
        println!("{:?}ok",age);
    }

}