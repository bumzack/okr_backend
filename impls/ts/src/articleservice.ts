import * as db from './db';
import {ArticlenModel} from "./models";

export const getArticles = async (offset: number, limit: number): Promise<ArticlenModel[]> => {
    const client = await db.pool.connect();
    const result = await client.query(`SELECT * FROM articles ORDER BY code ASC LIMIT ${limit}  OFFSET ${offset} `);
    const articles = result.rows as Array<ArticlenModel>;
    return articles
}
