fn main() {
use std::collections::HashMap;

let mut Fav_dish = HashMap::new();

Fav_dish.insert("Ali", "Biryani");
Fav_dish.insert("Hammad", "Tikka");
Fav_dish.insert("Tahir", "Pizza");


// let dish = Fav_dish.get("Ali");

// println!("{:?}", dish);


match Fav_dish.get("amir"){
    Some(fav) => println!("Favourite dish is {}" , fav),
    None => println!("Favourite dish does not exist")
}
}
