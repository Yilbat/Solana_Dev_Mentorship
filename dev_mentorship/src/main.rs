// fn main(){
//     let mut s = String::from("hello");
//     s.push_str(", world");
// println!("{s}");}



// fn main(){
//     let x = 5;
//     let y = x;

//     println!("the value of x is: {} and the value of y is : {}", x,y)
// }


// fn main(){
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s1}");
// }

// fn main(){
//     let s1 = String::from("hello");
//     let _len =calculate_length(&s1);

//     println!("{}", _len);
    
// }

// fn calculate_length(s: &String)-> usize{
//     s.len()
// }


// fn main(){
//     let mut s1 = String::from("hello");
//     let mut s2 = & mut s1;
//     let s3 = & mut s2;

//     println!("this is s3 which is {s3}");
// }

// fn main() {
//     // Use as many approaches as you can to make it work
//     let x = String::from("Hello world");
//     let y = x.clone();
//     println!("{}, {}",x, y);
// }


// fn main() {
//     let x = 5;
//     // Fill the blank
//     let p = &x;
 
//     println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
//  }

fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}