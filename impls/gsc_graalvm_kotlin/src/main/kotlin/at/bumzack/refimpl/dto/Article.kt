package at.bumzack.refimpl.dto

import java.math.BigDecimal
import java.time.LocalDateTime

data class Article(
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