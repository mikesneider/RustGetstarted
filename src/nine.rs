//about Collections

pub fn main(){
    let vec1:Vec<u32> = vec![1,2,3];
    let mut vec2:Vec<i16> = Vec::new();
    //println!("{:?}",vec1);
    vec2.push(5);
    vec2.push(6);
    vec2.push(7);
    vec2.push(8);
    println!("long del vect: {}",vec2.len());

    let tercero:&i16 = &vec2[3];
    println!("el valor tercero: {}",tercero);

    match vec2.get(4){
        Some(x) => println!("el valor es: {}", x),
        None => println!("el indice está por fuera de rango")
    }

    //si lo recorro sin el & no puedo utilizar a vec2 después
   /* for x in vec2 {
        println!("el valor es: {}", x);
    }*/

    for x in &vec2 {
        print!("el v es: {}\n", &x);
    }

    for x in &mut vec2 {
        *x *= 2;
        //println!("{}",*x);
    }

    println!("p prn de v: {:?}", vec2);

}