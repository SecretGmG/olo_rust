use crate::TO_FEYNMAN;
use crate::{
    OLOResult, OLOUnit
};
use num_complex::Complex;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
pub struct PyOLOResult {
    inner: OLOResult,
}
impl From<OLOResult> for PyOLOResult {
    fn from(value: OLOResult) -> Self {
        Self {inner: value}
    }
}

#[pymethods]
impl PyOLOResult {
    #[getter]
    fn epsilon_0(&self) -> Complex<f64> {
        self.inner.epsilon_0()
    }

    #[getter]
    fn epsilon_minus_1(&self) -> Complex<f64> {
        self.inner.epsilon_minus_1()
    }

    #[getter]
    fn epsilon_minus_2(&self) -> Complex<f64> {
        self.inner.epsilon_minus_2()
    }

    fn __repr__(&self) -> String {
        format!(
            "PyOLOResult(ε⁰={}, ε⁻¹={}, ε⁻²={})",
            self.inner.epsilon_0(),
            self.inner.epsilon_minus_1(),
            self.inner.epsilon_minus_2()
        )
    }
}

#[pyfunction]
fn one_point(m: Complex<f64>) -> PyOLOResult {
    crate::one_point(m).into()
}

#[pyfunction]
fn two_point(p: f64, m1: Complex<f64>, m2: Complex<f64>) -> PyOLOResult {
    crate::two_point(p, m1, m2).into()
}

#[pyfunction]
fn three_point(
    p1: f64,
    p2: f64,
    p3: f64,
    m1: Complex<f64>,
    m2: Complex<f64>,
    m3: Complex<f64>,
) -> PyOLOResult {
    crate::three_point(p1, p2, p3, m1, m2, m3).into()
}

#[pyfunction]
fn four_point(
    p1: f64,
    p2: f64,
    p3: f64,
    p4: f64,
    p12: f64,
    p23: f64,
    m1: Complex<f64>,
    m2: Complex<f64>,
    m3: Complex<f64>,
    m4: Complex<f64>,
) -> PyOLOResult {
    crate::four_point(p1, p2, p3, p4, p12, p23, m1, m2, m3, m4).into()
}

#[pyfunction]
fn set_renormalization_scale(mu: f64) {
    crate::set_renormalization_scale(mu);
}

#[pyfunction(signature = (unit_name, value = None))]
fn set_log_level(unit_name: &str, value: Option<i32>) -> PyResult<()> {
    let unit = match unit_name.to_lowercase().as_str() {
        "printall" => OLOUnit::PrintAll,
        "message" => OLOUnit::Message,
        "warning" => OLOUnit::Warning,
        "error" => OLOUnit::Error,
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid OLOUnit")),
    };
    crate::set_log_level(unit, value);
    Ok(())
}

#[pyfunction]
fn set_onshell_threshold(threshold: f64) {
    crate::set_onshell_threshold(threshold);
}

#[pymodule]
fn olo_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("TO_FEYNMAN", TO_FEYNMAN)?;
    m.add_function(wrap_pyfunction!(one_point, m)?)?;
    m.add_function(wrap_pyfunction!(two_point, m)?)?;
    m.add_function(wrap_pyfunction!(three_point, m)?)?;
    m.add_function(wrap_pyfunction!(four_point, m)?)?;
    m.add_function(wrap_pyfunction!(set_renormalization_scale, m)?)?;
    m.add_function(wrap_pyfunction!(set_log_level, m)?)?;
    m.add_function(wrap_pyfunction!(set_onshell_threshold, m)?)?;
    Ok(())
}
