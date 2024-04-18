package at.posselt.api


data class SysInfo(
    val author: String,
    val language: String,
    val framework: String,
    val multithreaded: Boolean,
    val comment: String,
    val version: String,
)
