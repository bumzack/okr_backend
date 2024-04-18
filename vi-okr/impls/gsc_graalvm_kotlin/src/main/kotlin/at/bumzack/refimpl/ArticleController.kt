package at.bumzack.refimpl

import at.bumzack.refimpl.dto.Article
import at.bumzack.refimpl.dto.ImportRequest
import at.bumzack.refimpl.dto.ImportResult
import at.bumzack.refimpl.dto.SysInfo
import org.springframework.http.MediaType
import org.springframework.web.bind.annotation.*
import java.util.*


@RestController
@RequestMapping("/api")
class ArticleController(
    val articleService: ArticleService
) {
    @PostMapping(
        value = ["/v1/articles/import"],
        consumes = [MediaType.APPLICATION_JSON_VALUE],
        produces = [MediaType.APPLICATION_JSON_VALUE]
    )
    @ResponseBody
    fun importArticles(@RequestBody request: ImportRequest): ImportResult {
        val importResult = articleService.importArticles(request.returnItems)
        if (Objects.nonNull(importResult.articles)) {
            val sorted = importResult.articles.stream()
                .sorted(Comparator.comparing(Article::pos))
                .sorted(Comparator.comparing(Article::code))
                .toList()
            importResult.articles = sorted
        }
        return importResult
    }

    @PostMapping(
        value = ["/v2/articles/import"],
        consumes = [MediaType.APPLICATION_JSON_VALUE],
        produces = [MediaType.APPLICATION_JSON_VALUE]
    )
    @ResponseBody
    fun importArticles2(@RequestBody request: ImportRequest): ImportResult {
        val importResult: ImportResult = articleService.importArticlesParallel(request.returnItems)
        if (Objects.nonNull(importResult.articles)) {
            val sorted = importResult.articles.stream()
                .sorted(Comparator.comparing(Article::pos))
                .sorted(Comparator.comparing(Article::code))
                .toList()
            importResult.articles = sorted
        }
        return importResult
    }


    @GetMapping("/v1/sysinfo")
    @ResponseBody
    fun sysinfo(): SysInfo {
        val sysInfo = SysInfo(
            author = "gsc",
            framework = "Spring Boot 3.2.2",
            comment = "naive & dumb",
            language = "Kotlin 1.9.0 // JVM",
            multithreaded = false,
            version = "v1",
        )
        return sysInfo
    }

    @GetMapping("/v2/sysinfo")
    @ResponseBody
    fun sysinfo2(): SysInfo {
        val sysInfo = SysInfo(
            author = "gsc",
            framework = "Spring Boot 3.2.2",
            comment = "parallel streams & dumb",
            language = "Kotlin 1.9.0 // JVM",
            multithreaded = true,
            version = "v2",
        )
        return sysInfo
    }

     @GetMapping("/v1g/sysinfo")
    @ResponseBody
    fun sysinfog(): SysInfo {
        val sysInfo = SysInfo(
            author = "gsc",
            framework = "Spring Boot 3.2.2",
            comment = "naive & dumb",
            language = "Kotlin 1.9.0 // GraalVM",
            multithreaded = false,
            version = "v1",
        )
        return sysInfo
    }

    @GetMapping("/v2g/sysinfo")
    @ResponseBody
    fun sysinfo2g(): SysInfo {
        val sysInfo = SysInfo(
            author = "gsc",
            framework = "Spring Boot 3.2.2",
            comment = "parallel streams & dumb",
            language = "Kotlin 1.9.0 // GraalVM",
            multithreaded = true,
            version = "v2",
        )
        return sysInfo
    }
}
