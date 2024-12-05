// Rust import statements
use std::io;
use std::str::FromStr;

// Rust enum for storing calculator operations
#[derive(PartialEq)]
#[derive(Debug)] // Rust macro for debug operation https://doc.rust-lang.org/std/fmt/trait.Debug.html
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    GCD,
    Modulo,
    Sqrt,
}

/*
*  Implements from_str for calculator operations enum
*/
impl FromStr for Operation {
    type Err = String; // Specifies error type

    /*
    *  Processes the input calculator option, throws error if operation is invalid
    *  Input: 
    *       input - A string slice
    *  Output: Result<Self, Self::Err>
    *       Self - The type of the trait (Operation)
    *       Self::Err - The error for this type
    */
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim() {  // Match the input to the available operations
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            "^" => Ok(Operation::Exponent),
            "gcd" => Ok(Operation::GCD),
            "%" => Ok(Operation::Modulo),
            "sqrt" => Ok(Operation::Sqrt),
            _ => Err(format!("Invalid operation: {}", input)), // Error
        }
    }
}

/*
*  Performs the necessary operation on two numbers and returns the result
*  Input:
*       op - The calculator operation
*       num1 - A 64-bit float
*       num2 - A 64-bit float
*  Output: Result<f64, String>
*       f64 - The 64-bit float result of the operation
*       String - The error from the operation
*/
fn calculate(op: Operation, num1: f64, num2: f64) -> Result<f64, String> {
        match op { // Match input operator with available operators
            Operation::Add => Ok(num1 + num2),
            Operation::Subtract => Ok(num1 - num2),
            Operation::Multiply => Ok(num1 * num2),
            Operation::Divide => {
                if num2 == 0.0 { // Throw error if division by zero
                    Err("Cannot divide by zero".to_string())
                } else {
                    Ok(num1 / num2)
                }
            }
            Operation::Exponent => Ok(num1.powf(num2)),
            Operation::GCD => {
                if num1.fract() != 0.0 || num2.fract() != 0.0 {
                    Err("GCD can only be calculated for integers".to_string())
                } else {
                    let a = num1 as u64;
                    let b = num2 as u64;
                    Ok(gcd(a, b) as f64)
                }
            }
            Operation::Modulo => {
                if num2 == 0.0 {
                    Err("Cannot divide by zero".to_string())
                } else {
                    Ok(num1 % num2)
                }
            },
            Operation::Sqrt => Err("Square root operation should be handled separately.".to_string()),
        }
}

fn square_root(num1: f64) -> Result<f64, String> {
    if num1 < 0.0 {
        Err("Cannot take the square root of a negative number".to_string())
    } else {
        Ok(num1.sqrt())
    }
}


/*
*  Function to calculate the greatest common divisor of two integers
*/
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}


fn main() {
    println!("Hello! This is a very basic calculator implemented in Rust.");
    println!("This calculator accepts integers and floats.");
    println!("Currently supported operations: + - * / ^ gcd");
    
    // Enter calculator loop
    loop {
        println!("Type 'exit' to quit");

        // Retrieve first number
        println!("Enter the first number:");
        let mut inp1 = String::new();  // Variable for first number input
        // Read user input, storing in inp1 and throwing error if program fails
        io::stdin().read_line(&mut inp1).expect("Failed to read/process input.");
        if inp1.trim().eq_ignore_ascii_case("exit") { // Exit program
            break;
        }
        let num1: f64 = match inp1.trim().parse() { // Process the input number
            Ok(n) => n,
            Err(_) => { // Throw error if invalid number
                println!("{} is invalid. Try again.", inp1.trim());
                continue;
            }
        };

        // Retrieve operation symbol
        println!("Enter the operation (+ - * / ^ gcd % sqrt):");
        let mut op_inp = String::new(); // Variable for operation input
        // Read user input, storing in op_inp, and throwing error if program fails
        io::stdin().read_line(&mut op_inp).expect("Failed to read/process input.");
        if op_inp.trim().eq_ignore_ascii_case("exit") { // Exit program
            break;
        }
        let operation = match op_inp.trim().parse::<Operation>() { // Process the operation
            Ok(op) => op,
            Err(e) => { // Throw error if invalid operation
                println!("{}", e);
                continue;
            }
        };

        if operation == Operation::Sqrt {
            match square_root(num1) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => println!("Error: {}", e),
            }
            continue; // Skip the rest of the loop for Sqrt
        }
        else {
            // Retrieve second number
            println!("Enter the second number:");
            let mut inp2 = String::new(); // Variable for second number
            // Read user input, storing in inp2, and throwing error if program fails
            io::stdin().read_line(&mut inp2).expect("Failed to read/process input.");
            if op_inp.trim().eq_ignore_ascii_case("exit") { // Exit program
                break;
            }
            let num2 = match inp2.trim().parse() { // Process the input number
                Ok(n) => n,
                Err(_) => {
                    println!("{} is invalid. Try again.", inp2.trim());
                    continue;
                }
            };

            // Perform calculation
            match calculate(operation, num1, num2) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => println!("Error: {}", e),
            }
        }

        println!("------------------------");
    }

    println!("Calculator was stopped. Goodbye!");
}