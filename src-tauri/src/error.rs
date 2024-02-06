//use surrealdb;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    DbError(#[from] surrealdb::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Error::Generic(e) => serializer.serialize_str(e.as_ref()),
            Error::DbError(e) => serializer.serialize_str(e.to_string().as_ref()),
        }
    }
}
