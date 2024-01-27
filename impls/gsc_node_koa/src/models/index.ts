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

export interface SysInfo {
    author: string;
    language: string;
    framework: string;
    multithreaded: boolean;
    comment: string;
}
