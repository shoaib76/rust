#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
   fn printing(&self){
         println!("{:?}",self)

   }
   fn welcome(user: &User) ->User{
     let s = String::from(&user.username) ;
      //let s = user.username;
      User {
      username: s,
      email: "String".to_string(),
      sign_in_count: 44,
      active: true,
  }
  
   }

}

fn main() {

        let user2 = User {
         username: "shoaib muneer".to_string(),
         email: "shoaib@shoaib.com".to_string(),
         sign_in_count: 3333,
         active: true
        };
        let mut user1 = User {
         email: String::from("someone@example.com"),
         username: String::from("someusername123"),
         active: true,
         sign_in_count: 1,
        };
           println!("");
       // user2.printing();
        User::welcome(&user1).printing();
          

        
           // println!("{:#?}",user1);
       // user1.email = String::from("anotheremail@example.com");
           // println!("{:#?}",user1);

        let mut array = [33,44,55,66];
        let mut tuple = (44,"ff",44);
        let mut primi = "shoaib";
        let mut primi1 = "muneer";
                primi = primi1;
           // println!("{} {}",primi,primi1);    
        let mut s1 = String::from("shoaib");
        let mut s2 = s1;
           // println!("{} ",s2);
        let mut x = String::from("100");
           // println!("{}",x);
            refre(&mut x);      
           // println!("{}",x)  ;  
        {
        let x1 = 123;    
        } 
        let x4 = [22,33,44,55] ;
        
        for num in (0..3){
            println!("{}",x4[num]);
        }

}


fn refre(x: &mut String){
            let mata = x.to_string();
            println!("{}    {}",x,mata);
}