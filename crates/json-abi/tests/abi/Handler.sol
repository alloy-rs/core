library IHandler {
    struct FooBar {
        Foo foo;
    }
}

interface Handler {
    struct Foo {
        uint256 newNumber;
    }

    function handle(IHandler.FooBar memory foobar) external;
}