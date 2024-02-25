pub fn fibonacci_number(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }
    let mut prev = 1;
    let mut curr = 1;
    for _ in 3..n + 1 {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
