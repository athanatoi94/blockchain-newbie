// interface ERC721 {
// event Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId);
// event Approval(address indexed _owner, address indexed _approved, uint256 indexed _tokenId);
// event ApprovalForAll(address indexed _owner, address indexed _operator, bool _approved);
//
// function balanceOf(address _owner) external view returns (uint256);
// function ownerOf(uint256 _tokenId) external view returns (address);
// function safeTransferFrom(address _from, address _to, uint256 _tokenId, bytes data) external payable;
// function safeTransferFrom(address _from, address _to, uint256 _tokenId) external payable;
// function transferFrom(address _from, address _to, uint256 _tokenId) external payable;
// function approve(address _approved, uint256 _tokenId) external payable;
// function setApprovalForAll(address _operator, bool _approved) external;
// function getApproved(uint256 _tokenId) external view returns (address);
// function isApprovedForAll(address _owner, address _operator) external view returns (bool);
// }

// 错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ERC721Error {
    //账户余额不足
    InsufficientBalance,
    //账户授权额度不足
    InsufficientAllowance,
    ArithmeticOverflow,
    //零地址
    ZeroAddress,
    //其它非法地址
    InvalidAddress,
}

pub type Address = [u8; 20];

pub trait ERC721 {
    fn emit_transfer(&self, from: Option<String>, to: Option<String>, token_id: u64);
    fn emit_approval(&self, owner: String, approved: String, token_id: u64);
    fn emit_approval_for_all(&self, owner: String, operator: String, approved: bool);

    fn balance_of(&self) -> i64;
    fn owner_of(&self, token_id: i64) -> Result<String, ERC721Error>;
    // 安全转移，bytes_data校验收款方能否接收
    fn safe_transfer_from(
        &self,
        from: Address,
        to: Address,
        token_id: i64,
        bytes_data: Option<Vec<u8>>,
    ) -> Result<(), ERC721Error>;
    fn transfer_from(&self, from: Address, to: Address, token_id: i64) -> Result<(), ERC721Error>;
    fn approve(&self, approved: Address, token_id: i64) -> Result<(), ERC721Error>;
    fn set_approval_for_all(
        &self,
        owner: Address,
        operator: Address,
        approved: bool,
    ) -> Result<(), ERC721Error>;
    fn get_approved(&self, token_id: i64) -> Result<Option<Address>, ERC721Error>;
    fn is_approved_for_all(&self, owner: Address, operator: Address) -> Result<bool, ERC721Error>;

    // 可选元数据函数
    fn name(&self) -> Option<String>;
    fn symbol(&self) -> Option<String>;
    fn token_uri(&self, token_id: u64) -> Result<String, ERC721Error>;

    // 扩展功能
    fn mint(&mut self, to: String, token_id: u64) -> Result<(), ERC721Error>;
    fn burn(&mut self, token_id: u64) -> Result<(), ERC721Error>;
    fn total_supply(&self) -> u64;
}
