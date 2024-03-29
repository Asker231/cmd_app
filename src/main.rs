use std::io;

fn main() {
   
   inputUser()

}

#[derive(Debug)]
struct  User{
     name:String,
     age:String,
     email:String,
}

fn inputUser(){

    let mut person = User{
        name:"".to_string(),
        age:"".to_string(),
        email:"".to_string(),
    };

    loop {
        println!("Print your name:");

        match io::stdin().read_line(&mut person.name){
            Ok(_)=>{

            },
            Err(e)=>println!("{}",e)
        }
        println!("Print your age:");
        match io::stdin().read_line(&mut person.age){
            Ok(_)=>{

            },
            Err(e)=>println!("{}",e)
        }
        println!("Print your email:");
        match io::stdin().read_line(&mut person.email){
            Ok(_)=>{

            },
            Err(e)=>println!("{}",e)
        }
        println!("{:?}",person )
    }

}
