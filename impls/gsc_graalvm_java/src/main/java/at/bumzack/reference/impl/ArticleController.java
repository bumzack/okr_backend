package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.ImportRequest;
import at.bumzack.reference.impl.dto.ImportResult;
import at.bumzack.reference.impl.dto.SysInfo;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.MediaType;
import org.springframework.util.CollectionUtils;
import org.springframework.web.bind.annotation.*;

import java.util.ArrayList;
import java.util.Comparator;

import static java.util.Objects.nonNull;


@RestController
@RequestMapping("/api")
public class ArticleController {
    private static final Logger LOG = LoggerFactory.getLogger(ArticleController.class);

    private final ArticleService articleService;

    public ArticleController(final ArticleService articleService) {
        this.articleService = articleService;
    }

    @PostMapping(value = "/v1/articles/import", consumes = MediaType.APPLICATION_JSON_VALUE, produces = MediaType.APPLICATION_JSON_VALUE)
    @ResponseBody
    public ImportResult importArticles(@RequestBody final ImportRequest request) {
        final ImportResult importResult = articleService.importArticles();
        LOG.info("importArticles  rows {}", importResult.getArticles().size());
        if (request.isReturnItems()) {
            final var sorted = importResult.getArticles().stream()
                    .sorted(Comparator.comparing(Article::getPos))
                    .sorted(Comparator.comparing(Article::getCode))
                    .toList();
            importResult.setArticles(sorted);
        } else {
            importResult.setArticles(new ArrayList<>());
        }
        return importResult;
    }

    @PostMapping(value = "/v2/articles/import", consumes = MediaType.APPLICATION_JSON_VALUE, produces = MediaType.APPLICATION_JSON_VALUE)
    @ResponseBody
    public ImportResult importArticles2(@RequestBody final ImportRequest request) {
        final ImportResult importResult = articleService.importArticles2();
        LOG.info("importArticles2  rows {}", importResult.getArticles().size());
        if (request.isReturnItems()) {
            final var sorted = importResult.getArticles().stream()
                    .sorted(Comparator.comparing(Article::getPos))
                    .sorted(Comparator.comparing(Article::getCode))
                    .toList();
            importResult.setArticles(sorted);
        } else {
            importResult.setArticles(new ArrayList<>());
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
        sysInfo.setLanguage("Graal VM & Java 21");
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
        sysInfo.setLanguage("Graal VM & Java 21");
        sysInfo.setMultithreaded(true);
        sysInfo.setVersion("v2");

        return sysInfo;
    }
}
