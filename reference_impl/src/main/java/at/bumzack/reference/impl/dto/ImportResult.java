package at.bumzack.reference.impl.dto;

import java.util.ArrayList;
import java.util.List;

import static java.util.Objects.nonNull;

public class ImportResult {
    private long linesProcessed;

    private long dbRowsWritten;

    private List<Article> articles;

    public ImportResult() {
        this.dbRowsWritten = 0L;
        this.linesProcessed = 0L;
    }

    public static ImportResult sum(final ImportResult r1, final ImportResult r2) {
        final var combined = new ImportResult();
        combined.setDbRowsWritten(r1.getDbRowsWritten() + r2.getDbRowsWritten());
        combined.setLinesProcessed(r1.getLinesProcessed() + r2.getLinesProcessed());
        final var arr = new ArrayList<Article>();
        if (nonNull(r1.getArticles())) {
            arr.addAll(r1.getArticles());
        }
        if (nonNull(r2.getArticles())) {
            arr.addAll(r2.getArticles());
        }
        combined.setArticles(arr);
        return combined;
    }

    public long getLinesProcessed() {
        return linesProcessed;
    }

    public void setLinesProcessed(long linesProcessed) {
        this.linesProcessed = linesProcessed;
    }

    public long getDbRowsWritten() {
        return dbRowsWritten;
    }

    public void setDbRowsWritten(long dbRowsWritten) {
        this.dbRowsWritten = dbRowsWritten;
    }

    public List<Article> getArticles() {
        return articles;
    }

    public void setArticles(List<Article> articles) {
        this.articles = articles;
    }
}
