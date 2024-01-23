package at.bumzack.refimpl.dto

data class SysInfo(
        var author: String = "null",
        var language: String = "null",
        var framework: String = "null",
        var multithreaded: Boolean = false,
        var comment: String = "null",
)