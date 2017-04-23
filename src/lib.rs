pub fn trial_factor(n: i64) -> (i64, i64) {
    for s in 2..1 + (n as f64).abs().sqrt().floor() as i64 {
        if n % s == 0 {
            return (s, (n/s))
        }
    }

    return (1, n)
}


#[cfg(test)]
mod tests {
    use trial_factor;
    #[test]
    fn it_works() {
        println!("{:?}", trial_factor(-4));
    }
}
