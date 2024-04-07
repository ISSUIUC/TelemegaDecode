// import express from "express"
const express = require("express")
import { appendFile, readFile } from "fs";
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
        // send all packets until theres none left
        res.json(packet_buffer)
        // empty the buffer
        packet_buffer = [];
    } else {
        res.json([])
    }
})

const gfsk = spawn("..\\cmake-build-release\\gfsk.exe", ["436350000", "436550000", "436750000"])
const date = new Date();
const log_path = "log" + date.getDay() + '.' + date.getHours() + '.' + date.getMinutes() + '.' + date.getSeconds();
console.log(log_path);
// const gfsk = spawn("..\\cmake-build-release\\gfsk.exe", ["434550000"])
const decode = new TextDecoder();

let stdin_buff = "";

function ingest_message(msg: string) {
    const json: GFSKMessage  = JSON.parse(msg);
    
    switch(json.type){
        case "packet":
            const packet = parse_packet(json);
            appendFile(log_path, JSON.stringify(packet), {}, ()=>{});
            packet_buffer.push(packet); // pushing packet to buffer
            break;
        case "center":
        case "closed":
        case "error":
        case "gain":
            console.log(json);
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
