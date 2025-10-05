use std::result::Result;

#[derive(Debug, PartialEq)]
pub enum ERC1155Error {
    Unauthorized,
    InsufficientBalance,
    ArraysLengthMismatch,
    ZeroAddress,
    SelfApproval,
}

pub trait ERC1155Events {
    /// 单一代币转移事件
    fn emit_transfer_single(&self, operator: String, from: String, to: String, id: u64, value: u64);

    /// 批量代币转移事件
    fn emit_transfer_batch(
        &self,
        operator: String,
        from: String,
        to: String,
        ids: Vec<u64>,
        values: Vec<u64>,
    );

    /// 授权事件
    fn emit_approval_for_all(&self, owner: String, operator: String, approved: bool);

    /// URI 更新事件
    fn emit_uri(&self, value: String, id: u64);
}

// ERC1155 核心接口
pub trait ERC1155 {
    // 必需函数
    fn balance_of(&self, account: String, id: u64) -> Result<u64, ERC1155Error>;

    fn balance_of_batch(
        &self,
        accounts: Vec<String>,
        ids: Vec<u64>,
    ) -> Result<Vec<u64>, ERC1155Error>;

    fn set_approval_for_all(
        &mut self,
        operator: String,
        approved: bool,
    ) -> Result<(), ERC1155Error>;

    fn is_approved_for_all(&self, owner: String, operator: String) -> Result<bool, ERC1155Error>;

    fn safe_transfer_from(
        &mut self,
        from: String,
        to: String,
        id: u64,
        value: u64,
        data: Vec<u8>,
    ) -> Result<(), ERC1155Error>;

    fn safe_batch_transfer_from(
        &mut self,
        from: String,
        to: String,
        ids: Vec<u64>,
        values: Vec<u64>,
        data: Vec<u8>,
    ) -> Result<(), ERC1155Error>;

    // 可选元数据函数
    fn uri(&self, id: u64) -> Result<String, ERC1155Error>;

    // 扩展功能
    fn mint(&mut self, to: String, id: u64, value: u64, data: Vec<u8>) -> Result<(), ERC1155Error>;

    fn mint_batch(
        &mut self,
        to: String,
        ids: Vec<u64>,
        values: Vec<u64>,
        data: Vec<u8>,
    ) -> Result<(), ERC1155Error>;

    fn burn(&mut self, from: String, id: u64, value: u64) -> Result<(), ERC1155Error>;

    fn burn_batch(
        &mut self,
        from: String,
        ids: Vec<u64>,
        values: Vec<u64>,
    ) -> Result<(), ERC1155Error>;

    fn total_supply(&self, id: u64) -> Option<u64>;
}
