#![allow(unused)]
use crate::cli::Cli;
use crate::logging::{APP, SYSTEM};
use std::fmt::{Debug, Display};
use hyper::Body;
use tower::{Service, ServiceExt};
use tracing::Level;
use tracing::{event, span};
use crate::service::{Request, Response};

pub struct Initialized;

pub struct System<App, S> {
    app: App,
    state: S,
}

impl<App, State> System<App, State>
{
    pub fn init<S, B>(app: App) -> System<App, Initialized>
        where
            App: Service<S, Response = Response<S, B>>,
            B: Send + 'static,
            App::Error: Debug + Display,
            App::Future: Send + 'static + Debug,
    {
        let span = span!(Level::TRACE, APP);
        let _enter = span.enter();
        event!(target: SYSTEM, Level::DEBUG, "application initialized");
        System {
            app,
            state: Initialized,
        }
    }
}

impl<App, State> System<App, State>
{
    pub async fn run<Req, Res>(mut self, middleware: Req)
    where
        App: Service<Req, Response = Res>,
        Res: Debug,
        App::Error: Debug + Display,
        App::Future: Send + 'static + Debug
    {
        loop {
            let app = match self.app.ready().await {
                Err(err) => {
                    event!(target: SYSTEM, Level::WARN, "system is busy");
                    continue;
                }
                Ok(app) => app,
            };

            let fut = app.call(middleware);
            event!(target: SYSTEM, Level::DEBUG, "received new task");

            let handle = tokio::spawn(async move {
                match fut.await {
                    Ok(res) => event!(target: SYSTEM, Level::INFO, "{:?}", res),
                    Err(err) => event!(target: SYSTEM, Level::ERROR, "error: {}", err),
                }
            });

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
