-- CREATE USER  prod  WITH PASSWORD 'prod';
-- CREATE DATABASE prod;

-- GRANT ALL PRIVILEGES ON DATABASE prod TO prod;

CREATE TABLE wiki_page
(
    dbid        serial PRIMARY KEY,
    id          INTEGER,
    title       TEXT,
    ns          VARCHAR(255),
    redirect    TEXT,
    timestamp   VARCHAR(255),
    contributor TEXT,
    comment     TEXT,
    model       TEXT,
    format      TEXT,
    "text"      TEXT,
    sha1        VARCHAR(255)
);
