use std::io;
use rand::Rng;
use std::cmp::Ordering;

/*Si recuerdo C++ la librería std io es para input - output, wow! */ 
fn main() {
    println!("Adivina el numerito!");
    let secret_number = rand::thread_rng().gen_range(1..10);
    let mut guess = String::new();
    //let mut algo = "Mike";
    /*con let declaro variable, (Como BASIC) :: es para el tipo de dato */
    /* Qué es "MUT?"*/ 
    /*"mut" es para variables mutables, "sin mut" para inmutables, "mu" para vacas 😁 */ 
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    println!("The secret number is: {}", secret_number);
    //io::stdin().read_line(&mut algo);
    //println!("tu algo {}",algo);
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    //lo anterior es, UTILIZO la variable "guess" le digo que va a ser un entero de 32 bits le quito los espacios "Trim"
    //y luego la "parseo", la convierto a entero.
    

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    //Aqui en lugar de IF utilizo la función comparación "match"
    //Donde comparo guess con Secret_number
}
