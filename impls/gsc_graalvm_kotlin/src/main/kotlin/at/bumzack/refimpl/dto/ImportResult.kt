package at.bumzack.refimpl.dto

import java.util.Objects.nonNull


data class ImportResult(
    var linesProcessed: Long = 0,
    var dbRowsWritten: Long = 0,
    var articles: List<Article> = emptyList(),
) {

    fun sum(r1: ImportResult, r2: ImportResult): ImportResult {
        var combined = ImportResult()
        combined.dbRowsWritten = r1.dbRowsWritten + r2.dbRowsWritten
        combined.linesProcessed = r1.linesProcessed + r2.linesProcessed
        val arr = ArrayList<Article>()
        if (nonNull(r1.articles)) {
            arr.addAll(r1.articles)
        }
        if (nonNull(r2.articles)) {
            arr.addAll(r2.articles)
        }
        combined.articles = arr
        return combined
    }


}