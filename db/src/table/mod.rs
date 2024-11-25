use surrealdb::{Datastore, Session};

pub struct SurrealTable<'a, T> {
    /// The name of the table in SurrealDB
    name: String,
    /// Reference to the [Datastore]() where this table is found
    ds: &'a Datastore,
    /// Reference to the [Session]() where this table is found
    session: &'a Session,

    /**
     * The underlying struct which this table will interact with
     */
    shape: &'a T,
}

impl TryFrom<T> for SurrealTable {
    type Error;

    fn try_from<T>(value: T) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl SurrealTable {
    pub fn new(name: &str, db: (&Datastore, &Session), shape: Option<&Struct>) {
        return self { name };
    }

    pub async fn add(&self, props: &BTreeMap<String, Value>) {
        //
    }
    pub async fn add_with_id(&self, props: &BTreeMap<String, Value>) {
        //
    }
    pub async fn delete(&self, id: &str) {
        //
    }

    pub async fn update(&self, id: &str, props: &BTreeMap<String, Value>) {
        //
    }

    pub async fn list(&self) {
        //
    }

    pub async fn list_where(&self, conditions: &BTreeMap<String, Value>) {
        //
    }
}
