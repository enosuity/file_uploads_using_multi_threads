# File Uploads Using Multi-Threads - Rust Crate

This project demonstrates file uploads using Rust, multi-threading, and chunking the uploaded files into parts. The uploaded chunks are stored in a PostgreSQL database. The project is Dockerized for easy setup, and includes instructions for interacting with the API via Postman.

## Repository

**GitHub Repository**:  
[https://github.com/enosuity/file_uploads_using_multi_threads.git](https://github.com/enosuity/file_uploads_using_multi_threads.git)

## Prerequisites

- **Rust** (Nightly version recommended)
- **Docker** & **Docker Compose**
- **Postman** (or any API testing tool)

## Getting Started

### Step 1: Clone the Repository

Clone the repository to your local machine:

```bash
git clone https://github.com/enosuity/file_uploads_using_multi_threads.git
cd file_uploads_using_multi_threads

```

### Step 2: Build and Run the Docker Containers

This project uses Docker Compose to set up the Rust API and PostgreSQL services. To build and start the services, follow these steps:

1. **Build and Start the Containers**

   Run the following command in your terminal:

   ```bash
   docker-compose up --build
