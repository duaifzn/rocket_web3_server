// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <=0.8.13;

contract ProofOfExistence {

    event ProofCreated(bytes32 indexed docMasterKeyHash, bytes32 indexed documentHash);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event IssuerAdded(address indexed account);
    event IssuerRemoved(address indexed account);
    event IssuerChecked(address indexed account);
    event ProofRevoked(bytes32 indexed docMasterKeyHash);

    address public owner;

    // issuers storage
    mapping (address => bool) public issuers;

    // hash & revocation stroage
    mapping (bytes32 => Proof) public proofs;

    struct Proof {
        bytes32 docHash;
        bool isRevoked;
        // not determined feature
        address author;
    }

    modifier onlyOwner() {
        require(owner == msg.sender, "Only owner is allowed to access this function.");
        _;
    }

    modifier onlyIssuer() {
        require(issuers[msg.sender] == true, "Only issuer is allowed to access this function.");
        _;
    }

    constructor() public {
        owner = msg.sender;
        issuers[msg.sender] = true;
    }

    // hash 上鏈
    function notarizeHash(bytes32 docMasterKeyHash, bytes32 documentHash) public onlyIssuer{
        require(proofs[docMasterKeyHash].docHash == bytes32(0), "The docHash has been writed.");
        proofs[docMasterKeyHash].docHash = documentHash;
        proofs[docMasterKeyHash].isRevoked = false;
        proofs[docMasterKeyHash].author = msg.sender;

        emit ProofCreated(docMasterKeyHash, documentHash);
    }

    function getHash(bytes32 docMasterKeyHash) public view returns (bytes32 documentHash){
        return proofs[docMasterKeyHash].docHash;
    }

    function revokeHash(bytes32 docMasterKeyHash) public onlyIssuer{
        require(proofs[docMasterKeyHash].isRevoked == false, "The docMasterKeyHash has already been revoked.");
        // not determined feature
        // require(msg.sender == proofs[docMasterKeyHash].author || msg.sender == owner, "Not authorized to revoke this document");

        proofs[docMasterKeyHash].isRevoked = true;

        emit ProofRevoked(docMasterKeyHash);
    }

    function isRevoked(bytes32 docMasterKeyHash) public view returns (bool revocationStatus){
        return proofs[docMasterKeyHash].isRevoked;
    }

    function addIssuer(address account) public onlyOwner {
        require(account != address(0), "The account is the zero address.");
        require(issuers[account]==false, "The account is already a issuer.");
        issuers[account] = true;
        emit IssuerAdded(account);
    }

    function delIssuer(address account) public onlyOwner {
        require(issuers[account]==true, "The account is not a issuer.");
        issuers[account] = false;
        emit IssuerRemoved(account);
    }

    function isIssuer(address account) public view returns(bool issuerStatus) {
        require(account != address(0), "The account is the zero address.");
        return issuers[account];
    }

    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "Ownable: new owner is the zero address");
        issuers[newOwner] = true;
        issuers[owner] = false;
        owner = newOwner;
        emit OwnershipTransferred(owner, newOwner);
    }
}