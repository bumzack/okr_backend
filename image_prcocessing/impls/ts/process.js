const {
    Worker,
} = require("worker_threads");


function runService() {
    return new Promise((resolve, reject) => {
        const worker = new Worker("./service.js", {workerData: "world"});
        worker.on("message", resolve);
        worker.on("error", reject);
        worker.on("exit", (code) => {
            if (code !== 0)
                reject(new Error(`Worker stopped with exit code ${code}`));
        });
    });
}

async function run() {
    const result = await runService("world xxx ");
    console.log(result);
}

run().catch((err) => console.error(err));
console.log("I should run immediately");
