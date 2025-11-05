use num_complex::Complex64;
use olo_rust::{TO_FEYNMAN, olo_2_point_complex, olo_3_point_complex};


/// Minkowski dot product: (E^2 - px^2 - py^2 - pz^2)
fn minkowski_dot(p: [f64; 4]) -> f64 {
    p[0] * p[0] - (p[1] * p[1] + p[2] * p[2] + p[3] * p[3])
}

#[test]
fn main() {
    // 2-point example (bubble)
    let p = Complex64::new(1.0, 0.0);
    let m1 = Complex64::new(0.5, 0.0);
    let m2 = Complex64::new(0.2, 0.0);
    let result = olo_2_point_complex(p, m1, m2);
    println!("2-point result: {:?}", result);

    // 3-point example (triangle)
    let k1 = [0.005, 0.0, 0.0, 0.005];
    let k2 = [0.005, 0.0, 0.0, -0.005];

    let p1 = Complex64::new(minkowski_dot(k1), 0.0);
    let p2 = Complex64::new(minkowski_dot(k2), 0.0);

    // p3 = (k1 + k2)^2
    let k3 = [k1[0] + k2[0], k1[1] + k2[1], k1[2] + k2[2], k1[3] + k2[3]];
    let p3 = Complex64::new(minkowski_dot(k3), 0.0);

    let m = Complex64::new(0.02, 0.0);

    let m1 = m * m;
    let m2 = m * m;
    let m3 = m * m;

    let result = olo_3_point_complex(p1, p2, p3, m1, m2, m3);
    println!("3-point result: {:?}", result);
    println!(
        "Interal value in Feynman convention: {:?}",
        result.epsilon_0() * TO_FEYNMAN
    )
}
