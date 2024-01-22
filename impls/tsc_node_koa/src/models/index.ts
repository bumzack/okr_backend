export interface ArticleModel {
    code: string;
    title: string;
    description: string;
    attributes: string;
    categories: string;
    pos: string;
    price: number;
    start_date: Date;
    end_date: Date;
}

export interface ImportResult {
    lines_processed: number;
    db_rows_written: number;
}
