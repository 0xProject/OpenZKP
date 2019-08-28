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

/*
  Interface containing actions a verifier can invoke on the state.
  The contract containing the state should implement these and verify correctness.
*/
contract IVerifierActions {
    /*
      Updates the state roots, and verifies that the old roots match.
      Implemented in the StateRoot contract.
    */
    function stateUpdate(
        uint256 oldVaultRoot,
        uint256 newVaultRoot,
        uint256 oldTradeRoot,
        uint256 newTradeRoot,
        uint256 vaultsTreeHeightSent,
        uint256 tradesTreeHeightSent,
        bytes calldata availabilityProof
    )
        external;

    /*
      Transfers funds from the on-chain deposit area to the off-chain area.
      Implemented in the Deposits contracts.
    */
    function acceptDeposit(
        uint256 starkKey,
        uint256 vaultId,
        uint256 tokenId,
        uint256 quantizedAmount
    )
        external;

    /*
      Transfers funds from the off-chain area to the on-chain withdrawal area.
      Implemented in the Withdrawals contracts.
    */
    function acceptWithdrawal(
        uint256 starkKey,
        uint256 tokenId,
        uint256 quantizedAmount
    )
        external;

    /*
      Implemented in the Escapes contracts.
    */
    function clearEscapeRequest(
        uint256 starkKey,
        uint256 vaultId
    )
        external;
}
