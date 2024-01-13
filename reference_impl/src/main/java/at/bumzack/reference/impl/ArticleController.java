package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Article;
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

    private final ArticleService articleService;

    public ArticleController(ArticleService articleService) {
        this.articleService = articleService;
    }

    @GetMapping("/articles/{pageNumber}/{pageSize}")
    public ResponseEntity<List<Article>> findPaginated(@PathVariable final int pageNumber,
                                                       @PathVariable final int pageSize) {
        LOG.info("findPaginated   pageNumber {}, pageSize {}", pageNumber, pageSize);
        final var fullArticles = articleService.findPaginated(pageNumber, pageSize);
        return ResponseEntity.ok(fullArticles);
    }

}
