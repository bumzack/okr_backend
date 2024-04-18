import {ArticleModel, ImportResult} from "./models";
import * as fs from "fs";
import moment from "moment";

const nReadlines = require('n-readlines');

export const importArticles = async (returnItems: boolean): Promise<ImportResult> => {
    const data = process.env.DATA as string
    console.log(`data  ${data}    returnItems  ${returnItems}`)
    const fileList = fs.readdirSync(data);
    const files = fileList.filter(f => {
        return f.toLowerCase().endsWith(".txt")
    })

    let db_rows_written = 0
    let lines_processed = 0

    let article_grouped_by_code_and_pos: Array<ArticleModel> = [];
    let articles_ready_to_write_to_db: Array<ArticleModel> = [];

    for (const f of files) {
        const s = `${data}/${f}`;
        console.log(`open file ${s}`)
        const broadbandLines = new nReadlines(s);
        let line: string | boolean = await broadbandLines.next();
        lines_processed++;

        let previousArticle: ArticleModel | undefined = undefined;
        // let article = await convert_to_article(line as string);

        while (true) {
            const article: ArticleModel = await convert_to_article(line as string);

            if (previousArticle === undefined) {
                // new grouping start - because first article ever
                article_grouped_by_code_and_pos.push(article)
            } else {
                // is article part of current group?
                if (article.code === previousArticle.code && article.pos === previousArticle.pos) {
                    article_grouped_by_code_and_pos.push(article)

                } else {
                    const cheapestArticle: ArticleModel = article_grouped_by_code_and_pos
                        .sort((a, b) => {
                            return a.price - b.price
                        })
                        .at(0) as ArticleModel
                    if (returnItems) {
                        articles_ready_to_write_to_db.push(cheapestArticle);
                    }
                    db_rows_written++;

                    // clear group and add article
                    article_grouped_by_code_and_pos = [];
                    article_grouped_by_code_and_pos.push(article);
                }
            }
            line = await broadbandLines.next();
            // console.log(`line  ${line}`)
            if (line === false) {
                break;
            }
            lines_processed++;
            previousArticle = article;
        }
        // write last article in file
        const cheapestArticle: ArticleModel = article_grouped_by_code_and_pos
            .sort((a, b) => {
                return a.price - b.price
            })
            .at(0) as ArticleModel
        if (returnItems) {
            articles_ready_to_write_to_db.push(cheapestArticle);
        }
        db_rows_written++;
        console.log(`end of file. ${s}`);
    }

    return {
        dbRowsWritten: db_rows_written,
        linesProcessed: lines_processed,
        articles: articles_ready_to_write_to_db,
    }
}


const LEN_CODE = 20;
const LEN_TITLE = 100;
const LEN_DESC = 1700;
const LEN_ATTRIBUTES = 200;
const LEN_CATEGORIES = 200;
const LEN_POS = 30;
const LEN_PRICE = 20;
const LEN_START_DATE = 25;
const LEN_END_DATE = 25;


const convert_to_article = async (line: string): Promise<ArticleModel> => {
    const start_title = LEN_CODE;
    const start_desc = start_title + LEN_TITLE;
    const start_attr = start_desc + LEN_DESC;
    const start_cat = start_attr + LEN_ATTRIBUTES;
    const start_pos = start_cat + LEN_CATEGORIES;
    const start_price = start_pos + LEN_POS;
    const start_start_date = start_price + LEN_PRICE;
    const start_end_date = start_start_date + LEN_START_DATE;
    //  const end_end_date = start_end_date + LEN_END_DATE;

    // console.log(`LINE  ${line}`)
    const code: string = trim0(String(line).substring(0, start_title).trim())
    const title: string = String(line).substring(start_title, start_desc).trim()
    const desc: string = String(line).substring(start_desc, start_attr).trim()
    const attr: string = String(line).substring(start_attr, start_cat).trim()
    const cat: string = String(line).substring(start_cat, start_pos).trim()
    const pos: string = trim0(String(line).substring(start_pos, start_price).trim())
    const s = String(line).substring(start_price, start_start_date);
    //  console.log(`price string  ${s}`)
    const price: number = parseFloat(s.trim())
    const startDate1: Date = new Date(parseInt(String(line).substring(start_start_date, start_end_date).trim()) * 1000)
    const endDate1: Date = new Date(parseInt(String(line).substring(start_end_date).trim()) * 1000)

    const startDate = (moment(startDate1)).format('YYYY-MM-DDTHH:mm:ss')
    const endDate = (moment(endDate1)).format('YYYY-MM-DDTHH:mm:ss')

    return {
        attributes: attr,
        categories: cat,
        code: code,
        description: desc,
        endDate,
        pos: pos,
        price: price,
        startDate,
        title: title
    };
}

const trim0 = (s: string): string => {
    let idx = 0
    while (s[idx] === '0') {
        idx++
    }
    return s.substring(idx)
}
