// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Logos {
    struct Trace {
        bytes input;
        bytes32[] keccakStates;
        string intent;
        address submitter;
        uint256 timestamp;
    }

    Trace[] public traces;

    event TraceSubmitted(uint indexed traceId, address indexed submitter, string intent);

    function submitTrace(bytes memory input, bytes32[] memory states, string memory intent) public {
        traces.push(Trace({
            input: input,
            keccakStates: states,
            intent: intent,
            submitter: msg.sender,
            timestamp: block.timestamp
        }));

        emit TraceSubmitted(traces.length - 1, msg.sender, intent);
    }

    function getTrace(uint traceId) public view returns (Trace memory) {
        require(traceId < traces.length, "Trace not found");
        return traces[traceId];
    }

    function traceCount() public view returns (uint256) {
        return traces.length;
    }
}