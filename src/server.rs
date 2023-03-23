use std::process::{abort};
use std::{env, thread};
use actix_http::{KeepAlive};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web::web::{Bytes};
use crate::handlers::{PredictCallback, run_python_function};
use pyo3::prelude::*;
use pyo3::types::{PyFunction};

#[pyclass]
pub struct Meteorite {
    predict_callback: Option<Py<PredictCallback>>,
}

#[pymethods]
impl Meteorite {
    #[new]
    fn __init__() -> Self {
        Self {
            predict_callback: None,
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

    fn start(&self, py: Python) -> PyResult<()> {
        env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
        env_logger::init();

        let asyncio = py.import("asyncio")?;
        let event_loop = asyncio.call_method0("new_event_loop")?;

        let predict_callback_copy = self.predict_callback.clone().unwrap();

        asyncio.call_method1("set_event_loop", (event_loop,))?;

        thread::spawn(move || {
            actix_web::rt::System::new().block_on(async move {
                let predict_callback = predict_callback_copy.clone();
                HttpServer::new(move || {
                    let predict_callback = predict_callback.clone();
                    let app = App::new();
                    app.route("/predict",
                              web::route().to(move |_req: HttpRequest, body: Bytes| {
                                  handle(predict_callback.clone(), body)
                              })
                             )
                        .wrap(Logger::default())
                        .default_service(web::get().to(HttpResponse::Ok))
                })
                .workers(1)
                .keep_alive(KeepAlive::Os)
                .bind(("0.0.0.0", 4000)).unwrap()
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
