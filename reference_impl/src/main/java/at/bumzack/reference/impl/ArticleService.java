package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.ArticleModel;
import at.bumzack.reference.impl.dto.ImportResult;
import at.bumzack.reference.impl.repository.ArticleRepository;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Component;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.math.BigDecimal;
import java.time.Instant;
import java.time.LocalDateTime;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.Objects;
import java.util.TimeZone;

import static java.util.Objects.nonNull;


@Component
public class ArticleService {
    private static final int LEN_CODE = 20;
    private static final int LEN_TITLE = 100;
    private static final int LEN_DESC = 1700;

    private static final int LEN_ATTRIBUTES = 200;

    private static final int LEN_CATEGORIES = 200;

    private static final int LEN_POS = 30;
    private static final int LEN_PRICE = 20;
    private static final int LEN_START = 25;
    // private static final int LEN_END = 25;


    public static final String PROPERTY_CODE = "code";
    private static final Logger LOG = LogManager.getLogger(ArticleService.class);
    private final ArticleRepository articleRepository;


    @Value("${sourcefilesFolder}")
    private String sourceFilesFolder;

    public ArticleService(final ArticleRepository articleRepository) {
        this.articleRepository = articleRepository;
    }

    public List<Article> findPaginated(final int pageNumber, final int pageSize) {
        final var page = PageRequest.of(pageNumber, pageSize, Sort.by(Sort.Direction.ASC, PROPERTY_CODE));
        return convert(articleRepository.findAll(page).toList());
    }

    private List<Article> convert(final List<ArticleModel> source) {
        return source.stream()
                .map(this::toArticle)
                .toList();
    }

    public ImportResult importArticles() {
        final var folder = new File(sourceFilesFolder);

        return Arrays.stream(Objects.requireNonNull(folder.listFiles()))
                .filter(file -> file.getName().contains(".txt"))
                .sorted(Comparator.comparing(File::getName))
                .map(this::tryProcessFile)
                .reduce(new ImportResult(), ImportResult::sum, ImportResult::sum);
    }

    public ImportResult importArticles2() {
        final var folder = new File(sourceFilesFolder);

        return Arrays.stream(Objects.requireNonNull(folder.listFiles()))
                .filter(file -> file.getName().contains(".txt"))
                .sorted(Comparator.comparing(File::getName))
                .toList()
                .parallelStream()
                .map(this::tryProcessFile)
                .reduce(new ImportResult(), ImportResult::sum, ImportResult::sum);
    }

    private ImportResult tryProcessFile(final File f) {
        try {
            final var res = processFile(f);
            LOG.info("filename {},  linesProcessed  {},   dbRowsWritten  {} ", f.getName(), res.getLinesProcessed(), res.getDbRowsWritten());
            return res;
        } catch (final IOException e) {
            LOG.error("error processing file ", e);
        }
        return null;
    }

    private ImportResult processFile(final File f) throws IOException {
        final var reader = new BufferedReader(new FileReader(f));
        long linesProcessed = 0;
        long dbRowsWritten = 0;

        String line = reader.readLine();
        linesProcessed++;

        final var article_grouped_by_code_and_pos = new ArrayList<ArticleModel>();
        final var articles_ready_to_write_to_db = new ArrayList<ArticleModel>();

        if (nonNull(line)) {
            var article = line2article(line);
            while (line != null) {
                LOG.info("line {},    article    code {}, pos {}, price  {}", linesProcessed,article.getCode(), article.getPos(), article.getPrice());
                final var firstInGroup = article;

                // read until code && pos change
                while (nonNull(line) && article.getCode().equals(firstInGroup.getCode()) && article.getPos().equals(firstInGroup.getPos())) {
                    article_grouped_by_code_and_pos.add(article);
                    line = reader.readLine();
                    linesProcessed++;
                    if (nonNull(line)) {
                        article = line2article(line);
                        LOG.info("line {},    article    code {}, pos {}, price  {}", linesProcessed,article.getCode(), article.getPos(), article.getPrice());
                    }
                }

                LOG.info("article finished grouping in line {},      code {}, pos {}, cnt_articles  {}",linesProcessed,firstInGroup.getCode(), firstInGroup.getPos(), article_grouped_by_code_and_pos.size());

                // find cheapest article
                final var cheapestArticle = article_grouped_by_code_and_pos.stream()
                        .sorted(Comparator.comparing(ArticleModel::getPrice))
                        .limit(1)
                        .toList();
                articles_ready_to_write_to_db.add(cheapestArticle.getFirst());

                // reset array and add the first article that wa
                article_grouped_by_code_and_pos.clear();
                if (line != null) {
                    article_grouped_by_code_and_pos.add(article);
                }

                if (articles_ready_to_write_to_db.size() > 0) {
                    // articleRepository.saveAll(articles);
                    dbRowsWritten += articles_ready_to_write_to_db.size();
                    articles_ready_to_write_to_db.forEach(a -> {
                        LOG.info("article ready to write to DB   code {}, pos {}", a.getCode(), a.getPos());
                    });
                    articles_ready_to_write_to_db.clear();
                }

                line = reader.readLine();
            }
        }

        final var importResult = new ImportResult();
        importResult.setDbRowsWritten(dbRowsWritten);
        importResult.setLinesProcessed(linesProcessed);
        return importResult;
    }


    private ArticleModel line2article(final String line) {
        final var article = new ArticleModel();
        int beginDesc = LEN_CODE + LEN_TITLE;
        int beginAttr = LEN_CODE + LEN_TITLE + LEN_DESC;
        int beginCat = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES;
        int beginPos = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES;
        int beginPrice = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS;
        int beginStartDate = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS + LEN_PRICE;
        int beginEndDate = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS + LEN_PRICE + LEN_START;

        article.setCode(trimLeadingZeroes(line.substring(0, LEN_CODE)));
        article.setTitle(line.substring(LEN_CODE, beginDesc).trim());
        article.setDescription(line.substring(beginDesc, beginAttr).trim());
        article.setAttributes(line.substring(beginAttr, beginCat).trim());
        article.setCategories(line.substring(beginCat, beginPos).trim());
        article.setPos(trimLeadingZeroes(line.substring(beginPos, beginPrice).trim()));
        article.setPrice(BigDecimal.valueOf(Double.parseDouble(line.substring(beginPrice, beginStartDate))));
        article.setStartDate(LocalDateTime.from(LocalDateTime.ofInstant(Instant.ofEpochMilli(Long.parseLong(line.substring(beginStartDate, beginEndDate))),
                TimeZone.getDefault().toZoneId())));
        article.setEndDate(LocalDateTime.from(LocalDateTime.ofInstant(Instant.ofEpochMilli(Long.parseLong(line.substring(beginEndDate))),
                TimeZone.getDefault().toZoneId())));
        return article;
    }

    private String trimLeadingZeroes(final String s) {
        int i = 0;
        while (s.charAt(i) == '0') {
            i++;
        }
        return s.substring(i);
    }

    private Article toArticle(final ArticleModel article) {
        final var target = new Article();
        target.setAttributes(article.getAttributes());
        target.setCategories(article.getCategories());
        target.setCode(article.getCode());
        target.setDescription(article.getDescription());
        target.setId(article.getId());
        target.setPos(article.getPos());
        target.setTitle(article.getTitle());
        target.setStartDate(article.getStartDate().toString());
        target.setEndDate(article.getEndDate().toString());

        return target;
    }
}
