package at.bumzack.refimpl

import at.bumzack.refimpl.dto.ImportRequest
import at.bumzack.refimpl.dto.ImportResult
import at.bumzack.refimpl.dto.SysInfo
import org.springframework.http.MediaType
import org.springframework.web.bind.annotation.*


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
        return articleService.importArticles()
    }

    @PostMapping(
        value = ["/v2/articles/import"],
        consumes = [MediaType.APPLICATION_JSON_VALUE],
        produces = [MediaType.APPLICATION_JSON_VALUE]
    )
    @ResponseBody
    fun importArticles2(@RequestBody request: ImportRequest): ImportResult {
        return articleService.importArticles2()
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
            language = "Graal VM & Kotlin 1.9.0",
            multithreaded = true,
            version = "v2",
        )
        return sysInfo
    }
}



