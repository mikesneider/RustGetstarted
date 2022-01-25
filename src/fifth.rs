
 //#[derive(Debug)] 
 struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
    count: contadores
}

enum contadores {
    v1(String),
    v10(String),
    v20(u32)
}

//este enum existe por Default, el enum  Option<T>
enum Option<T> {
    None,
    Some(T)
}
pub fn main(){

    let some_number = Some(5);
    let some_text = Some("MIke");
    //let some_abscence: Option<i32> = None;
    println!("some number {:?}",some_number);

    let user1 = User {
        active: true,
        username: String::from("Mike"),
        email: String::from("mike@gmail.com"),
        sign_in_count: 1,
        count: contadores::v1(String::from("el contador v1"))
    };
    println!("{}",user1.username);
    //println!("{:?}",user1);
    let user2 = User {
        email: String::from("another@example.com"),
        sign_in_count: 10,
        ..user1
    };
    let user3 = User {
        active: false,
        username: String::from("Peluchin"),
        email: String::from("Peluchin@gmail.com"),
        sign_in_count: 20,
        count: contadores::v20(326)
    };
    println!("{}",user2.username);
    //println!("rect1 is {:?}", user2);
    //dbg!(user2);
    println!("sign in + el numero que le envio: {}",user2.cambiar2(&user3,8));
    //println!("imp el enum {:?}",user1.count);

}


impl User{
    fn cambiar2(&self,otro: &User, elnuevo: u32) -> u32 {
        (self.sign_in_count + elnuevo + otro.sign_in_count)
    }
}
fn cambiar_contador(usuario: &User, elnuevo: u32) -> u32 {
    (usuario.sign_in_count + elnuevo)
}

fn build_user(email: String, username: String) -> User {
    //Cuando tengo el mismo nomnre. e.g. "emial" no tengo que escribir 
    //email: email
    //lo hago como se muestra a continuación
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
        count: contadores::v10(String::from("el tipo v10"))
    }
}

//funcion imprimir tipo de dato
fn print_type_of<T>(_: &T) {
    println!("tipo de elemento: {}", std::any::type_name::<T>())
}


