
use pyo3::prelude::*;

use fossil_delta::{delta, deltainv};

#[pymodule]
fn fossil_delta(_py: Python, m:&PyModule) -> PyResult<()> {
    /// Calculates delta between two given strings a and b
    ///
    /// >>> d = fossil_delta.delta(a, b)
    /// >>> s = fossil_delta.deltainv(b, d)
    /// >>> assert s == a
    ///
    #[pyfn(m, "delta")]
    fn py_delta(_py: Python, a:&str, b:&str) -> PyResult<String> {
        let res = delta(a, b);
        Ok(res)
    }
    /// Applies delta tp given text and returns changed text
    ///
    /// >>> d = fossil_delta.delta(a, b)
    /// >>> s = fossil_delta.deltainv(b, d)
    /// >>> assert s == a
    ///
    #[pyfn(m, "deltainv")]
    fn py_deltainv(_py: Python, b:&str, d:&str) -> PyResult<String> {
        let res = deltainv(b, d);
        Ok(res)
    }
    Ok(())
}
