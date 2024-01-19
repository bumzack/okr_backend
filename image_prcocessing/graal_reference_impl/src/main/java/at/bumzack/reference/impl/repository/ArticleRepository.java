package at.bumzack.reference.impl.repository;


import at.bumzack.reference.impl.dto.ArticleModel;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;

public interface ArticleRepository extends JpaRepository<ArticleModel, Long> {
}

