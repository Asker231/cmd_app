use std::io;
use colored::Colorize;
fn main() {
   input_user()
}


#[derive(Debug)]
enum Gender{
    Men(String),
    Women(String)
}


#[derive(Debug)]
struct  User{
     name:String,
     age:String,
     email:String,
     gender:Gender
}

fn input_user(){
    let mut person = User{
        name:"".to_string(),
        age:"".to_string(),
        email:"".to_string(),
        gender:Gender::Men("Men".to_string())

    };
    loop {
        println!("{}","Print your name:".bold().blue());
        match io::stdin().read_line(&mut person.name){
            Ok(_)=>{

            },
            Err(e)=>println!("{}",e)
        }
        println!("{}","Print your age:".bold().green());
        match io::stdin().read_line(&mut person.age){
            Ok(_)=>{

            },
            Err(e)=>println!("{}",e)
        }
        println!("{}","Print your email:".bold().bright_cyan());
        match io::stdin().read_line(&mut person.email){
            Ok(_)=>{},
            Err(e)=>println!("{}",e)
        }
        println!("{:?}",person);
    }

}

fn clos(){
    let mut a = 0;
    return a+=1;
}

