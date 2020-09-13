CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "image"(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    file_name text NOT NULL,
    file_size integer NOT NULL,
    image bytea NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);