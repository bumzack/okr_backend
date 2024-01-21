package at.bumzack.reference.impl.repository;


import at.bumzack.reference.impl.dto.ArticleModel;
import org.springframework.data.jpa.repository.JpaRepository;

public interface ArticleRepository extends JpaRepository<ArticleModel, Long> {
}

