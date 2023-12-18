-- CREATE USER  prod  WITH PASSWORD 'prod';
-- CREATE DATABASE prod;

-- GRANT ALL PRIVILEGES ON DATABASE prod TO prod;


CREATE TABLE images
(
    id       serial PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    image    TEXT         NOT NULL
);

CREATE TABLE articles
(
    id          serial PRIMARY KEY,
    code        VARCHAR(255) NOT NULL,
    title       VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL
);

CREATE TABLE art2img
(
    id         serial  NOT NULL,
    article_id INTEGER NOT NULL,
    image_id   INTEGER NOT NULL,
    PRIMARY KEY (article_id, image_id),
    FOREIGN KEY (article_id)
        REFERENCES articles (id),
    FOREIGN KEY (image_id)
        REFERENCES images (id)
);

CREATE TABLE resolutions
(
    id         serial       NOT NULL,
    resolution VARCHAR(255) NOT NULL
);

