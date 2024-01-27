package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.ImportResult;
import at.bumzack.reference.impl.dto.SysInfo;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.web.bind.annotation.*;

import java.util.Comparator;
import java.util.List;


@RestController
@RequestMapping("/api")
public class ArticleController {
    private static final Logger LOG = LogManager.getLogger(ArticleController.class);

    private final ArticleService articleService;


    public ArticleController(final ArticleService articleService) {
        this.articleService = articleService;
    }

    @GetMapping("/v1/articles/{pageNumber}/{pageSize}")
    @ResponseBody
    public List<Article> findPaginated(@PathVariable final int pageNumber,
                                       @PathVariable final int pageSize) {
        LOG.info("findPaginated   pageNumber {}, pageSize {}", pageNumber, pageSize);
        return articleService.findPaginated(pageNumber, pageSize);
    }

    @PostMapping("/v1/articles/import/{returnItems}")
    @ResponseBody
    public ImportResult importArticles(@PathVariable final boolean returnItems) {
        final ImportResult importResult = articleService.importArticles(returnItems);
        final var sorted = importResult.getArticles().stream()
                .sorted(Comparator.comparing(Article::getCode))
                .sorted(Comparator.comparing(Article::getPrice))
                .toList();
        importResult.setArticles(sorted);
        return importResult;
    }

    @PostMapping("/v2/articles/import/{returnItems}")
    @ResponseBody
    public ImportResult importArticles2(@PathVariable final boolean returnItems) {
        final ImportResult importResult = articleService.importArticles2(returnItems);
        final var sorted = importResult.getArticles().stream()
                .sorted(Comparator.comparing(Article::getCode))
                .sorted(Comparator.comparing(Article::getPrice))
                .toList();
        importResult.setArticles(sorted);
        return importResult;
    }

    @GetMapping("/v1/sysinfo")
    @ResponseBody
    public SysInfo sysinfo() {
        final var sysInfo = new SysInfo();
        sysInfo.setAuthor("gsc");
        sysInfo.setFramework("Spring Boot 3.2.2");
        sysInfo.setComment("naive & dumb");
        sysInfo.setLanguage("Java 21");
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
        sysInfo.setComment("naive & dumb");
        sysInfo.setLanguage("Java 21");
        sysInfo.setMultithreaded(true);
        sysInfo.setVersion("v2");

        return sysInfo;
    }
}
