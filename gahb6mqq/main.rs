use std::io::{self, Read};

fn view_angle(ox: f64, oy: f64, xs: &[f64], ys: &[f64]) -> f64 {
    let mut a = Vec::with_capacity(xs.len());
    for i in 0..xs.len() {
        let mut t = (ys[i] - oy).atan2(xs[i] - ox);
        if t < 0.0 {
            t += std::f64::consts::TAU;
        }
        a.push(t);
    }
    a.sort_by(|u, v| u.partial_cmp(v).unwrap());

    let mut gap: f64 = 0.0;
    for i in 1..a.len() {
        gap = gap.max(a[i] - a[i - 1]);
    }
    gap = gap.max(a[0] + std::f64::consts::TAU - a[a.len() - 1]);
    std::f64::consts::TAU - gap
}

fn eval(t: f64, r: f64, xs: &[f64], ys: &[f64]) -> f64 {
    let ox = r * t.cos();
    let oy = r * t.sin();
    view_angle(ox, oy, xs, ys)
}

fn norm(mut t: f64) -> f64 {
    t %= std::f64::consts::TAU;
    if t < 0.0 {
        t += std::f64::consts::TAU;
    }
    t
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let r: f64 = it.next().unwrap().parse().unwrap();

    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    for _ in 0..n {
        xs.push(it.next().unwrap().parse::<f64>().unwrap());
        ys.push(it.next().unwrap().parse::<f64>().unwrap());
    }

    let mut best_t: f64 = 0.0;
    let mut best: f64 = eval(0.0, r, &xs, &ys);

    const BASE: usize = 2000;
    for i in 1..BASE {
        let t = (i as f64) * std::f64::consts::TAU / (BASE as f64);
        let v = eval(t, r, &xs, &ys);
        if v > best {
            best = v;
            best_t = t;
        }
    }

    let mut step = std::f64::consts::TAU / (BASE as f64);

    for _ in 0..120 {
        let t_l = norm(best_t - step);
        let t_r = norm(best_t + step);

        let v_l = eval(t_l, r, &xs, &ys);
        let v_m = eval(best_t, r, &xs, &ys);
        let v_r = eval(t_r, r, &xs, &ys);

        if v_l >= v_m && v_l >= v_r {
            best_t = t_l;
            best = v_l;
        } else if v_r >= v_m && v_r >= v_l {
            best_t = t_r;
            best = v_r;
        } else {
            best = v_m;
            step *= 0.7;
        }
    }

    println!("{:.12}", best);
}
