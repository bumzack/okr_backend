package at.bumzack.reference.impl;

import at.bumzack.reference.impl.dto.Article;
import at.bumzack.reference.impl.dto.ArticleModel;
import at.bumzack.reference.impl.repository.ArticleRepository;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Component;

import java.util.List;


@Component
public class ArticleService {

    public static final String PROPERTY_CODE = "code";
    private static final Logger LOG = LoggerFactory.getLogger(ArticleService.class);
    private final ArticleRepository articleRepository;


    @Value("${sourcefilesFolder}")
    private String sourceFilesFolder;


    public ArticleService(final ArticleRepository articleRepository) {
        this.articleRepository = articleRepository;
    }

    public List<Article> findPaginated(final int pageNumber, final int pageSize) {
        final var p = PageRequest.of(pageNumber, pageSize, Sort.by(Sort.Direction.ASC, PROPERTY_CODE));
        return convert(articleRepository.findAll(p).toList());
    }

    private List<Article> convert(final List<ArticleModel> source) {
        return source.stream()
                .map(a -> {
                    final var target = new Article();
                    target.setAttributes(a.getAttributes());
                    target.setCategories(a.getCategories());
                    target.setCode(a.getCode());
                    target.setDescription(a.getDescription());
                    target.setId(a.getId());
                    target.setPos(a.getPos());
                    target.setTitle(a.getTitle());
                    target.setStartDate(a.getStartDate()!=null ? a.getStartDate().toString(): "v");
                    target.setEndDate(a.getEndDate()!=null ? a.getEndDate().toString(): "v");

                    return target;
                }).toList();
    }
//
//    public ImportResult importArticles() {
//        final File currentDirFile = new File(".");
//        final String helper = currentDirFile.getAbsolutePath();
//
//        LOG.info("currentDirFile {}", currentDirFile.getName());
//        LOG.info("helper {}", helper);
//        LOG.info("sourceFilesFolder {}", sourceFilesFolder);
//        final var absPath = helper + "/" + sourceFilesFolder;
//        LOG.info("absPath {}", absPath);
//
//        final File folder = new File(absPath);
//
//        for (final File fileEntry : Objects.requireNonNull(folder.listFiles())) {
//            if (fileEntry.isDirectory()) {
//                LOG.info("directory {}", fileEntry.getName());
//            } else {
//                LOG.info("file {}", fileEntry.getName());
//            }
//        }
//
//        final var fileNames = Arrays.stream(Objects.requireNonNull(folder.listFiles()))
//                .filter(file -> file.getName().contains(".txt"))
//                .toList();
//
//        LOG.info("===================================================================   ");
//        LOG.info("filenames   ");
//        fileNames
//                .forEach(LOG::info);
//        LOG.info("===================================================================   ");
//
//        return fileNames.stream()
//                .sorted(Comparator.comparing(File::getName))
//                .map(this::tryProcessFile)
//                .reduce(new ImportResult(), ImportResult::sum, ImportResult::sum);
//    }
//
//    private ImportResult tryProcessFile(final File f) {
//        try {
//            final var res = processFile(f);
//            LOG.info("filename {}  ,  linesProcessed  {},   dbRowsWritten  {} ", f.getName(), res.getLinesProcessed(), res.getDbRowsWritten());
//            return res;
//        } catch (IOException e) {
//            throw new RuntimeException(e);
//        }
//    }
//
//    private ImportResult processFile(final File f) throws IOException {
//        final BufferedReader reader = new BufferedReader(new FileReader(f));
//
//        String line = reader.readLine();
//        final var tmp = new ArrayList<ArticleModel>();
//        final var articles = new ArrayList<ArticleModel>();
//        long linesProcessed = 0;
//        long dbRowsWritten = 0;
//        while (line != null) {
//            final var article = processLine(line);
//            if (!tmp.isEmpty()) {
//                final var last = tmp.getLast();
//                // group by code and pos
//                if (last.getCode().equals(article.getCode()) && (last.getPos().equals(article.getPos()))) {
//                    tmp.add(article);
//                } else {
//                    final var c = tmp.stream()
//                            .sorted(Comparator.comparing(ArticleModel::getPrice))
//                            .limit(1)
//                            .toList();
//                    articles.add(c.getFirst());
//                    tmp.clear();
//                }
//            } else {
//                tmp.add(article);
//            }
//            linesProcessed++;
//
//            if (articles.size() > 50) {
//                // articles.forEach(LOG::info);
//                // articleRepository.saveAll(articles);
//                dbRowsWritten += articles.size();
//                //  LOG.info("filename {}  ,  {} articles  written", f.getName(), articles.size());
//                articles.clear();
//            }
//
//            line = reader.readLine();
//        }
//
//        final var importResult = new ImportResult();
//        importResult.setDbRowsWritten(dbRowsWritten);
//        importResult.setLinesProcessed(linesProcessed);
//
//        return importResult;
//    }
//
//    private static final int LEN_CODE = 20;
//    private static final int LEN_TITLE = 100;
//    private static final int LEN_DESC = 1700;
//
//    private static final int LEN_ATTRIBUTES = 200;
//
//    private static final int LEN_CATEGORIES = 200;
//
//    private static final int LEN_POS = 30;
//    private static final int LEN_PRICE = 20;
//    private static final int LEN_START = 25;
//    // private static final int LEN_END = 25;
//
//
//    private ArticleModel processLine(final String line) {
//        final var article = new ArticleModel();
//        int beginDesc = LEN_CODE + LEN_TITLE;
//        int beginAttr = LEN_CODE + LEN_TITLE + LEN_DESC;
//        int beginCat = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES;
//        int beginPos = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES;
//        int beginPrice = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS;
//        int beginStartDate = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS + LEN_PRICE;
//        int beginEndDate = LEN_CODE + LEN_TITLE + LEN_DESC + LEN_ATTRIBUTES + LEN_CATEGORIES + LEN_POS + LEN_PRICE + LEN_START;
//
//        article.setCode(trimLeadingZeroes(line.substring(0, LEN_CODE)));
//        article.setTitle(line.substring(LEN_CODE, beginDesc).trim());
//        article.setDescription(line.substring(beginDesc, beginAttr).trim());
//        article.setAttributes(line.substring(beginAttr, beginCat).trim());
//        article.setCategories(line.substring(beginCat, beginPos).trim());
//        article.setPos(trimLeadingZeroes(line.substring(beginPos, beginPrice).trim()));
//        article.setPrice(BigDecimal.valueOf(Double.parseDouble(line.substring(beginPrice, beginStartDate))));
//        article.setStartDate(LocalDateTime.from(LocalDateTime.ofInstant(Instant.ofEpochMilli(Long.parseLong(line.substring(beginStartDate, beginEndDate))),
//                TimeZone.getDefault().toZoneId())));
//        article.setEndDate(LocalDateTime.from(LocalDateTime.ofInstant(Instant.ofEpochMilli(Long.parseLong(line.substring(beginEndDate))),
//                TimeZone.getDefault().toZoneId())));
//        return article;
//    }
//
//    private String trimLeadingZeroes(final String s) {
//        int i = 0;
//        while (s.charAt(i) == '0') {
//            i++;
//        }
//        return s.substring(i);
//    }
}
