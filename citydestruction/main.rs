use std::io::{self, Read};

struct FastInput {
    b: Vec<u8>,
    i: usize,
}
impl FastInput {
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

fn attacks(h: i128, d: i128) -> i128 {
    if h <= 0 { 0 } else { (h + d - 1) / d }
}

fn main() {
    let mut inp = FastInput::new();
    let t = inp.next_i64() as usize;

    let mut out = String::new();
    for _ in 0..t {
        let n = inp.next_i64() as usize;
        let d = inp.next_i64() as i128;

        let mut h = vec![0i128; n];
        let mut e = vec![0i128; n];
        for i in 0..n { h[i] = inp.next_i64() as i128; }
        for i in 0..n { e[i] = inp.next_i64() as i128; }

        if n == 1 {
            out.push_str(&attacks(h[0], d).to_string());
            out.push('\n');
            continue;
        }

        let inf = i128::MAX / 4;
        let mut dp = [inf; 2];
        for s0 in 0..=1 {
            let incoming = if s0 == 1 { e[1] } else { 0 };
            dp[s0] = attacks(h[0] - incoming, d);
        }

        for i in 1..(n - 1) {
            let mut nxt = [inf; 2];
            for prev in 0..=1 {
                if dp[prev] == inf { continue; }
                let incoming_left = if prev == 0 { e[i - 1] } else { 0 };
                for s in 0..=1 {
                    let incoming_right = if s == 1 { e[i + 1] } else { 0 };
                    let cand = dp[prev] + attacks(h[i] - incoming_left - incoming_right, d);
                    if cand < nxt[s] { nxt[s] = cand; }
                }
            }
            dp = nxt;
        }

        let mut ans = inf;
        for prev in 0..=1 {
            let incoming = if prev == 0 { e[n - 2] } else { 0 };
            let cand = dp[prev] + attacks(h[n - 1] - incoming, d);
            if cand < ans { ans = cand; }
        }

        out.push_str(&ans.to_string());
        out.push('\n');
    }
    print!("{out}");
}
