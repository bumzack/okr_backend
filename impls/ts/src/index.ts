import Router from 'koa-router';
import koaBody from 'koa-body';
import Koa from 'koa';
import logger from 'koa-logger';


import * as db from './db';
import * as dotenv from "dotenv";
import {getResolutions} from "./resolutionservice";
import {getArticles} from "./articleservice";
import {getArt2Img} from "./art2imgservice";

const router = new Router();
const app = new Koa();

app.use(logger())
app.use(koaBody());
app.use(router.routes());
app.use(router.allowedMethods());

dotenv.config()

const port = process.env.PORT || 5000


router.get('/api/v1/resolutions', async (ctx: Koa.Context, next: Koa.Next) => {
    const client = await db.pool.connect();
    //  const email = ctx.params.email;
    // console.log(`got an email?:   ${email}`);
    try {
        const articles = await getArticles(0, 3);
        console.log(`got articles: ${JSON.stringify(articles, null, 4)}`);

        const article_id = articles[0].id
        const art2imgs = await getArt2Img(article_id)
        console.log(`got art2imgs: ${JSON.stringify(art2imgs, null, 4)}`)

        const resolutions = await getResolutions()
        console.log(`got resolutions: ${JSON.stringify(resolutions, null, 4)}`)
        if (resolutions.length > 0) {
            ctx.body = JSON.stringify(resolutions)
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