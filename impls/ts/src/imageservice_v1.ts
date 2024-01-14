import * as db from './db';
import {ImageModel} from "./models";

export const getImages = async (imageIds: number[]): Promise<ImageModel[]> => {
    if (imageIds.length < 1) {
        return []
    }
    const client = await db.pool.connect()
    const ids = imageIds.join(",")
    const result = await client.query(`SELECT * FROM images WHERE id IN ( ${ids} ) `)
    return result.rows as Array<ImageModel>
}
