"use strict";
exports.__esModule = true;
// import express from "express"
var express = require("express");
var child_process_1 = require("child_process");
var packet_1 = require("./packet");
// import path from "path";
var path = require("path");
var server = express();
var packet_buffer = [];
server.use(express.static(path.join(__dirname, 'public')));
server.get("/", function (req, res) {
    res.render("index.html");
});
server.get("/getdata", function (req, res) {
    if (packet_buffer.length > 0) {
        res.json(packet_buffer[0]);
        packet_buffer.splice(0, 1);
    }
    else {
        res.json({});
    }
});
var gfsk = (0, child_process_1.spawn)("../GFSK/cmake-build-release/gfsk.exe", ["434650000"]);
var decode = new TextDecoder();
var stdin_buff = "";
function ingest_message(msg) {
    var json = JSON.parse(msg);
    switch (json.type) {
        case "center":
            break;
        case "packet":
            var packet = (0, packet_1.parse_packet)(json);
            packet_buffer.push(packet);
            break;
        case "closed":
            break;
        case "error":
            break;
        case "gain":
            break;
    }
}
gfsk.stdout.on("data", function (msg) {
    var str = decode.decode(msg);
    for (var i = 0; i < str.length; i++) {
        if (str[i] == '\n') {
            ingest_message(stdin_buff);
            stdin_buff = "";
        }
        else {
            stdin_buff += str[i];
        }
    }
});
gfsk.stderr.on("data", function (msg) {
    console.log("stderr", decode.decode(msg).trimEnd());
});
gfsk.on("exit", function (code) {
    console.log("Exit", code);
});
server.listen(8084, function () {
    console.log("Begin");
});
