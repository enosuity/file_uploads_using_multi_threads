use crate::schema::*;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = files)]
pub struct FileUpload {
    pub fileid: Uuid,
    pub chunk: Vec<u8>,
}