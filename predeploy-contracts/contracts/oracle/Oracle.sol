pragma solidity ^0.6.0;

import "../utils/SystemContract.sol";

contract Oracle is SystemContract {
    /**
     * @dev Get the price of the currency_id.
     * Returns the (price, timestamp)
     */
    function getPrice(address token)
    public
    view
    returns (uint256)
    {
        require(token != address(0), "Oracle: token is zero address");

        uint256[2] memory input;

        input[0] = 0;
        input[1] = uint256(token);

        uint256[1] memory output;

        assembly {
            if iszero(
                staticcall(gas(), 0x0000000000000000403, input, 0x40, output, 0x20)
            ) {
                revert(0, 0)
            }
        }
        return (output[0]);
    }
}
