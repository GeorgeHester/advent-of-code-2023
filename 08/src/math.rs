pub fn lcm(a: u64, b: u64) -> u64
{
    return (a * b) / gcd(a, b);
}

pub fn lcm_vec(input: Vec<u64>) -> u64
{
    return input
        .iter()
        .map(|value| *value)
        .reduce(|a, b| lcm(a, b))
        .unwrap();
}

pub fn gcd(mut a: u64, mut b: u64) -> u64
{
    while a % b > 0
    {
        let r: u64 = a % b;
        a = b;
        b = r;
    }

    return b;
}
