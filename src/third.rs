//este archivo es para codear sobre Ownership de RUST

pub fn main(){
    let mut a1 = String::from("Mike");
    let mut a2 = a1;
    //println!("{}",a1); //aqui me marca error, porque a1 no existe. lo eliminé de momoria al asignarselo a otro
    a2.push_str("_2 ");
    println!("{}",a2);
    let mut a3 = a2.clone();
    println!("a2: {}, - a3: {}",a2,a3);
    println!("{:?}",calculate_length(a2));
}

fn calculate_length(s: String) -> (String, usize, u8) {
    let length = s.len(); // len() returns the length of a String
    let minum: u8 = s.trim().as_bytes()[2]; //como hacer parar un u32 char
    (s, length, minum)
}