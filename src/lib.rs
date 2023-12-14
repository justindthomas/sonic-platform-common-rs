use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

pub fn poc() {
    let x = Python::with_gil(|py| {
        let platform = PyModule::import(py, "sonic_platform.platform").unwrap();
        let psus: &PyList = platform
            .call_method0("Platform").unwrap()
            .call_method0("get_chassis").unwrap()
            .call_method0("get_all_psus").unwrap()
            .extract::<&PyList>().unwrap();

        println!("{psus:#?}");
    });
}

