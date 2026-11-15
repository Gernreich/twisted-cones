//! Checks every number the Twisted Cones pages show, from the same formulas the
//! pages draw with. Prints the figures and exits non-zero if any check fails.
use std::f64::consts::{PI, TAU};

static mut FAILS: u32 = 0;
fn check(name: &str, got: f64, want: f64, tol: f64) {
    let ok = (got - want).abs() <= tol;
    println!("{} {:<52} {:>14.6}  (want {:.6})", if ok { "ok  " } else { "FAIL" }, name, got, want);
    if !ok { unsafe { FAILS += 1 } }
}

// ---------- the series ----------
// 1. Twisted ellipse sweep: base ellipse (10,0,0)-(0,0,190), y within ±10; rise h turns
//    tau*h/1000 CCW and shrinks 1 - 0.9h/1000 about the axis end (0,0,190).
fn twisted(t: f64, h: f64, m: f64) -> [f64; 3] {
    let s = 1.0 - 0.9 * h / 1000.0;
    let th = TAU * h / 1000.0;
    let (dx, dy, dz) = (m * (5.0 + 5.0 * t.cos()) * s, 10.0 * t.sin() * s, (95.0 - 95.0 * t.cos() - 190.0) * s);
    [dx * th.cos() - dy * th.sin(), dx * th.sin() + dy * th.cos(), 190.0 + h + dz]
}
// 2. Spiral ellipse sweep: (20,0,0)-(10,0,190), shrinks about its centre, centre radius 15 -> 1.
fn spiral(t: f64, h: f64) -> [f64; 3] {
    let k = h / 1000.0;
    let (s, rc) = (1.0 - 0.9 * k, 15.0 - 14.0 * k);
    let th = TAU * k;
    let r = rc + 5.0 * t.cos() * s;
    let v = 10.0 * t.sin() * s;
    [r * th.cos() - v * th.sin(), r * th.sin() + v * th.cos(), 95.0 + h - 95.0 * t.cos() * s]
}
const ZT: f64 = 95.0 + 1000.0 * 15.0 / 14.0; // the centre line reaches the z axis here
fn centre_radius(z: f64) -> f64 { 15.0 - 14.0 * (z - 95.0) / 1000.0 }

// ---------- the variations ----------
#[derive(Clone, Copy)]
struct V { shape: &'static str, tip: &'static str, twist: &'static str, turns: f64, taper: &'static str, copies: u32, gap: f64, scale: f64 }
fn unit1(shape: &str, phi: f64) -> (f64, f64) {
    let (c, s) = (phi.cos(), phi.sin());
    match shape {
        "circle" => (c, s),
        "ellT" => (c, 2.0 * s),
        "ellR" => (2.0 * c, s),
        "square" => (c.signum() * c.abs().sqrt(), s.signum() * s.abs().sqrt()),
        "triangle" => { let n = 3.0; let psi = (phi - PI).rem_euclid(TAU / n); let r = (PI / n).cos() / (psi - PI / n).cos(); (r * c, r * s) }
        "trilobe" => { let r = 1.0 + 0.35 * (3.0 * (phi - PI)).cos(); (r * c, r * s) }
        _ => panic!("shape {shape}"),
    }
}
fn unit(v: &V, phi: f64, u: f64) -> (f64, f64) {
    let a = unit1(v.shape, phi);
    if v.tip == "same" { return a; }
    let b = unit1(v.tip, phi);
    (a.0 + (b.0 - a.0) * u, a.1 + (b.1 - a.1) * u)
}
fn rho(taper: &str, u: f64) -> f64 {
    let r0 = 16.33;
    match taper { "cone" => r0 * (1.0 - u), "exp" => r0 * 100f64.powf(-u), "bell" => r0 * (1.0 - u).powf(2.5), "swell" => r0 * (1.0 - u) * (1.0 + 3.0 * u), _ => panic!() }
}
fn theta(v: &V, u: f64) -> f64 {
    let f = match v.twist { "linear" => u, "accel" => u * u, "decel" => 1.0 - (1.0 - u) * (1.0 - u), "reverse" => if u < 0.5 { u } else { 1.0 - u }, _ => panic!() };
    TAU * (-0.095 + v.turns * f)
}
fn slice(v: &V, u: f64, n: usize) -> Vec<(f64, f64)> {
    let r = rho(v.taper, u);
    let loc: Vec<(f64, f64)> = (0..n).map(|j| { let (x, y) = unit(v, j as f64 / n as f64 * TAU, u); (x * r, y * r) }).collect();
    let mx = loc.iter().map(|q| q.0).fold(f64::MAX, f64::min);
    let th = theta(v, u);
    loc.iter().map(|&(x, y)| { let a = x - mx + v.gap; (a * th.cos() - y * th.sin(), a * th.sin() + y * th.cos()) }).collect()
}
fn reach(v: &V) -> (f64, f64) {
    let (mut near, mut far) = (f64::MAX, 0f64);
    for i in 0..1000 { for q in slice(v, i as f64 / 1000.0, 720) { let r = q.0.hypot(q.1); near = near.min(r); far = far.max(r); } }
    (near, far)
}

fn main() {
    println!("-- twisted ellipse sweep");
    check("semi-major axis", (10f64.powi(2) + 190f64.powi(2)).sqrt() / 2.0, 95.1315, 1e-4);
    let mut off = 0f64;
    for i in 0..=10000 { let p = twisted(PI, i as f64 * 0.1, 1.0); off = off.max(p[0].hypot(p[1])); }
    check("axis end's distance from the z axis, worst", off, 0.0, 1e-12);
    let r = twisted(0.0, 1000.0, 1.0);
    check("far end at h=1000, x", r[0], 1.0, 1e-9);
    check("far end at h=1000, z", r[2], 1171.0, 1e-9);
    let mut d = 0f64;
    for i in 0..=1000 { for j in 0..360 { let (h, t) = (i as f64, j as f64 / 360.0 * TAU);
        let p = twisted(t, h, -1.0); let q = twisted(-t, h, 1.0);
        d = d.max((p[0] + q[0]).abs() + (p[1] + q[1]).abs() + (p[2] - q[2]).abs()); } }
    check("mirror = original turned 180 deg, worst gap", d, 0.0, 1e-9);

    println!("-- spiral ellipse sweep");
    let mut m = f64::MAX;
    for i in 0..=2000 { for j in 0..720 { let p = spiral(j as f64 / 720.0 * TAU, i as f64 * 0.5); m = m.min(p[0].hypot(p[1])); } }
    check("closest any point comes to the z axis", m, 0.5, 1e-6);
    check("centre radius at h=1000", centre_radius(1095.0), 1.0, 1e-12);
    let lo = (-95f64, TAU * -95.0 / 1000.0);
    check("centre line at z=0, x", centre_radius(0.0) * lo.1.cos(), 13.5062, 1e-4);
    check("centre line at z=0, y", centre_radius(0.0) * lo.1.sin(), -9.1788, 1e-4);
    check("centre line reaches the axis at z", ZT, 1166.4286, 1e-4);

    println!("-- spiral circle sweep and elliptic cone sweep");
    check("base radius at z=0", centre_radius(0.0), 16.33, 1e-12);
    // every level slice (circle, and ellipse 1:2) must touch the z axis and never cross it
    for (name, k) in [("circle", 1.0), ("ellipse 1:2", 2.0)] {
        let mut near = f64::MAX;
        for i in 0..1166 { let z = i as f64; let q = centre_radius(z); let th = TAU * (z - 95.0) / 1000.0;
            for j in 0..3600 { let u = j as f64 / 3600.0 * TAU; let (a, b) = (q + q * u.cos(), k * q * u.sin());
                near = near.min((a * th.cos() - b * th.sin()).hypot(a * th.sin() + b * th.cos())); } }
        check(&format!("{name} slices: closest to the z axis"), near, 0.0, 1e-9);
    }
    // The tilted right cone the user asked about first: lean and roundness of its level slices
    let alpha = 0.5 * (2.0 * 16.33 / ZT).atan();
    check("tilted right cone lean, degrees", alpha.to_degrees(), 0.8019, 1e-4);

    println!("-- variations");
    // the variations' base cone turns exactly once (the sweeps turned tau per 1000 of rise, 1.166 turns)
    let b = V { shape: "ellT", tip: "same", twist: "linear", turns: 1.0, taper: "cone", copies: 1, gap: 0.0, scale: 1.0 };
    check("base turns, base to tip", (theta(&b, 1.0) - theta(&b, 0.0)) / TAU, 1.0, 1e-12);
    let cases: Vec<(&str, V, f64, f64)> = vec![
        ("0a spiral circle", V { shape: "circle", ..b }, 0.0, 32.66),
        ("0b elliptic cone", b, 0.0, 37.713),
        ("2 long axis outward", V { shape: "ellR", ..b }, 0.0, 65.32),
        ("3b triangle", V { shape: "triangle", ..b }, 0.0, 28.284),
        ("3c three lobes", V { shape: "trilobe", ..b }, 0.0, 39.286),
        ("8 swell", V { taper: "swell", ..b }, 0.0, 50.283),
        ("10a gap 4", V { gap: 4.0, ..b }, 4.0, 40.221),
        ("ellipse to circle", V { tip: "circle", ..b }, 0.0, 37.713),
        ("triangle to circle, speeds up, 3 turns", V { shape: "triangle", tip: "circle", twist: "accel", turns: 3.0, ..b }, 0.0, 28.284),
        ("three copies, 5 turns", V { copies: 3, turns: 5.0, ..b }, 0.0, 37.713),
    ];
    for (name, v, near, far) in &cases {
        let (n, f) = reach(v);
        check(&format!("{name}: closest to axis"), n * v.scale, *near, 1e-3);
        check(&format!("{name}: farthest from axis"), f * v.scale, *far, 1e-3);
        let _ = v.copies;
    }
    check("6 reverses: turns at the midpoint", (theta(&V { twist: "reverse", ..b }, 0.5) / TAU) + 0.095, 0.5, 1e-12);
    check("12a layers of 3 mm at 1/4 scale", (ZT * 0.25 / 3.0).floor(), 97.0, 0.0);
    check("12b layers of 3 mm at full scale", (ZT / 3.0).floor(), 388.0, 0.0);

    let f = unsafe { FAILS };
    if f > 0 { eprintln!("{f} check(s) failed"); std::process::exit(1); }
    println!("all checks pass");
}
