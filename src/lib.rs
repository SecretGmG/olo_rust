use std::os::raw::c_double;
use num_complex::Complex64;

unsafe extern "C" {
    pub fn olo_a0(rslt: *mut Complex64, mm: *const c_double, rmu: *const c_double);
}

pub fn double_a0(mm: f64, rmu: f64) -> [Complex64; 3] {
    let mut rslt = [Complex64::new(0.0, 0.0); 3];
    unsafe { olo_a0(rslt.as_mut_ptr(), &mm, &rmu) };
    rslt
}
