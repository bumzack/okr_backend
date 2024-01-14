package at.bumzack.reference.impl.dto;

public class Resolution {
    private long id;

    private String name;
    private int width;
    private int height;
    private boolean original;

    public Resolution() {
    }

    public boolean isOriginal() {
        return original;
    }

    public void setOriginal(final boolean original) {
        this.original = original;
    }

    public long getId() {
        return id;
    }

    public void setId(final long id) {
        this.id = id;
    }

    public String getName() {
        return name;
    }

    public void setName(final String name) {
        this.name = name;
    }

    public int getWidth() {
        return width;
    }

    public void setWidth(final int width) {
        this.width = width;
    }

    public int getHeight() {
        return height;
    }

    public void setHeight(final int height) {
        this.height = height;
    }

    @Override
    public String toString() {
        return "Resolution{" +
                "id=" + id +
                ", name='" + name + '\'' +
                ", width=" + width +
                ", height=" + height +
                ", original=" + original +
                '}';
    }
}
