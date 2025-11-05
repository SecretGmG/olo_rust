use num_complex::Complex64;
use olo_rust::{
    one_point, two_point, three_point, four_point,
};

#[test]
fn test_olo_1_point_complex() {
    let r = one_point(Complex64::new(100.0, -1.4));
    println!("{}", r);
}

#[test]
fn test_olo_2_point_complex() {
    let r = two_point(
        Complex64::new(1.0, 0.0),
        Complex64::new(0.5, 0.0),
        Complex64::new(0.5, 0.0),
    );
    println!("{}", r);
}
#[test]
fn test_olo_3_point_complex() {
    let m_sr = 0.0004;

    let s1 = Complex64::new(0.01, 0.0); // p1^2
    let s2 = Complex64::new(0.01, 0.0); // p2^2
    let s3 = Complex64::new(0.001, 0.0); // (p1+p2)^2
    let m1_sq = Complex64::new(m_sr, 0.0);
    let m2_sq = Complex64::new(m_sr, 0.0);
    let m3_sq = Complex64::new(m_sr, 0.0);
    let r = three_point(s1, s2, s3, m1_sq, m2_sq, m3_sq);
    print!("{}", r)
}
#[test]
fn test_olo_4_point_complex() {
    let r = four_point(
        Complex64::new(0.1, 0.0),
        Complex64::new(0.1, 0.0),
        Complex64::new(0.1, 0.0),
        Complex64::new(0.1, 0.0),
        Complex64::new(0.1, 0.0),
        Complex64::new(0.1, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
        Complex64::new(0.0, 0.0),
    );
    println!("{}", r)
}
