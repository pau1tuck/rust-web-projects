CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,
    title varchar(256) NOT NULL,
    author varchar(256) NOT NULL,
    content text,
    published BOOLEAN NOT NULL DEFAULT 'f'
);