use near_sdk::json_types::U128;
use near_sdk::serde::Deserialize;
use near_sdk::env;

/**
 * Account IDs in NEAR are just strings.
 */
pub type AccountId = String;

/**
 * Gas is u64
 */
pub type Gas = u64;

/**
 * Amounts, Balances, and Money in NEAR are u128.
 */
pub type Amount = U128;

pub type Balance = Amount;

pub type Money = Amount;

/**
 * Timestamp in NEAR is a number.
 */
pub type Timestamp = u64;

/**
 * == CONSTANTS ================================================================
 *
 * ONE_NEAR = unit of NEAR token in yocto Ⓝ (1e24)
 * XCC_GAS = gas for cross-contract calls, ~5 Tgas (teragas = 1e12) per "hop"
 * MIN_ACCOUNT_BALANCE = 3 NEAR min to keep account alive via storage staking
 *
 * TODO: revisit MIN_ACCOUNT_BALANCE after some real data is included b/c this
 *  could end up being much higher
 */

pub const ONE_NEAR: Amount = U128::from(1_000_000_000_000_000_000_000_000u128);
pub const XCC_GAS: Gas = 20_000_000_000_000;
pub const MIN_ACCOUNT_BALANCE: Amount = ONE_NEAR * U128::from(3);

/**
 * == FUNCTIONS ================================================================
 */

/**
 * @function asNEAR
 * @param amount {U128} - Yocto Ⓝ token quantity as an unsigned 128-bit integer
 * @returns {String}    - Amount in NEAR, as a string
 *
 * @example
 *
 *    asNEAR(U128::from(7000000000000000000000000u128))
 *    // => "7"
 */
pub fn as_near(amount: Amount) -> String {
    (amount.0 / ONE_NEAR.0).to_string()
}

/**
 * @function toYocto
 * @param amount {u32} - Integer to convert
 * @returns {U128}     - Amount in yocto Ⓝ as an unsigned 128-bit integer
 *
 * @example
 *
 *    toYocto(7)
 *    // => 7000000000000000000000000
 */
pub fn to_yocto(amount: u32) -> Amount {
    ONE_NEAR * U128::from(amount)
}

pub fn _random_num(max_number: u32) -> u32 {
    let mut buf = [0u8; 4];
    env::random_seed(&mut buf);
    let random_number = (u32::from(buf[0]) << 24)
        | (u32::from(buf[1]) << 16)
        | (u32::from(buf[2]) << 8)
        | u32::from(buf[3]);
    random_number % max_number
}
