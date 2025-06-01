enum Pradum{
    Value(String),
    Empty,
}

fn greet (p:Pradum){
    match p{
        Pradum::Value(n)=>println!("Hello Pradum"),
        Pradum::Empty=> println!("Jeffry bezos"),
    }
}

fn main(){
    greet(Pradum::Value("22".to_string()));
    greet(Pradum::Empty);
}