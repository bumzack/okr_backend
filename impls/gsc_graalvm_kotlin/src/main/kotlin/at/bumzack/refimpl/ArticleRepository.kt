package at.bumzack.refimpl

import at.bumzack.refimpl.dto.ArticleModel
import org.springframework.data.jpa.repository.JpaRepository

interface ArticleRepository : JpaRepository<ArticleModel, Long> {
}