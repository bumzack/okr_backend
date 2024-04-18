package at.bumzack.reference.impl;

import static java.util.Objects.nonNull;

import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.ImportRequest;
import at.bumzack.reference.impl.dto.ImportResult;
import at.bumzack.reference.impl.dto.SysInfo;
import java.util.Comparator;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.ResponseBody;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api")
public class ArticleController {

    private static final Logger LOG = LoggerFactory.getLogger(
        ArticleController.class
    );

    private final ArticleService articleService;

    public ArticleController(final ArticleService articleService) {
        this.articleService = articleService;
    }

    @PostMapping(
        value = "/v1/articles/import",
        consumes = MediaType.APPLICATION_JSON_VALUE,
        produces = MediaType.APPLICATION_JSON_VALUE
    )
    @ResponseBody
    public ImportResult importArticles(
        @RequestBody final ImportRequest request
    ) {
        final ImportResult importResult = articleService.importArticles(
            request.isReturnItems()
        );
        if (nonNull(importResult.getArticles())) {
            final var sorted = importResult
                .getArticles()
                .stream()
                .sorted(Comparator.comparing(Article::getPos))
                .sorted(Comparator.comparing(Article::getCode))
                .toList();
            importResult.setArticles(sorted);
        }
        return importResult;
    }

    @PostMapping(
        value = "/v2/articles/import",
        consumes = MediaType.APPLICATION_JSON_VALUE,
        produces = MediaType.APPLICATION_JSON_VALUE
    )
    @ResponseBody
    public ImportResult importArticles2(
        @RequestBody final ImportRequest request
    ) {
        final ImportResult importResult = articleService.importArticlesParallel(
            request.isReturnItems()
        );
        if (nonNull(importResult.getArticles())) {
            final var sorted = importResult
                .getArticles()
                .stream()
                .sorted(Comparator.comparing(Article::getPos))
                .sorted(Comparator.comparing(Article::getCode))
                .toList();
            importResult.setArticles(sorted);
        }
        return importResult;
    }

    @GetMapping("/v1/sysinfo")
    @ResponseBody
    public SysInfo sysinfo() {
        final var sysInfo = new SysInfo();
        sysInfo.setAuthor("gsc");
        sysInfo.setFramework("Spring Boot 3.2.2");
        sysInfo.setComment("naive & dumb");
        sysInfo.setLanguage("Java 21 // JVM");
        sysInfo.setMultithreaded(false);
        sysInfo.setVersion("v1");

        return sysInfo;
    }

    @GetMapping("/v2/sysinfo")
    @ResponseBody
    public SysInfo sysinfo2() {
        final var sysInfo = new SysInfo();
        sysInfo.setAuthor("gsc");
        sysInfo.setFramework("Spring Boot 3.2.2");
        sysInfo.setComment("multithreading via parallelStream");
        sysInfo.setLanguage("Java 21 // JVM");
        sysInfo.setMultithreaded(true);
        sysInfo.setVersion("v2");

        return sysInfo;
    }

    @GetMapping("/v1g/sysinfo")
    @ResponseBody
    public SysInfo sysinfog() {
        final var sysInfo = new SysInfo();
        sysInfo.setAuthor("gsc");
        sysInfo.setFramework("Spring Boot 3.2.2");
        sysInfo.setComment("naive & dumb");
        sysInfo.setLanguage("Java 21 // GraalVM");
        sysInfo.setMultithreaded(false);
        sysInfo.setVersion("v1");

        return sysInfo;
    }

    @GetMapping("/v2g/sysinfo")
    @ResponseBody
    public SysInfo sysinfo2g() {
        final var sysInfo = new SysInfo();
        sysInfo.setAuthor("gsc");
        sysInfo.setFramework("Spring Boot 3.2.2");
        sysInfo.setComment("multithreading via parallelStream");
        sysInfo.setLanguage("Java 21 // GraalVM");
        sysInfo.setMultithreaded(true);
        sysInfo.setVersion("v2");

        return sysInfo;
    }
}
