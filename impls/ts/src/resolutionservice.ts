import * as db from './db';
import {ResolutionModel} from "./models";

export const getResolutions = async (): Promise<ResolutionModel[]> => {
    const client = await db.pool.connect();
    const result = await client.query('SELECT * FROM resolutions ');
    const resolutions = result.rows as Array<ResolutionModel>;

    const original: ResolutionModel = resolutions.find(r => r.resolution === "original") as ResolutionModel
    const others: ResolutionModel[] = resolutions.filter(r => r.resolution !== "original")

    const tmp: ResolutionModel[] = []
    tmp.push(original)

    return tmp.concat(others)
}
