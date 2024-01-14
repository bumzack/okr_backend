import Router from 'koa-router';
import koaBody from 'koa-body';
import Koa from 'koa';
import logger from 'koa-logger';


import * as db from './db';
import * as dotenv from "dotenv";
import {getResolutions} from "./resolutionservice";
import {Resolution} from "./models";
import {processAllImagesV1} from "./image-utils-v1";
import {processAllImagesV2} from "./image-utils-v2";
import {getArticlesV3} from "./articleservice_v3";

const router = new Router();
const app = new Koa();

app.use(logger())
app.use(koaBody());
app.use(router.routes());
app.use(router.allowedMethods());

dotenv.config()

const port = process.env.PORT || 5000

router.get('/api/v1/articles/:pagenumber/:pagesize', async (ctx: Koa.Context, next: Koa.Next) => {
    const client = await db.pool.connect();
    const pagenumber = ctx.params.pagenumber as number;
    const pagesize = ctx.params.pagesize as number;
    console.log(`got url parameters?: pagenumber  ${pagenumber}  pagesize ${pagesize}  `);
    try {
        const resolutions: Array<Resolution> = await getResolutions()

        const offset = pagenumber * pagesize
        const articles = await processAllImagesV1(offset, pagesize, resolutions);
        if (articles.length > 0) {
            ctx.body = JSON.stringify(articles)
        } else {
            ctx.status = 404
        }
    } finally {
        client.release()
    }
    await next;
});


router.get('/api/v2/articles/:pagenumber/:pagesize', async (ctx: Koa.Context, next: Koa.Next) => {
    const client = await db.pool.connect();
    const pagenumber = ctx.params.pagenumber as number;
    const pagesize = ctx.params.pagesize as number;
    console.log(`got url parameters?: pagenumber  ${pagenumber}  pagesize ${pagesize}  `);
    try {
        const resolutions: Array<Resolution> = await getResolutions()

        const offset = pagenumber * pagesize
        const articles = await processAllImagesV2(offset, pagesize, resolutions);
        if (articles.length > 0) {
            ctx.body = JSON.stringify(articles)
        } else {
            ctx.status = 404
        }
    } finally {
        client.release()
    }
    await next;
});


router.get('/api/v3/articles/:pagenumber/:pagesize', async (ctx: Koa.Context, next: Koa.Next) => {
    const client = await db.pool.connect();
    const pagenumber = ctx.params.pagenumber as number;
    const pagesize = ctx.params.pagesize as number;
    console.log(`got url parameters?: pagenumber  ${pagenumber}  pagesize ${pagesize}  `);
    try {
        // const resolutions: Array<Resolution> = await getResolutions()

        const offset = pagenumber * pagesize
        const articles = await getArticlesV3(offset, pagesize);
        articles.forEach(e => {
            e.image_as_json_pixels_array = ""
            e.description = ""
        })
        if (articles.length > 0) {
            ctx.body = JSON.stringify(articles)
        } else {
            ctx.status = 404
        }
    } finally {
        client.release()
    }
    await next;
});

app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});