extern crate rand;

pub fn trial_factor(n: i64) -> (i64, i64) {
    for s in 2..1 + (n as f64).abs().sqrt().floor() as i64 {
        if n % s == 0 {
            return (s, (n/s))
        }
    }

    return (1, n)
}

pub fn pollard_rho_factor(n: i64) -> (i64, i64) {
    let g = |x: i64| ((x*x) + 1) % n;

    fn gcd(mut m: i64, mut n: i64) -> i64 {
        while m != 0 {
            let old_m = m;
            m = n % m;
            n = old_m;
        }

        n.abs()
    }

    let mut x: i64 = rand::random::<u8>() as i64 % 10;
    let mut y: i64 = x;
    let mut d: i64 = 1;

    while d == 1 {
        x = g(x);
        y = g(g(y));
        d = gcd((x - y).abs(), n);
    }

    if d == n { 
        return (1, n);
    } else {
        return (d, n/d);
    }
}


#[cfg(test)]
mod tests {
    use *;
    #[test]
    fn trial_test() {
        println!("{:?}", trial_factor(751 * 757));
    }

    #[test]
    fn pollard_rho_test() {
        println!("{:?}", pollard_rho_factor(751 * 757));
    }
}
