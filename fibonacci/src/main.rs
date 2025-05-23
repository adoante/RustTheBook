fn fib_o_rama(mut fib: i128) -> i128 {
    let mut prev = 0;
    let mut curr = 1;
    let mut temp;

    while fib - 1 > 0 {
        temp = curr;
        curr = prev + curr;
        prev = temp;
        fib = fib - 1;
        println!("{fib}: {curr}")
    }

    curr
}

fn main() {
    let fibby = fib_o_rama(1000);
    println!("{fibby}");
}
