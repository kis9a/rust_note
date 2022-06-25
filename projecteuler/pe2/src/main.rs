fn main() {
    println!("{}", rec_fibonacci(1, 1, 0));
}

fn is_even(i: i32) -> bool {
    if i % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

fn rec_fibonacci(a: i32, b: i32, i: i32) -> i32 {
    let n = a + b;
    if n < 4000000 {
        if is_even(n) {
            rec_fibonacci(b, n, i + n)
        } else {
            rec_fibonacci(b, n, i)
        }
    } else {
        return i;
    }
}
