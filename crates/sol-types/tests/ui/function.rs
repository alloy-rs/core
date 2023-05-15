use ethers_sol_types::sol;

sol! {
    function missingParens;
}

sol! {
    function missingSemi1()
}

sol! {
    function missingSemi2() external
}

sol! {
    function missingSemi3() returns (uint256)
}

sol! {
    function semiNotBrace1() {}
}

sol! {
    function semiNotBrace2() external {}
}

sol! {
    function semiNotBrace3() returns (uint256) {}
}

sol! {
    function singleComma(,);
}

// OK
sol! {
    function trailingComma1(bytes,);
    function trailingComma2(bytes a,);
    function trailingComma3(bytes memory a,);
}

sol! {
    function badReturn1() returns;
}

sol! {
    function badReturn2() returns();
}

sol! {
    function a() private;
    function b() internal;
    function c() public;
    function d() external;

    function e() pure;
    function f() view;
    function g() constant;
    function h() payable;

    function i() virtual;
    function j() immutable;

    function k() override(Interface.k);
    function l() myModifier("a", 0);

    function m() external view returns (uint256);
    function n() public pure returns (uint256,);
}

fn main() {}
