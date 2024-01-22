import Router from 'koa-router';
import koaBody from 'koa-body';
import Koa from 'koa';
import logger from 'koa-logger';


import * as db from './db';
import * as dotenv from "dotenv";
import {getArticles} from "./articleservice";

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

        const offset = pagenumber * pagesize
        const articles = await getArticles(offset, pagesize);
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


router.post('/api/v1/articles/import', async (ctx: Koa.Context, next: Koa.Next) => {
    const client = await db.pool.connect();
    try {
        const articles = await importArticles();
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