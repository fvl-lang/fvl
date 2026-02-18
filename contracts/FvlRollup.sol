// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract FvlRollup {
    bytes32 public latestStateRoot;
    uint256 public latestBlockNumber;
    address public sequencer;

    event StateRootSubmitted(
        uint256 indexed blockNumber,
        bytes32 stateRoot,
        address submitter
    );

    constructor() {
        sequencer = msg.sender;
    }

    function submitStateRoot(
        uint256 blockNumber,
        bytes32 stateRoot
    ) external {
        require(msg.sender == sequencer, "Only sequencer can submit");
        require(blockNumber > latestBlockNumber, "Block number must increase");

        latestBlockNumber = blockNumber;
        latestStateRoot = stateRoot;

        emit StateRootSubmitted(blockNumber, stateRoot, msg.sender);
    }

    function getLatest() external view returns (
        uint256 blockNumber,
        bytes32 stateRoot
    ) {
        return (latestBlockNumber, latestStateRoot);
    }
}