interface LargeTuple {
    struct Response {
        bytes output1;
        bytes output2;
        bytes output3;
        bytes output4;
        bytes output5;
        bytes output6;
        bytes output7;
        bytes output8;
        bytes output9;
        bytes output10;
        bytes output11;
        bytes output12;
        bytes output13;
    }

    function doSomething(uint160 input) external view returns (Response memory);
}