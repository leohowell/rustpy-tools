extern crate regex;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use regex::Regex;

#[pyclass]
struct RegexUtil {}

#[pymethods]
impl RegexUtil {
    #[staticmethod]
    fn is_match(regex: &str, content: &str) -> bool {
        let re = Regex::new(regex).unwrap();
        re.is_match(content)
    }
}

#[pyfunction]
fn is_match(regex: &str, content: &str) -> bool {
    let re = Regex::new(regex).unwrap();
    re.is_match(content)
}


#[pymodule]
fn pyo3_regex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RegexUtil>()?;
    m.add_wrapped(wrap_pyfunction!(is_match))?;
    m.add("version", "0.1.0")?;

    Ok(())
}
