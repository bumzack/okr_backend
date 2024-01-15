import * as db from './db';
import {ArticleAndImage} from "./models";

export const getArticlesV3 = async (offset: number, limit: number): Promise<ArticleAndImage[]> => {
    const client = await db.pool.connect()
    const result = await client.query(`
SELECT articles.id,  articles.title,  articles.description, articles.code,
images.image_as_json_pixels_array, images.width, images.height, images.filename,
COUNT(articles.code) OVER(PARTITION BY articles.code) AS count
FROM articles
JOIN art2img ON art2img.article_id = articles.id
JOIN images ON art2img.image_id = images.id
ORDER BY articles.code ASC
LIMIT ${limit}  OFFSET ${offset} `)
    return result.rows as Array<ArticleAndImage>
}

//
// SELECT articles.id,  articles.title,  articles.description, articles.code,
//     images.image_as_json_pixels_array, images.width, images.height, images.filename
// FROM articles
// JOIN art2img ON art2img.article_id = articles.id
// JOIN images ON art2img.image_id = images.id
// ORDER BY articles.code ASC
// LIMIT 2  OFFSET 0;
