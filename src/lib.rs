pub fn trial_factor(n: i64) -> (i64, i64) {
    for s in 2..1 + (n as f64).abs().sqrt().floor() as i64 {
        if n % s == 0 {
            return (s, (n/s))
        }
    }

    return (1, n)
}

fn sieve_of_eratosthenes(n: u64) /* -> Vec<u64> */ {
    let mut numbers = (0..n+1).collect::<Vec<u64>>();
    numbers[1] = 0;

    for i in 2..(n as f64).sqrt().floor() as u64 + 1 {
        if numbers[i as usize] < 2 {
            continue;
        }

        for k in 2 .. n/i + 1 {
            numbers[(k*i) as usize] = 0;
        }

        println!("{:?}", i);
    }

    println!("{:?}", numbers);
}


#[cfg(test)]
mod tests {
    use *;
    #[test]
    fn it_works() {
        sieve_of_eratosthenes(121);
        println!("{:?}", trial_factor(-4));
    }
}
