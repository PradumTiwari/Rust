trait CanJump{
    fn jump(&self);
}

trait CanAttack{
    fn attack(&self);
}

struct Ninja;
struct Robot;

impl CanJump for Ninja{
    fn jump(&self) {
        println!("Ninja Can jump");
    }
}

fn perform_jump<T:CanJump>(player:T){
    player.jump();
}


fn main(){
    let ninja=Ninja;
    
    perform_jump(ninja);
}