const { workerData, parentPort } = require('worker_threads')
const crypto = require("crypto");

// You can do any heavy stuff here, in a synchronous way
// without blocking the "main thread"
const codes = [];
for (let i = 0; i < 500; i++) {
    const val = doHeavyStuff(`${workerData}-${i + 1}`);
    codes.push(val);
}

function doHeavyStuff(item) {
    return crypto
        .createHmac("sha256", "secret")
        .update(new Array(10000).fill(item).join("."))
        .digest("hex");
}
parentPort.postMessage(codes);
