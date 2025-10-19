use core::f64;
use num_complex::Complex64;

/// FFI declarations
mod ffi {
    use super::*;
    unsafe extern "C" {
        pub fn __avh_olo_dp_MOD_olo_onshell(threshold: *const f64);
        pub fn __avh_olo_dp_MOD_a0_c(r: *mut Complex64, m: *const Complex64);
        pub fn __avh_olo_dp_MOD_b0cc(
            r: *mut Complex64,
            p: *const Complex64,
            m1: *const Complex64,
            m2: *const Complex64,
        );
        pub fn __avh_olo_dp_MOD_c0cc(
            r: *mut Complex64,
            p1: *const Complex64,
            p2: *const Complex64,
            p3: *const Complex64,
            m1: *const Complex64,
            m2: *const Complex64,
            m3: *const Complex64,
        );
        pub fn __avh_olo_dp_MOD_d0cc(
            r: *mut Complex64,
            p1: *const Complex64,
            p2: *const Complex64,
            p3: *const Complex64,
            p4: *const Complex64,
            p12: *const Complex64,
            p13: *const Complex64,
            m1: *const Complex64,
            m2: *const Complex64,
            m3: *const Complex64,
            m4: *const Complex64,
        );
    }
}

/// Sets the on-shell threshold for OneLOop calculations.
///
/// # Arguments
/// * `threshold` – Threshold for treating values as on-shell. Typical small value: 1e-12.
pub fn olo_onshell(threshold: f64) {
    unsafe {
        ffi::__avh_olo_dp_MOD_olo_onshell(&threshold);
    }
}

/// Computes the 1-point scalar function A0(m²) with Feynman prescription.
pub fn olo_1_point_complex(mm: Complex64) -> [Complex64; 3] {
    let mut r = [Complex64::ZERO; 3]; // stack-allocated, aligned
    unsafe { ffi::__avh_olo_dp_MOD_a0_c(r.as_mut_ptr(), &mm) }
    r
}

/// Computes the 2-point scalar function B0(p², mm1², mm2²) with Feynman prescription.
pub fn olo_2_point_complex(p: Complex64, mm1: Complex64, mm2: Complex64) -> [Complex64; 3] {
    let mut r = [Complex64::ZERO; 3];
    unsafe { ffi::__avh_olo_dp_MOD_b0cc(r.as_mut_ptr(), &p, &mm1, &mm2) }
    r
}

/// Computes the 3-point scalar function C0(p1², p2², p3², mm1², mm2², mm3²) with Feynman prescription.
pub fn olo_3_point_complex(
    p1: Complex64,
    p2: Complex64,
    p3: Complex64,
    mm1: Complex64,
    mm2: Complex64,
    mm3: Complex64,
) -> [Complex64; 3] {
    let mut r = [Complex64::ZERO; 3];
    unsafe { ffi::__avh_olo_dp_MOD_c0cc(r.as_mut_ptr(), &p1, &p2, &p3, &mm1, &mm2, &mm3) }
    r
}

/// Computes the 4-point scalar function D0(p1², p2², p3², p4², p12², p23², mm1², mm2², mm3², mm4²)
/// with Feynman prescription.
pub fn olo_4_point_complex(
    p1: Complex64,
    p2: Complex64,
    p3: Complex64,
    p4: Complex64,
    p12: Complex64,
    p23: Complex64,
    mm1: Complex64,
    mm2: Complex64,
    mm3: Complex64,
    mm4: Complex64,
) -> [Complex64; 3] {
    let mut r = [Complex64::ZERO; 3];
    unsafe {
        ffi::__avh_olo_dp_MOD_d0cc(
            r.as_mut_ptr(),
            &p1,
            &p2,
            &p3,
            &p4,
            &p12,
            &p23,
            &mm1,
            &mm2,
            &mm3,
            &mm4,
        )
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex64;

    #[test]
    fn test_olo_1_point_complex() {
        olo_onshell(1e-10);
        let r = olo_1_point_complex(Complex64::new(100.0, -1.4));
        for (i, c) in r.iter().enumerate() {
            println!("A0[{}] = {} + {}i", i, c.re, c.im)
        }
    }

    #[test]
    fn test_olo_2_point_complex() {
        olo_onshell(1e-10);
        let r = olo_2_point_complex(
            Complex64::new(1.0, 0.0),
            Complex64::new(0.5, 0.0),
            Complex64::new(0.5, 0.0),
        );
        for (i, c) in r.iter().enumerate() {
            println!("B0[{}] = {} + {}i", i, c.re, c.im);
        }
    }
    #[test]
    fn test_olo_3_point_complex() {
        olo_onshell(1e-10);

        // external momenta squared (s1, s2, s3) and internal masses squared
        let p_in_sr = 2.0*0.005_f64.powi(2);
        let p_out_sr = 0.005_f64.powi(2);
        let m_sr = 0.02_f64.powi(2);
        let s1 = Complex64::new(p_in_sr, 0.0); // p1^2
        let s2 = Complex64::new(p_in_sr, 0.0); // p2^2
        let s3 = Complex64::new(p_out_sr, 0.0); // (p1+p2)^2

        let m1_sq = Complex64::new(m_sr, 0.0);
        let m2_sq = Complex64::new(m_sr, 0.0);
        let m3_sq = Complex64::new(m_sr, 0.0);

        let r = olo_3_point_complex(s1, s2, s3, m1_sq, m2_sq, m3_sq);

        for (i, c) in r.iter().enumerate() {
            println!("C0[{}] = {} + {}i", i, c.re, c.im);
        }
    }

    #[test]
    fn test_olo_4_point_complex() {
        let r = olo_4_point_complex(
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
        );
        for (i, c) in r.iter().enumerate() {
            println!("D0[{}] = {} + {}i", i, c.re, c.im);
        }
    }
}
