package at.bumzack.reference.impl.dto;

import java.util.List;


public class FullArticle {
    private long id;
    private String code;
    private String title;
    private String description;
    private List<FullImage> images;

    public FullArticle() {
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

    public String getTitle() {
        return title;
    }

    public void setTitle(final String title) {
        this.title = title;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(final String description) {
        this.description = description;
    }


    public List<FullImage> getImages() {
        return images;
    }

    public void setImages(final List<FullImage> images) {
        this.images = images;
    }
}
