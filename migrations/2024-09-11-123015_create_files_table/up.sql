-- Your SQL goes here
CREATE TABLE files (
    fileid UUID NOT NULL,          -- Stores the UUID of the file
    chunk BYTEA NOT NULL,          -- Stores the binary chunk data
    chunk_index INT NOT NULL,      -- Stores the index of the chunk
    PRIMARY KEY (fileid, chunk_index) -- Composite primary key ensures uniqueness
);

