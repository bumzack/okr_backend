import * as db from './db';
import {ArticleModel} from "./models";

export const getArticles = async (offset: number, limit: number): Promise<ArticleModel[]> => {
    const client = await db.pool.connect()
    const result = await client.query(`SELECT * FROM articles ORDER BY code ASC LIMIT ${limit}  OFFSET ${offset} `)
    return result.rows as Array<ArticleModel>
}
