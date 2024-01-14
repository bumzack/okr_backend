package at.bumzack.reference.impl.dto;

import jakarta.persistence.*;

@Entity
@Table(name = "art2img")
public class Art2ImgModel {
    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    private long id;

    @Column(name = "article_id")
    private Long articleId;

    @Column(name = "image_id")
    private Long imageId;

    public Art2ImgModel() {
    }

    public long getId() {
        return id;
    }

    public void setId(final long id) {
        this.id = id;
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
