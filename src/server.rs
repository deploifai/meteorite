use std::process::{abort};
use std::{thread};
use actix_http::{KeepAlive};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::web::{Bytes};
use env_logger::Env;

use crate::handlers::{PredictCallback, run_python_function};
use crate::middleware::{Middleware};

use pyo3::prelude::*;
use pyo3::types::{PyFunction};


#[pyclass]
pub struct Meteorite {
    predict_callback: Option<Py<PredictCallback>>,
    middlewares: Middleware
}

#[pymethods]
impl Meteorite {
    #[new]
    fn __init__() -> Self {
        Self {
            predict_callback: None,
            middlewares: Middleware::new()
        }
    }

    fn predict(&mut self, wraps: &PyFunction) -> PyResult<()> {
        Python::with_gil(|_py| {
            let inner_fn: Py<PredictCallback> = Py::new(_py, PredictCallback{
                wraps: Py::from(wraps)
            }).unwrap();
            self.predict_callback = Some(inner_fn);
            Ok(())
        })
    }

    fn middleware(&mut self, wraps: &PyFunction) {
        let wrapper_function = Py::from(wraps);
        self.middlewares.add_new_function(wrapper_function)
    }

    fn start(&self, py: Python, port: Option<u16>) -> PyResult<()> {
        env_logger::init_from_env(Env::default().default_filter_or("info"));

        let asyncio = py.import("asyncio")?;
        let event_loop = asyncio.call_method0("new_event_loop")?;

        let predict_callback_copy = self.predict_callback.clone().unwrap();
        let middleware_functions = self.middlewares.clone();

        asyncio.call_method1("set_event_loop", (event_loop,))?;

        thread::spawn(move || {
            actix_web::rt::System::new().block_on(async move {
                let predict_callback = predict_callback_copy.clone();
                let middleware_functions = middleware_functions.clone();
                HttpServer::new(move || {
                    let predict_callback = predict_callback.clone();
                    let middleware_functions = middleware_functions.clone();
                    let app = App::new();
                    app.route("/predict",
                              web::route().to(move |_req: HttpRequest, body: Bytes| {
                                  let mut middleware_functions = middleware_functions.clone();
                                  middleware_functions.run_middleware_functions();
                                  handle(predict_callback.clone(), body)
                              })
                             )
                        .wrap(Logger::default())
                        .wrap(Logger::new("%a %{User-Agent}i"))
                        .default_service(web::get().to(HttpResponse::Ok))
                })
                .workers(1)
                .keep_alive(KeepAlive::Os)
                .bind(("0.0.0.0", port.unwrap_or(4000)))
                .unwrap()
                .run()
                .await
                .unwrap();
            });
        });

        let event_loop = event_loop.call_method0("run_forever");
        if event_loop.is_err() {
            abort();
        }

        Ok(())
    }
}

async fn handle(predict_callback: Py<PredictCallback>, body: Bytes) -> HttpResponse {
    let mut response_builder = HttpResponse::Ok();
    match run_python_function(predict_callback, body).await {
        Ok(output) => {
            let headers = output.headers;
            for header in headers.keys() {
                let key = header.clone();
                let value = headers.get(header).unwrap().clone();
                response_builder.insert_header((key, value));
            }
            response_builder.body(output.body)
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}
