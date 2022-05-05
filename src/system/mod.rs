use crate::cli::Cli;
use crate::logging::{APP, SYSTEM};
use crate::service::Response;
use std::fmt::{Debug, Display};
use tower::{Service, ServiceExt};
use tracing::Level;
use tracing::{event, span};

pub struct Initialized<App> {
    app: App,
}

pub struct System<State> {
    state: State,
}

impl<App> System<Initialized<App>> {
    pub fn init<S, B>(app: App) -> System<Initialized<App>>
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
            state: Initialized { app },
        }
    }
}

impl System<Initialized<Cli>> {
    pub async fn run(self) {
        loop {
            let mut app = self.state.app.clone();
            let app = match app.ready().await {
                Err(err) => {
                    event!(target: SYSTEM, Level::WARN, "system is busy; {}", err);
                    continue;
                }
                Ok(app) => app,
            };

            event!(target: SYSTEM, Level::DEBUG, "received new request");
            let fut = app.call(app.clone());

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
