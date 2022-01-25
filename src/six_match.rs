

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("pasé por Penny {}",value_in_cents(Coin::Dime));
            5
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(h) => Some(h + 1),
    }
}



pub fn main(){
    println!("el valor de Penny es {}", value_in_cents(Coin::Penny));
    let five = Some(-5);
    print_type_of(&five);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("el valor de six es {}",six.unwrap());
    print_type_of(&none);
    //println!("el valor de none es {}",none.unwrap());+

    
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }
    
        fn add_fancy_hat() {println!("add fancy hat");}
        fn remove_fancy_hat() {}
        fn reroll() {println!("ninguna ooption");}
    
    
}

fn print_type_of<T>(_: &T) {
    println!("tipo de elemento: {}", std::any::type_name::<T>())
}