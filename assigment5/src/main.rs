
mod moduleA {
    pub fn function1(value: f32) -> f32 {
        super:: moduleB::function2(value*9.0)
    }
}
mod moduleB {
   pub fn function2(value: f32) -> f32 {
        super::moduleC::function3(value/5.0)
    
    }
}

mod moduleC {
   pub fn function3(value: f32) -> f32 {
        value+32.0
    }
}

mod moduleD {
   pub fn function4(value: f32) -> f32 {
        super::moduleE::function5(value-32.0)
    
    }
}

mod moduleE {
    pub fn function5(value: f32) -> f32 {
        super::moduleF::function6(value*5.0)
    
    }
}
mod moduleF {
    pub fn function6(name: f32) -> f32 {
        name/9.0
    }
}

fn main() {
  let value = moduleA::function1(4.0);  
    println!("{}",value);
    println!("{}",moduleD::function4(value));        
}
