use std::collections::HashMap;
use pyo3::{Py, PyAny, PyObject, PyResult, Python};


#[derive(Clone)]
pub struct Middleware {
    middleware_functions: HashMap<String, Py<PyAny>>
}

impl Middleware {
    pub fn new() -> Self {
        Self {
            middleware_functions: HashMap::new()
        }
    }

    pub fn add_new_function(&mut self, py_function: Py<PyAny>) {
        let unique_id = cuid2::cuid();
        self.middleware_functions.insert(unique_id, py_function);
    }

    pub fn run_middleware_functions(&mut self) {
        for (_key, py_function) in self.middleware_functions.clone().into_iter() {
            Python::with_gil(|py| -> PyResult<PyObject> {
                py_function.call0(py)
            }).expect("Could not run the middleware");
        }
    }
}
