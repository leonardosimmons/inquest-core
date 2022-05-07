use crate::logging::SYSTEM;
use crate::service::Response;
use std::fmt::{Debug, Display};
use tower::{Service, ServiceExt};
use tracing::{event, Level};

pub struct System;

impl System {
    pub async fn run<App, Req, B>(mut app: App, req: Req)
    where
        App: Service<Req, Response = Response<B>>,
        B: Debug,
        App::Error: Debug + Display,
        App::Future: Send + 'static
    {
        loop {
            let app = match app.ready().await {
                Err(err) => {
                    event!(target: SYSTEM, Level::WARN, "system is busy; {}", err);
                    continue;
                }
                Ok(app) => app,
            };

            event!(target: SYSTEM, Level::DEBUG, "received new request");
            let fut = app.call(req);

            let handle = tokio::spawn(async move {
                match fut.await {
                    Ok(res) => event!(target: SYSTEM, Level::INFO, "{:?}", res),
                    Err(err) => event!(target: SYSTEM, Level::ERROR, "error: {}", err),
                }
            });

            match handle.await {
                Ok(_) => {
                    event!(target: SYSTEM, Level::DEBUG, "request complete");
                    break;
                }
                Err(err) => {
                    event!(
                        target: SYSTEM,
                        Level::ERROR,
                        "error processing request; {:?}",
                        err
                    );
                    break;
                }
            }
        }
        event!(target: SYSTEM, Level::DEBUG, "shutdown");
    }
}
