//use std::io "io" is library come from std(standard library) and it used to obtain use input and then print the result as output so we need ot vbring the io input/output library into scope
//rust each line of code end with";"
//scope 界限







use std ::io;
fn main() {
    println!("guess the number!");
    println!("please input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("You guessed:{guess}");

    // check if guess == machine random then return "correct" otherwise return "wrong"


    //println!("Hello, world!");
}


