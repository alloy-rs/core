use alloy_sol_types::sol;

sol! {
    event MissingParens1
}

sol! {
    event MissingParens2 anonymous;
}

sol! {
    event MissingParens3;
}

sol! {
    event MissingSemi1()
}

sol! {
    event MissingSemi2() anonymous
}

sol! {
    event FourIndexedParameters(bool indexed, bool indexed, bool indexed, bool indexed);
}

sol! {
    event FiveIndexedParameters(bool indexed, bool indexed, bool indexed, bool indexed, bool indexed);
}

sol! {
    event FourIndexedParametersAnonymous(bool indexed, bool indexed, bool indexed, bool indexed) anonymous;
}

sol! {
    event FiveIndexedParametersAnonymous(bool indexed, bool indexed, bool indexed, bool indexed, bool indexed) anonymous;
}

sol! {
    event ALotOfParameters(bool, bool, bool, bool, bool, bool, bool, bool, bool, bool);
    event ALotOfParametersAnonymous(bool, bool, bool, bool, bool, bool, bool, bool, bool, bool) anonymous;
}

sol! {
    event TrailingComma(uint256,);
    event Valid(uint256);
}

fn main() {}

struct A {}
