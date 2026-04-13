use std::io::{self, Read};

struct Scanner {
    b: Vec<u8>,
    i: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self { b: s.into_bytes(), i: 0 }
    }
    fn next_i64(&mut self) -> i64 {
        while self.i < self.b.len() && self.b[self.i].is_ascii_whitespace() {
            self.i += 1;
        }
        let mut sign = 1;
        if self.i < self.b.len() && self.b[self.i] == b'-' {
            sign = -1;
            self.i += 1;
        }
        let mut v = 0i64;
        while self.i < self.b.len() && !self.b[self.i].is_ascii_whitespace() {
            v = v * 10 + (self.b[self.i] - b'0') as i64;
            self.i += 1;
        }
        v * sign
    }
}

fn main() {
    const MOD: i64 = 1_000_000_007;

    let mut sc = Scanner::new();
    let n = sc.next_i64() as usize;
    let k = sc.next_i64() as usize;

    let mut xs = vec![0i64; n];
    let mut ys = vec![0i64; n];
    for i in 0..n {
        xs[i] = sc.next_i64();
        ys[i] = sc.next_i64();
    }

    let mut dp = vec![vec![i128::MAX / 4; k + 1]; n];
    dp[0][0] = 0;

    for i in 1..n {
        for j in 0..i {
            let skipped = i - j - 1;
            if skipped > k {
                continue;
            }
            let dx = (xs[i] - xs[j]) as i128;
            let dy = (ys[i] - ys[j]) as i128;
            let cost = dx * dx + dy * dy;

            for used in skipped..=k {
                let prev = dp[j][used - skipped];
                if prev == i128::MAX / 4 {
                    continue;
                }
                let cand = prev + cost;
                if cand < dp[i][used] {
                    dp[i][used] = cand;
                }
            }
        }
    }

    let mut ans = i128::MAX / 4;
    for used in 0..=k {
        if dp[n - 1][used] < ans {
            ans = dp[n - 1][used];
        }
    }

    let out = (ans % MOD as i128 + MOD as i128) % MOD as i128;
    println!("{}", out as i64);
}
