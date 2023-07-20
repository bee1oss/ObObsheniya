use std::fmt::Display;

fn main() {

}

fn dinamikfunc(){
    let string = String::from("Hello");
    let num = 777;

    print_value(num,string);
}

fn print_value<T: Display,S:Display>(value:T,text:S){
    println!("{} {}",value,text);
}