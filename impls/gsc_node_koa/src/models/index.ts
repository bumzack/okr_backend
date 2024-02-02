export interface ArticleModel {
    code: string;
    description: string;
    title: string;
    categories: string;
    attributes: string;
    price: number;
    pos: string;
    startDate: string;
    endDate: string;
}

export interface ImportResult {
    linesProcessed: number;
    dbRowsWritten: number;
    articles: Array<ArticleModel>;
}

export interface SysInfo {
    author: string;
    language: string;
    framework: string;
    multithreaded: boolean;
    comment: string;
    version: string;
}

export interface ImportRequest {
    returnItems: boolean;
}
