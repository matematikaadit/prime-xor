fn sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    is_prime
}

fn main() {
    use std::iter;
    use std::io::{stdout, Write};
    use std::env;

    let stdout = stdout();
    let mut stdout = stdout.lock();

    let mut args = env::args();
    let len = args.nth(1)
        .expect("Need first argument for the dimension")
        .parse()
        .unwrap();

    let max = (0..len)
        .flat_map(|n| (0..len).zip(iter::repeat(n)))
        .map(|(u, v)| u ^ v)
        .max()
        .unwrap();

    let is_prime = sieve(max);
    writeln!(stdout, "P4").unwrap();
    writeln!(stdout, "{0} {0}", len).unwrap();
    let dim = (len + 1) / 8;
    let mut bytes = Vec::with_capacity(dim * dim);
    for row in 0..len {
        let mut output = 0;
        let mut count = 0;
        for col in 0..len {
            let bit = is_prime[row ^ col] as u8;
            output = (output << 1) | bit;
            count += 1;
            if count == 8 {
                bytes.push(output);
                count = 0;
                output = 0;
            }
        }
        if count > 0 {
            output <<= 8 - count;
            bytes.push(output);
        }
    }
    stdout.write(&bytes).unwrap();
}
