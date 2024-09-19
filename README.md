//! # File Uploads Using Multi-Threads - Rust Crate
//! 
//! This crate demonstrates file uploads using Rust, multi-threading, and chunking the uploaded files into parts. The uploaded chunks are stored in a PostgreSQL database. The project is Dockerized for easy setup, and includes instructions for interacting with the API via Postman.
//! 
//! ## Repository
//! 
//! **GitHub Repository**:  
//! [https://github.com/enosuity/file_uploads_using_multi_threads.git](https://github.com/enosuity/file_uploads_using_multi_threads.git)
//! 
//! ## Prerequisites
//! 
//! - **Rust** (Nightly version recommended)
//! - **Docker** & **Docker Compose**
//! - **Postman** (or any API testing tool)
//! 
//! ## Getting Started
//! 
//! ### Step 1: Clone the Repository
//! 
//! Clone the repository to your local machine:
//! 
//! ```bash
//! git clone https://github.com/enosuity/file_uploads_using_multi_threads.git
//! cd file_uploads_using_multi_threads
//! ```
//! 
//! ### Step 2: Build and Run the Docker Containers
//! 
//! This project uses Docker Compose to set up the Rust API and PostgreSQL services. To build and start the services, run:
//! 
//! ```bash
//! docker-compose up --build
//! ```
//! 
//! This command will:
//! - Build the Rust API.
//! - Set up a PostgreSQL database instance.
//! - Expose the API on `http://localhost:5000`.
//! 
//! If everything is set up correctly, you should see the output indicating the Rust API is running on port 5000 and the PostgreSQL service on port 5432.
//! 
//! ### Step 3: Database Migration
//! 
//! Run the Diesel migrations to set up the required database schema:
//! 
//! ```bash
//! docker-compose exec api diesel migration run
//! ```
//! 
//! This will create the necessary tables in the PostgreSQL database.
//! 
//! ### Step 4: API Endpoints
//! 
//! The following API endpoints are available:
//! 
//! #### 1. Upload a File in Chunks
//! 
//! - **Method**: `POST`
//! - **URL**: `http://localhost:5000/upload`
//! - **Body**: Multipart form data containing the file under the key `file_data`
//! 
//! Use the following `curl` command to test file upload:
//! 
//! ```bash
//! curl -X POST http://localhost:5000/upload -F "file_data=@/path/to/your/file"
//! ```
//! 
//! In Postman:
//! - Set the request method to `POST`.
//! - URL: `http://localhost:5000/upload`
//! - Under the `Body` tab, select `form-data`, add a key `file_data`, and upload the file.
//! 
//! The response will contain a UUID representing the `file_id` for the uploaded file.
//! 
//! #### 2. Retrieve File by ID
//! 
//! - **Method**: `GET`
//! - **URL**: `http://localhost:5000/file/<file_id>`
//! 
//! This retrieves all chunks of the file by its UUID and reconstructs the file.
//! 
//! Use the following `curl` command:
//! 
//! ```bash
//! curl -X GET http://localhost:5000/file/<file_id>
//! ```
//! 
//! In Postman:
//! - Set the request method to `GET`.
//! - URL: `http://localhost:5000/file/<file_id>` (replace `<file_id>` with the actual UUID).
//! 
//! The response will contain the details of the uploaded file and its chunks.
//! 
//! ### Step 5: Run Tests
//! 
//! If you want to run the tests, use the following command:
//! 
//! ```bash
//! cargo test
//! ```
//! 
//! This will execute the unit tests defined in the project.
//! 
//! ## Docker Compose Overview
//! 
//! The project includes a `docker-compose.yml` file with two services:
//! 
//! - **API**: The Rust backend API, running on port 5000.
//! - **PostgreSQL**: The PostgreSQL database, running on port 5432.
//! 
//! The following environment variables are set for the API service:
//! - `DATABASE_URL`: The connection string for PostgreSQL.
//! 
//! ## Postman API Testing
//! 
//! 1. **Upload a File**:
//!     - Open Postman and create a new `POST` request.
//!     - URL: `http://localhost:5000/upload`
//!     - Go to the `Body` tab, select `form-data`, and add a key called `file_data`.
//!     - Select and upload the file you want to test with.
//!     - Click `Send`.
//! 
//! 2. **Retrieve the File by UUID**:
//!     - After uploading, note the `file_id` returned in the response.
//!     - Create a new `GET` request.
//!     - URL: `http://localhost:5000/file/<file_id>` (replace `<file_id>` with the actual UUID).
//!     - Click `Send` to retrieve the file details.
//! 
//! ## Troubleshooting
//! 
//! - Ensure Docker and Docker Compose are properly installed and running.
//! - Verify the API and database services are up by checking the logs.
//! - If you encounter database errors, try restarting the services using `docker-compose down` followed by `docker-compose up`.
//! 
//! ## License
//! 
//! This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
