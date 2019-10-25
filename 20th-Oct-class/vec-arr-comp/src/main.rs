use std::io;

fn main() {

// // Arrays
// // Arrays are the data type in which we have fixed number elements but we can change the 
// // the elements by declaring it mutable. It is also homogenous

// // immutable arrays

// println!("----Arrays----");

//     let arr = [1,2,3];

//     println!("{}" , arr[1]);

// // mutable arrays

//     let mut arr1 = [1,2,3];

//     arr1[1] = 25 ;

//     println!("{}" , arr1[1]);

// // Changing elements by input

//     let mut inpArr = String::new();

//     io::stdin().read_line(&mut inpArr)
//         .expect("Failed to read line");

//     let inpArr1 : i32 = inpArr.trim().parse().unwrap();

//     arr1[2] = inpArr1 ;

//     println!("{:?}" , arr1); 

// vectors

// immutable vectors

println!("----vectors----");

    let vec = vec![1,2,3];

  //  println!("{}" , vec[1]);

// mutable vectors

    let mut vec1 = vec![1,2,3];

    vec1[1] = 26 ;
    vec1.push(21);

    println!("{}" , vec1[3]);
    println!("{}" , vec1[1]);

 //   println!("{:?}" , vec1 );

    vec1.pop();

 //   println!("{:?}" , vec1 );

// Changing elements by input

    let mut inpVec = String::new();

    io::stdin().read_line(&mut inpVec)
        .expect("Failed to read line");

    let inpVec1 : i32 = inpVec.trim().parse().expect("input is wrong");

    vec1[2] = inpVec1;  // Changing element through input
    vec1.push(inpVec1); // creating element element through input

    println!("{:?}" , vec1); 


}