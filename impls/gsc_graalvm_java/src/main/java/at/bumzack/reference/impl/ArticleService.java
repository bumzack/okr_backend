package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.ImportResult;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.math.BigDecimal;
import java.time.Instant;
import java.time.LocalDateTime;
import java.util.*;

import static java.util.Objects.isNull;
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
    private static final Logger LOG = LoggerFactory.getLogger(ArticleService.class);


    @Value("${sourcefilesFolder}")
    private String sourceFilesFolder;

    public ArticleService() {
    }

    public ImportResult importArticles(boolean returnItems) {
        LOG.info("sourceFilesFolder {}", sourceFilesFolder);
        final var folder = new File(sourceFilesFolder);

        return Arrays.stream(Objects.requireNonNull(folder.listFiles()))
                .filter(file -> file.getName().contains(".txt"))
                .sorted(Comparator.comparing(File::getName))
                .map(f -> tryProcessFile(f, returnItems))
                .reduce(new ImportResult(), ImportResult::sum, ImportResult::sum);
    }

    public ImportResult importArticles2(boolean returnItems) {
        final var folder = new File(sourceFilesFolder);

        return Arrays.stream(Objects.requireNonNull(folder.listFiles()))
                .filter(file -> file.getName().contains(".txt"))
                .sorted(Comparator.comparing(File::getName))
                .toList()
                .parallelStream()
                .map(f -> tryProcessFile(f, returnItems))
                .reduce(new ImportResult(), ImportResult::sum, ImportResult::sum);
    }

    private ImportResult tryProcessFile(final File f, boolean returnItems) {
        try {
            final var res = processFile(f, returnItems);
            LOG.info("filename {},  linesProcessed  {},   dbRowsWritten  {} ", f.getName(), res.getLinesProcessed(), res.getDbRowsWritten());
            return res;
        } catch (final IOException e) {
            LOG.error("error processing file ", e);
        }
        return null;
    }

    private ImportResult processFile(final File f, boolean returnItems) throws IOException {
        try (final var reader = new BufferedReader(new FileReader(f))) {
            long linesProcessed = 0;
            long dbRowsWritten = 0;

            String line = reader.readLine();
            linesProcessed++;

            final var article_grouped_by_code_and_pos = new ArrayList<Article>();
            final var articles_ready_to_write_to_db = new ArrayList<Article>();

            if (nonNull(line)) {
                Article article;
                Article prevArticle = null;
                while (true) {
                    article = line2article(line);
                    // LOG.info("line {},    article    code {}, pos {}, price  {}", linesProcessed, article.getCode(), article.getPos(), article.getPrice());

                    if (isNull(prevArticle)) {
                        // new grouping start - because first article ever
                        article_grouped_by_code_and_pos.add(article);
                    } else {
                        // is article part of current group?
                        if (article.getCode().equals(prevArticle.getCode()) && article.getPos().equals(prevArticle.getPos())) {
                            article_grouped_by_code_and_pos.add(article);
                        } else {
                            // article is not part of current group -> find cheapeast
                            final var cheapestArticle = article_grouped_by_code_and_pos.stream()
                                    .sorted(Comparator.comparing(Article::getPrice))
                                    .limit(1)
                                    .toList();
                            if (returnItems) {
                                articles_ready_to_write_to_db.add(cheapestArticle.getFirst());
                            }
                            dbRowsWritten++;

                            // clear group and add article
                            article_grouped_by_code_and_pos.clear();
                            article_grouped_by_code_and_pos.add(article);
                        }
                    }

                    line = reader.readLine();
                    if (isNull(line)) {
                        break;
                    }
                    linesProcessed++;
                    prevArticle = article;
                }

                // write last article in file
                final var cheapestArticle = article_grouped_by_code_and_pos.stream()
                        .sorted(Comparator.comparing(Article::getPrice))
                        .limit(1)
                        .toList();
                if (returnItems) {
                    articles_ready_to_write_to_db.add(cheapestArticle.getFirst());
                }
                dbRowsWritten++;

                // LOG.info("articles_ready_to_write_to_db   size   {}", articles_ready_to_write_to_db.size());

                // articles_ready_to_write_to_db.forEach(a -> LOG.info("article in DB  code {}, pos {}, price {}", a.getCode(), a.getPos(), a.getPrice()));
            }

            final var importResult = new ImportResult();
            importResult.setDbRowsWritten(dbRowsWritten);
            importResult.setLinesProcessed(linesProcessed);
            importResult.setArticles(articles_ready_to_write_to_db);

            return importResult;
        }
    }

    private Article line2article(final String line) {
        final var article = new Article();
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
        final String startDateStr = line.substring(beginStartDate, beginEndDate);
        final String endDateStr = line.substring(beginEndDate);
        final LocalDateTime start = LocalDateTime.ofInstant(Instant.ofEpochSecond(Long.parseLong(startDateStr)),
                TimeZone.getDefault().toZoneId());
        final LocalDateTime end = LocalDateTime.ofInstant(Instant.ofEpochSecond(Long.parseLong(endDateStr)),
                TimeZone.getDefault().toZoneId());
        article.setStartDate(LocalDateTime.from(start).toString());
        article.setEndDate(LocalDateTime.from(end).toString());
        return article;
    }

    private String trimLeadingZeroes(final String s) {
        int i = 0;
        while (s.charAt(i) == '0') {
            i++;
        }
        return s.substring(i);
    }
}
