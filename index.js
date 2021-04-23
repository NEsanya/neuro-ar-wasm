const express = require("express")
const https = require("https")
const fs = require("fs")
const wifi = require("node-wifi")
const ws = require("ws")
const app = express()

app.use(express.static(__dirname + '/static'));

wifi.init({
    iface: null
})

app.get("/", (_req, res) => {
    res.render("index")
})

const server = https.createServer(
    { key: fs.readFileSync('./security/cert.key'), cert: fs.readFileSync('./security/cert.pem')},
    app
).listen(3000)

const wss = new ws.Server({ port: 8080 })
wss.on('connection', socket => {
    setInterval(() => wifi.scan((err, netw) => err ? console.log(err) : socket.send(JSON.stringify(netw))), 1000)
})

module.exports = server