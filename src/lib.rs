use num_complex::Complex64;

/// C-compatible complex type
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Cf64 {
    pub re: f64,
    pub im: f64,
}

impl Cf64 {
    pub fn new(c: Complex64) -> Self {
        Self { re: c.re, im: c.im }
    }

    pub fn into_complex(self) -> Complex64 {
        Complex64::new(self.re, self.im)
    }
}

/// Helper for functions returning arrays of 3 complex numbers
#[repr(C, align(16))]
pub struct Align3(pub [Cf64; 3]);

impl Align3 {
    pub fn new() -> Self {
        Self([Cf64 { re: 0.0, im: 0.0 }; 3])
    }

    pub fn into_array(self) -> [Complex64; 3] {
        self.0.map(|c| c.into_complex())
    }

    pub fn as_mut_ptr(&mut self) -> *mut Cf64 {
        self.0.as_mut_ptr()
    }
}

/// FFI declarations
mod ffi {
    use super::Cf64;

    unsafe extern "C" {
        pub fn __avh_olo_qp_MOD_olo_onshell(threshold: *const f64);
        pub fn __avh_olo_qp_MOD_a0_c(r: *mut Cf64, m: *const Cf64);
        pub fn __avh_olo_qp_MOD_b0cc(
            r: *mut Cf64,
            p: *const Cf64,
            m1: *const Cf64,
            m2: *const Cf64,
        );
        pub fn __avh_olo_qp_MOD_c0cc(
            r: *mut Cf64,
            p1: *const Cf64,
            p2: *const Cf64,
            p3: *const Cf64,
            m1: *const Cf64,
            m2: *const Cf64,
            m3: *const Cf64,
        );
        pub fn __avh_olo_qp_MOD_d0cc(
            r: *mut Cf64,
            p1: *const Cf64,
            p2: *const Cf64,
            p3: *const Cf64,
            p4: *const Cf64,
            p12: *const Cf64,
            p13: *const Cf64,
            m1: *const Cf64,
            m2: *const Cf64,
            m3: *const Cf64,
            m4: *const Cf64,
        );
    }
}

/// Sets the on-shell threshold for OneLOop calculations.
///
/// # Arguments
/// * `threshold` – Threshold for treating values as on-shell. Typical small value: 1e-12.
pub fn olo_onshell(threshold: f64) {
    unsafe {
        ffi::__avh_olo_qp_MOD_olo_onshell(&threshold);
    }
}

/// Computes the 1-point scalar function A0(m²) with Feynman prescription.
pub fn olo_1_point_complex(mm: Complex64) -> [Complex64; 3] {
    let mut r: Align3 = Align3::new(); // stack-allocated, aligned
    let mm_cf = Cf64::new(mm);
    unsafe { ffi::__avh_olo_qp_MOD_a0_c(r.as_mut_ptr(), &mm_cf) }
    r.into_array()
}

/// Computes the 2-point scalar function B0(p², mm1², mm2²) with Feynman prescription.
pub fn olo_2_point_complex(p: Complex64, mm1: Complex64, mm2: Complex64) -> [Complex64; 3] {
    let mut r: Align3 = Align3::new();
    let pp = Cf64::new(p);
    let mm1_cf = Cf64::new(mm1);
    let mm2_cf = Cf64::new(mm2);
    unsafe { ffi::__avh_olo_qp_MOD_b0cc(r.as_mut_ptr(), &pp, &mm1_cf, &mm2_cf) }
    r.into_array()
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
    let mut r: Align3 = Align3::new();
    let pp1 = Cf64::new(p1);
    let pp2 = Cf64::new(p2);
    let pp3 = Cf64::new(p3);
    let mm1_cf = Cf64::new(mm1);
    let mm2_cf = Cf64::new(mm2);
    let mm3_cf = Cf64::new(mm3);
    unsafe {
        ffi::__avh_olo_qp_MOD_c0cc(r.as_mut_ptr(), &pp1, &pp2, &pp3, &mm1_cf, &mm2_cf, &mm3_cf)
    }
    r.into_array()
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
    let mut r: Align3 = Align3::new();
    let pp1 = Cf64::new(p1);
    let pp2 = Cf64::new(p2);
    let pp3 = Cf64::new(p3);
    let pp4 = Cf64::new(p4);
    let pp12 = Cf64::new(p12);
    let pp23 = Cf64::new(p23);
    let mm1_cf = Cf64::new(mm1);
    let mm2_cf = Cf64::new(mm2);
    let mm3_cf = Cf64::new(mm3);
    let mm4_cf = Cf64::new(mm4);
    unsafe {
        ffi::__avh_olo_qp_MOD_d0cc(
            r.as_mut_ptr(),
            &pp1,
            &pp2,
            &pp3,
            &pp4,
            &pp12,
            &pp23,
            &mm1_cf,
            &mm2_cf,
            &mm3_cf,
            &mm4_cf,
        )
    }
    r.into_array()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex64;

    #[test]
    fn test_olo_1_point_complex() {
        olo_onshell(1e-10);
        let r = olo_1_point_complex(Complex64::new(0.0, 0.0));
        for (i, c) in r.iter().enumerate() {
            println!("A0[{}] = {} + {}i", i, c.re, c.im)
        }
    }

    #[test]
    fn test_olo_2_point_complex() {
        let r = olo_2_point_complex(
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
        );
        for (i, c) in r.iter().enumerate() {
            println!("B0[{}] = {} + {}i", i, c.re, c.im);
        }
    }

    #[test]
    fn test_olo_3_point_complex() {
        let r = olo_3_point_complex(
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
        );
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
    #[test]
    fn verify_sizes() {
        println!("Cf64 size: {}", std::mem::size_of::<Cf64>()); // 16
        println!("Align3 size: {}", std::mem::size_of::<Align3>()); // 48
        println!("Align3 align: {}", std::mem::align_of::<Align3>()); // 16
    }
}
