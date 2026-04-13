use std::io::{self, Read};

fn main() {
    const MOD: i64 = 1_000_000_007;

    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    const MAX: usize = 1_000_000;

    let mut spf = vec![0usize; MAX + 1];
    for i in 2..=MAX {
        if spf[i] == 0 {
            spf[i] = i;
            let mut j = i * i;
            while j <= MAX {
                if spf[j] == 0 {
                    spf[j] = i;
                }
                j += i;
            }
        }
    }

    let mut cnt = vec![0u32; MAX + 1];
    for _ in 0..n {
        let mut x: usize = it.next().unwrap().parse().unwrap();
        while x > 1 {
            let p = spf[x];
            let mut c = 0u32;
            while x % p == 0 {
                x /= p;
                c += 1;
            }
            cnt[p] += c;
        }
    }

    let mut ans = 1i64;
    for c in cnt.into_iter().skip(2) {
        if c > 0 {
            ans = ans * ((c as i64) + 1) % MOD;
        }
    }

    println!("{ans}");
}
