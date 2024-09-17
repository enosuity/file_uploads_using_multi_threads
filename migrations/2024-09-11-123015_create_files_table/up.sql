-- Your SQL goes here
CREATE TABLE files (
    fileid UUID NOT NULL,       -- Stores the UUID of the file
    chunk BYTEA NOT NULL,       -- Stores the binary chunk data
    PRIMARY KEY (fileid)        -- Composite primary key ensures uniqueness
);

