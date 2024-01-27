package at.bumzack.refimpl.dto

data class ImportResult(
    val linesProcessed: Long,
    val dbRowsWritten: Long,
)