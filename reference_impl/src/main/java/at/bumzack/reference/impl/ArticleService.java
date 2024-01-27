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
import java.util.*;


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

//    public ArticleService() {
//    }


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
        final File currentDirFile = new File(".");
        final String helper = currentDirFile.getAbsolutePath();
        final var absPath = helper + "/" + sourceFilesFolder;
        LOG.info("absPath {}", absPath);

        final File folder = new File(absPath);

        return Arrays.stream(Objects.requireNonNull(folder.listFiles()))
                .filter(file -> file.getName().contains(".txt"))
                .sorted(Comparator.comparing(File::getName))
                .map(this::tryProcessFile)
                .reduce(new ImportResult(), ImportResult::sum, ImportResult::sum);
    }

    public ImportResult importArticles2() {
        final File currentDirFile = new File(".");
        final String helper = currentDirFile.getAbsolutePath();
        final var absPath = helper + "/" + sourceFilesFolder;

        final File folder = new File(absPath);

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

        String line = reader.readLine();
        final var article_grouped_by_code_and_pos = new ArrayList<ArticleModel>();
        final var articles_ready_to_write_to_db = new ArrayList<ArticleModel>();

        long linesProcessed = 0;
        long dbRowsWritten = 0;

        while (line != null) {
            final var article = line2article(line);
            if (!article_grouped_by_code_and_pos.isEmpty()) {
                final var last = article_grouped_by_code_and_pos.getLast();
                // group by code and pos
                if (last.getCode().equals(article.getCode()) && (last.getPos().equals(article.getPos()))) {
                    article_grouped_by_code_and_pos.add(article);
                } else {
                    final var cheapestArticle = article_grouped_by_code_and_pos.stream()
                            .sorted(Comparator.comparing(ArticleModel::getPrice))
                            .limit(1)
                            .toList();
                    articles_ready_to_write_to_db.add(cheapestArticle.getFirst());
                    article_grouped_by_code_and_pos.clear();
                }
            } else {
                article_grouped_by_code_and_pos.add(article);
            }
            linesProcessed++;

            if (articles_ready_to_write_to_db.size() > 50) {
                // articleRepository.saveAll(articles);
                dbRowsWritten += articles_ready_to_write_to_db.size();
                articles_ready_to_write_to_db.clear();
            }

            line = reader.readLine();
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
