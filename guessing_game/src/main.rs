// use rand::Rng;
// use std::cmp::Ordering;
//use std::io;

fn addition(a: i8, b:i8) -> i8{
     a+b 
}

fn main() {
    let x = 5;
    let y = {
        x + 1
    };
    println!("{}", y);
    println!("{}", x);
    print_measure(5, 'm');
    let sum = addition(5,12);
    println!("{}", sum);
    let sum = addition(sum, sum);
    println!("{}", sum);
    let s = String::from("Hello") ;
    let s = take_ownership(s);
    make_copy(x);
    println!("{}", s);
    let len = calculate_length(&s) ;
    println!("length is {}", len);
    

    /*
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--


        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    */
}

fn print_measure(value:i8, unit_label:char){
    println!("value : {value} unit = {unit_label}");
}

fn take_ownership(some_string:String)-> String{
    println!("{}", some_string);
    some_string

}

fn make_copy(some_integer : i8){
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize{
    s.len()
}