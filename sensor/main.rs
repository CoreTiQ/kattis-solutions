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

fn color_order(p: u128, adj: &[u128], order: &mut Vec<usize>, bound: &mut Vec<usize>) {
    order.clear();
    bound.clear();

    let mut masks: Vec<u128> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();

    let mut t = p;
    while t != 0 {
        let v = t.trailing_zeros() as usize;
        t &= t - 1;

        let mut c = 0usize;
        while c < masks.len() && (adj[v] & masks[c]) != 0 {
            c += 1;
        }
        if c == masks.len() {
            masks.push(0);
            groups.push(Vec::new());
        }
        masks[c] |= 1u128 << v;
        groups[c].push(v);
    }

    for (c, g) in groups.into_iter().enumerate() {
        for v in g {
            order.push(v);
            bound.push(c + 1);
        }
    }
}

fn dfs(r: u128, r_sz: usize, mut p: u128, adj: &[u128], best: &mut u128, best_sz: &mut usize) {
    if p == 0 {
        if r_sz > *best_sz {
            *best_sz = r_sz;
            *best = r;
        }
        return;
    }

    let mut order = Vec::new();
    let mut bound = Vec::new();
    color_order(p, adj, &mut order, &mut bound);

    for idx in (0..order.len()).rev() {
        let v = order[idx];
        let bit = 1u128 << v;

        if r_sz + bound[idx] <= *best_sz {
            return;
        }

        dfs(r | bit, r_sz + 1, p & adj[v], adj, best, best_sz);
        p &= !bit;
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next_i64() as usize;
    let d = sc.next_i64() as i128;
    let d2 = d * d;

    let mut xs = vec![0i64; n];
    let mut ys = vec![0i64; n];
    for i in 0..n {
        xs[i] = sc.next_i64();
        ys[i] = sc.next_i64();
    }

    let mut adj = vec![0u128; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = xs[i] as i128 - xs[j] as i128;
            let dy = ys[i] as i128 - ys[j] as i128;
            if dx * dx + dy * dy <= d2 {
                adj[i] |= 1u128 << j;
                adj[j] |= 1u128 << i;
            }
        }
    }

    let mut best = 0u128;
    let mut best_sz = 0usize;

    let cand = if n == 128 { u128::MAX } else { (1u128 << n) - 1 };
    dfs(0, 0, cand, &adj, &mut best, &mut best_sz);

    println!("{best_sz}");
    let mut out = Vec::new();
    let mut t = best;
    while t != 0 {
        let v = t.trailing_zeros() as usize;
        t &= t - 1;
        out.push(v + 1);
    }
    out.sort_unstable();
    for i in 0..out.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", out[i]);
    }
    println!();
}
