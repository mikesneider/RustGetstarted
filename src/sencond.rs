//second.rs es sobre tuplas, arrays, funciones, condicionales y ciclos

pub fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    let minum: i32 = "-34".parse().expect("lola");
    println!("{}",minum);
    let mycat = '🎅';
    println!("{}",mycat);

    let mitupla = ("hola",3.4,500);
    println!("uno {} dos {} tres {}",mitupla.0,mitupla.1,mitupla.2);

    //Arrays
    let miarray = [1;5];
    println!("{:?}",miarray);
    //para imprimir un array (y una estructura) debo utilizar :? dentro de {} => {:?}
    let array2 = [1,2,3,4,5,6];
    println!("el elemento 3 es {}", array2[2]);
    println!("el valor de la funcion es: {}",sumame(array2[3]));

    let mivar = if mitupla.2 > 100 {true} else {false};
    println!("mivar {}",mivar);
    mainloop();
    miwhile();
}


fn sumame(x: u32)->u32{
    x*2
    //si quito el ; ese es mi return
}


fn mainloop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
fn miwhile(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    println!("LIFTOFF!!!");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    
}
