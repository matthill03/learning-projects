use std::io;

fn main() {

    let mut input_text = String::new();

    println!("Input a number:");
    io::stdin().read_line(&mut input_text).expect("Failed to read from stdin");

    let num = input_text.trim().parse::<i32>().expect("Invalid input");

    let reveresed = reverse_num(num);

    if num < 0 {
        println!("This is not a plaindrome number!!!");
    }
    else if num == reveresed {
        println!("This is a plaindrome number!!");
    }
    else {
        println!("This is not a plaindrome number!!!");
    }

}

fn reverse_num(mut num: i32) -> i32 {
    let mut remainder: i32;
    let mut reverse: i32 = 0;

    while num != 0 {
        remainder = num % 10; 
        reverse = reverse * 10 + remainder;
        num /= 10;
    }

    return reverse;
    
}

