package at.bumzack.reference.impl.dto;

import jakarta.persistence.*;

import java.math.BigDecimal;

@Entity
@Table(name = "articles")
public class ArticleModel {
    @Id
    @SequenceGenerator(name = "articles_local_seq", sequenceName = "articles_seq", allocationSize = 1)
    @GeneratedValue(strategy = GenerationType.AUTO)
    private long id;

    private String code;

    private String description;

    private String title;

    private String categories;
    private String attributes;
    private BigDecimal price;
    private String pos;
    @Column(name = "start_data")

    private Long startDate;
    @Column(name = "end_data")
    private Long endDate;

    public ArticleModel() {
    }

    public long getId() {
        return id;
    }

    public void setId(long id) {
        this.id = id;
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

    public Long getStartDate() {
        return startDate;
    }

    public void setStartDate(Long startDate) {
        this.startDate = startDate;
    }

    public Long getEndDate() {
        return endDate;
    }

    public void setEndDate(Long endDate) {
        this.endDate = endDate;
    }

    @Override
    public String toString() {
        return "ArticleModel{" +
                "id=" + id +
                ", code='" + code + '\'' +
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
