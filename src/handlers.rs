use std::collections::HashMap;
use actix_web::web::{Bytes};
use pyo3::{Py, PyAny, Python};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use serde_json::{Map, Value};

#[derive(Clone)]
#[pyclass]
pub struct Response {
    pub body: Bytes,
    pub headers: HashMap<String, String>
}

#[pymethods]
impl Response {
    #[new]
    pub fn new(body: &PyAny) -> Self {
        // TODO: Being ambitious in the future to return streams?
        let mut headers = HashMap::new();
        if let Ok(value) = body.downcast::<PyString>() {
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
            Self {
                body: Bytes::from(value.to_string()),
                headers
            }
        } else if let Ok(value) = body.downcast::<PyDict>() {
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            let keys: Vec<String> = value.keys().extract().unwrap();
            let mut body = Map::new();
            for key in keys {
                let value = value.get_item(key.clone()).unwrap().to_string();
                body.insert(key.clone(), Value::String(value));
            }
            let final_data = Value::Object(body);
            Self {
                body: Bytes::from(final_data.to_string()),
                headers
            }
        } else {
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
            Self {
                body: Bytes::from(""),
                headers
            }
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct PredictCallback {
    pub wraps: Py<PyAny>
}

#[pymethods]
impl PredictCallback {
    fn __call__<'a>(&'a self, py: Python<'a>, data: PyObject) -> PyResult<&'a PyAny> {
        self.wraps.as_ref(py).call1((data,))
    }
}

pub async fn run_python_function(func_: Py<PredictCallback>, body: Bytes) -> Result<Response, PyErr> {
    let bytes = body.as_ref();
    Python::with_gil(|py| -> Result<Response, PyErr> {
        match func_.call1(py, (bytes, )) {
            Ok(retval) => Ok(Response::new(retval.as_ref(py))),
            Err(err) => {
                println!("{}\n{}", err.traceback(py).expect("").format().unwrap(), err.to_string());
                Err(err)
            }
        }
    })
}
