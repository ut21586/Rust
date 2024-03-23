//use std::io "io" is library come from std(standard library) and it used to obtain use input and then print the result as output so we need ot vbring the io input/output library into scope
//rust each line of code end with";"
//scope 界限
//prelude: its a set that rust has a set of items defined in the standard library that it brings into the scope of every program.
//"use" if a type you want to use isn't in the prelude, you have to bring that type into scope explicitly with a use statement, using the std::io library provides you with a number of udeful features, including the ability to accept user input.
// "let mut guess = String::new();" create a variable to store the use input.
//"let apple = 5;" use "let" statetement to create a new variable and assign a new value with it.
//you still






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


