import { ethers } from "hardhat";

(async() =>{
    try{
        const ProofOfExistence = await ethers.getContractFactory('ProofOfExistence');
        const proofOfExistence = await ProofOfExistence.deploy();
        await proofOfExistence.deployed();
        console.log("ProofOfExistence deployed to:", proofOfExistence.address);
    }catch(err){
        console.log(err)
    }
})()