CREATE TABLE groups (
    id UUID NOT NULL PRIMARY KEY,
    name VARCHAR(128) UNIQUE,
    pass VARCHAR(128),
    private_key VARCHAR(2048),
    public_key VARCHAR(2048),
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
);

CREATE TABLE users (
    id UUID NOT NULL PRIMARY KEY,
    group_id UUID REFERENCES groups (id) ON DELETE CASCADE,
    name VARCHAR(128),
    pass VARCHAR(128),
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    CONSTRAINT unique_name_in_group UNIQUE (group_id, name)
);


CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';


CREATE TRIGGER update_your_table_modtime
BEFORE UPDATE ON groups
FOR EACH ROW
EXECUTE PROCEDURE update_timestamp();
