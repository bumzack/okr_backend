package at.bumzack.refimpl

import at.bumzack.refimpl.dto.Article
import at.bumzack.refimpl.dto.ImportResult
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Service
import java.io.BufferedReader
import java.io.File
import java.io.FileReader
import java.io.IOException
import java.math.BigDecimal
import java.time.Instant
import java.time.LocalDateTime
import java.util.*


private val LOG = LoggerFactory.getLogger(ArticleService::class.java)

private const val LEN_CODE = 20
private const val LEN_TITLE = 100
private const val LEN_DESC = 1700
private const val LEN_ATTRIBUTES = 200
private const val LEN_CATEGORIES = 200
private const val LEN_POS = 30
private const val LEN_PRICE = 20
private const val LEN_START = 25

@Service
class ArticleService {
    val LEN_CODE: Int = 20
    val LEN_TITLE: Int = 100
    val LEN_DESC: Int = 1700
    val LEN_ATTRIBUTES: Int = 200
    val LEN_CATEGORIES: Int = 200
    val LEN_POS: Int = 30
    val LEN_PRICE: Int = 20
    val LEN_START: Int = 25


    // private static final int LEN_END = 25;
    @Value("\${sourcefilesFolder}")
    private val sourceFilesFolder: String? = null


    fun importArticles(returnItems: Boolean): ImportResult {
        LOG.info("sourceFilesFolder {}", sourceFilesFolder)
        val folder = File(sourceFilesFolder!!)

        val res = Arrays.stream(Objects.requireNonNull(folder.listFiles()))
            .filter { file -> file.name.contains(".txt") }
            .sorted(Comparator.comparing { obj -> obj.name })
            .map { f -> tryProcessFile(f, returnItems) }
            .toList()

        var linesProcessed: Long = 0
        var dbRowsWritten: Long = 0

        val articles = mutableListOf<Article>()
        res.forEach { r ->
            linesProcessed += r?.linesProcessed ?: 0
            dbRowsWritten += r?.dbRowsWritten ?: 0
            articles.addAll(r?.articles ?: emptyList())
        }

        return ImportResult(
            linesProcessed = linesProcessed,
            dbRowsWritten = dbRowsWritten,
            articles = articles,
        )
    }

    fun importArticlesParallel(returnItems: Boolean): ImportResult {
        LOG.info("sourceFilesFolder {}", sourceFilesFolder)
        val folder = File(sourceFilesFolder!!)

        val res = Arrays.stream(Objects.requireNonNull(folder.listFiles()))
            .filter { file: File -> file.name.contains(".txt") }
            .sorted(Comparator.comparing { obj: File -> obj.name })
            .toList()
            .parallelStream()
            .map { f -> tryProcessFile(f, returnItems) }
            .toList()

        var linesProcessed: Long = 0
        var dbRowsWritten: Long = 0

        val articles = mutableListOf<Article>()
        res.forEach { r ->
            linesProcessed += r?.linesProcessed ?: 0
            dbRowsWritten += r?.dbRowsWritten ?: 0
            articles.addAll(r?.articles ?: emptyList())
        }

        return ImportResult(
            linesProcessed = linesProcessed,
            dbRowsWritten = dbRowsWritten,
            articles = articles,
        )
    }


    private fun tryProcessFile(f: File, returnItems: Boolean): ImportResult? {
        try {
            if (returnItems) {
                val res = processFile(f)
                LOG.info("filename {},  linesProcessed  {},   dbRowsWritten  {} ", f.name, res.linesProcessed, res.dbRowsWritten)
                return res
            } else {
                val res = processFileNoReturnItems(f)
                LOG.info("filename {},  linesProcessed  {},   dbRowsWritten  {} ", f.name, res.linesProcessed, res.dbRowsWritten)
                return res
            }
        } catch (e: IOException) {
            LOG.error("error processing file ", e)
        }
        return null
    }

    @Throws(IOException::class)
    private fun processFile(f: File): ImportResult {
        val reader = BufferedReader(FileReader(f))
        var linesProcessed: Long = 0
        var dbRowsWritten: Long = 0

        var line = reader.readLine()
        linesProcessed++

        val article_grouped_by_code_and_pos = ArrayList<Article>()
        val articles_ready_to_write_to_db = ArrayList<Article>()

        if (Objects.nonNull(line)) {
            var article: Article
            var prevArticle: Article? = null
            while (true) {
                article = line2article(line)

                if (Objects.isNull(prevArticle)) {
                    // new grouping start - because first article ever
                    article_grouped_by_code_and_pos.add(article)
                } else {
                    // is article part of current group?
                    if (article.code == prevArticle?.code && article.pos == prevArticle.pos) {
                        article_grouped_by_code_and_pos.add(article)
                    } else {
                        // article is not part of current group -> find cheapest
                        val cheapestArticle = article_grouped_by_code_and_pos.stream()
                            .sorted(Comparator.comparing(Article::price))
                            .limit(1)
                            .toList()
                        // comment for "returnItems == false"
                        articles_ready_to_write_to_db.add(cheapestArticle.first())

                        dbRowsWritten++

                        // clear group and add article
                        article_grouped_by_code_and_pos.clear()
                        article_grouped_by_code_and_pos.add(article)
                    }
                }

                line = reader.readLine()
                if (Objects.isNull(line)) {
                    break
                }
                linesProcessed++
                prevArticle = article
            }

            // write last article in file
            val cheapestArticle = article_grouped_by_code_and_pos.stream()
                .sorted(Comparator.comparing(Article::price))
                .limit(1)
                .toList()

            // comment for "returnItems == false"
            articles_ready_to_write_to_db.add(cheapestArticle.first())

            dbRowsWritten++
        }

        val importResult = ImportResult()
        importResult.dbRowsWritten = dbRowsWritten
        importResult.linesProcessed = linesProcessed
        importResult.articles = articles_ready_to_write_to_db

        return importResult
    }

    @Throws(IOException::class)
    private fun processFileNoReturnItems(f: File): ImportResult {
        val reader = BufferedReader(FileReader(f))
        var linesProcessed: Long = 0
        var dbRowsWritten: Long = 0

        var line = reader.readLine()
        linesProcessed++

        val article_grouped_by_code_and_pos = ArrayList<Article>()
        val articles_ready_to_write_to_db = ArrayList<Article>()

        if (Objects.nonNull(line)) {
            var article: Article
            var prevArticle: Article? = null
            while (true) {
                article = line2article(line)

                if (Objects.isNull(prevArticle)) {
                    // new grouping start - because first article ever
                    article_grouped_by_code_and_pos.add(article)
                } else {
                    // is article part of current group?
                    if (article.code.equals(prevArticle?.code) && article.pos.equals(prevArticle?.pos)) {
                        article_grouped_by_code_and_pos.add(article)
                    } else {
                        // article is not part of current group -> find cheapest
                        val cheapestArticle = article_grouped_by_code_and_pos.stream()
                            .sorted(Comparator.comparing(Article::price))
                            .limit(1)
                            .toList()

                        // comment for "returnItems == false"
                        // articles_ready_to_write_to_db.add(cheapestArticle.getFirst());
                        dbRowsWritten++

                        // clear group and add article
                        article_grouped_by_code_and_pos.clear()
                        article_grouped_by_code_and_pos.add(article)
                    }
                }

                line = reader.readLine()
                if (Objects.isNull(line)) {
                    break
                }
                linesProcessed++
                prevArticle = article
            }

            // write first article in list to response
            val cheapestArticle = article_grouped_by_code_and_pos.stream()
                .sorted(Comparator.comparing(Article::price))
                .limit(1)
                .toList()
            // comment for "returnItems == false"
//            articles_ready_to_write_to_db.add(cheapestArticle.getFirst());
            dbRowsWritten++
        }

        val importResult = ImportResult()
        importResult.dbRowsWritten = dbRowsWritten
        importResult.linesProcessed = linesProcessed
        importResult.articles = articles_ready_to_write_to_db

        return importResult
    }

    private fun line2article(line: String): Article {

        val beginDesc = LEN_CODE + LEN_TITLE
        val beginAttr = LEN_CODE + LEN_TITLE + LEN_DESC
        val beginCat = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES
        val beginPos = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES
        val beginPrice = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS
        val beginStartDate = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS + LEN_PRICE
        val beginEndDate = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS + LEN_PRICE + LEN_START


        val startDateStr = line.substring(beginStartDate, beginEndDate)
        val endDateStr = line.substring(beginEndDate)
        val start = LocalDateTime.ofInstant(
            Instant.ofEpochSecond(startDateStr.toLong()),
            TimeZone.getDefault().toZoneId()
        )
        val end = LocalDateTime.ofInstant(
            Instant.ofEpochSecond(endDateStr.toLong()),
            TimeZone.getDefault().toZoneId()
        )

        val article = Article(
            code = trimLeadingZeroes(line.substring(0, LEN_CODE)),
            title = line.substring(LEN_CODE, beginDesc).trim { it <= ' ' },
            description = line.substring(beginDesc, beginAttr).trim { it <= ' ' },
            attributes = line.substring(beginAttr, beginCat).trim { it <= ' ' },
            categories = line.substring(beginCat, beginPos).trim { it <= ' ' },
            pos = trimLeadingZeroes(line.substring(beginPos, beginPrice).trim { it <= ' ' }),
            price = BigDecimal.valueOf(line.substring(beginPrice, beginStartDate).toDouble()),
            startDate = LocalDateTime.from(start).toString(), endDate = LocalDateTime.from(end).toString()
        )

        return article
    }

    private fun trimLeadingZeroes(s: String): String {
        var i = 0
        while (s[i] == '0') {
            i++
        }
        return s.substring(i)
    }
}
