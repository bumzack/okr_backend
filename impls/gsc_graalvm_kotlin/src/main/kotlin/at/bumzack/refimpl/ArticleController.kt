package at.bumzack.refimpl

import at.bumzack.refimpl.dto.Article
import at.bumzack.refimpl.dto.ImportResult
import at.bumzack.refimpl.dto.SysInfo
import org.slf4j.LoggerFactory
import org.springframework.web.bind.annotation.*

private val LOG = LoggerFactory.getLogger(ArticleController::class.java)

@RestController
@RequestMapping("/api")
class ArticleController(
        val articleService: ArticleService
) {

    @GetMapping("/v1/articles/{pageNumber}/{pageSize}")
    @ResponseBody
    fun findPaginatedV1(@PathVariable pageNumber: Int,
                        @PathVariable pageSize: Int): List<Article> {
        LOG.info("findPaginated   pageNumber {}, pageSize {}", pageNumber, pageSize)
        val fullArticles: List<Article> = articleService.findPaginated(pageNumber, pageSize)
        return fullArticles
    }

    @PostMapping("/v1/articles/import")
    @ResponseBody
    fun importArticles(): ImportResult {
        return articleService.importArticles()
    }

    @GetMapping("/v1/sysinfo")
    @ResponseBody
    fun sysinfo(): SysInfo {
        val sysInfo = SysInfo(
                author = "gsc",
                framework = "Spring Boot 3.2.2",
                comment = "naive & dumb",
                language = "Graal VM & Kotlin 1.9.0",
                multithreaded = false,
        )
        LOG.info("sysinfo {}", sysInfo)
        return sysInfo
    }
}



