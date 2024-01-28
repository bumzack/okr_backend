package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.ImportRequest;
import at.bumzack.reference.impl.dto.ImportResult;
import at.bumzack.reference.impl.dto.SysInfo;
import org.springframework.http.MediaType;
import org.springframework.web.bind.annotation.*;

import java.util.Comparator;


@RestController
@RequestMapping("/api")
public class ArticleController {
    private final ArticleService articleService;

    public ArticleController(final ArticleService articleService) {
        this.articleService = articleService;
    }


    @PostMapping(value = "/v1/articles/import", consumes = MediaType.APPLICATION_JSON_VALUE, produces = MediaType.APPLICATION_JSON_VALUE)
    @ResponseBody
    public ImportResult importArticles(@RequestBody final ImportRequest request) {
        final ImportResult importResult = articleService.importArticles(request.isReturnItems());
        final var sorted = importResult.getArticles().stream()
                .sorted(Comparator.comparing(Article::getPos))
                .sorted(Comparator.comparing(Article::getCode))
                .toList();
        importResult.setArticles(sorted);
        return importResult;
    }

    @PostMapping(value = "/v2/articles/import", consumes = MediaType.APPLICATION_JSON_VALUE, produces = MediaType.APPLICATION_JSON_VALUE)
    @ResponseBody
    public ImportResult importArticles2(@RequestBody final ImportRequest request) {
        final ImportResult importResult = articleService.importArticles2(request.isReturnItems());
        final var sorted = importResult.getArticles().stream()
                .sorted(Comparator.comparing(Article::getPos))
                .sorted(Comparator.comparing(Article::getCode))
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
        sysInfo.setComment("multithreading via parallelStream");
        sysInfo.setLanguage("Java 21");
        sysInfo.setMultithreaded(true);
        sysInfo.setVersion("v2");

        return sysInfo;
    }
}
