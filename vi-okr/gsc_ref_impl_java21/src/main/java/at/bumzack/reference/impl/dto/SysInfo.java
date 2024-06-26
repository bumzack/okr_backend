package at.bumzack.reference.impl.dto;

public class SysInfo {
    private String author;

    private String language;

    private String framework;

    private boolean multithreaded;

    private String comment;

    private String version;

    public SysInfo() {
    }

    public String getAuthor() {
        return author;
    }

    public void setAuthor(final String author) {
        this.author = author;
    }

    public String getLanguage() {
        return language;
    }

    public void setLanguage(final String language) {
        this.language = language;
    }

    public String getFramework() {
        return framework;
    }

    public void setFramework(final String framework) {
        this.framework = framework;
    }

    public boolean isMultithreaded() {
        return multithreaded;
    }

    public void setMultithreaded(final boolean multithreaded) {
        this.multithreaded = multithreaded;
    }

    public String getComment() {
        return comment;
    }

    public void setComment(final String comment) {
        this.comment = comment;
    }

    public String getVersion() {
        return version;
    }

    public void setVersion(final String version) {
        this.version = version;
    }
}
