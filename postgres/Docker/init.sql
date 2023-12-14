-- CREATE USER  testinger;
--
-- CREATE DATABASE     testinger;
-- -- CREATE SCHEMA       dev;
-- GRANT ALL PRIVILEGES ON DATABASE testinger TO testinger;

--grant usage on schema public to public;
--grant create on schema public to public;


CREATE TABLE images (
    id  serial PRIMARY KEY,
    filename  VARCHAR(255) NOT NULL,
    image TEXT NOT NULL
);

CREATE TABLE  articles (
    id  serial PRIMARY KEY,
    article_code  VARCHAR(255) NOT NULL,
    title  VARCHAR(255) NOT NULL,
    description  VARCHAR(255) NOT NULL
);

CREATE TABLE  art2img (
    article_id  INTEGER  NOT NULL,
    image_id  INTEGER  NOT NULL,
    PRIMARY KEY (article_id, image_id),
    FOREIGN KEY (article_id)
        REFERENCES articles (id),
    FOREIGN KEY (image_id)
        REFERENCES images (id)
);

