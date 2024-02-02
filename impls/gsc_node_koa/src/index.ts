import Router from 'koa-router';
import koaBody from 'koa-body';
import Koa from 'koa';
import logger from 'koa-logger';


import * as dotenv from "dotenv";
import {ImportRequest, SysInfo} from "./models";
import {importArticles} from "./articleservice";

const router = new Router();
const app = new Koa();

app.use(logger())
app.use(koaBody());
app.use(router.routes());
app.use(router.allowedMethods());

dotenv.config()

const port = process.env.PORT || 5000


router.get('/api/v1/sysinfo', async (ctx: Koa.Context, next: Koa.Next) => {
    const sysinfo: SysInfo = {
        author: "gsc",
        comment: "TypeScript ",
        framework: "koajs",
        language: "node / TypeScript",
        multithreaded: false,
        version: "v1"
    }
    ctx.body = JSON.stringify(sysinfo)
    await next;
});

router.post('/api/v1/articles/import', async (ctx: Koa.Context, next: Koa.Next) => {
    const req = ctx.request.body as ImportRequest;

    const res = await importArticles(req.returnItems);
    res.articles = res.articles
        .sort((a, b) => {
            return (a > b ? -1 : 1)
        })
        .sort((a, b) => {
            return (a > b ? -1 : 1)
        })

    ctx.body = JSON.stringify(res)
    await next;
});

app.listen(port, () => {
    console.log(`started. listening on port ${port}`);
});