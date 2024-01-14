const Pool = require('pg-pool')

const db =  process.env.DB || "dev"
const username =  process.env.USERNAME || "dev"
const password =  process.env.PASSWORD || "dev"
const port =  process.env.DBPORT || 5432
const host =  process.env.HOST || "localhost"


const pool = new Pool({
    host: host,
    database: db,
    user: username,
    password: password,
    port: port,
    ssl: false,
    max: 50, // set pool max size to 20
    idleTimeoutMillis: 1000, // close idle clients after 1 second
    connectionTimeoutMillis: 1000, // return an error after 1 second if connection could not be established
    maxUses: 7500, // close (and replace) a connection after it has been used 7500 times (see below for discussion)
});


export {pool};