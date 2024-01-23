package at.bumzack.refimpl.dto

data class SysInfo(
        val author: String = "null",
        val language: String = "null",
        val framework: String = "null",
        val multithreaded: Boolean = false,
        val comment: String = "null",
)