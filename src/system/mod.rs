use crate::logging::SYSTEM;
use crate::service::{IntoRequest, IntoResponse, Request, Response};
use std::fmt::{Debug, Display};
use tower::{Service, ServiceExt};
use tracing::{event, Level};

/// Runs and executes services
pub struct System<App> {
    app: App
}

impl<App> System<App> {
    /// Binds service to the current application executor
    pub fn bind<Req, Res>(app: App) -> Self
    where
        App: Service<Req, Response = Res>,
        Res: Debug,
        App::Error: Debug + Display,
        App::Future: Send + 'static
    {
        Self { app }
    }

    /// Runs specified request through current service
    pub async fn run<T, B>(mut self, request: T)
    where
        App: Service<Request<T>, Response = Response<B>>,
        T: IntoRequest<T>,
        B: IntoResponse<B> + Debug,
        App::Error: Debug + Display,
        App::Future: Send + 'static
    {
        loop {
            let app = match self.app.ready().await {
                Err(_err) => {
                    event!(target: SYSTEM, Level::WARN, "system is busy...");
                    continue;
                }
                Ok(app) => app,
            };

            event!(target: SYSTEM, Level::DEBUG, "received new request");
            let fut = app.call(request.into_request());

            let handle = tokio::spawn(async move {
                event!(target: SYSTEM, Level::DEBUG, "processing request...");
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
