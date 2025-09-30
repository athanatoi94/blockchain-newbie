fn main() {
    println!("Hello, world!");
}

// 核心类型
pub type Address = [u8; 20];
pub type U256 = [u8; 32];

// 错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ERC20Error {
    //账户余额不足
    InsufficientBalance,
    //账户授权额度不足
    InsufficientAllowance,
    ArithmeticOverflow,
    //零地址
    ZeroAddress,
    //其它非法地址
    InvalidAddress
}

pub trait IERC20 {
    fn total_supply(&self) -> U256;
    fn balance_of(&self, account: Address) -> U256;
    // 查看owner授权给spender的剩余额度
    fn allowance(&self, owner: Address, spender: Address) -> U256;

    fn transfer(&mut self, to: Address, value: U256) -> Result<(), ERC20Error>;
    fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> Result<(), ERC20Error>;
    // 授权他人一定额度
    fn approve(&mut self, spender: Address, value: U256) -> Result<(), ERC20Error>;
}