package at.bumzack.reference.impl.dto;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

@Entity
@Table(name = "art2img")
public class Art2Img {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    private long id;

    @Column(name = "code")
    private String code;

    @Column(name = "article_id")
    private Long articleId;

    @Column(name = "image_id")
    private Long imageId;

    public Art2Img() {
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

    public Long getArticleId() {
        return articleId;
    }

    public void setArticleId(final Long articleId) {
        this.articleId = articleId;
    }

    public Long getImageId() {
        return imageId;
    }

    public void setImageId(final Long imageId) {
        this.imageId = imageId;
    }
}
