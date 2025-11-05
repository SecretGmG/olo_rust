use crate::{
    OLOResult, OLOUnit, olo_1_point_complex, olo_2_point_complex, olo_3_point_complex,
    olo_4_point_complex, olo_onshell, olo_renormalization_scale, olo_unit,
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
fn olo_1_point_complex_py(m: Complex<f64>) -> PyOLOResult {
    olo_1_point_complex(m).into()
}

#[pyfunction]
fn olo_2_point_complex_py(p: Complex<f64>, m1: Complex<f64>, m2: Complex<f64>) -> PyOLOResult {
    olo_2_point_complex(p, m1, m2).into()
}

#[pyfunction]
fn olo_3_point_complex_py(
    p1: Complex<f64>,
    p2: Complex<f64>,
    p3: Complex<f64>,
    m1: Complex<f64>,
    m2: Complex<f64>,
    m3: Complex<f64>,
) -> PyOLOResult {
    olo_3_point_complex(p1, p2, p3, m1, m2, m3).into()
}

#[pyfunction]
fn olo_4_point_complex_py(
    p1: Complex<f64>,
    p2: Complex<f64>,
    p3: Complex<f64>,
    p4: Complex<f64>,
    p12: Complex<f64>,
    p23: Complex<f64>,
    m1: Complex<f64>,
    m2: Complex<f64>,
    m3: Complex<f64>,
    m4: Complex<f64>,
) -> PyOLOResult {
    olo_4_point_complex(p1, p2, p3, p4, p12, p23, m1, m2, m3, m4).into()
}

#[pyfunction]
fn olo_scale_py(mu: f64) {
    olo_renormalization_scale(mu);
}

#[pyfunction]
fn set_olo_unit_py(unit_name: &str, value: Option<i32>) -> PyResult<()> {
    let unit = match unit_name.to_lowercase().as_str() {
        "printall" => OLOUnit::PrintAll,
        "message" => OLOUnit::Message,
        "warning" => OLOUnit::Warning,
        "error" => OLOUnit::Error,
        _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid OLOUnit")),
    };
    olo_unit(unit, value);
    Ok(())
}

#[pyfunction]
fn olo_onshell_py(threshold: f64) {
    olo_onshell(threshold);
}

#[pymodule]
fn olo_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(olo_1_point_complex_py, m)?)?;
    m.add_function(wrap_pyfunction!(olo_2_point_complex_py, m)?)?;
    m.add_function(wrap_pyfunction!(olo_3_point_complex_py, m)?)?;
    m.add_function(wrap_pyfunction!(olo_4_point_complex_py, m)?)?;
    m.add_function(wrap_pyfunction!(olo_scale_py, m)?)?;
    m.add_function(wrap_pyfunction!(set_olo_unit_py, m)?)?;
    m.add_function(wrap_pyfunction!(olo_onshell_py, m)?)?;
    Ok(())
}
