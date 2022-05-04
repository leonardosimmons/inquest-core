#![allow(unused)]
use crate::logging::{APP, SYSTEM};
use std::fmt::{Debug, Display};
use tower::{Service, ServiceExt};
use tracing::{event, span};
use tracing::Level;
use crate::cli::Cli;

pub trait IntoRequest {
    fn into_request<Req>(self) -> Request<Req>;
}
pub trait IntoResponse{
    fn into_request<Req>(self) -> Response<Req>;
}

struct Request<Req> {
    inner: Req
}

struct Response<Req> {
    inner: Req
}

pub struct Initialized;

pub struct System<App, S> {
    app: App,
    state: S
}

impl<App, State> System<App, State>
where
    App: Service<Request<Req>, Response = impl IntoResponse>,
    App::Error: Debug + Display,
    App::Future: Send + 'static + Debug,
{
    pub fn init(app: App) -> System<App, Initialized>
    {
        let span = span!(Level::TRACE, APP);
        let _enter = span.enter();
        event!(target: SYSTEM, Level::DEBUG, "application initialized");
        System {
            app,
            state: Initialized
        }
    }
}

impl System<Cli, Initialized> {
    pub async fn run(mut self)
    {
        loop {
            let app = match self.app.ready().await {
                Err(err) => {
                    event!(target: SYSTEM, Level::WARN, "system is busy: {}", err);
                    continue;
                }
                Ok(app) => app
            };

            let fut = app.call(self.app);
            event!(target: SYSTEM, Level::DEBUG, "received new task");

            let handle = tokio::spawn(async move {
                match fut.await {
                    Ok(res) => event!(target: SYSTEM, Level::INFO, "{:?}", res),
                    Err(err) => event!(target: SYSTEM, Level::ERROR, "error: {}", err),
                }
            });

            // TEMP
            if let Ok(_) = handle.await {
                event!(target: SYSTEM, Level::DEBUG, "request complete");
                break;
            } else {
                event!(target: SYSTEM, Level::ERROR, "something went wrong");
                break;
            }
        }
        event!(target: SYSTEM, Level::DEBUG, "shutdown");
    }
}

