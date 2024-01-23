interface ZRXToken {
    event Approval(address indexed _owner, address indexed _spender, uint256 _value);
    event Transfer(address indexed _from, address indexed _to, uint256 _value);

    constructor();

    function allowance(address _owner, address _spender) external returns (uint256);
    function approve(address _spender, uint256 _value) external returns (bool);
    function balanceOf(address _owner) external returns (uint256);
    function decimals() external returns (uint8);
    function name() external returns (string memory);
    function symbol() external returns (string memory);
    function totalSupply() external returns (uint256);
    function transfer(address _to, uint256 _value) external returns (bool);
    function transferFrom(address _from, address _to, uint256 _value) external returns (bool);
}