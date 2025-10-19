use core::f64;
use num_complex::Complex64;
use std::{f64::consts::PI, fmt};

/// Conversion factor from the Ellis–Zanderighi / OneLOop normalization of
/// one-loop scalar integrals to the textbook Feynman-diagram normalization.
///
/// The constant `TO_FEYNMAN` converts a LoopTools-normalized integral
/// to the Feynman-diagram normalization in the limit ε → 0 and μ = 1:
///
/// Numerically, this is `-1/(16 π^2)`.
pub const TO_FEYNMAN: f64 = -1.0 / (16.0 * PI * PI);

/// Represents the Laurent expansion coefficients of a one-loop scalar function
/// in dimensional regularization.
///
/// The coefficients correspond to the expansion in the dimensional regularization
/// parameter `ε = (4-d)/2`:
///
/// - `values[0]` → ε⁰ coefficient
/// - `values[1]` → ε⁻¹ coefficient (vanishes for IR-finite cases)
/// - `values[2]` → ε⁻² coefficient (vanishes for IR-finite cases)
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
/// * `threshold` – Threshold for treating values as on-shell.
pub fn olo_onshell(threshold: f64) {
    unsafe {
        ffi::__avh_olo_dp_MOD_olo_onshell(&threshold);
    }
}

/// Computes the 1-point scalar (tadpole) function for a propagator.
///
/// # Arguments
/// * `m` - The squared mass of the propagator.
///         The imaginary part should be non-positive.
///
/// # Returns
/// A `ResultOLO` containing the evaluated complex scalar integral.
///
/// # Notes
/// This uses the LoopTools / AVH OLO convention internally. To convert to
/// standard Feynman-diagram normalization, multiply by `TO_FEYNMAN`.
pub fn olo_1_point_complex(m: Complex64) -> ResultOLO {
    let mut r = ResultOLO::default(); // stack-allocated, aligned
    unsafe { ffi::__avh_olo_dp_MOD_a0_c(r.as_mut_ptr(), &m) }
    r
}

/// Computes the 2-point scalar (bubble) function for two propagators.
///
/// # Arguments
/// * `p`  - The squared momentum flowing through the propagator pair.
///          The imaginary part should be 0.
/// * `m1` - The squared mass of the first propagator.
///          The imaginary part should be non-positive.
/// * `m2` - The squared mass of the second propagator.
///          The imaginary part should be non-positive
///
/// # Returns
/// A `ResultOLO` containing the evaluated complex scalar integral.
///
/// # Notes
/// This uses the LoopTools / AVH OLO convention internally. To convert to
/// standard Feynman-diagram normalization, multiply by `TO_FEYNMAN`.
pub fn olo_2_point_complex(p: Complex64, m1: Complex64, m2: Complex64) -> ResultOLO {
    let mut r = ResultOLO::default();
    unsafe { ffi::__avh_olo_dp_MOD_b0cc(r.as_mut_ptr(), &p, &m1, &m2) }
    r
}

/// Computes the 3-point scalar (triangle) function for three propagators.
///
/// # Arguments
/// * `p1` - The squared momentum of the first leg. The imaginary part should be 0.
/// * `p2` - The squared momentum of the second leg. The imaginary part should be 0.
/// * `p3` - The squared momentum of the third leg, usually `(p1 + p2)^2`. Imaginary part should be 0.
/// * `m1` - The squared mass of the first propagator. Imaginary part should be non-positive.
/// * `m2` - The squared mass of the second propagator. Imaginary part should be non-positive.
/// * `m3` - The squared mass of the third propagator. Imaginary part should be non-positive.
///
/// # Returns
/// A `ResultOLO` containing the evaluated complex scalar integral.
///
/// # Notes
/// This uses the LoopTools / AVH OLO convention internally. To convert to
/// standard Feynman-diagram normalization, multiply by `TO_FEYNMAN`.
pub fn olo_3_point_complex(
    p1: Complex64,
    p2: Complex64,
    p3: Complex64,
    m1: Complex64,
    m2: Complex64,
    m3: Complex64,
) -> ResultOLO {
    let mut r = ResultOLO::default();
    unsafe { ffi::__avh_olo_dp_MOD_c0cc(r.as_mut_ptr(), &p1, &p2, &p3, &m1, &m2, &m3) }
    r
}

/// Computes the 4-point scalar function D0(p1², p2², p3², p4², p12², p23², mm1², mm2², mm3², mm4²)
/// with Feynman prescription.
/// Computes the 4-point scalar (box) function for four propagators.
///
/// # Arguments
/// * `p1`   - The squared momentum of the first leg. Imaginary part should be 0.
/// * `p2`   - The squared momentum of the second leg. Imaginary part should be 0.
/// * `p3`   - The squared momentum of the third leg. Imaginary part should be 0.
/// * `p4`   - The squared momentum of the fourth leg, usually `p1 + p2 + p3`. Imaginary part should be 0.
/// * `p12`  - The squared momentum of the sum `p1 + p2`. Imaginary part should be 0.
/// * `p23`  - The squared momentum of the sum `p2 + p3`. Imaginary part should be 0.
/// * `m1`   - The squared mass of the first propagator. Imaginary part should be non-positive.
/// * `m2`   - The squared mass of the second propagator. Imaginary part should be non-positive.
/// * `m3`   - The squared mass of the third propagator. Imaginary part should be non-positive.
/// * `m4`   - The squared mass of the fourth propagator. Imaginary part should be non-positive.
///
/// # Returns
/// A `ResultOLO` containing the evaluated complex scalar integral.
///
/// # Notes
/// This uses the LoopTools / AVH OLO convention internally. To convert to
/// standard Feynman-diagram normalization, multiply by `TO_FEYNMAN`.
pub fn olo_4_point_complex(
    p1: Complex64,
    p2: Complex64,
    p3: Complex64,
    p4: Complex64,
    p12: Complex64,
    p23: Complex64,
    m1: Complex64,
    m2: Complex64,
    m3: Complex64,
    m4: Complex64,
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
            &m1,
            &m2,
            &m3,
            &m4,
        )
    }
    r
}
#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::{Complex, Complex64};

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
    fn test_olo_3_point_complex() {
        olo_onshell(1e-10);
        // external momenta squared (s1, s2, s3) and internal masses squared
        let m_sr = 0.0004;

        let s1 = Complex64::new(0.0, 0.0); // p1^2
        let s2 = Complex64::new(0.0, 0.0); // p2^2
        let s3 = Complex64::new(0.0001, 0.0); // (p1+p2)^2
        let m1_sq = Complex64::new(m_sr, 0.0);
        let m2_sq = Complex64::new(m_sr, 0.0);
        let m3_sq = Complex64::new(m_sr, 0.0);
        let r = olo_3_point_complex(s1, s2, s3, m1_sq, m2_sq, m3_sq);
        print!("{}", r)
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

        let m = Complex64::new(0.02,0.0);

        let m1 = m*m;
        let m2 = m*m;
        let m3 = m*m;

        let result = olo_3_point_complex(p1, p2, p3, m1, m2, m3);
        println!("3-point result: {:?}", result);
        println!("Interal value in Feynman convention: {:?}", result.epsilon_0()*TO_FEYNMAN)
    }
}
