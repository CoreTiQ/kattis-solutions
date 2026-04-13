use std::collections::HashMap;
use std::io::{self, Read};

struct In {
    b: Vec<u8>,
    i: usize,
}
impl In {
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

#[derive(Clone, Copy)]
struct Pt {
    x: i64,
    y: i64,
}

fn cross(a: Pt, b: Pt, c: Pt) -> i128 {
    let abx = b.x - a.x;
    let aby = b.y - a.y;
    let acx = c.x - a.x;
    let acy = c.y - a.y;
    (abx as i128) * (acy as i128) - (aby as i128) * (acx as i128)
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    if a < 0 { a = -a; }
    if b < 0 { b = -b; }
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    if a == 0 { 1 } else { a }
}

fn normalize(n: i128, d: i128) -> (i128, i128) {
    let mut n = n;
    let mut d = d;
    if d < 0 {
        d = -d;
        n = -n;
    }
    let g = gcd(n, d);
    (n / g, d / g)
}

fn isqrt(x: i128) -> i128 {
    let mut r = (x as f64).sqrt() as i128;
    while r * r > x { r -= 1; }
    while (r + 1) * (r + 1) <= x { r += 1; }
    r
}

fn main() {
    let mut input = In::new();
    let n = input.next_i64() as usize;

    let mut pts = Vec::with_capacity(n);
    for _ in 0..n {
        let x1 = input.next_i64();
        let y1 = input.next_i64();
        let x2 = input.next_i64();
        let y2 = input.next_i64();
        pts.push((Pt { x: x1, y: y1 }, Pt { x: x2, y: y2 }));
    }

    let mut mp: HashMap<(i128, i128, i128, i128), i128> = HashMap::new();

    for i in 0..n {
        let (a, b) = pts[i];
        for j in (i + 1)..n {
            let (c, d) = pts[j];

            let c1 = cross(a, b, c);
            let c2 = cross(a, b, d);
            if c1 == 0 || c2 == 0 || (c1 > 0) == (c2 > 0) {
                continue;
            }

            let c3 = cross(c, d, a);
            let c4 = cross(c, d, b);
            if c3 == 0 || c4 == 0 || (c3 > 0) == (c4 > 0) {
                continue;
            }

            let rx = (b.x - a.x) as i128;
            let ry = (b.y - a.y) as i128;
            let sx = (d.x - c.x) as i128;
            let sy = (d.y - c.y) as i128;

            let rxs = rx * sy - ry * sx;
            if rxs == 0 {
                continue;
            }

            let qpx = (c.x - a.x) as i128;
            let qpy = (c.y - a.y) as i128;
            let t_num = qpx * sy - qpy * sx;

            let x_num = (a.x as i128) * rxs + rx * t_num;
            let y_num = (a.y as i128) * rxs + ry * t_num;

            let (xn, xd) = normalize(x_num, rxs);
            let (yn, yd) = normalize(y_num, rxs);
            *mp.entry((xn, xd, yn, yd)).or_insert(0) += 1;
        }
    }

    let mut add = 0i128;
    for (_k, m) in mp {
        let disc = 1 + 8 * m;
        let t = (1 + isqrt(disc)) / 2;
        add += t - 1;
    }

    let pieces = n as i128 + 1 + add;
    println!("{pieces}");
}
