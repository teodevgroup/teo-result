use indexmap::IndexMap;
use pyo3::{PyErr, import_exception, Python, IntoPy, PyObject};
use pyo3::types::{PyDict, PyType};
use crate::Error;

import_exception!(teo, TeoException);

impl From<PyErr> for Error {
    fn from(value: PyErr) -> Self {
        let result: Result<Error, Error> = Python::with_gil(|py| {
            if value.get_type(py).is(PyType::new::<TeoException>(py)) {
                let py_object: PyObject = value.clone_ref(py).into_py(py);
                let code: u16 = py_object.getattr(py, "code")?.extract(py)?;
                let message: String = py_object.getattr(py, "message")?.extract(py)?;
                let errors_py = py_object.getattr(py, "errors")?;
                let errors = if errors_py.is_none(py) {
                    None
                } else {
                    let dict: &PyDict = errors_py.extract(py)?;
                    let mut map_result: IndexMap<String, String> = IndexMap::new();
                    for (k, v) in dict.iter() {
                        let k_string: String = k.extract()?;
                        let v_string: String = v.extract()?;
                        map_result.insert(k_string, v_string);
                    }
                    Some(map_result)
                };
                let mut teo_error = if let Some(errors) = errors {
                    Error::new_with_code_errors(message, code, errors)
                } else {
                    Error::new_with_code(message, code)
                };
                teo_error.assign_platform_native_object(value);
                Err(teo_error)
            } else {
                let mut error = Error::new(value.to_string());
                error.assign_platform_native_object(value);
                Err(error)
            }
        });
        result.unwrap_or_else(|e| e)
    }
}

