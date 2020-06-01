#[derive(Debug)]

enum Move {
    Run,
    Walk,
    Sit,
}
fn inc1(num: Option<i32>,mut t: u8) -> Option<i32> {
    match num {
        None => {println!("none");t = 5;
                None},
        Some(mut r) => {println!("ffff{}hhhhh{}",r,t);
                    if r == 0{
                        println!("zero")
                    }
                    r=7;
                    Some(r) }
        
            
        
    
    
    }
}

fn main() {

    let f = Some(6);
    let h: Option<i32> = None;
    println!("{:?}",inc1(f,7));
    println!("{:?}ddd",inc1(h, 7));
}
