package at.bumzack.reference.impl.dto;

public class Image {

    private Long id;
    private String filename;
    private String image;
    private String resolution;
    private Integer width;
    private Integer height;

    public Image() {
    }

    public Long getId() {
        return id;
    }

    public void setId(final Long id) {
        this.id = id;
    }

    public String getFilename() {
        return filename;
    }

    public void setFilename(final String filename) {
        this.filename = filename;
    }

    public String getImage() {
        return image;
    }

    public void setImage(final String image) {
        this.image = image;
    }

    public String getResolution() {
        return resolution;
    }

    public void setResolution(final String resolution) {
        this.resolution = resolution;
    }

    public Integer getWidth() {
        return width;
    }

    public void setWidth(Integer width) {
        this.width = width;
    }

    public Integer getHeight() {
        return height;
    }

    public void setHeight(Integer height) {
        this.height = height;
    }
}
