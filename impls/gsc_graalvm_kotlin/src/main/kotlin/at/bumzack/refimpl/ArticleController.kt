package at.bumzack.refimpl

import at.bumzack.refimpl.dto.ImportResult
import at.bumzack.refimpl.dto.SysInfo
import org.slf4j.LoggerFactory
import org.springframework.http.ResponseEntity
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

private val LOG = LoggerFactory.getLogger(ArticleController::class.java)


@RestController
@RequestMapping("/api")
class ArticleController(
        val articleService: ArticleService
) {

//    @GetMapping("/v1/articles/{pageNumber}/{pageSize}")
//    fun findPaginatedV1(@PathVariable pageNumber: Int,
//                        @PathVariable pageSize: Int): ResponseEntity<List<Article>> {
//        LOG.info("findPaginated   pageNumber {}, pageSize {}", pageNumber, pageSize)
//        val fullArticles: List<Article> = articleService.findPaginated(pageNumber, pageSize)
//        return ResponseEntity.ok<List<Article>>(fullArticles)
//    }

    @PostMapping("/v1/articles/import")
    fun importArticles(): ResponseEntity<ImportResult> {
        val res: ImportResult = articleService.importArticles()

        return ResponseEntity.ok<ImportResult>(res)
    }

    @GetMapping("/v1/sysinfo")
    fun sysinfoV1(): ResponseEntity<SysInfo> {
        val sysInfo = SysInfo(
                author = "gsc",
                framework = "Spring Boot 3.2.2",
                comment = "naive & dumb",
                language = "Graal VM & Kotlin 1.9.0",
                multithreaded = false,
        )

        LOG.info("sysInfo {}", sysInfo)

        return ResponseEntity.ok(sysInfo)
    }
}



