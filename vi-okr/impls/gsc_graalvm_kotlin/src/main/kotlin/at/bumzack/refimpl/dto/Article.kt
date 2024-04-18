package at.bumzack.refimpl.dto

import java.math.BigDecimal

data class Article(
    val code: String,
    val description: String,
    val title: String,
    val categories: String,
    val attributes: String,
    val price: BigDecimal,
    val pos: String,
    val startDate: String,
    val endDate: String,
)