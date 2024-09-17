use diesel::prelude::*;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket::{get, post};
use tokio::io::AsyncReadExt;
use crate::models::file_upload::FileUpload;
use crate::schema;
use super::establish_connection_pg;
use uuid::Uuid as MyUuid;
use rocket_dyn_templates::{context, Template};
use std::sync::{Arc, Mutex};
use std::thread;
use rocket::form::Form;
use rocket::fs::TempFile;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(FromForm)]
pub struct FileData<'r> {
    file_data: TempFile<'r>,
}

#[post("/upload", data = "<file>")]
pub async fn upload_file(file: Form<FileData<'_>>) -> Result<Json<MyUuid>> {
    let mut file_content = Vec::new();
    
    // Read the file data into file_content (Vec<u8>)
    file.file_data.open().await.unwrap().read_to_end(&mut file_content).await.unwrap();

    let chunk_size = 1024 * 1024; // 1 MB chunks
    let file_id = MyUuid::new_v4();

    // Split the file content into chunks
    let chunks: Vec<Vec<u8>> = file_content
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())  // Convert each &[u8] into Vec<u8>
        .collect();                   // Collect into Vec<Vec<u8>>

    let connection = Arc::new(Mutex::new(establish_connection_pg()));
    let mut handles = vec![];

    // Multi-threaded processing: Save each chunk in parallel
    for chunk in chunks {
        let connection = Arc::clone(&connection);
        let file_id = file_id.clone();

        let handle = thread::spawn(move || {
            let new_chunk = FileUpload {
                fileid: file_id,
                chunk,
            };

            let conn = &mut *connection.lock().unwrap();
            diesel::insert_into(schema::files::dsl::files)
                .values(new_chunk)
                .execute(conn)
                .expect("Error saving file chunk");
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Ok(Json(file_id))
}

#[get("/file/<f_id>")]
pub fn get_file(f_id: MyUuid) -> Template {
  use self::schema::files::dsl::*;

  let mut connection = &mut establish_connection_pg();
  
  let results: Vec<FileUpload> = files
    .filter(fileid.eq(f_id))
    .select((fileid, chunk))
    .load::<FileUpload>(connection)    
    
    .expect("Error loading file chunks");

  Template::render("files", context! {files: &results, count: results.len()})
}
