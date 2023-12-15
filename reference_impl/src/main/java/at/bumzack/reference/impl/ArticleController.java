package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.FullArticle;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;


@RestController
@RequestMapping("/api")
public class ArticleController {

    private final FullArticleService fullArticleService;

    public ArticleController(FullArticleService fullArticleService) {
        this.fullArticleService = fullArticleService;
    }

    @GetMapping("/articles")
    public ResponseEntity<List<FullArticle>> getAllTutorials() {
        final var fullArticles = fullArticleService.findAll();
        return ResponseEntity.ok(fullArticles);
    }
}
