use std::env;
// use std::io::Write;
use std::str::FromStr;

fn main() {
    let args: Vec<_> = env::args().collect(); // https://doc.rust-lang.org/beta/std/env/fn.args.html
    if args.len() < 2 { // The first element is traditionally the path of the executable
        eprintln!("Usage: fibonacci <n>");
        std::process::exit(1);
    }
    match u64::from_str(&args[1]) { // https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str
        Ok(n) => println!("Fibonacci({}): {}", n, fibonacci(n)), // https://stackoverflow.com/questions/27588416/how-to-send-output-to-stderr
        Err(e) => eprintln!("Cannot parse '{}': {}", args[1], e),
    }
}

// mut: by default variables are immutable, `mut` makes the variable
// mutable (pronounced 'mute').
// It's not needed in here, just for the sake of the example.
// The compiler won't be happy with this ¯\_(ツ)_/¯
fn fibonacci(mut n: u64) -> u64 {
    assert!(n >= 1); // the '!' in the end of 'assert' means it's a macro invocation (*not* a function call)
    // There's also 'debug_assert!' <- assertions ignoerd on a compiled program
    if n == 1 {
        return 1
    }
    let mut sum = 0; // Types can be inferred by the compiler, we can specify them however:
    let mut i: u64 = 0;
    let mut curr= 1;
    let mut prev = 0;
    while i < n - 1 {
        sum = prev + curr;
        prev = curr;
        curr = sum;
        i += 1;
    }
    sum // return value -- the last statement in the function, CANNOT END WITH ';'
}

#[test] // This is an *attribute* - https://doc.rust-lang.org/reference/attributes.html
fn test_fibonacci() {
    assert_eq!(fibonacci(8), 21);
    assert_eq!(fibonacci(50), 12586269025);
    assert_eq!(fibonacci(1), 1);
}

#[test]
#[should_panic]
fn test_fibonacci_invalid_args() {
    fibonacci(0);
}
