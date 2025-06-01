fn add_world(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut name=String::from("Bob");

    let r1=&mut name;
    r1.push('l');
    println!("{}",r1);

}
