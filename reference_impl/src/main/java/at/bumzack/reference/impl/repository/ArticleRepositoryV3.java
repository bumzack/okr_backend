package at.bumzack.reference.impl.repository;


import at.bumzack.reference.impl.dto.ArticleAndImageModel;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;

import java.util.List;

public interface ArticleRepositoryV3 extends JpaRepository<ArticleAndImageModel, Long> {

    @Query("SELECT articles.id,  articles.title,  articles.description, articles.code,  " +
            "images.image_as_json_pixels_array, images.width, images.height, images.filename  " +
            "FROM articles  " +
            "JOIN art2img ON (art2img.article_id = articles.id)  " +
            "JOIN images ON (art2img.image_id = images.id)  " +
            "WHERE articles.id IN (  " +
            "    SELECT id FROM articles   " +
            "    ORDER BY articles.code ASC  " +
            "    LIMIT :limit  OFFSET :offset  " +
            ")")
    List<ArticleAndImageModel> find(int offset, int limit);
}

