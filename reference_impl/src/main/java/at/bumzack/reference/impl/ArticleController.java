package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.ImportResult;
import at.bumzack.reference.impl.dto.SysInfo;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.io.IOException;
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
    public ResponseEntity<List<Article>> findPaginatedV1(@PathVariable final int pageNumber,
                                                         @PathVariable final int pageSize) {
        LOG.info("findPaginated   pageNumber {}, pageSize {}", pageNumber, pageSize);
        final var fullArticles = articleService.findPaginated(pageNumber, pageSize);
        return ResponseEntity.ok(fullArticles);
    }

    @PostMapping("/v1/articles/import")
    public ResponseEntity<ImportResult> importArticles() throws IOException {
        final var res = articleService.importArticles();

        return ResponseEntity.ok(res);
    }

    @GetMapping("/v1/sysinfo")
    public ResponseEntity<SysInfo> sysinfoV1() {
        final var sysInfo = new SysInfo();
        sysInfo.setAuthor("gsc");
        sysInfo.setFramework("Spring Boot 3.2.1");
        sysInfo.setComment("naive & dumb");
        sysInfo.setLanguage("Java 21");
        sysInfo.setMultithreaded(false);

        return ResponseEntity.ok(sysInfo);
    }


}
