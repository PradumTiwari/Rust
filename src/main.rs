fn print_item<T:std::fmt::Debug>(item:T){
    println!("Item : {:?}",item);
}

fn main(){
    print_item(10);
}