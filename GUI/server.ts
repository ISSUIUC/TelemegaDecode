// import express from "express"
const express = require("express")
import { readFile } from "fs";
import { spawn } from "child_process";
import { GFSKMessage, parse_packet  } from "./packet";
// import path from "path";
const path = require("path")

const server = express();
let packet_buffer: {}[] = []

server.use(express.static(path.join(__dirname, 'public')));
server.get("/", (req,res)=>{
    res.render("index.html");
});
server.get("/getdata", (req,res)=>{
    if(packet_buffer.length > 0){
        res.json(packet_buffer[0])
        packet_buffer.splice(0,1);
    } else {
        res.json({})
    }
})

const gfsk = spawn("../GFSK/cmake-build-release/gfsk.exe", ["434650000"])
const decode = new TextDecoder();

let stdin_buff = "";

function ingest_message(msg: string) {
    const json: GFSKMessage  = JSON.parse(msg);
    switch(json.type){
        case "center":
            break
        case "packet":
            const packet = parse_packet(json);
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

gfsk.stdout.on("data", msg=>{
    const str = decode.decode(msg);
    for(let i = 0; i < str.length; i++){
        if(str[i] == '\n'){
            ingest_message(stdin_buff);
            stdin_buff = "";
        } else {
            stdin_buff += str[i];
        }
    }
})

gfsk.stderr.on("data", msg=>{
    console.log("stderr", decode.decode(msg).trimEnd())
})

gfsk.on("exit", code=>{
    console.log("Exit", code);
})

server.listen(8084, ()=>{
    console.log("Begin");
})
