package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.SysInfo;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;


@RestController
@RequestMapping("/api")
public class ArticleController {
    private static final Logger LOG = LogManager.getLogger(ArticleController.class);

    private final ArticleServiceV1 articleServiceV1;

    private final ArticleServiceV2 articleServiceV2;


    public ArticleController(final ArticleServiceV1 articleServiceV1,
                             final ArticleServiceV2 articleServiceV2) {
        this.articleServiceV1 = articleServiceV1;
        this.articleServiceV2 = articleServiceV2;
    }

    @GetMapping("/v1/articles/{pageNumber}/{pageSize}")
    public ResponseEntity<List<Article>> findPaginatedV1(@PathVariable final int pageNumber,
                                                         @PathVariable final int pageSize) {
        LOG.info("findPaginated   pageNumber {}, pageSize {}", pageNumber, pageSize);
        final var fullArticles = articleServiceV1.findPaginated(pageNumber, pageSize);
        return ResponseEntity.ok(fullArticles);
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

    @GetMapping("/v2/articles/{pageNumber}/{pageSize}")
    public ResponseEntity<List<Article>> findPaginatedV2(@PathVariable final int pageNumber,
                                                         @PathVariable final int pageSize) {
        LOG.info("findPaginated   pageNumber {}, pageSize {}", pageNumber, pageSize);
        final var fullArticles = articleServiceV2.findPaginated(pageNumber, pageSize);
        return ResponseEntity.ok(fullArticles);
    }


    @GetMapping("/v2/sysinfo")
    public ResponseEntity<SysInfo> sysinfoV2() {
        final var sysInfo = new SysInfo();
        sysInfo.setAuthor("gsc");
        sysInfo.setFramework("Spring Boot 3.2.1");
        sysInfo.setComment("image manipulation optimised");
        sysInfo.setLanguage("Java 21");
        sysInfo.setMultithreaded(false);

        return ResponseEntity.ok(sysInfo);
    }


//    @GetMapping("/v3/articles/{pageNumber}/{pageSize}")
//    public ResponseEntity<List<ArticleAndImageModel>> findPaginatedV3(@PathVariable final int pageNumber,
//                                                                      @PathVariable final int pageSize) {
//        LOG.info("findPaginated   pageNumber {}, pageSize {}", pageNumber, pageSize);
//        final var fullArticles = articleServiceV3.findPaginated(pageNumber, pageSize);
//        return ResponseEntity.ok(fullArticles);
//    }
//
//
//    @GetMapping("/v3/sysinfo")
//    public ResponseEntity<SysInfo> sysinfoV3() {
//        final var sysInfo = new SysInfo();
//        sysInfo.setAuthor("gsc");
//        sysInfo.setFramework("Spring Boot 3.2.1");
//        sysInfo.setComment("image manipulation optimised; DB access optimized");
//        sysInfo.setLanguage("Java 21");
//        sysInfo.setMultithreaded(false);
//
//        return ResponseEntity.ok(sysInfo);
//    }


}
