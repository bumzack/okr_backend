package at.bumzack.reference.impl.dto;

import java.math.BigDecimal;

public class Article {
    private String code;
    private String description;
    private String title;
    private String categories;
    private String attributes;
    private BigDecimal price;
    private String pos;
    private String startDate;
    private String endDate;

    public Article() {
    }


    public String getCode() {
        return code;
    }

    public void setCode(String code) {
        this.code = code;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public String getTitle() {
        return title;
    }

    public void setTitle(String title) {
        this.title = title;
    }

    public String getCategories() {
        return categories;
    }

    public void setCategories(String categories) {
        this.categories = categories;
    }

    public String getAttributes() {
        return attributes;
    }

    public void setAttributes(String attributes) {
        this.attributes = attributes;
    }

    public BigDecimal getPrice() {
        return price;
    }

    public void setPrice(BigDecimal price) {
        this.price = price;
    }

    public String getPos() {
        return pos;
    }

    public void setPos(String pos) {
        this.pos = pos;
    }

    public String getStartDate() {
        return startDate;
    }

    public void setStartDate(String startDate) {
        this.startDate = startDate;
    }

    public String getEndDate() {
        return endDate;
    }

    public void setEndDate(String endDate) {
        this.endDate = endDate;
    }

    @Override
    public String toString() {
        return "ArticleModel{" +
                "code=" + code +
                ", description='" + description + '\'' +
                ", title='" + title + '\'' +
                ", categories='" + categories + '\'' +
                ", attributes='" + attributes + '\'' +
                ", price=" + price +
                ", pos='" + pos + '\'' +
                ", startDate=" + startDate +
                ", endDate=" + endDate +
                '}';
    }
}
