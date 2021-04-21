const express = require("express")
const https = require("https")
const fs = require("fs")
const app = express()

app.use(express.static(__dirname + '/static'));

app.get("/", (_req, res) => {
    res.render("index")
})

const server = https.createServer(
    { key: fs.readFileSync('./security/cert.key'), cert: fs.readFileSync('./security/cert.pem')},
    app
).listen(3000)

module.exports = server