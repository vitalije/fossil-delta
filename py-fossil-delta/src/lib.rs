use fossil_delta::{delta, deltainv};
use pyo3::prelude::*;

#[pymodule]
fn fossil_delta(_py: Python, m: &PyModule) -> PyResult<()> {
    /// Calculates delta between two given strings a and b
    ///
    /// >>> d = fossil_delta.delta(a, b)
    /// >>> s = fossil_delta.deltainv(b, d)
    /// >>> assert s == a
    ///
    #[pyfn(m, "delta")]
    fn py_delta(py: Python, a: &str, b: &str) -> PyResult<PyObject> {
        let res = delta(a, b);
        Ok(res.to_object(py))
    }
    /// Applies delta tp given text and returns changed text
    ///
    /// >>> d = fossil_delta.delta(a, b)
    /// >>> s = fossil_delta.deltainv(b, d)
    /// >>> assert s == a
    ///
    #[pyfn(m, "deltainv")]
    fn py_deltainv(py: Python, b: &str, d: PyObject) -> PyResult<String> {
        let dv: Vec<u8> = d.extract(py).unwrap();
        String::from_utf8(deltainv(b, &dv))
            .map_err(|e| e.into())
    }
    Ok(())
}
