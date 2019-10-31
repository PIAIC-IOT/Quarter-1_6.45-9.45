fn main() {

use std::collections::HashMap;

let mut Fav_dish = HashMap::new();

Fav_dish.insert("Ali", "Biryani");
Fav_dish.insert("Hammad", "Tikka");
Fav_dish.insert("Tahir", "Pizza");

// Fav_dish.remove("Ali");

for (key, value) in &Fav_dish {
    println!("{}: {}", key, value);
}

}
