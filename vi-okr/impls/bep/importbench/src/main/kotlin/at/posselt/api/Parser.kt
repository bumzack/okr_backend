package at.posselt.api

import com.fasterxml.jackson.annotation.JsonFormat
import com.fasterxml.jackson.annotation.JsonIncludeProperties
import java.nio.file.Path
import java.time.Instant
import kotlin.io.path.isRegularFile
import kotlin.io.path.listDirectoryEntries
import kotlin.io.path.useLines

data class LineIndices(val from: Int, val to: Int) {
    val toExclusive = to + 1
}

data class ColumnAndIndices(val column: Column, val indices: LineIndices)
data class Column(
    val name: String,
    val length: Int,
    val padLeft: Char? = ' ',
    val padRight: Char? = ' ',
)

internal fun buildIndices(columns: Array<Column>): Map<String, ColumnAndIndices> {
    val summedUpLengths = columns
        .map { it.length }
        .scan(0) { prev, curr -> prev + curr }
    val endIndicesPlus1 = summedUpLengths.asSequence().drop(1)
    val indices = summedUpLengths.asSequence()
        .zip(endIndicesPlus1)
        .map { (from, to) -> LineIndices(from, to - 1) }
    return columns.asSequence()
        .zip(indices)
        .map { (column, indices) -> column.name to ColumnAndIndices(column, indices) }
        .toMap()

}

private val columns = arrayOf(
    Column("code", 20, padLeft = '0'),
    Column("title", 100),
    Column("description", 1700),
    Column("attributes", 200),
    Column("categories", 200),
    Column("pos", 30, padLeft = '0', padRight = ' '),
    Column("price", 20, padLeft = '0', padRight = '0'),
    Column("startDate", 25, padLeft = '0', padRight = '0'),
    Column("endDate", 25, padLeft = '0', padRight = '0'),
)

private val indices = buildIndices(columns)

internal fun parseLine(line: String, column: ColumnAndIndices): String {
    return line.substring(column.indices.from, column.indices.toExclusive)
        .let { value -> column.column.padLeft?.let { value.trimStart(it) } ?: value }
        .let { value -> column.column.padRight?.let { value.trimEnd(it) } ?: value }
}

@JsonIncludeProperties(
    value = [
        "price",
        "pos",
        "code",
        "title",
        "description",
        "attributes",
        "categories",
        "startDate",
        "endDate",
    ]
)
data class ArticleLine(val line: String) {
    // eagerly parsed values because they are always needed
    val price = parseLine(line, indices["price"]!!).toFloat()
    val pos = parseLine(line, indices["pos"]!!).toInt()
    val code = parseLine(line, indices["code"]!!)

    // lazily parse the rest
    val title: String
        get() = parseLine(line, indices["title"]!!)
    val description: String
        get() = parseLine(line, indices["description"]!!)
    val attributes: String
        get() = parseLine(line, indices["attributes"]!!)
    val categories: String
        get() = parseLine(line, indices["categories"]!!)

    @get:JsonFormat(shape = JsonFormat.Shape.SCALAR, pattern = "yyyy-MM-dd'T'HH:mm:ss", timezone = "UTC")
    val startDate: Instant
        get() = Instant.ofEpochSecond(parseLine(line, indices["startDate"]!!).toLong())

    @get:JsonFormat(shape = JsonFormat.Shape.SCALAR, pattern = "yyyy-MM-dd'T'HH:mm:ss", timezone = "UTC")
    val endDate: Instant
        get() = Instant.ofEpochSecond(parseLine(line, indices["endDate"]!!).toLong())
}

data class ProcessedArticles(val cheapest: ArticleLine, val processedRows: Int)

fun parseLine(lines: Sequence<String>, callback: (ProcessedArticles) -> Unit) {
    var parsedLines = 0
    var currentPos: Int? = null
    var currentCode: String? = null
    var lowestPrice: ArticleLine? = null
    for (line in lines) {
        val article = ArticleLine(line)
        parsedLines += 1
        if (currentCode != article.code || currentPos != article.pos) {
            // on the first iteration, this will be null; in all subsequent iterations we
            // are jumping into this block when a new group is started
            if (lowestPrice != null) {
                callback(ProcessedArticles(lowestPrice, parsedLines))
                parsedLines = 0
            }
            currentCode = article.code
            currentPos = article.pos
            lowestPrice = article
        } else {
            if (lowestPrice == null || article.price < lowestPrice.price) {
                lowestPrice = article
            }
        }
    }
    // the last line will never be added otherwise
    if (lowestPrice != null) {
        callback(ProcessedArticles(lowestPrice, parsedLines))
    }
}


fun forEachArticle(files: List<Path>, callback: (ProcessedArticles) -> Unit) {
    files.forEach { path ->
        println("Beginning parsing file $path")
        path.useLines {
            parseLine(it, callback)
        }
        println("Finished parsing file $path")
    }
}

internal fun filesInDirectory(path: Path): List<Path> {
    return path.listDirectoryEntries("*.txt")
        .asSequence()
        .filter { it.isRegularFile() }
        .toList()
}
