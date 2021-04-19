const express = require("express")
const https = require("https")
const fs = require("fs")
const app = express()

app.get("/", (req, res) => {
    res.render("./static/index.html")
})
const server = https.createServer(
    { key: fs.readFileSync('./security/cert.key'), cert: fs.readFileSync('./security/cert.pem')},
    app
).listen(3000)