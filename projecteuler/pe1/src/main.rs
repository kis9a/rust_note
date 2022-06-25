fn main() {
    let mut res = 0;
    for i in 1..1001 {
        if i % 3 == 0 && i % 5 == 0 {
            res += i
        }
    }
    println!("{}", res);
}
