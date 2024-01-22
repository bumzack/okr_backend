-- CREATE USER  shop  WITH PASSWORD 'shop';
-- CREATE DATABASE shop;

-- GRANT ALL PRIVILEGES ON DATABASE shop TO shop;


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

ALTER TABLE public.articles OWNER TO shop;

