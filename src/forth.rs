pub fn main() {
    //let mut s1 = String::from("Mike");
/*
    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    println!("tamano s {}",r1.len());
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);*/
    let s1 = String::from("Mike profe");
    println!("tamano antes de espacio: {}", first_word(&s1));
    println!("tamano personalizado: {}",parteme_palabra(&s1, 1,4));
}

fn parteme_palabra(s: &String, desde: usize, hasta: usize) -> &str {
    &s[desde..hasta]
}
fn print_type_of<T>(_: &T) {
    println!("tipo de elemento: {}", std::any::type_name::<T>())
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("bytes: {:?}",bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            print_type_of(&i);
            return i;
        }
    }

    s.len()
}

fn calculate_length(s: &mut String) -> usize {
    println!("{}",*s);
    s.push_str("_1");
    //s = s.to_owned() + " _1";
    s.len()
}




