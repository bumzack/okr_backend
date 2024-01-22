import * as db from './db';
import {ArticleModel, ImportResult} from "./models";
import * as fs from "fs";

const nReadlines = require('n-readlines');

export const getArticles = async (offset: number, limit: number): Promise<ArticleModel[]> => {
    const client = await db.pool.connect()
    const result = await client.query(`SELECT *
                                       FROM articles
                                       ORDER BY code ASC
                                       LIMIT ${limit} OFFSET ${offset} `)
    return result.rows as Array<ArticleModel>
}


export const insertArticle = async (article: ArticleModel, client: any): Promise<void> => {
    //  console.log(`article ${JSON.stringify(article,null,4)}`)
    // const result = await client.query(`INSERT INTO articles (code, title, description, attributes, categories, pos,
    //                                                          price, start_date, end_date)
    //                                    VALUES ('${article.code}', '${article.title}', '${article.description}',
    //                                            '${article.attributes}', '${article.categories}', '${article.pos}',
    //                                            ${article.price}, '${article.start_date}', '${article.end_date}' ) `)

    let s = `INSERT INTO articles (code, title, description, attributes, categories, pos,
                                   price, start_date, end_date)
             VALUES ('${article.code}', '${article.title}', '${article.description}',
                     '${article.attributes}', '${article.categories}', '${article.pos}',
                     ${article.price}, now(), now()) `;
    //  console.log(`query  ${s} `)
    const result = await client.query(s)

    // console.log(`result from insert ${result}`)
    return;
}

export const importArticles = async (): Promise<ImportResult> => {
    const fileList = fs.readdirSync("/home/bumzack/stoff/rust/okr_backend/rust/");
    const files = fileList.filter(f => {
        console.log(`filename ${f.toLowerCase()}`)
        return f.toLowerCase().endsWith(".txt")
    })

    let db_rows_written = 0
    let lines_processed = 0
    const client = await db.pool.connect()

    console.log("XXXXXXXXXXXXXXXX ")
    files.forEach(f => console.log("file " + f))

    for (const f of files) {
        const s = `/home/bumzack/stoff/rust/okr_backend/rust/${f}`;
        console.log(`open file ${s}`)
        const broadbandLines = new nReadlines(s);
        let line;
        let articles: Array<ArticleModel> = [];
        let current_article_pos: Array<ArticleModel> = [];

        while (line = await broadbandLines.next()) {
            const article = await convert_to_article(line);
            let last = current_article_pos[current_article_pos.length - 1]

            if (current_article_pos.length > 0) {
                if (last.code === article.code && last.pos === article.pos) {
                    current_article_pos.push(article)
                } else {
                    current_article_pos.sort((a, b) => {
                        return a.price - b.price
                    })
                    articles.push(current_article_pos[0])
                    current_article_pos = []
                }
            } else {
                current_article_pos.push(article)
            }

            if (articles.length > 50) {
                // articles.forEach(a => {
                //     insertArticle(a, client)
                // })
                // console.log(`written articles to db ${articles.length},   lines_processed ${lines_processed}`)
                db_rows_written += articles.length;
                articles = []
            }
            lines_processed++
        }
        console.log('end of file.');
    }

    return {
        db_rows_written: db_rows_written,
        lines_processed: lines_processed,
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
    const title: string = String(line).substring(start_title, start_desc - 1).trim()
    const desc: string = String(line).substring(start_desc, start_attr - 1).trim()
    const attr: string = String(line).substring(start_attr, start_cat - 1).trim()
    const cat: string = String(line).substring(start_cat, start_pos - 1).trim()
    const pos: string = trim0(String(line).substring(start_pos, start_price - 1).trim())
    const price: number = parseFloat(String(line).substring(start_price, start_start_date - 1).trim())
    const startDate: Date = new Date(parseInt(String(line).substring(start_start_date, start_end_date - 1).trim()) * 1000)
    const endDate: Date = new Date(parseInt(String(line).substring(start_end_date).trim()) * 1000)

    return {
        attributes: attr,
        categories: cat,
        code: code,
        description: desc,
        end_date: endDate,
        pos: pos,
        price: price,
        start_date: startDate,
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
