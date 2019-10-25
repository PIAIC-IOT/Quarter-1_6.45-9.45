fn main() {
    
// to strings

    let mut a = "hello".to_string();
    
// push_str

    a.push_str(" world ");

    println!("{}", a);

// push
 
    a.push('1') ;

    println!("{}", a);  

// concatenation


     let s1 = String::from("Hello, ");
     let s2 = String::from("world!");
     let s3 = String::from(" PIAIC");

     let s3 = s1 + &s2 + &s3; // note s1 has been moved here and can no longer be used

     println!("{}", s3);
//     println!("{}", s2);   What will happen ?
//     println!("{}", s2);   What will happen ?

// format

     let s1 = String::from("tic");
     let s2 = String::from("tac");
     let s3 = String::from("toe");

     let s = format!("{}-{}-{}", s1, s2, s3);

// purpose of slicing

let hello = "Здравствуйте";

let s = &hello[0..4];

println!("{}" , s);

}
