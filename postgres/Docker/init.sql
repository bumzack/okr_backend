-- CREATE USER  shop  WITH PASSWORD 'shop';
-- CREATE DATABASE shop;

-- GRANT ALL PRIVILEGES ON DATABASE shop TO shop;


CREATE ROLE shop WITH LOGIN PASSWORD 'shop';
CREATE DATABASE shop WITH OWNER shop;
\c shop shop;


GRANT ALL
ON ALL TABLES
IN SCHEMA "public"
TO shop;

psql -d shop -U shop


CREATE TABLE articles
(
    id          serial   NOT NULL,
    code        VARCHAR(12)    NOT NULL,
    title       TEXT,
    description TEXT           NOT NULL,
    categories  TEXT           NOT NULL,
    attributes  TEXT           NOT NULL,
    start_data  TIMESTAMP      NOT NULL,
    end_data    TIMESTAMP      NOT NULL,
    pos         VARCHAR(12)    NOT NULL,
    price       NUMERIC(14, 4) NOT NULL,
    PRIMARY KEY (id)
);

CREATE UNIQUE INDEX articles_id_idx ON articles (id);
CREATE UNIQUE INDEX articles_pos_code_idx ON articles (code, pos);



ALTER SEQUENCE public.articles_seq
    OWNER TO shop;
