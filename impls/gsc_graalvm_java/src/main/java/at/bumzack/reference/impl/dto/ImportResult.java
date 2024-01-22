//package at.bumzack.reference.impl.dto;
//
//public class ImportResult {
//    private long linesProcessed;
//
//    private long dbRowsWritten;
//
//
//    public ImportResult() {
//        this.dbRowsWritten = 0L;
//        this.linesProcessed = 0L;
//    }
//
//    public Long getLinesProcessed() {
//        return linesProcessed;
//    }
//
//    public void setLinesProcessed(Long linesProcessed) {
//        this.linesProcessed = linesProcessed;
//    }
//
//    public Long getDbRowsWritten() {
//        return dbRowsWritten;
//    }
//
//    public void setDbRowsWritten(Long dbRowsWritten) {
//        this.dbRowsWritten = dbRowsWritten;
//    }
//
//    @Override
//    public String toString() {
//        return "ImportResult{" +
//                "linesProcessed=" + linesProcessed +
//                ", dbRowsWritten=" + dbRowsWritten +
//                '}';
//    }
//
//    public static ImportResult sum(ImportResult r1, ImportResult r2) {
//        final var combined = new ImportResult();
//        combined.setDbRowsWritten(r1.getDbRowsWritten() + r2.getDbRowsWritten());
//        combined.setLinesProcessed(r1.getLinesProcessed() + r2.getLinesProcessed());
//        return combined;
//    }
//}
