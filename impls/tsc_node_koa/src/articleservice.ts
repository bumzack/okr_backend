 import * as db from './db';
import {ArticleModel} from "./models";

const nReadlines = require('n-readlines');

export const getArticles = async (offset: number, limit: number): Promise<ArticleModel[]> => {
    const client = await db.pool.connect()
    const result = await client.query(`SELECT *
                                       FROM articles
                                       ORDER BY code ASC
                                       LIMIT ${limit} OFFSET ${offset} `)
    return result.rows as Array<ArticleModel>
}


export const insertArticle = async (article: ArticleModel): Promise<void> => {
    const client = await db.pool.connect()
    const result = await client.query(`SELECT *
                                       FROM articles
                                       ORDER BY code ASC
                                       LIMIT ${limit} OFFSET ${offset} `)
    return result.rows as Array<ArticleModel>
}


export const importArticles = async (): Promise<void> => {

    const broadbandLines = new nReadlines('broadband.sql');

    let line;
    let lineNumber = 1;

    while (line = broadbandLines.next()) {
        const article = convert_to_article(line);
        console.log(`Line ${lineNumber} has: ${line.toString('ascii')}`);
        lineNumber++;
    }

    console.log('end of file.');

    const client = await db.pool.connect()
    const result = await client.query(`SELECT *
                                       FROM articles
                                       ORDER BY code ASC
                                       LIMIT ${limit} OFFSET ${offset} `)
    return
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

    const code: string = trim0(line.substring(0, start_title).trim())
    const title: string = line.substring(start_title, start_desc - 1).trim()
    const desc: string = line.substring(start_desc, start_attr - 1).trim()
    const attr: string = line.substring(start_attr, start_cat - 1).trim()
    const cat: string = line.substring(start_cat, start_pos - 1).trim()
    const pos: string = trim0(line.substring(start_pos, start_price - 1).trim())
    const price: number = parseFloat(line.substring(start_price, start_start_date - 1).trim())
    const startDate: Date = new Date(parseInt(line.substring(start_start_date, start_end_date - 1).trim()) * 1000)
    const endDate: Date = new Date(parseInt(line.substring(start_end_date).trim()) * 1000)

    return  {
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
