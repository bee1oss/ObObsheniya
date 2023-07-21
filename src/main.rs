use std::collections::hash_map::Keys;
use std::fmt::Display;

struct Data<T, K> {
    d1: T,
    d2: K,
}

impl<T, K> Data<T, K> {
    fn get_data_1(&self)->&T{
        &self.d1
    }
    fn get_data_2(&self)->&K{
        &self.d2
    }
}

fn main() {
    let data = Data {
        d1: 2.32,
        d2: "July".to_string(),
    };
    println!("Data 1:{}\nData 2:{}",data.get_data_1(),data.get_data_2());
}

fn dinamikfunc() {
    let string = String::from("Hello");
    let num = 777;

    print_value(num, string);
}

fn print_value<T: Display, S: Display>(value: T, text: S) {
    println!("{} {}", value, text);
}
