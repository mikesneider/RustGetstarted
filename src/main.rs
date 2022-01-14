use std::io;

/*Si recuerdo C++ la librería std io es para input - output, wow! */ 
fn main() {
    println!("Adivina el numerito!");

    let mut guess = String::new();
    let mut algo = "Mike";
    /*con let declaro variable, (Como BASIC) :: es para el tipo de dato */
    /* Qué es "MUT?"*/ 
    /*"mut" es para variables mutables, "sin mut" para inmutables, "mu" para vacas 😁 */ 
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    //io::stdin().read_line(&mut algo);
    println!("tu algo {}",algo);
}
