// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <=0.8.15;

contract ProofOfExistence {

    event ProofCreated(bytes32 indexed docMasterKeyHash, bytes32 indexed documentHash);

    address public owner;

    // hash & revocation stroage
    mapping (bytes32 => Proof) public proofs;

    struct Proof {
        bytes32 docHash;
        // not determined feature
        address author;
    }

    modifier onlyOwner() {
        require(owner == msg.sender, "Only owner is allowed to access this function.");
        _;
    }

    constructor() public {
        owner = msg.sender;
    }

    // hash 上鏈
    function notarizeHash(bytes32 docMasterKeyHash, bytes32 documentHash) public onlyOwner{
        require(proofs[docMasterKeyHash].docHash == bytes32(0), "The docHash has been writed.");
        proofs[docMasterKeyHash].docHash = documentHash;
        proofs[docMasterKeyHash].author = msg.sender;

        emit ProofCreated(docMasterKeyHash, documentHash);
    }

    function getHash(bytes32 docMasterKeyHash) public view returns (bytes32 documentHash){
        return proofs[docMasterKeyHash].docHash;
    }
}