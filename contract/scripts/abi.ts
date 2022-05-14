import { readFileSync, writeFileSync } from "fs";
import path from "path";

(async () =>{
    let contractJson = JSON.parse(
        readFileSync(
            path.resolve(
                __dirname, '../artifacts/contracts/ProofOfExistence.sol/ProofOfExistence.json'),
                {encoding:'utf8'})
    );
    writeFileSync('./abi.json',JSON.stringify(contractJson.abi), {encoding: 'utf8'})
    writeFileSync('./bytecode.json',JSON.stringify(contractJson.bytecode), {encoding: 'utf8'})
})()