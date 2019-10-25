
#![allow(unused_variables)]
fn main() {
let s = String::from("first second");

let mut hello = &s[0..5];

let world = &s[6..12];

println!("hello: {} , world: {}" , hello , world) ;

}