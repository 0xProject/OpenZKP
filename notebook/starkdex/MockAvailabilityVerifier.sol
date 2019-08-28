/*
  Copyright 2019 ZeroEx Intl & StarkWare Industries.

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.

  The laws and regulations applicable to the use and exchange of digital assets
  and blockchain-native tokens, including through any software developed using the
  licensed work created by ZeroEx Intl. and StarkWare Industries (the “Work”),
  vary by jurisdiction.

  As set forth in the Apache License, Version 2.0 applicable to the Work,
  developers are “solely responsible for determining the appropriateness of using
  or redistributing the Work,” which includes responsibility for ensuring
  compliance with any such applicable laws and regulations.

  See the Apache License, Version 2.0 for the specific language governing all
  applicable permissions and limitations.
*/

pragma solidity ^0.5.2;

import "IAvailabilityVerifier.sol";

contract MockAvailabilityVerifier is IAvailabilityVerifier {
    function verifyAvailabilityProof(
        uint256 newVaultRoot,
        uint256 heightVaultTree,
        uint256 sequenceNumber,
        bytes calldata opaqueAvailabilityProofs
    )
        external view {
        // solium-disable-previous-line no-empty-blocks
    }
}
