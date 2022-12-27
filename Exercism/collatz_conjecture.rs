pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut number = n;
    let mut steps = 0;
    while number > 1 {
        if number % 2 == 0 {
            number = number / 2;
        } else {
            number.checked_mul(3)?.checked_add(1)?; //check that there is no under/overflow by mutiplying by 3 and then adding 1, if under/overflow returns None
            number = number * 3 + 1;
        }
        steps += 1;
    }
    Some(steps)
}
