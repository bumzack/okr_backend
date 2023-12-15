package at.bumzack.reference.impl.repository;


import at.bumzack.reference.impl.dto.Article;
import org.springframework.data.jpa.repository.JpaRepository;

public interface ArticleRepository extends JpaRepository<Article, Long> {

}
