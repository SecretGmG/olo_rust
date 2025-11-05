use num_complex::{Complex64, ComplexFloat};
use olo_rust::{three_point, four_point, set_renormalization_scale};

#[test]
fn test_olo_scale() {
    // Set scale to 91.1876 (Z boson mass, GeV)
    let mu = 91.1876;
    set_renormalization_scale(mu);

    // Just verify it can be called without crashing
    println!("Successfully set OneLOop renormalisation scale to {}", mu);
}
#[test]
fn test_olo_3_point_ir_divergent_with_scales() {
    let m_sr = 0.0;

    // IR-divergent momenta squared
    let s1 = Complex64::new(0.01, 0.0);
    let s2 = Complex64::new(0.01, 0.0);
    let s3 = Complex64::new(0.0, 0.0);

    let m1_sq = Complex64::new(m_sr, 0.0);
    let m2_sq = Complex64::new(m_sr, 0.0);
    let m3_sq = Complex64::new(m_sr, 0.0);

    // First scale
    let mu1 = 1.0;
    set_renormalization_scale(mu1);
    let r1 = three_point(s1, s2, s3, m1_sq, m2_sq, m3_sq);

    // Second scale
    let mu2 = 100.0;
    set_renormalization_scale(mu2);
    let r2 = three_point(s1, s2, s3, m1_sq, m2_sq, m3_sq);

    println!("First Scale:  {:?}", r1);
    println!("Second Scale: {:?}", r2);

    let difference = (r1.epsilon_0() - r2.epsilon_0()).abs()
        + (r1.epsilon_minus_1() - r2.epsilon_minus_1()).abs()
        + (r1.epsilon_minus_2() + r2.epsilon_minus_2()).abs();
    assert!(difference > 0.01)
}
#[test]
fn test_olo_4_point_ir_divergent_with_scales() {
    let m_sr = 0.0;

    let p1 = Complex64::new(0.01, 0.0);
    let p2 = Complex64::new(0.02, 0.0);
    let p3 = Complex64::new(0.03, 0.0);
    let p4 = Complex64::new(0.04, 0.0);

    let p12 = Complex64::new(0.0, 0.0);
    let p23 = Complex64::new(0.0, 0.0);

    let m1_sq = Complex64::new(m_sr, 0.0);
    let m2_sq = Complex64::new(m_sr, 0.0);
    let m3_sq = Complex64::new(m_sr, 0.0);
    let m4_sq = Complex64::new(m_sr, 0.0);

    // First scale
    let mu1 = 1.0;
    set_renormalization_scale(mu1);
    let r1 = four_point(p1, p2, p3, p4, p12, p23, m1_sq, m2_sq, m3_sq, m4_sq);

    // Second scale
    let mu2 = 100.0;
    set_renormalization_scale(mu2);
    let r2 = four_point(p1, p2, p3, p4, p12, p23, m1_sq, m2_sq, m3_sq, m4_sq);

    println!("First Scale {}:  {:?}", mu1, r1);
    println!("Second Scale {}: {:?}", mu2, r2);

    let difference = (r1.epsilon_0() - r2.epsilon_0()).abs()
        + (r1.epsilon_minus_1() - r2.epsilon_minus_1()).abs()
        + (r1.epsilon_minus_2() + r2.epsilon_minus_2()).abs();
    assert!(difference > 0.01)
}
