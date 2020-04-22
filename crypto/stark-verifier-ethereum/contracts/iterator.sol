pragma solidity ^0.6.4;


library Iterators {
    // This memory pointer contains an index and a refrence to data
    // It will work as an iterator with a .next() function which returns
    // the next data, and a .has_next() which returns a bool;
    // NOTE - No solidity generics means we will need iterators for each type.
    struct IteratorBytes32 {
        uint256 index;
        bytes32[] data_pointer;
    }

    // Creates a memory refrence to an interator which starts at the front of
    // this array.
    function init_iterator(bytes32[] memory data) internal pure returns (IteratorBytes32 memory result) {
        result.data_pointer = data;
        result.index = 0;
    }

    // Returns the next element in the array or reverts if called on an empty iterator.
    function next(IteratorBytes32 memory iterator) internal pure returns (bytes32) {
        iterator.index++;
        return iterator.data_pointer[iterator.index - 1];
    }

    // Returns a bool indicating that this iterator has a next element.
    function has_next(IteratorBytes32 memory iterator) internal pure returns (bool) {
        return iterator.index < iterator.data_pointer.length;
    }

    struct IteratorUint {
        uint256 index;
        uint256[] data_pointer;
    }

    // Creates a memory refrence to an interator which starts at the front of
    // this array.
    function init_iterator(uint256[] memory data) internal pure returns (IteratorUint memory result) {
        result.data_pointer = data;
        result.index = 0;
    }

    // Returns the next element in the array or reverts if called on an empty iterator.
    function next(IteratorUint memory iterator) internal pure returns (uint256) {
        iterator.index++;
        return iterator.data_pointer[iterator.index - 1];
    }

    // Returns a bool indicating that this iterator has a next element.
    function has_next(IteratorUint memory iterator) internal pure returns (bool) {
        return iterator.index < iterator.data_pointer.length;
    }
}
