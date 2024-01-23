package at.bumzack.refimpl

import at.bumzack.refimpl.dto.Article
import at.bumzack.refimpl.dto.ArticleModel
import at.bumzack.refimpl.dto.ImportResult
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Value
import org.springframework.data.domain.Page
import org.springframework.data.domain.PageRequest
import org.springframework.stereotype.Service
import java.io.BufferedReader
import java.io.File
import java.io.IOException
import java.math.BigDecimal
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

private const val PROPERTY_CODE = "code"


@Service
class ArticleService(
        val articleRepository: ArticleRepository
) {

    @Value("\${sourcefilesFolder}")
    private val sourceFilesFolder: String = ""

    fun findPaginated(pageNumber: Int, pageSize: Int): List<Article> {
        val p: PageRequest = PageRequest.of(pageNumber, pageSize, org.springframework.data.domain.Sort.by(org.springframework.data.domain.Sort.Direction.ASC, PROPERTY_CODE))
        val findAll: Page<ArticleModel> = articleRepository.findAll(p)
        val findAll2: MutableList<ArticleModel> =findAll.toList()
        return convert(findAll2)
    }

    private fun convert(source: MutableList<ArticleModel>): List<Article> {
        return source.stream()
                .map { a: ArticleModel ->
                    val target = Article (
                            attributes = a.attributes,
                            categories = a.categories,
                            code = a.code,
                            description = a.description,
                            pos = a.pos,
                            title = a.title,
                            // TODO fix
                            startDate = LocalDateTime.now(),
                            endDate = LocalDateTime.now(),
                            price = a.price
                    )
                    target
                }.toList()
    }

    fun importArticles(): ImportResult {
//        final File currentDirFile = new File(".");
//        final String helper = currentDirFile.getAbsolutePath();
//
//        LOG.info("currentDirFile {}", currentDirFile.getName());
//        LOG.info("helper {}", helper);
//        LOG.info("sourceFilesFolder {}", sourceFilesFolder);
        val absPath = sourceFilesFolder
        LOG.info("absPath {}", absPath)

        val folder = File(absPath)

        for (fileEntry in Objects.requireNonNull<Array<File>>(folder.listFiles())) {
            if (fileEntry.isDirectory()) {
                LOG.info("directory {}", fileEntry.getName())
            } else {
                LOG.info("file {}", fileEntry.getName())
            }
        }

        val fileNames: List<File> = Arrays.stream(Objects.requireNonNull<Array<File>>(folder.listFiles()))
                .filter { file: File -> file.getName().contains(".txt") }
                .toList()

        LOG.info("===================================================================   ")
        LOG.info("filenames   ")
        fileNames
                .forEach(java.util.function.Consumer { f: File? -> LOG.info("filename {}", f) })
        LOG.info("===================================================================   ")

        val res = fileNames.stream()
                .sorted(Comparator.comparing { obj: File ->
                    obj.getName()
                })
                .map { f: File -> this.tryProcessFile(f) }
//                .reduce<ImportResult>(
//                        ImportResult(linesProcessed = 0, dbRowsWritten = 0),
//                        BiFunction<ImportResult, ImportResult, ImportResult> { r1: ImportResult?, r2: ImportResult? -> ImportResult.sum(r1, r2) }, BinaryOperator<ImportResult> { r1: ImportResult?, r2: ImportResult? -> ImportResult.sum(r1, r2) })
        var linesProcessed: Long = 0
        var dbRowsWritten: Long = 0

        res.forEach { r ->
            linesProcessed += r.linesProcessed
            dbRowsWritten += r.dbRowsWritten
        }

        return ImportResult(
                linesProcessed = linesProcessed,
                dbRowsWritten = dbRowsWritten,
        )
    }

    private fun tryProcessFile(f: File): ImportResult {
        try {
            val res: ImportResult = processFile(f)
            LOG.info("filename {}  ,  linesProcessed  {},   dbRowsWritten  {} ", f.getName(), res.linesProcessed, res.dbRowsWritten)
            return res
        } catch (e: IOException) {
            throw java.lang.RuntimeException(e)
        }
    }

    private fun processFile(f: File): ImportResult {
        val reader = BufferedReader(java.io.FileReader(f))

        var line: String? = reader.readLine()
        val tmp: ArrayList<ArticleModel> = ArrayList<ArticleModel>()
        val articles: ArrayList<ArticleModel> = ArrayList<ArticleModel>()
        var linesProcessed: Long = 0
        var dbRowsWritten: Long = 0
        while (line != null) {
            val article = processLine(line)
            if (tmp.isNotEmpty()) {
                val last: ArticleModel = tmp.last()
                // group by code and pos
                if (last.code == article.code && (last.pos == article.pos)) {
                    tmp.add(article)
                } else {
                    val c: List<ArticleModel> = tmp.stream()
                            .sorted(Comparator.comparing { obj: ArticleModel -> obj.price })
                            .limit(1)
                            .toList()
                    articles.add(c.first())
                    tmp.clear()
                }
            } else {
                tmp.add(article)
            }
            linesProcessed++

            if (articles.size > 50) {
                // articles.forEach(LOG::info);
                articleRepository.saveAll(articles);
                dbRowsWritten += articles.size.toLong()
                //  LOG.info("filename {}  ,  {} articles  written", f.getName(), articles.size());
                articles.clear()
            }

            line = reader.readLine()
        }

        val importResult = ImportResult(
                linesProcessed = linesProcessed,
                dbRowsWritten = dbRowsWritten
        )
        return importResult
    }

    // private static final int LEN_END = 25;
    private fun processLine(line: String): ArticleModel {
        val beginDesc = LEN_CODE + LEN_TITLE
        val beginAttr = LEN_CODE + LEN_TITLE + LEN_DESC
        val beginCat = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES
        val beginPos = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES
        val beginPrice = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS
        val beginStartDate = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS + LEN_PRICE
        val beginEndDate = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS + LEN_PRICE + LEN_START
        val article = ArticleModel(
                code = trimLeadingZeroes(line.substring(0, LEN_CODE)),
                title = line.substring(LEN_CODE, beginDesc).trim { it <= ' ' },
                description = line.substring(beginDesc, beginAttr).trim { it <= ' ' },
                attributes = line.substring(beginAttr, beginCat).trim { it <= ' ' },
                categories = line.substring(beginCat, beginPos).trim { it <= ' ' },
                pos = trimLeadingZeroes(line.substring(beginPos, beginPrice).trim { it <= ' ' }),
                price = BigDecimal.valueOf(line.substring(beginPrice, beginStartDate).toDouble()),
                startDate = LocalDateTime.from(LocalDateTime.ofInstant(java.time.Instant.ofEpochMilli(line.substring(beginStartDate, beginEndDate).toLong()), TimeZone.getDefault().toZoneId())),
                endDate = LocalDateTime.from(LocalDateTime.ofInstant(java.time.Instant.ofEpochMilli(line.substring(beginEndDate).toLong()), TimeZone.getDefault().toZoneId())),
                id = null,
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
