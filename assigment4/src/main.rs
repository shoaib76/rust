#[derive(Debug)]
enum Direction { Forward, Left, Backward, Right}

impl Direction {
    fn iterate(&self,direction: &Direction)  -> Direction {
       use Direction::*;
    
        
        match direction {
            Forward => Left,
            Left => Backward,
            Backward => Right,
            Right => Forward
                
            }
            
            
        }
    }    
       
       // let robot_move = [Forward, Left, Backward, Right, Forward, Left, Backward, Right, Forward, Left ];
       // println!("{:?}",robot_move[i]);
            
        
        
        

        
    


fn main(){
  
    let mut enumvar = Direction::Forward;
    
    for i in 0..10 {

        enumvar = enumvar.iterate(&enumvar);
            println!("{:?}",enumvar);
    }
       
}

