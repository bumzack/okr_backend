package at.bumzack.reference.impl.dto;

import jakarta.persistence.Column;

public class ArticleAndImageModel {
    private long id;

    private String code;

    private String description;

    private String title;
    private String filename;

    @Column(name = "image_as_rgb_png")
    private String imagePng;

    @Column(name = "image_as_json_pixels_array")
    private String imageJson;

    private Integer width;
    private Integer height;


    public ArticleAndImageModel() {
    }

    public long getId() {
        return id;
    }

    public void setId(final long id) {
        this.id = id;
    }

    public String getCode() {
        return code;
    }

    public void setCode(final String code) {
        this.code = code;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(final String description) {
        this.description = description;
    }

    public String getTitle() {
        return title;
    }

    public void setTitle(final String title) {
        this.title = title;
    }

    public String getFilename() {
        return filename;
    }

    public void setFilename(final String filename) {
        this.filename = filename;
    }

    public String getImagePng() {
        return imagePng;
    }

    public void setImagePng(final String imagePng) {
        this.imagePng = imagePng;
    }

    public String getImageJson() {
        return imageJson;
    }

    public void setImageJson(final String imageJson) {
        this.imageJson = imageJson;
    }

    public Integer getWidth() {
        return width;
    }

    public void setWidth(final Integer width) {
        this.width = width;
    }

    public Integer getHeight() {
        return height;
    }

    public void setHeight(final Integer height) {
        this.height = height;
    }


}
