use pyo3::prelude::*;
use numpy::ndarray::{ ArrayViewMutD };
use numpy::PyReadwriteArrayDyn;

/// A Python module implemented in Rust. The name of this function is the Rust module name!
#[pymodule]
fn testlib(_py: Python, m: &PyModule) -> PyResult<()> {
    /// This is the actual function `multiply`.
    /// ArrayViewMutD<'_, f64> is a mutable NumPy float64 array.
    fn multiply(a: f64, mut x: ArrayViewMutD<'_, f64>) {
        x *= a;
    }

    /// This is a wrapper around the function `multiply`.
    /// This function receives the Python NumPy parameter as a PyReadwriteArrayDyn
    /// and casts it to a Rust-friendly type, which we then pass to the actual function.
    #[pyfn(m)]
    #[pyo3(name = "multiply")]
    fn mult_py<'py>(a: f64, mut x: PyReadwriteArrayDyn<'py, f64>) {
        let x = x.as_array_mut();
        multiply(a, x);
    }

    Ok(())
}
