//! # OneLOop Rust Wrapper
//! 
//! This crate provides a **safe, idiomatic Rust interface** to the [OneLOop](https://helac-phegas.web.cern.ch/OneLOop.html) Fortran library for computing **scalar one-loop integrals**. It wraps the standard one-loop scalar functions (1-point to 4-point) and exposes them as Rust functions returning a `OLOResult` struct containing the Laurent expansion coefficients in dimensional regularization.
//! 
//! ## Features
//! 
//! - Safe Rust wrappers for OneLOop scalar functions:
//!   - `one_point` → 1-point (tadpole) function
//!   - `two_point` → 2-point (bubble) function
//!   - `three_point` → 3-point (triangle) function
//!   - `four_point` → 4-point (box) function
//! - Returns `OLOResult` with Laurent expansion coefficients:
//!   - `ε⁰` → finite term
//!   - `ε⁻¹` → first-order divergence (zero if IR-finite)
//!   - `ε⁻²` → second-order divergence (zero if IR-finite)
//! - Conversion to standard Feynman-diagram normalization via `TO_FEYNMAN`.
//! - Configurable logging, renormalization scale and on-shell thresholds.
//! 
//! ## Example
//! 
//! ```rust
//! use num_complex::Complex64;
//! use oneloop_bridge::{TO_FEYNMAN, two_point, three_point};
//! 
//! let p = 1.0;
//! let m1 = Complex64::new(0.5, 0.0);
//! let m2 = Complex64::new(0.2, 0.0);
//! let result = two_point(p, m1, m2);
//! println!("2-point result: {:?}", result);
//! println!("Finite term in Feynman convention: {:?}", result.epsilon_0() * TO_FEYNMAN);
//! ```
//! 
//! ## License
//! Licensed under the GNU GENERAL PUBLIC LICENSE Version 3.
use core::f64;
use num_complex::Complex64;
use std::{f64::consts::PI, fmt};
use std::ffi::CString;

#[cfg(feature = "python")]
mod python;


/// Conversion factor from the Ellis-Zanderighi / OneLOop normalization of
/// one-loop scalar integrals to the textbook Feynman-diagram normalization.
///
/// The constant `TO_FEYNMAN` converts a LoopTools-normalized integral
/// to the Feynman-diagram normalization.
///
/// Numerically, this is `-1/(16 π^2)`.
pub const TO_FEYNMAN: f64 = -1.0 / (16.0 * PI * PI);

/// Represents the Laurent expansion coefficients of a one-loop scalar function
/// in dimensional regularization.
///
/// The coefficients correspond to the expansion in the dimensional regularization
/// parameter `ε = (4-d)/2`:
///
/// - `values[0]`  ε⁰ coefficient
/// - `values[1]`  ε⁻¹ coefficient (vanishes for IR-finite cases)
/// - `values[2]`  ε⁻² coefficient (vanishes for IR-finite cases)
#[derive(Clone, Copy, Default, PartialEq)]
pub struct OLOResult {
    values: [Complex64; 3],
}

impl OLOResult {
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

impl fmt::Display for OLOResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ε⁰: {}, ε⁻¹: {}, ε⁻²: {}",
            self.values[0], self.values[1], self.values[2]
        )
    }
}

impl fmt::Debug for OLOResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResultOLO")
            .field("epsilon_0", &self.values[0])
            .field("epsilon_minus_1", &self.values[1])
            .field("epsilon_minus_2", &self.values[2])
            .finish()
    }
}

/// Units that control OneLOop’s print/message behavior.
#[derive(Debug, Clone, Copy)]
pub enum OLOUnit {
    PrintAll,
    Message,
    Warning,
    Error,
}


/// FFI declarations
mod ffi {
    use super::*;
    unsafe extern "C" {
        pub fn __avh_olo_units_MOD_set_unit(message: *const i8, val: *const i32);
        pub fn __avh_olo_dp_MOD_olo_onshell(threshold: *const f64);
        pub fn __avh_olo_dp_MOD_olo_scale(mu_scale: *const f64);
        pub fn __avh_olo_dp_set_mu(mu: *const f64);
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
            p23: *const Complex64,
            m1: *const Complex64,
            m2: *const Complex64,
            m3: *const Complex64,
            m4: *const Complex64,
        );
    }
}

/// Sets the renormalization scale for OneLOop calculations.
///
/// # Arguments
/// * `mu` - The renormalization scale μ (f64).
pub fn set_renormalization_scale(mu: f64) {
    unsafe {
        ffi::__avh_olo_dp_MOD_olo_scale(&mu);
    }
}

/// Sets the output unit for OneLOop messages.
///
/// # Arguments
/// * `unit`  - The type of message to configure (`PrintAll`, `Message`, `Warning`, `Error`).
/// * `value` - The Fortran unit number to direct output to (default: 6 = stdout).
pub fn set_log_level(unit: OLOUnit, fortran_unit_number: Option<i32>) {
    let val = fortran_unit_number.unwrap_or(6);
    let msg = match unit {
        OLOUnit::PrintAll => "printall",
        OLOUnit::Message  => "message",
        OLOUnit::Warning  => "warning",
        OLOUnit::Error    => "error",
    };

    let c_msg = CString::new(msg).expect("CString failed");
    unsafe {
        ffi::__avh_olo_units_MOD_set_unit(c_msg.as_ptr(), &val);
    }
}


/// Sets the on-shell threshold for OneLOop calculations.
///
/// # Arguments
/// * `threshold` - Threshold for treating values as on-shell.
pub fn set_onshell_threshold(threshold: f64) {
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
/// This uses the Ellis-Zanderighi normalization convention. To convert to
/// standard Feynman-diagram normalization, multiply by `TO_FEYNMAN`.
pub fn one_point(m: Complex64) -> OLOResult {
    let mut r = OLOResult::default(); // stack-allocated, aligned
    unsafe { ffi::__avh_olo_dp_MOD_a0_c(r.as_mut_ptr(), &m) }
    r
}

/// Computes the 2-point scalar (bubble) function for two propagators.
///
/// # Arguments
/// * `p`  - The squared momentum flowing through the propagator pair.
/// * `m1` - The squared mass of the first propagator.
///          The imaginary part should be non-positive.
/// * `m2` - The squared mass of the second propagator.
///          The imaginary part should be non-positive
///
/// # Returns
/// A `ResultOLO` containing the evaluated complex scalar integral.
///
/// # Notes
/// This uses the Ellis-Zanderighi normalization convention. To convert to
/// standard Feynman-diagram normalization, multiply by `TO_FEYNMAN`.
pub fn two_point(p: f64, m1: Complex64, m2: Complex64) -> OLOResult {
    let mut r = OLOResult::default();
    unsafe { ffi::__avh_olo_dp_MOD_b0cc(r.as_mut_ptr(), &p.into(), &m1, &m2) }
    r
}

/// Computes the 3-point scalar (triangle) function for three propagators.
///
/// # Arguments
/// * `p1` - The squared momentum of the first leg.
/// * `p2` - The squared momentum of the second leg.
/// * `p3` - The squared momentum of the third leg, usually `(p1 + p2)^2`.
/// * `m1` - The squared mass of the first propagator. Imaginary part should be non-positive.
/// * `m2` - The squared mass of the second propagator. Imaginary part should be non-positive.
/// * `m3` - The squared mass of the third propagator. Imaginary part should be non-positive.
///
/// # Returns
/// A `ResultOLO` containing the evaluated complex scalar integral.
///
/// # Notes
/// This uses the Ellis-Zanderighi normalization convention. To convert to
/// standard Feynman-diagram normalization, multiply by `TO_FEYNMAN`.
pub fn three_point(
    p1: f64,
    p2: f64,
    p3: f64,
    m1: Complex64,
    m2: Complex64,
    m3: Complex64,
) -> OLOResult {
    let mut r = OLOResult::default();
    unsafe { ffi::__avh_olo_dp_MOD_c0cc(r.as_mut_ptr(), &p1.into(), &p2.into(), &p3.into(), &m1, &m2, &m3) }
    r
}

/// Computes the 4-point scalar function D0(p1², p2², p3², p4², p12², p23², mm1², mm2², mm3², mm4²)
/// with Feynman prescription.
/// Computes the 4-point scalar (box) function for four propagators.
///
/// # Arguments
/// * `p1`   - The squared momentum of the first leg.
/// * `p2`   - The squared momentum of the second leg.
/// * `p3`   - The squared momentum of the third leg.
/// * `p4`   - The squared momentum of the fourth leg `p1 + p2 + p3`.
/// * `p12`  - The squared momentum of the sum `p1 + p2`.
/// * `p23`  - The squared momentum of the sum `p2 + p3`.
/// * `m1`   - The squared mass of the first propagator. Imaginary part should be non-positive.
/// * `m2`   - The squared mass of the second propagator. Imaginary part should be non-positive.
/// * `m3`   - The squared mass of the third propagator. Imaginary part should be non-positive.
/// * `m4`   - The squared mass of the fourth propagator. Imaginary part should be non-positive.
///
/// # Returns
/// A `ResultOLO` containing the evaluated complex scalar integral.
///
/// # Notes
/// This uses the Ellis-Zanderighi normalization convention. To convert to
/// standard Feynman-diagram normalization, multiply by `TO_FEYNMAN`.
pub fn four_point(
    p1:  f64,
    p2:  f64,
    p3:  f64,
    p4:  f64,
    p12: f64,
    p23: f64,
    m1: Complex64,
    m2: Complex64,
    m3: Complex64,
    m4: Complex64,
) -> OLOResult {
    let mut r = OLOResult::default();
    unsafe {
        ffi::__avh_olo_dp_MOD_d0cc(
            r.as_mut_ptr(),
            &p1.into(),
            &p2.into(),
            &p3.into(),
            &p4.into(),
            &p12.into(),
            &p23.into(),
            &m1,
            &m2,
            &m3,
            &m4,
        )
    }
    r
}