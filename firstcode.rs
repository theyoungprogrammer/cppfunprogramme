



// this is firecode.rs file
//this is rust-lang programming code
//this is only for practising purpose and its author is Gowrish VisagaPerumal



use std::io; //using input and output crate





// main function is important

fn main() {


    println!("hello world"); // printing the universal begineer code the hello world
    let x: (i32, f64, i32) = (78, 8.9, 87); // creating a tuple
    let yup = x.2; //defining y var as subset of a tuple 
    println!("{}", yup); // printing a tuple
    gowrish(); //calling the function gowrish()


}   
  
 
//new function

fn gowrish() {


    let jack = [65, 675, 2]; // creating a array
    let jk = jack[1]; // creating a variable
    println!("{}", jk); // printing array complex data type
    let mut s=String::new(); //creating new variable 
    println!("enter your number:"); //output
    io::stdin().read_line(&mut s); //input
    let mut v = String::new(); //creating mutable variable
    println!("enter your preference of language:"); //prefered language
    io::stdin().read_line(&mut v); //taking input 
    VisagaPerumal(); //calling the fn visagaperumal()



}
  
// creating new function

fn VisagaPerumal() { 


    println!("this is from function Visagaperumal"); // lets have fun in this function 
    let mut arr = ["gowrish", "visaga", "hellboy"]; // creating a array of names
    println!("this an array of names :: {:?}", arr); // printing an array
    let mut yurnam = String::new();//creating new variable which will store your name and other stuff
    println!("want your name in the list! type your name below down here:"); //output code
    io::stdin().read_line(&mut yurnam); // input code
    println!("hurray your name was added to the list!!"); // just a part of fun
    let mut hell = ["gowrish", "visaga", "hellboy", &mut yurnam]; // making your name registerd in this array
    println!("{:?}", hell); // printing this modified yurname enterd array
    let mut integer = String::new(); //again input
    println!("what is your favorite number"); // asking qp 
    io::stdin().read_line(&mut integer); // input from the user
    println!("you have entered {}", integer); // printing the output given from the user 
    let foolishness: &str = "gowrishmeow";
    println!("{}", foolishness);
    
 



}

