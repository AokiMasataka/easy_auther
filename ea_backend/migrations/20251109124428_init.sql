-- Add migration script here
CREATE TABLE managers (
    id UUID NOT NULL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    email VARCHAR(128) NOT NULL,
    pass VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);


CREATE TABLE users (
    id UUID NOT NULL PRIMARY KEY,
    name VARCHAR(128) NOT NULL,
    email VARCHAR(128) NOT NULL,
    pass VARCHAR(128) NOT NULL,
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);


CREATE TABLE authorize_codes (
    authorize_code VARCHAR(256) NOT NULL PRIMARY KEY,
    code_challenge VARCHAR(256) NOT NULL,
    user_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_your_table_modtime
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE update_timestamp()
