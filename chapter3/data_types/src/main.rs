/// Data Types lesson
use std::io;


/// Example from Rust Book, chapter 3.2
fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

/*fn main() {
    const fp: f64 = -3.14;
    let it: i32 = fp as i32;
    let mut array = [fp as f32; -fp as usize];
    print_type_of(&array);
    print_type_of(&array[0]);
    println!("array[0] value: {}", array[0]);
    println!("{} conv to int: {}", fp, it);
    let us: u8 = 0xFF;
    let si: i8 = 0xffu8 as i8;
    println!("unsigned: {}; signed: {}", us, si);
    let mut user_input: UserInput;
    loop {
        user_input = get_input();
        match user_input {
            UserInput::Number(num) => {
                let u : u8 = num as u8;
                let s : i8 = num as i8;
                let s32 : i32 = s as i32;
                let u_gt_s : bool = u as i32 > s32;
                println!("num: 0x{:x}", num);
                println!("signed: {}; unsigned: {}", s, u);
                println!("s32: {}", s32);
                println!("is unsigned > signed? {}", u_gt_s);
            }
            UserInput::Invalid(s) => println!("Not a number: {}", s),
            UserInput::Quit() => break,
        }
    }
}*/

// The following is curtesy of...
// https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// Question: dymanic String object is in this mut enum.
// The string is owned by the enum.
// When the enum is reassigned,
// Is the String's buffer deallocated?
// Based on Rust's guarantees, it should, right?
// OK, I guess that this enum owns the String.
#[derive(Debug)]
enum UserInput {
    Number(i32),
    Invalid(String),
    Quit(),
}

fn get_input() -> UserInput {
    println!("Enter a number, or q[uit]");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse() {
        Ok(value) => UserInput::Number(value),
        Err(_) => {
            let thechar = input.trim().chars().nth(0);
            if let Some(letter) = thechar {
                if letter == 'q' {
                    UserInput::Quit()
                } else {
                    UserInput::Invalid(input)
                }
            } else {
                UserInput::Invalid(input)
            }
        }
    }
}

