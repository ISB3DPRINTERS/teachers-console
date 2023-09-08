/* eslint-disable */

const http = require("http");
const fs = require("fs");
const path = require("path");

const host = process.env.HOST || "localhost";
const port = parseInt(process.env.PORT || "4000");

const extContentType = {
    ".js": "text/javascript",
    ".css": "text/css",
    ".json": "application/json",
    ".png": "image/png",
    ".jpg": "image/jpg",
    ".jpeg": "image/jpeg",
    ".html": "text/html",
    ".mp3": "audio/mpeg",
    ".wav": "audio/wav",
};

http.createServer((req, res) => {
    let filePath = "." + req.url;

    if (filePath.endsWith("/") || filePath == "./")
        filePath = "./index.html";
    
    if (!fs.existsSync(filePath) || fs.statSync(filePath).isDirectory())
        filePath = "./index.html";
    
    const ext = path.extname(filePath);
    const contentType = extContentType[ext] || "application/octet-stream";

    fs.readFile(filePath, (err, data) => {
        if (err) {
            res.writeHead(500);
            res.end("An unknown error occured.");
            res.end();
        }

        res.writeHead(200, { "Content-Type": contentType });
        res.end(data, "utf-8");
    });
}).listen(port, host);
