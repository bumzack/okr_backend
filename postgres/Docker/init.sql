-- CREATE USER  prod  WITH PASSWORD 'prod';
-- CREATE DATABASE prod;

-- GRANT ALL PRIVILEGES ON DATABASE prod TO prod;



CREATE TABLE images
(
    id                         serial PRIMARY KEY,
    filename                   VARCHAR(255) NOT NULL,
    image_as_rgb_png           TEXT         NOT NULL,
    image_as_json_pixels_array TEXT         NOT NULL,
    width                      INTEGER      NOT NULL,
    height                     INTEGER      NOT NULL
);

CREATE UNIQUE INDEX image_filename_idx ON images (filename);
CREATE UNIQUE INDEX image_id_idx ON images (id);

CREATE TABLE articles
(
    id          serial PRIMARY KEY,
    code        VARCHAR(255) NOT NULL,
    title       VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);


CREATE UNIQUE INDEX article_id_ix ON articles (id);
CREATE UNIQUE INDEX article_code_ix ON articles (code);


CREATE TABLE art2img
(
    id         serial  NOT NULL,
    article_id INTEGER NOT NULL,
    image_id   INTEGER NOT NULL,
    FOREIGN KEY (article_id)
        REFERENCES articles (id),
    FOREIGN KEY (image_id)
        REFERENCES images (id)
);


CREATE UNIQUE INDEX art2img_id_idx ON art2img (id);
CREATE UNIQUE INDEX art2img_article_id_idx ON art2img (article_id);
CREATE UNIQUE INDEX art2img_image_id_idx ON art2img (image_id);


CREATE TABLE resolutions
(
    id         serial       NOT NULL,
    resolution VARCHAR(255) NOT NULL
);

CREATE UNIQUE INDEX resolutions_id_idx ON resolutions (id);
CREATE UNIQUE INDEX resolutions_resolution_idx ON resolutions (resolution);

