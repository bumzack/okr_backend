package at.bumzack.refimpl.dto

import jakarta.persistence.*
import java.math.BigDecimal
import java.time.LocalDateTime

@Entity
@Table(name = "articles")
data class ArticleModel(
        @Id
        @GeneratedValue(strategy = GenerationType.AUTO)
        val id: Long?,
        val code: String,
        val description: String,
        val title: String,
        val categories: String,
        val attributes: String,
        val price: BigDecimal,
        val pos: String,
        val startDate: LocalDateTime,
        val endDate: LocalDateTime,
)