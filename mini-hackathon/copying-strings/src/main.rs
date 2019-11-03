 use std::io;

 fn main(){


  println!("Enter String:");  
 let mut element = String::new();
        io::stdin().read_line(&mut element).expect("Failed to read lines");   

  println!("How many copies of String you need:");
 let mut number = String::new();
        io::stdin().read_line(&mut number);
     let mut num : usize = number.trim().parse().unwrap(); 

     let inpstn = String::from(element.trim());

 //     let repeated = element.repeat(num);
//     let s = format!("{} copies of {} are :{}",number,word,repeated);
     println!("{} copies of {} are {}",num,&inpstn,copy(num,&inpstn));
 }
 fn copy(copy1: usize, string1: &String) -> String {
     let mut stcopy = String::new();
     for a in 0..copy1 {
         stcopy = stcopy + &string1;
     }
     stcopy
 }

