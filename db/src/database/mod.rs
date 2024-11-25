use surrealdb::{Datastore, Session};

pub struct Database {
    pub ds: Datastore,
    pub session: Session,
}

/// Converts a string into the filename/path for the database
/// file
impl From<&str> for Database {
    fn from(value: &str) -> Self {
        println!("file database: {value}");
        return Self {
            ds: Datastore::new(value),
            session: Session::for_db(("default", "default"), db),
        };
    }
}

impl From<(&str, &str)> for Database {
    fn from(value: (&str, &str)) -> Self {
        println!("file database: {}. With namespace {}", value.0, value.1)
    }
}
