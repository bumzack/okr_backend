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
import java.time.ZoneId
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
    @Value("\${sourcefilesFolder}")
    private val sourceFilesFolder: String = ""
    fun importArticles(): ImportResult {
        LOG.info("sourceFilesFolder {}", sourceFilesFolder)
        val folder = File(sourceFilesFolder)

        val res = Arrays.stream(Objects.requireNonNull<Array<File>>(folder.listFiles()))
                .filter { file: File -> file.name.contains(".txt") }
                .sorted(Comparator.comparing<File, String> { obj: File -> obj.name })
                .map<ImportResult?> { f: File ->
                    tryProcessFile(
                            f
                    )
                }
                .toList();

        var linesProcessed: Long = 0
        var dbRowsWritten: Long = 0


        val articles = mutableListOf<Article>()
        res.forEach { r ->
            linesProcessed += r.linesProcessed
            dbRowsWritten += r.dbRowsWritten
            articles.addAll(r.articles)
        }

        return ImportResult(
                linesProcessed = linesProcessed,
                dbRowsWritten = dbRowsWritten,
                articles = articles,
        )


    }

    fun importArticles2(): ImportResult {
        LOG.info("sourceFilesFolder {}", sourceFilesFolder)
        val folder = File(sourceFilesFolder)

        val res = Arrays.stream(Objects.requireNonNull<Array<File>>(folder.listFiles()))
                .filter { file: File -> file.name.contains(".txt") }
                .sorted(Comparator.comparing { obj: File -> obj.name })
                .toList()
                .parallelStream()
                .map<ImportResult?> { f: File ->
                    tryProcessFile(f)
                }
                .toList();

        var linesProcessed: Long = 0
        var dbRowsWritten: Long = 0


        val articles = mutableListOf<Article>()
        res.forEach { r ->
            linesProcessed += r.linesProcessed
            dbRowsWritten += r.dbRowsWritten
            articles.addAll(r.articles)
        }
        return ImportResult(
                linesProcessed = linesProcessed,
                dbRowsWritten = dbRowsWritten,
                articles = articles,
        )

    }

    private fun tryProcessFile(f: File): ImportResult? {
        try {
            val res: ImportResult = processFile(f)
            LOG.info(
                    "filename {},  linesProcessed  {},   dbRowsWritten  {} ",
                    f.name,
                    res.linesProcessed,
                    res.dbRowsWritten
            )
            return res
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

        val article_grouped_by_code_and_pos: ArrayList<Article> =
                ArrayList<Article>()
        val articles_ready_to_write_to_db: ArrayList<Article> =
                ArrayList<Article>()

        if (Objects.nonNull(line)) {
            var article: Article
            var prevArticle: Article? = null
            while (true) {
                article = line2article(line)

                // LOG.info("line {},    article    code {}, pos {}, price  {}", linesProcessed, article.getCode(), article.getPos(), article.getPrice());
                if (Objects.isNull(prevArticle)) {
                    // new grouping start - because first article ever
                    article_grouped_by_code_and_pos.add(article)
                } else {
                    // is article part of current group?
                    if (article.code == prevArticle!!.code && article.pos == prevArticle.pos) {
                        article_grouped_by_code_and_pos.add(article)
                    } else {
                        // article is not part of current group -> find cheapeast
                        val cheapestArticle: List<Article> =
                                article_grouped_by_code_and_pos.stream()
                                        .sorted(Comparator.comparing({ obj: Article -> obj.price }))
                                        .limit(1)
                                        .toList()
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
            val cheapestArticle: List<Article> = article_grouped_by_code_and_pos.stream()
                    .sorted(Comparator.comparing { obj: Article -> obj.price })
                    .limit(1)
                    .toList()
                articles_ready_to_write_to_db.add(cheapestArticle.first())
            dbRowsWritten++

            // LOG.info("articles_ready_to_write_to_db   size   {}", articles_ready_to_write_to_db.size());

            // articles_ready_to_write_to_db.forEach(a -> LOG.info("article in DB  code {}, pos {}, price {}", a.getCode(), a.getPos(), a.getPrice()));
        }

        val importResult = ImportResult()
        importResult.dbRowsWritten = dbRowsWritten
        importResult.linesProcessed = linesProcessed
        importResult.articles = articles_ready_to_write_to_db

        return importResult
    }


    private fun line2article(line: String): Article {
        val beginDesc: Int =
                LEN_CODE + LEN_TITLE
        val beginAttr: Int =
                LEN_CODE + LEN_TITLE + LEN_DESC
        val beginCat: Int =
                LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES
        val beginPos: Int =
                LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES
        val beginPrice: Int =
                LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS
        val beginStartDate: Int =
                LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS + LEN_PRICE
        val beginEndDate: Int =
                LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS + LEN_PRICE + LEN_START

        val startDateStr = line.substring(beginStartDate, beginEndDate)
        val endDateStr = line.substring(beginEndDate)
        val start = LocalDateTime.ofInstant(
                Instant.ofEpochSecond(startDateStr.toLong()),
                ZoneId.of("UTC")
        )
        val end = LocalDateTime.ofInstant(
                Instant.ofEpochSecond(endDateStr.toLong()),
                ZoneId.of("UTC")
        )

        val article = Article(
                code = trimLeadingZeroes(line.substring(0, LEN_CODE)),
                title = line.substring(LEN_CODE, beginDesc).trim { it <= ' ' },
                description = line.substring(beginDesc, beginAttr).trim { it <= ' ' },
                attributes = line.substring(beginAttr, beginCat).trim { it <= ' ' },
                categories = line.substring(beginCat, beginPos).trim { it <= ' ' },
                pos = trimLeadingZeroes(line.substring(beginPos, beginPrice).trim { it <= ' ' }),
                price = BigDecimal.valueOf(line.substring(beginPrice, beginStartDate).toDouble()),
                startDate = LocalDateTime.from(start).toString(),
                endDate = LocalDateTime.from(end).toString(),
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
