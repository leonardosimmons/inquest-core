#![allow(unused)]
use rusqlite;
use std::rc::Rc;

// TEST
pub struct PersonTestTable {
    id: u32,
    name: String,
    data: String,
}

// Database Types
pub struct Error {
    pub code: rusqlite::ErrorCode,
    pub message: String,
}

pub struct Request<P> {
    table: String,
    query: String,
    kind: P,
}

pub struct Response<R> {
    pub data: R,
}

pub struct TableRow {
    pub key: String,
    pub attr: String,
}

pub struct NoParams;
pub struct WithParams<'a> {
    params: &'a [&'a dyn rusqlite::ToSql],
}

// Query Types
enum QueryType {
    Create,
    Insert,
    Update,
    Delete,
    Undefined,
}

// Controller States
pub struct UnInitialized;
pub struct Initialized;

pub struct Create;
pub struct Insert;
pub struct Update;
pub struct Delete;

pub struct Pending<P> {
    request: Request<P>,
}

pub struct Completed<R> {
    pub response: Result<Response<R>, Error>,
}

// Main Controller
pub struct Controller<S> {
    conn: Rc<rusqlite::Connection>,
    pub state: S,
}

// Database Traits
pub trait DatabaseCreateController {
    fn table(self, name: &str, contents: Vec<TableRow>) -> Controller<Pending<NoParams>>;
}

impl Controller<UnInitialized> {
    pub fn new(connection: Rc<rusqlite::Connection>) -> Controller<Initialized> {
        Controller {
            conn: connection,
            state: Initialized,
        }
    }
}

impl Controller<Initialized> {
    pub fn create(mut self) -> Controller<Create> {
        Controller {
            conn: self.conn,
            state: Create,
        }
    }
}

impl Controller<Pending<NoParams>> {
    pub fn execute(mut self) -> Controller<Completed<usize>> {
        match self.conn.execute(&*self.state.request.query, []) {
            Ok(res) => Controller {
                conn: self.conn,
                state: Completed {
                    response: Ok(Response { data: res }),
                },
            },
            Err(err) => Controller {
                conn: self.conn,
                state: Completed {
                    response: Err(Error {
                        code: rusqlite::ErrorCode::InternalMalfunction,
                        message: String::from("Failed to execute query"),
                    }),
                },
            },
        }
    }
}

impl DatabaseCreateController for Controller<Create> {
    fn table(mut self, name: &str, contents: Vec<TableRow>) -> Controller<Pending<NoParams>> {
        let mut buffer = String::from("");
        for d in contents {
            if buffer.len() == 0 {
                buffer = format!("{} {}", d.key.to_lowercase(), d.attr.to_uppercase());
            } else {
                buffer = format!(
                    "{}, {} {}",
                    buffer,
                    d.key.to_lowercase(),
                    d.attr.to_uppercase()
                );
            }
        }

        let fixed_name = name.to_lowercase();
        Controller {
            conn: self.conn,
            state: Pending {
                request: Request {
                    table: String::from(fixed_name.clone()),
                    query: format!("CREATE TABLE {} ({})", fixed_name.clone(), buffer),
                    kind: NoParams,
                },
            },
        }
    }
}
