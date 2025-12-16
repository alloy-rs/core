library IHandler {
    struct FooBar {
        Foo foo;
    }
}

interface Handler {
    function handle(IHandler.FooBar memory foobar) external;
}
struct Foo {
    uint256 newNumber;
}