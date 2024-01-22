import * as db from './db';
import {Resolution, ResolutionModel} from "./models";

export const getResolutions = async (): Promise<Resolution[]> => {
    const client = await db.pool.connect();
    const result = await client.query('SELECT * FROM resolutions ');
    const tmp = result.rows as Array<ResolutionModel>;

    const original: ResolutionModel = tmp.find(r => r.resolution === "original") as ResolutionModel
    const others: ResolutionModel[] = tmp.filter(r => r.resolution !== "original")

    const tmp1: ResolutionModel[] = []
    tmp1.push(original)

    let resolutionModels = tmp1.concat(others);
    return resolutionModels.map(rm => {
        if (rm.resolution === "original") {
            const r: Resolution = {
                height: -1,
                id: rm.id,
                name: rm.resolution,
                original: true,
                width: -1
            }
            return r
        } else {
            const width_heigth = rm.resolution.split("x")
            const width = parseInt(width_heigth[0])
            const height = parseInt(width_heigth[1])
            const r: Resolution = {
                height: height,
                id: rm.id,
                name: rm.resolution,
                original: false,
                width: width
            }
            return r
        }
    })
}
