use std::thread;
use pyo3::create_exception;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyException;
create_exception!(exogress, ExogressError, PyException);

fn extract_key(kwds: &PyDict, key: &str) -> Result<String, pyo3::PyErr> {
    Ok(
        kwds
            .get_item(key)
            .ok_or_else(|| ExogressError::new_err(format!("no {} supplied", key)))?
            .extract()
            .map_err(|e| ExogressError::new_err(format!("bad {} supplied: {}", key, e)))?)
}

#[pyfunction(kwds = "**")]
fn spawn(py: Python, kwds: Option<&PyDict>) -> PyResult<()> {
    let kwds = kwds.ok_or_else(|| ExogressError::new_err("no credentials supplied"))?;

    let access_key_id: String = extract_key(&kwds, "access_key_id")?;
    let secret_access_key: String = extract_key(&kwds, "secret_access_key")?;
    let account: String = extract_key(&kwds, "account")?;
    let project: String = extract_key(&kwds, "project")?;

    py.allow_threads(move || {
        thread::spawn(move || {
            if let Err(e) = exogress_client_lib::spawn(access_key_id, secret_access_key, account, project) {
                println!("error spawning exogress: {}", e);
            }
        })
    });

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn exogress(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(spawn))?;
    m.add("ExogressError", py.get_type::<ExogressError>())?;

    Ok(())
}
