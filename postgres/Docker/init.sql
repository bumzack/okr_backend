-- CREATE USER  shop  WITH PASSWORD 'shop';
-- CREATE DATABASE shop;

-- GRANT ALL PRIVILEGES ON DATABASE shop TO shop;


CREATE TABLE articles
(
    id          bigserial                NOT NULL,
    code        VARCHAR(12)              NOT NULL,
    title       TEXT,
    description TEXT                     NOT NULL,
    categories  TEXT                     NOT NULL,
    attributes  TEXT                     NOT NULL,
    start_date  TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date    TIMESTAMP WITH TIME ZONE NOT NULL,
    pos         VARCHAR(12)              NOT NULL,
    price       DOUBLE PRECISION         NOT NULL
);

ALTER TABLE public.articles OWNER TO shop;

