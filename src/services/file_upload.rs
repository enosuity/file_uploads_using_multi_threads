use base64::encode;
use diesel::prelude::*;
use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
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

    let chunk_size = 512 * 1024; // 0.5 MB chunks
    let file_id = MyUuid::new_v4();

    // Split the file content into chunks
    let chunks: Vec<Vec<u8>> = file_content
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())  // Convert each &[u8] into Vec<u8>
        .collect();                   // Collect into Vec<Vec<u8>>

    let connection = Arc::new(Mutex::new(establish_connection_pg()));
    let mut handles = vec![];

    // Multi-threaded processing: Save each chunk in parallel
    for (i, chunk) in chunks.into_iter().enumerate() {
        let connection = Arc::clone(&connection);
        let file_id = file_id.clone();

        let handle = thread::spawn(move || {
            let new_chunk = FileUpload {
                fileid: file_id,
                chunk_index: (i as i32),
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

// #[get("/file/<f_id>")]
// pub fn get_file(f_id: MyUuid) -> Template {
//   use self::schema::files::dsl::*;

//   let connection = &mut establish_connection_pg();  
//   let results: Vec<FileUpload> = files
//     .filter(fileid.eq(f_id))
//     .select((fileid, chunk, chunk_index))
//     .load::<FileUpload>(connection)   
//     .expect("Error loading file chunks");

//   Template::render("files", context! {files: &results, count: results.len()})
// }

#[get("/file/<f_id>")]
pub fn get_file(f_id: MyUuid) -> Template {
    use self::schema::files::dsl::*;
    let connection = &mut establish_connection_pg();
    
    // Fetch all the chunks associated with the file ID
    let results: Vec<FileUpload> = files
        .filter(fileid.eq(f_id))
        .order(chunk_index.asc()) // Ensure the chunks are ordered correctly
        .load::<FileUpload>(connection)
        .expect("Error loading file chunks");

    // Merge the chunks into a single file (Vec<u8>)
    let mut full_file: Vec<u8> = Vec::new();
    let record_total = results.len();
    for chk in results {
        full_file.extend(chk.chunk);  // Append each chunk to the full file
    }

    let base64_image = encode(&full_file);

     // Create a data URI for the image
     let mime_type = "image/jpg";  // Adjust according to the image type (e.g., "image/jpeg")
     let image_data_uri = format!("data:{};base64,{}", mime_type, base64_image);

    // // Convert the full file data to a string (or handle it as binary if needed)
    // let file_content_as_string = String::from_utf8_lossy(&full_file).to_string();
    // println!("file_content_as_string ===> {:?}", file_content_as_string);

    // // Render the template with the full file content (as string or binary data)
    let context_data = context! {
        fileid: f_id.to_string(),
        chunk: image_data_uri,
        chunk_count: record_total
    };

    // Ok(Custom(Status::Ok, Template::render("files", context_data)))
    Template::render("files", context_data)
}