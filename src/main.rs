fn interprdocut(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn collatz_length(mut n: i32) -> i32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    println!("{}", interprdocut(1, 2, 3));
    let n = 20;
    println!("fib({n}) = {}", fib(n));

    let mut x = 100;
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x = {x}");

    for elem in [1, 2, 3].iter() {
        println!("{}", elem);
    }

    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }

    println!("collatz_length(11) = {}", collatz_length(11));
}
