import * as db from './db';
import {Art2ImgModel} from "./models";

export const getArt2Img = async (article_id: number): Promise<Art2ImgModel[]> => {
    const client = await db.pool.connect();
    const result = await client.query(`SELECT * FROM art2img WHERE  article_id = ${article_id}`);
    const art2imgs = result.rows as Array<Art2ImgModel>;
    return art2imgs
}
