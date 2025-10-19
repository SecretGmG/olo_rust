use core::f64;
use num_complex::Complex64;
use std::fmt;

/// Represents the Laurent expansion coefficients of a one-loop scalar function
/// in dimensional regularization (Eq.(10) in the ONELOOP context).
///
/// The coefficients correspond to the expansion in the dimensional regularization
/// parameter `ε = (4-d)/2`:
///
/// - `values[0]` → ε⁰ coefficient
/// - `values[1]` → ε⁻¹ coefficient (vanishes for IR-finite cases)
/// - `values[2]` → ε⁻² coefficient (vanishes for IR-finite cases)
///
/// By convention, the imaginary parts of all coefficients are non-positive.
/// If a positive imaginary part is encountered, it should be flipped to satisfy this rule.
///
/// **IR-finiteness / divergence:**
/// - IR-finite cases: `values[1]` and `values[2]` are identically zero.
/// - IR-divergent cases: `values[1]` and/or `values[2]` are non-zero, as determined
///   by the input configuration (internal masses, external momenta).
///   Divergent cases are only returned if the input exactly matches the known IR-divergent configurations.
///
/// This struct is primarily used as the return type of the `olo_*_complex` functions,
/// which compute scalar one-loop functions (A0, B0, C0, D0) with Feynman prescription.
///
/// # Example
/// ```rust
/// let r = olo_1_point_complex(Complex64::new(100.0, -1.0));
/// println!("ε⁰: {}, ε⁻¹: {}, ε⁻²: {}", r.epsilon_0(), r.epsilon_minus_1(), r.epsilon_minus_2());
/// ```

#[derive(Clone, Copy, Default)]
pub struct ResultOLO {
    values: [Complex64; 3],
}

impl ResultOLO {
    /// Returns a mutable pointer to the internal values array
    fn as_mut_ptr(&mut self) -> *mut Complex64 {
        self.values.as_mut_ptr()
    }

    /// Getter for the ε⁰ coefficient
    pub fn epsilon_0(&self) -> Complex64 {
        self.values[0]
    }

    /// Getter for the ε⁻¹ coefficient
    pub fn epsilon_minus_1(&self) -> Complex64 {
        self.values[1]
    }

    /// Getter for the ε⁻² coefficient
    pub fn epsilon_minus_2(&self) -> Complex64 {
        self.values[2]
    }
}

impl fmt::Display for ResultOLO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ε⁰: {}, ε⁻¹: {}, ε⁻²: {}",
            self.values[0], self.values[1], self.values[2]
        )
    }
}

impl fmt::Debug for ResultOLO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResultOLO")
            .field("epsilon_0", &self.values[0])
            .field("epsilon_minus_1", &self.values[1])
            .field("epsilon_minus_2", &self.values[2])
            .finish()
    }
}

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
pub fn olo_1_point_complex(mm: Complex64) -> ResultOLO {
    let mut r = ResultOLO::default(); // stack-allocated, aligned
    unsafe { ffi::__avh_olo_dp_MOD_a0_c(r.as_mut_ptr(), &mm) }
    r
}

/// Computes the 2-point scalar function B0(p², mm1², mm2²) with Feynman prescription.
pub fn olo_2_point_complex(p: Complex64, mm1: Complex64, mm2: Complex64) -> ResultOLO {
    let mut r = ResultOLO::default();
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
) -> ResultOLO {
    let mut r = ResultOLO::default();
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
) -> ResultOLO {
    let mut r = ResultOLO::default();
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
        println!("{}", r);
    }

    #[test]
    fn test_olo_2_point_complex() {
        olo_onshell(1e-10);
        let r = olo_2_point_complex(
            Complex64::new(1.0, 0.0),
            Complex64::new(0.5, 0.0),
            Complex64::new(0.5, 0.0),
        );
        println!("{}", r);
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
        println!("{}", r)
    }
}
