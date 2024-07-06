// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.25;

interface ITaikoL1 {
    struct HookCall {
        address hook;
        bytes data;
    }

    struct BlockParams {
        address assignedProver; // DEPRECATED, value ignored.
        address coinbase;
        bytes32 extraData;
        bytes32 parentMetaHash;
        HookCall[] hookCalls; // DEPRECATED, value ignored.
        bytes signature;
        uint32 l1StateBlockNumber;
        uint64 timestamp;
    }

    struct BlockMetadata {
        bytes32 l1Hash;
        bytes32 difficulty;
        bytes32 blobHash;
        bytes32 extraData;
        bytes32 depositsHash;
        address coinbase;
        uint64 id;
        uint32 gasLimit;
        uint64 timestamp;
        uint64 l1Height;
        uint16 minTier;
        bool blobUsed;
        bytes32 parentMetaHash;
        address sender;
    }

    struct EthDeposit {
        address recipient;
        uint96 amount;
        uint64 id;
    }

    function proposeBlock(bytes calldata _params, bytes calldata _txList)
        external
        payable
        returns (BlockMetadata memory meta_, EthDeposit[] memory deposits_);
}
