package at.posselt.api

import java.nio.file.Path
import java.nio.file.Paths
import kotlin.test.Test
import kotlin.test.assertEquals


class ParserKtTest {
    private fun getTestDirectory(): Path =
        Paths.get(object {}.javaClass.getResource("/data/").path)

    @Test
    fun `should parse files in directory`() {
        val files = filesInDirectory(getTestDirectory())
            .toList()
        assertEquals(1, files.size)
        assertEquals("articles_000001.txt", files[0].toFile().name)
    }


    @Test
    fun `should parse first line from data file`() {
        val files = filesInDirectory(getTestDirectory())
        val articles = mutableListOf<ArticleLine>()
        var processed = 0
        var written = 0
        forEachArticle(files) {
            articles.add(it.cheapest)
            processed += it.processedRows
            written += 1
        }
        assertEquals(3, written)
        assertEquals(10, processed)

        val line = articles.first()
        assertEquals("1", line.code)
        assertEquals("Article with code 00000001", line.title)
        assertEquals(1494.7479f, line.price)
    }

    @Test
    fun `should build indices`() {
        val columns = arrayOf(
            Column("code", 20),
            Column("test", 100),
            Column("kick", 1),
        )
        val result = buildIndices(columns)
        assertEquals(ColumnAndIndices(columns[0], LineIndices(0, 19)), result["code"])
        assertEquals(ColumnAndIndices(columns[1], LineIndices(20, 119)), result["test"])
        assertEquals(ColumnAndIndices(columns[2], LineIndices(120, 120)), result["kick"])
    }

    @Test
    fun `should parse line`() {
        assertEquals(
            "b",
            parseLine("abcdefg", ColumnAndIndices(Column(name = "test", length = 1), LineIndices(1, 1)))
        )
        assertEquals(
            "abcd",
            parseLine("abcdefg", ColumnAndIndices(Column(name = "test", length = 1), LineIndices(0, 3)))
        )
        assertEquals(
            "bcd",
            parseLine("abcdefg", ColumnAndIndices(Column(name = "test", length = 1, padLeft = 'a'), LineIndices(0, 3)))
        )
        assertEquals(
            "bc",
            parseLine(
                "abcdefg",
                ColumnAndIndices(Column(name = "test", length = 1, padLeft = 'a', padRight = 'd'), LineIndices(0, 3))
            )
        )
    }
}