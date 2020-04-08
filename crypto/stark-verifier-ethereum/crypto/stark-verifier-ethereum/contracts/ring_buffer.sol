pragma solidity ^0.6.4;


library RingBuffer {
    // This struct is a queeue with a fixed length, reading indexed pairs.
    // NOTE - This struct does NOT implement bounds checking, pushing
    // too much into the array will result in memory corruption.
    // NOTE - Reading an empty buffer returns zero values.
    struct IndexRingBuffer {
        uint256 front;
        uint256 back;
        uint256[] indexes;
        bytes32[] data;
        bool is_empty;
    }

    // Adds an element to the buffer by pushing to the array and
    // and wrapping around if the back is at the end of the array.
    function add_to_rear(IndexRingBuffer memory buffer, uint256 index, bytes32 data) internal pure {
        // If the buffer is empty set it to a single element state.
        // Otherwise we push and move the back index mod the length.
        if (buffer.is_empty) {
            buffer.front = 0;
            buffer.back = 0;
            buffer.is_empty = false;
            buffer.indexes[0] = index;
            buffer.data[0] = data;
        } else {
            // We could add a check here that the push doesn't put the buffer into
            // the state where front = back, but we don't want reverts here.
            uint256 next = (buffer.back + 1) % buffer.data.length;
            buffer.data[next] = data;
            buffer.indexes[next] = index;
            buffer.back = next;
        }
    }

    // Removes an element from the front of the buffer, and moves the front forward.
    function remove_front(IndexRingBuffer memory buffer) internal pure returns (uint256 index, bytes32 data) {
        // If the buffer is empty return 0, 0
        if (buffer.is_empty) {
            return (0, 0);
        }

        // Loads the return data
        (index, data) = peak_front(buffer);
        if (buffer.front == buffer.back) {
            // If we are in a single element state and we remove the element it's now empty.
            buffer.is_empty = true;
        } else {
            // Otherwise we move the front forward, cutting out the element
            buffer.front = (buffer.front + 1) % buffer.data.length;
        }
    }

    // Returns a copy of the next index pair
    function peak_front(IndexRingBuffer memory buffer) internal pure returns (uint256 index, bytes32 data) {
        // Return zero for the empty buffer
        if (buffer.is_empty) {
            return (0, 0);
        }
        // Otherwise just load the data
        data = buffer.data[buffer.front];
        index = buffer.indexes[buffer.front];
    }

    // Checks if the ring buffer contains anything, by checking if the front is the back
    function has_next(IndexRingBuffer memory buffer) internal pure returns (bool) {
        return !buffer.is_empty;
    }
}
