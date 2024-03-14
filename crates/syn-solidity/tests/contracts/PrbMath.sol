// Common.sol
//
// Common mathematical functions needed by both SD59x18 and UD60x18. Note that these global functions do not
// always operate with SD59x18 and UD60x18 numbers.

/*//////////////////////////////////////////////////////////////////////////
                                CUSTOM ERRORS
//////////////////////////////////////////////////////////////////////////*/

/// @notice Thrown when the resultant value in {mulDiv} overflows uint256.
error PRBMath_MulDiv_Overflow(uint256 x, uint256 y, uint256 denominator);

/// @notice Thrown when the resultant value in {mulDiv18} overflows uint256.
error PRBMath_MulDiv18_Overflow(uint256 x, uint256 y);

/// @notice Thrown when one of the inputs passed to {mulDivSigned} is `type(int256).min`.
error PRBMath_MulDivSigned_InputTooSmall();

/// @notice Thrown when the resultant value in {mulDivSigned} overflows int256.
error PRBMath_MulDivSigned_Overflow(int256 x, int256 y);

/*//////////////////////////////////////////////////////////////////////////
                                    CONSTANTS
//////////////////////////////////////////////////////////////////////////*/

/// @dev The maximum value a uint128 number can have.
uint128 constant MAX_UINT128 = type(uint128).max;

/// @dev The maximum value a uint40 number can have.
uint40 constant MAX_UINT40 = type(uint40).max;

/// @dev The unit number, which the decimal precision of the fixed-point types.
uint256 constant UNIT = 1e18;

/// @dev The unit number inverted mod 2^256.
uint256 constant UNIT_INVERSE = 78156646155174841979727994598816262306175212592076161876661_508869554232690281;

/// @dev The largest power of two that divides the decimal value of `UNIT`. The logarithm of this value is the least significant
/// bit in the binary representation of `UNIT`.
uint256 constant UNIT_LPOTD = 262144;

/*//////////////////////////////////////////////////////////////////////////
                                    FUNCTIONS
//////////////////////////////////////////////////////////////////////////*/

/// @notice Calculates the binary exponent of x using the binary fraction method.
/// @dev Has to use 192.64-bit fixed-point numbers. See https://ethereum.stackexchange.com/a/96594/24693.
/// @param x The exponent as an unsigned 192.64-bit fixed-point number.
/// @return result The result as an unsigned 60.18-decimal fixed-point number.
/// @custom:smtchecker abstract-function-nondet
function exp2(uint256 x) pure returns (uint256 result) {
    unchecked {
        // Start from 0.5 in the 192.64-bit fixed-point format.
        result = 0x800000000000000000000000000000000000000000000000;

        // The following logic multiplies the result by $\sqrt{2^{-i}}$ when the bit at position i is 1. Key points:
        //
        // 1. Intermediate results will not overflow, as the starting point is 2^191 and all magic factors are under 2^65.
        // 2. The rationale for organizing the if statements into groups of 8 is gas savings. If the result of performing
        // a bitwise AND operation between x and any value in the array [0x80; 0x40; 0x20; 0x10; 0x08; 0x04; 0x02; 0x01] is 1,
        // we know that `x & 0xFF` is also 1.
        if (x & 0xFF00000000000000 > 0) {
            if (x & 0x8000000000000000 > 0) {
                result = (result * 0x16A09E667F3BCC909) >> 64;
            }
            if (x & 0x4000000000000000 > 0) {
                result = (result * 0x1306FE0A31B7152DF) >> 64;
            }
            if (x & 0x2000000000000000 > 0) {
                result = (result * 0x1172B83C7D517ADCE) >> 64;
            }
            if (x & 0x1000000000000000 > 0) {
                result = (result * 0x10B5586CF9890F62A) >> 64;
            }
            if (x & 0x800000000000000 > 0) {
                result = (result * 0x1059B0D31585743AE) >> 64;
            }
            if (x & 0x400000000000000 > 0) {
                result = (result * 0x102C9A3E778060EE7) >> 64;
            }
            if (x & 0x200000000000000 > 0) {
                result = (result * 0x10163DA9FB33356D8) >> 64;
            }
            if (x & 0x100000000000000 > 0) {
                result = (result * 0x100B1AFA5ABCBED61) >> 64;
            }
        }

        if (x & 0xFF000000000000 > 0) {
            if (x & 0x80000000000000 > 0) {
                result = (result * 0x10058C86DA1C09EA2) >> 64;
            }
            if (x & 0x40000000000000 > 0) {
                result = (result * 0x1002C605E2E8CEC50) >> 64;
            }
            if (x & 0x20000000000000 > 0) {
                result = (result * 0x100162F3904051FA1) >> 64;
            }
            if (x & 0x10000000000000 > 0) {
                result = (result * 0x1000B175EFFDC76BA) >> 64;
            }
            if (x & 0x8000000000000 > 0) {
                result = (result * 0x100058BA01FB9F96D) >> 64;
            }
            if (x & 0x4000000000000 > 0) {
                result = (result * 0x10002C5CC37DA9492) >> 64;
            }
            if (x & 0x2000000000000 > 0) {
                result = (result * 0x1000162E525EE0547) >> 64;
            }
            if (x & 0x1000000000000 > 0) {
                result = (result * 0x10000B17255775C04) >> 64;
            }
        }

        if (x & 0xFF0000000000 > 0) {
            if (x & 0x800000000000 > 0) {
                result = (result * 0x1000058B91B5BC9AE) >> 64;
            }
            if (x & 0x400000000000 > 0) {
                result = (result * 0x100002C5C89D5EC6D) >> 64;
            }
            if (x & 0x200000000000 > 0) {
                result = (result * 0x10000162E43F4F831) >> 64;
            }
            if (x & 0x100000000000 > 0) {
                result = (result * 0x100000B1721BCFC9A) >> 64;
            }
            if (x & 0x80000000000 > 0) {
                result = (result * 0x10000058B90CF1E6E) >> 64;
            }
            if (x & 0x40000000000 > 0) {
                result = (result * 0x1000002C5C863B73F) >> 64;
            }
            if (x & 0x20000000000 > 0) {
                result = (result * 0x100000162E430E5A2) >> 64;
            }
            if (x & 0x10000000000 > 0) {
                result = (result * 0x1000000B172183551) >> 64;
            }
        }

        if (x & 0xFF00000000 > 0) {
            if (x & 0x8000000000 > 0) {
                result = (result * 0x100000058B90C0B49) >> 64;
            }
            if (x & 0x4000000000 > 0) {
                result = (result * 0x10000002C5C8601CC) >> 64;
            }
            if (x & 0x2000000000 > 0) {
                result = (result * 0x1000000162E42FFF0) >> 64;
            }
            if (x & 0x1000000000 > 0) {
                result = (result * 0x10000000B17217FBB) >> 64;
            }
            if (x & 0x800000000 > 0) {
                result = (result * 0x1000000058B90BFCE) >> 64;
            }
            if (x & 0x400000000 > 0) {
                result = (result * 0x100000002C5C85FE3) >> 64;
            }
            if (x & 0x200000000 > 0) {
                result = (result * 0x10000000162E42FF1) >> 64;
            }
            if (x & 0x100000000 > 0) {
                result = (result * 0x100000000B17217F8) >> 64;
            }
        }

        if (x & 0xFF000000 > 0) {
            if (x & 0x80000000 > 0) {
                result = (result * 0x10000000058B90BFC) >> 64;
            }
            if (x & 0x40000000 > 0) {
                result = (result * 0x1000000002C5C85FE) >> 64;
            }
            if (x & 0x20000000 > 0) {
                result = (result * 0x100000000162E42FF) >> 64;
            }
            if (x & 0x10000000 > 0) {
                result = (result * 0x1000000000B17217F) >> 64;
            }
            if (x & 0x8000000 > 0) {
                result = (result * 0x100000000058B90C0) >> 64;
            }
            if (x & 0x4000000 > 0) {
                result = (result * 0x10000000002C5C860) >> 64;
            }
            if (x & 0x2000000 > 0) {
                result = (result * 0x1000000000162E430) >> 64;
            }
            if (x & 0x1000000 > 0) {
                result = (result * 0x10000000000B17218) >> 64;
            }
        }

        if (x & 0xFF0000 > 0) {
            if (x & 0x800000 > 0) {
                result = (result * 0x1000000000058B90C) >> 64;
            }
            if (x & 0x400000 > 0) {
                result = (result * 0x100000000002C5C86) >> 64;
            }
            if (x & 0x200000 > 0) {
                result = (result * 0x10000000000162E43) >> 64;
            }
            if (x & 0x100000 > 0) {
                result = (result * 0x100000000000B1721) >> 64;
            }
            if (x & 0x80000 > 0) {
                result = (result * 0x10000000000058B91) >> 64;
            }
            if (x & 0x40000 > 0) {
                result = (result * 0x1000000000002C5C8) >> 64;
            }
            if (x & 0x20000 > 0) {
                result = (result * 0x100000000000162E4) >> 64;
            }
            if (x & 0x10000 > 0) {
                result = (result * 0x1000000000000B172) >> 64;
            }
        }

        if (x & 0xFF00 > 0) {
            if (x & 0x8000 > 0) {
                result = (result * 0x100000000000058B9) >> 64;
            }
            if (x & 0x4000 > 0) {
                result = (result * 0x10000000000002C5D) >> 64;
            }
            if (x & 0x2000 > 0) {
                result = (result * 0x1000000000000162E) >> 64;
            }
            if (x & 0x1000 > 0) {
                result = (result * 0x10000000000000B17) >> 64;
            }
            if (x & 0x800 > 0) {
                result = (result * 0x1000000000000058C) >> 64;
            }
            if (x & 0x400 > 0) {
                result = (result * 0x100000000000002C6) >> 64;
            }
            if (x & 0x200 > 0) {
                result = (result * 0x10000000000000163) >> 64;
            }
            if (x & 0x100 > 0) {
                result = (result * 0x100000000000000B1) >> 64;
            }
        }

        if (x & 0xFF > 0) {
            if (x & 0x80 > 0) {
                result = (result * 0x10000000000000059) >> 64;
            }
            if (x & 0x40 > 0) {
                result = (result * 0x1000000000000002C) >> 64;
            }
            if (x & 0x20 > 0) {
                result = (result * 0x10000000000000016) >> 64;
            }
            if (x & 0x10 > 0) {
                result = (result * 0x1000000000000000B) >> 64;
            }
            if (x & 0x8 > 0) {
                result = (result * 0x10000000000000006) >> 64;
            }
            if (x & 0x4 > 0) {
                result = (result * 0x10000000000000003) >> 64;
            }
            if (x & 0x2 > 0) {
                result = (result * 0x10000000000000001) >> 64;
            }
            if (x & 0x1 > 0) {
                result = (result * 0x10000000000000001) >> 64;
            }
        }

        // In the code snippet below, two operations are executed simultaneously:
        //
        // 1. The result is multiplied by $(2^n + 1)$, where $2^n$ represents the integer part, and the additional 1
        // accounts for the initial guess of 0.5. This is achieved by subtracting from 191 instead of 192.
        // 2. The result is then converted to an unsigned 60.18-decimal fixed-point format.
        //
        // The underlying logic is based on the relationship $2^{191-ip} = 2^{ip} / 2^{191}$, where $ip$ denotes the,
        // integer part, $2^n$.
        result *= UNIT;
        result >>= (191 - (x >> 64));
    }
}

/// @notice Finds the zero-based index of the first 1 in the binary representation of x.
///
/// @dev See the note on "msb" in this Wikipedia article: https://en.wikipedia.org/wiki/Find_first_set
///
/// Each step in this implementation is equivalent to this high-level code:
///
/// ```solidity
/// if (x >= 2 ** 128) {
///     x >>= 128;
///     result += 128;
/// }
/// ```
///
/// Where 128 is replaced with each respective power of two factor. See the full high-level implementation here:
/// https://gist.github.com/PaulRBerg/f932f8693f2733e30c4d479e8e980948
///
/// The Yul instructions used below are:
///
/// - "gt" is "greater than"
/// - "or" is the OR bitwise operator
/// - "shl" is "shift left"
/// - "shr" is "shift right"
///
/// @param x The uint256 number for which to find the index of the most significant bit.
/// @return result The index of the most significant bit as a uint256.
/// @custom:smtchecker abstract-function-nondet
function msb(uint256 x) pure returns (uint256 result) {
    // 2^128
    assembly ("memory-safe") {
        let factor := shl(7, gt(x, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
        x := shr(factor, x)
        result := or(result, factor)
    }
    // 2^64
    assembly ("memory-safe") {
        let factor := shl(6, gt(x, 0xFFFFFFFFFFFFFFFF))
        x := shr(factor, x)
        result := or(result, factor)
    }
    // 2^32
    assembly ("memory-safe") {
        let factor := shl(5, gt(x, 0xFFFFFFFF))
        x := shr(factor, x)
        result := or(result, factor)
    }
    // 2^16
    assembly ("memory-safe") {
        let factor := shl(4, gt(x, 0xFFFF))
        x := shr(factor, x)
        result := or(result, factor)
    }
    // 2^8
    assembly ("memory-safe") {
        let factor := shl(3, gt(x, 0xFF))
        x := shr(factor, x)
        result := or(result, factor)
    }
    // 2^4
    assembly ("memory-safe") {
        let factor := shl(2, gt(x, 0xF))
        x := shr(factor, x)
        result := or(result, factor)
    }
    // 2^2
    assembly ("memory-safe") {
        let factor := shl(1, gt(x, 0x3))
        x := shr(factor, x)
        result := or(result, factor)
    }
    // 2^1
    // No need to shift x any more.
    assembly ("memory-safe") {
        let factor := gt(x, 0x1)
        result := or(result, factor)
    }
}

/// @notice Calculates x*y÷denominator with 512-bit precision.
///
/// @dev Credits to Remco Bloemen under MIT license https://xn--2-umb.com/21/muldiv.
///
/// Notes:
/// - The result is rounded toward zero.
///
/// Requirements:
/// - The denominator must not be zero.
/// - The result must fit in uint256.
///
/// @param x The multiplicand as a uint256.
/// @param y The multiplier as a uint256.
/// @param denominator The divisor as a uint256.
/// @return result The result as a uint256.
/// @custom:smtchecker abstract-function-nondet
function mulDiv(uint256 x, uint256 y, uint256 denominator) pure returns (uint256 result) {
    // 512-bit multiply [prod1 prod0] = x * y. Compute the product mod 2^256 and mod 2^256 - 1, then use
    // use the Chinese Remainder Theorem to reconstruct the 512-bit result. The result is stored in two 256
    // variables such that product = prod1 * 2^256 + prod0.
    uint256 prod0; // Least significant 256 bits of the product
    uint256 prod1; // Most significant 256 bits of the product
    assembly ("memory-safe") {
        let mm := mulmod(x, y, not(0))
        prod0 := mul(x, y)
        prod1 := sub(sub(mm, prod0), lt(mm, prod0))
    }

    // Handle non-overflow cases, 256 by 256 division.
    if (prod1 == 0) {
        unchecked {
            return prod0 / denominator;
        }
    }

    // Make sure the result is less than 2^256. Also prevents denominator == 0.
    if (prod1 >= denominator) {
        revert PRBMath_MulDiv_Overflow(x, y, denominator);
    }

    ////////////////////////////////////////////////////////////////////////////
    // 512 by 256 division
    ////////////////////////////////////////////////////////////////////////////

    // Make division exact by subtracting the remainder from [prod1 prod0].
    uint256 remainder;
    assembly ("memory-safe") {
        // Compute remainder using the mulmod Yul instruction.
        remainder := mulmod(x, y, denominator)

        // Subtract 256 bit number from 512-bit number.
        prod1 := sub(prod1, gt(remainder, prod0))
        prod0 := sub(prod0, remainder)
    }

    unchecked {
        // Calculate the largest power of two divisor of the denominator using the unary operator ~. This operation cannot overflow
        // because the denominator cannot be zero at this point in the function execution. The result is always >= 1.
        // For more detail, see https://cs.stackexchange.com/q/138556/92363.
        uint256 lpotdod = denominator & (~denominator + 1);
        uint256 flippedLpotdod;

        assembly ("memory-safe") {
            // Factor powers of two out of denominator.
            denominator := div(denominator, lpotdod)

            // Divide [prod1 prod0] by lpotdod.
            prod0 := div(prod0, lpotdod)

            // Get the flipped value `2^256 / lpotdod`. If the `lpotdod` is zero, the flipped value is one.
            // `sub(0, lpotdod)` produces the two's complement version of `lpotdod`, which is equivalent to flipping all the bits.
            // However, `div` interprets this value as an unsigned value: https://ethereum.stackexchange.com/q/147168/24693
            flippedLpotdod := add(div(sub(0, lpotdod), lpotdod), 1)
        }

        // Shift in bits from prod1 into prod0.
        prod0 |= prod1 * flippedLpotdod;

        // Invert denominator mod 2^256. Now that denominator is an odd number, it has an inverse modulo 2^256 such
        // that denominator * inv = 1 mod 2^256. Compute the inverse by starting with a seed that is correct for
        // four bits. That is, denominator * inv = 1 mod 2^4.
        uint256 inverse = (3 * denominator) ^ 2;

        // Use the Newton-Raphson iteration to improve the precision. Thanks to Hensel's lifting lemma, this also works
        // in modular arithmetic, doubling the correct bits in each step.
        inverse *= 2 - denominator * inverse; // inverse mod 2^8
        inverse *= 2 - denominator * inverse; // inverse mod 2^16
        inverse *= 2 - denominator * inverse; // inverse mod 2^32
        inverse *= 2 - denominator * inverse; // inverse mod 2^64
        inverse *= 2 - denominator * inverse; // inverse mod 2^128
        inverse *= 2 - denominator * inverse; // inverse mod 2^256

        // Because the division is now exact we can divide by multiplying with the modular inverse of denominator.
        // This will give us the correct result modulo 2^256. Since the preconditions guarantee that the outcome is
        // less than 2^256, this is the final result. We don't need to compute the high bits of the result and prod1
        // is no longer required.
        result = prod0 * inverse;
    }
}

/// @notice Calculates x*y÷1e18 with 512-bit precision.
///
/// @dev A variant of {mulDiv} with constant folding, i.e. in which the denominator is hard coded to 1e18.
///
/// Notes:
/// - The body is purposely left uncommented; to understand how this works, see the documentation in {mulDiv}.
/// - The result is rounded toward zero.
/// - We take as an axiom that the result cannot be `MAX_UINT256` when x and y solve the following system of equations:
///
/// $$
/// \begin{cases}
///     x * y = MAX\_UINT256 * UNIT \\
///     (x * y) \% UNIT \geq \frac{UNIT}{2}
/// \end{cases}
/// $$
///
/// Requirements:
/// - Refer to the requirements in {mulDiv}.
/// - The result must fit in uint256.
///
/// @param x The multiplicand as an unsigned 60.18-decimal fixed-point number.
/// @param y The multiplier as an unsigned 60.18-decimal fixed-point number.
/// @return result The result as an unsigned 60.18-decimal fixed-point number.
/// @custom:smtchecker abstract-function-nondet
function mulDiv18(uint256 x, uint256 y) pure returns (uint256 result) {
    uint256 prod0;
    uint256 prod1;
    assembly ("memory-safe") {
        let mm := mulmod(x, y, not(0))
        prod0 := mul(x, y)
        prod1 := sub(sub(mm, prod0), lt(mm, prod0))
    }

    if (prod1 == 0) {
        unchecked {
            return prod0 / UNIT;
        }
    }

    if (prod1 >= UNIT) {
        revert PRBMath_MulDiv18_Overflow(x, y);
    }

    uint256 remainder;
    assembly ("memory-safe") {
        remainder := mulmod(x, y, UNIT)
        result :=
            mul(
                or(
                    div(sub(prod0, remainder), UNIT_LPOTD),
                    mul(sub(prod1, gt(remainder, prod0)), add(div(sub(0, UNIT_LPOTD), UNIT_LPOTD), 1))
                ),
                UNIT_INVERSE
            )
    }
}

/// @notice Calculates x*y÷denominator with 512-bit precision.
///
/// @dev This is an extension of {mulDiv} for signed numbers, which works by computing the signs and the absolute values separately.
///
/// Notes:
/// - The result is rounded toward zero.
///
/// Requirements:
/// - Refer to the requirements in {mulDiv}.
/// - None of the inputs can be `type(int256).min`.
/// - The result must fit in int256.
///
/// @param x The multiplicand as an int256.
/// @param y The multiplier as an int256.
/// @param denominator The divisor as an int256.
/// @return result The result as an int256.
/// @custom:smtchecker abstract-function-nondet
function mulDivSigned(int256 x, int256 y, int256 denominator) pure returns (int256 result) {
    if (x == type(int256).min || y == type(int256).min || denominator == type(int256).min) {
        revert PRBMath_MulDivSigned_InputTooSmall();
    }

    // Get hold of the absolute values of x, y and the denominator.
    uint256 xAbs;
    uint256 yAbs;
    uint256 dAbs;
    unchecked {
        xAbs = x < 0 ? uint256(-x) : uint256(x);
        yAbs = y < 0 ? uint256(-y) : uint256(y);
        dAbs = denominator < 0 ? uint256(-denominator) : uint256(denominator);
    }

    // Compute the absolute value of x*y÷denominator. The result must fit in int256.
    uint256 resultAbs = mulDiv(xAbs, yAbs, dAbs);
    if (resultAbs > uint256(type(int256).max)) {
        revert PRBMath_MulDivSigned_Overflow(x, y);
    }

    // Get the signs of x, y and the denominator.
    uint256 sx;
    uint256 sy;
    uint256 sd;
    assembly ("memory-safe") {
        // "sgt" is the "signed greater than" assembly instruction and "sub(0,1)" is -1 in two's complement.
        sx := sgt(x, sub(0, 1))
        sy := sgt(y, sub(0, 1))
        sd := sgt(denominator, sub(0, 1))
    }

    // XOR over sx, sy and sd. What this does is to check whether there are 1 or 3 negative signs in the inputs.
    // If there are, the result should be negative. Otherwise, it should be positive.
    unchecked {
        result = sx ^ sy ^ sd == 0 ? -int256(resultAbs) : int256(resultAbs);
    }
}

/// @notice Calculates the square root of x using the Babylonian method.
///
/// @dev See https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method.
///
/// Notes:
/// - If x is not a perfect square, the result is rounded down.
/// - Credits to OpenZeppelin for the explanations in comments below.
///
/// @param x The uint256 number for which to calculate the square root.
/// @return result The result as a uint256.
/// @custom:smtchecker abstract-function-nondet
function sqrt(uint256 x) pure returns (uint256 result) {
    if (x == 0) {
        return 0;
    }

    // For our first guess, we calculate the biggest power of 2 which is smaller than the square root of x.
    //
    // We know that the "msb" (most significant bit) of x is a power of 2 such that we have:
    //
    // $$
    // msb(x) <= x <= 2*msb(x)$
    // $$
    //
    // We write $msb(x)$ as $2^k$, and we get:
    //
    // $$
    // k = log_2(x)
    // $$
    //
    // Thus, we can write the initial inequality as:
    //
    // $$
    // 2^{log_2(x)} <= x <= 2*2^{log_2(x)+1} \\
    // sqrt(2^k) <= sqrt(x) < sqrt(2^{k+1}) \\
    // 2^{k/2} <= sqrt(x) < 2^{(k+1)/2} <= 2^{(k/2)+1}
    // $$
    //
    // Consequently, $2^{log_2(x) /2} is a good first approximation of sqrt(x) with at least one correct bit.
    uint256 xAux = uint256(x);
    result = 1;
    if (xAux >= 2 ** 128) {
        xAux >>= 128;
        result <<= 64;
    }
    if (xAux >= 2 ** 64) {
        xAux >>= 64;
        result <<= 32;
    }
    if (xAux >= 2 ** 32) {
        xAux >>= 32;
        result <<= 16;
    }
    if (xAux >= 2 ** 16) {
        xAux >>= 16;
        result <<= 8;
    }
    if (xAux >= 2 ** 8) {
        xAux >>= 8;
        result <<= 4;
    }
    if (xAux >= 2 ** 4) {
        xAux >>= 4;
        result <<= 2;
    }
    if (xAux >= 2 ** 2) {
        result <<= 1;
    }

    // At this point, `result` is an estimation with at least one bit of precision. We know the true value has at
    // most 128 bits, since it is the square root of a uint256. Newton's method converges quadratically (precision
    // doubles at every iteration). We thus need at most 7 iteration to turn our partial result with one bit of
    // precision into the expected uint128 result.
    unchecked {
        result = (result + x / result) >> 1;
        result = (result + x / result) >> 1;
        result = (result + x / result) >> 1;
        result = (result + x / result) >> 1;
        result = (result + x / result) >> 1;
        result = (result + x / result) >> 1;
        result = (result + x / result) >> 1;

        // If x is not a perfect square, round the result toward zero.
        uint256 roundedResult = x / result;
        if (result >= roundedResult) {
            result = roundedResult;
        }
    }
}

/// @notice Thrown when trying to cast a SD1x18 number that doesn't fit in UD2x18.
error PRBMath_SD1x18_ToUD2x18_Underflow(SD1x18 x);

/// @notice Thrown when trying to cast a SD1x18 number that doesn't fit in UD60x18.
error PRBMath_SD1x18_ToUD60x18_Underflow(SD1x18 x);

/// @notice Thrown when trying to cast a SD1x18 number that doesn't fit in uint128.
error PRBMath_SD1x18_ToUint128_Underflow(SD1x18 x);

/// @notice Thrown when trying to cast a SD1x18 number that doesn't fit in uint256.
error PRBMath_SD1x18_ToUint256_Underflow(SD1x18 x);

/// @notice Thrown when trying to cast a SD1x18 number that doesn't fit in uint40.
error PRBMath_SD1x18_ToUint40_Overflow(SD1x18 x);

/// @notice Thrown when trying to cast a SD1x18 number that doesn't fit in uint40.
error PRBMath_SD1x18_ToUint40_Underflow(SD1x18 x);

/// @notice Thrown when taking the absolute value of `MIN_SD59x18`.
error PRBMath_SD59x18_Abs_MinSD59x18();

/// @notice Thrown when ceiling a number overflows SD59x18.
error PRBMath_SD59x18_Ceil_Overflow(SD59x18 x);

/// @notice Thrown when converting a basic integer to the fixed-point format overflows SD59x18.
error PRBMath_SD59x18_Convert_Overflow(int256 x);

/// @notice Thrown when converting a basic integer to the fixed-point format underflows SD59x18.
error PRBMath_SD59x18_Convert_Underflow(int256 x);

/// @notice Thrown when dividing two numbers and one of them is `MIN_SD59x18`.
error PRBMath_SD59x18_Div_InputTooSmall();

/// @notice Thrown when dividing two numbers and one of the intermediary unsigned results overflows SD59x18.
error PRBMath_SD59x18_Div_Overflow(SD59x18 x, SD59x18 y);

/// @notice Thrown when taking the natural exponent of a base greater than 133_084258667509499441.
error PRBMath_SD59x18_Exp_InputTooBig(SD59x18 x);

/// @notice Thrown when taking the binary exponent of a base greater than 192e18.
error PRBMath_SD59x18_Exp2_InputTooBig(SD59x18 x);

/// @notice Thrown when flooring a number underflows SD59x18.
error PRBMath_SD59x18_Floor_Underflow(SD59x18 x);

/// @notice Thrown when taking the geometric mean of two numbers and their product is negative.
error PRBMath_SD59x18_Gm_NegativeProduct(SD59x18 x, SD59x18 y);

/// @notice Thrown when taking the geometric mean of two numbers and multiplying them overflows SD59x18.
error PRBMath_SD59x18_Gm_Overflow(SD59x18 x, SD59x18 y);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in SD1x18.
error PRBMath_SD59x18_IntoSD1x18_Overflow(SD59x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in SD1x18.
error PRBMath_SD59x18_IntoSD1x18_Underflow(SD59x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in UD2x18.
error PRBMath_SD59x18_IntoUD2x18_Overflow(SD59x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in UD2x18.
error PRBMath_SD59x18_IntoUD2x18_Underflow(SD59x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in UD60x18.
error PRBMath_SD59x18_IntoUD60x18_Underflow(SD59x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in uint128.
error PRBMath_SD59x18_IntoUint128_Overflow(SD59x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in uint128.
error PRBMath_SD59x18_IntoUint128_Underflow(SD59x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in uint256.
error PRBMath_SD59x18_IntoUint256_Underflow(SD59x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in uint40.
error PRBMath_SD59x18_IntoUint40_Overflow(SD59x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in uint40.
error PRBMath_SD59x18_IntoUint40_Underflow(SD59x18 x);

/// @notice Thrown when taking the logarithm of a number less than or equal to zero.
error PRBMath_SD59x18_Log_InputTooSmall(SD59x18 x);

/// @notice Thrown when multiplying two numbers and one of the inputs is `MIN_SD59x18`.
error PRBMath_SD59x18_Mul_InputTooSmall();

/// @notice Thrown when multiplying two numbers and the intermediary absolute result overflows SD59x18.
error PRBMath_SD59x18_Mul_Overflow(SD59x18 x, SD59x18 y);

/// @notice Thrown when raising a number to a power and hte intermediary absolute result overflows SD59x18.
error PRBMath_SD59x18_Powu_Overflow(SD59x18 x, uint256 y);

/// @notice Thrown when taking the square root of a negative number.
error PRBMath_SD59x18_Sqrt_NegativeInput(SD59x18 x);

/// @notice Thrown when the calculating the square root overflows SD59x18.
error PRBMath_SD59x18_Sqrt_Overflow(SD59x18 x);

/// @notice Thrown when trying to cast a UD2x18 number that doesn't fit in SD1x18.
error PRBMath_UD2x18_IntoSD1x18_Overflow(UD2x18 x);

/// @notice Thrown when trying to cast a UD2x18 number that doesn't fit in uint40.
error PRBMath_UD2x18_IntoUint40_Overflow(UD2x18 x);

/// @notice Thrown when ceiling a number overflows UD60x18.
error PRBMath_UD60x18_Ceil_Overflow(UD60x18 x);

/// @notice Thrown when converting a basic integer to the fixed-point format overflows UD60x18.
error PRBMath_UD60x18_Convert_Overflow(uint256 x);

/// @notice Thrown when taking the natural exponent of a base greater than 133_084258667509499441.
error PRBMath_UD60x18_Exp_InputTooBig(UD60x18 x);

/// @notice Thrown when taking the binary exponent of a base greater than 192e18.
error PRBMath_UD60x18_Exp2_InputTooBig(UD60x18 x);

/// @notice Thrown when taking the geometric mean of two numbers and multiplying them overflows UD60x18.
error PRBMath_UD60x18_Gm_Overflow(UD60x18 x, UD60x18 y);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in SD1x18.
error PRBMath_UD60x18_IntoSD1x18_Overflow(UD60x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in SD59x18.
error PRBMath_UD60x18_IntoSD59x18_Overflow(UD60x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in UD2x18.
error PRBMath_UD60x18_IntoUD2x18_Overflow(UD60x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in uint128.
error PRBMath_UD60x18_IntoUint128_Overflow(UD60x18 x);

/// @notice Thrown when trying to cast a UD60x18 number that doesn't fit in uint40.
error PRBMath_UD60x18_IntoUint40_Overflow(UD60x18 x);

/// @notice Thrown when taking the logarithm of a number less than 1.
error PRBMath_UD60x18_Log_InputTooSmall(UD60x18 x);

/// @notice Thrown when calculating the square root overflows UD60x18.
error PRBMath_UD60x18_Sqrt_Overflow(UD60x18 x);

// NOTICE: the "u" prefix stands for "unwrapped".

/// @dev Euler's number as an SD59x18 number.
SD59x18 constant E = SD59x18.wrap(2_718281828459045235);

/// @dev The maximum input permitted in {exp}.
int256 constant uEXP_MAX_INPUT = 133_084258667509499440;
SD59x18 constant EXP_MAX_INPUT = SD59x18.wrap(uEXP_MAX_INPUT);

/// @dev The maximum input permitted in {exp2}.
int256 constant uEXP2_MAX_INPUT = 192e18 - 1;
SD59x18 constant EXP2_MAX_INPUT = SD59x18.wrap(uEXP2_MAX_INPUT);

/// @dev Half the UNIT number.
int256 constant uHALF_UNIT = 0.5e18;
SD59x18 constant HALF_UNIT = SD59x18.wrap(uHALF_UNIT);

/// @dev $log_2(10)$ as an SD59x18 number.
int256 constant uLOG2_10 = 3_321928094887362347;
SD59x18 constant LOG2_10 = SD59x18.wrap(uLOG2_10);

/// @dev $log_2(e)$ as an SD59x18 number.
int256 constant uLOG2_E = 1_442695040888963407;
SD59x18 constant LOG2_E = SD59x18.wrap(uLOG2_E);

/// @dev The maximum value an SD59x18 number can have.
int256 constant uMAX_SD59x18 = 57896044618658097711785492504343953926634992332820282019728_792003956564819967;
SD59x18 constant MAX_SD59x18 = SD59x18.wrap(uMAX_SD59x18);

/// @dev The maximum whole value an SD59x18 number can have.
int256 constant uMAX_WHOLE_SD59x18 = 57896044618658097711785492504343953926634992332820282019728_000000000000000000;
SD59x18 constant MAX_WHOLE_SD59x18 = SD59x18.wrap(uMAX_WHOLE_SD59x18);

/// @dev The minimum value an SD59x18 number can have.
int256 constant uMIN_SD59x18 = -57896044618658097711785492504343953926634992332820282019728_792003956564819968;
SD59x18 constant MIN_SD59x18 = SD59x18.wrap(uMIN_SD59x18);

/// @dev The minimum whole value an SD59x18 number can have.
int256 constant uMIN_WHOLE_SD59x18 = -57896044618658097711785492504343953926634992332820282019728_000000000000000000;
SD59x18 constant MIN_WHOLE_SD59x18 = SD59x18.wrap(uMIN_WHOLE_SD59x18);

/// @dev PI as an SD59x18 number.
SD59x18 constant PI = SD59x18.wrap(3_141592653589793238);

/// @dev The unit number, which gives the decimal precision of SD59x18.
int256 constant uUNIT = 1e18;
SD59x18 constant UNIT = SD59x18.wrap(1e18);

/// @dev The unit number squared.
int256 constant uUNIT_SQUARED = 1e36;
SD59x18 constant UNIT_SQUARED = SD59x18.wrap(uUNIT_SQUARED);

/// @dev Zero as an SD59x18 number.
SD59x18 constant ZERO = SD59x18.wrap(0);

/// @notice Casts a UD60x18 number into SD1x18.
/// @dev Requirements:
/// - x must be less than or equal to `uMAX_SD1x18`.
function intoSD1x18(UD60x18 x) pure returns (SD1x18 result) {
    uint256 xUint = UD60x18.unwrap(x);
    if (xUint > uint256(int256(uMAX_SD1x18))) {
        revert CastingErrors.PRBMath_UD60x18_IntoSD1x18_Overflow(x);
    }
    result = SD1x18.wrap(int64(uint64(xUint)));
}

/// @notice Casts a UD60x18 number into UD2x18.
/// @dev Requirements:
/// - x must be less than or equal to `uMAX_UD2x18`.
function intoUD2x18(UD60x18 x) pure returns (UD2x18 result) {
    uint256 xUint = UD60x18.unwrap(x);
    if (xUint > uMAX_UD2x18) {
        revert CastingErrors.PRBMath_UD60x18_IntoUD2x18_Overflow(x);
    }
    result = UD2x18.wrap(uint64(xUint));
}

/// @notice Casts a UD60x18 number into SD59x18.
/// @dev Requirements:
/// - x must be less than or equal to `uMAX_SD59x18`.
function intoSD59x18(UD60x18 x) pure returns (SD59x18 result) {
    uint256 xUint = UD60x18.unwrap(x);
    if (xUint > uint256(uMAX_SD59x18)) {
        revert CastingErrors.PRBMath_UD60x18_IntoSD59x18_Overflow(x);
    }
    result = SD59x18.wrap(int256(xUint));
}

/// @notice Casts a UD60x18 number into uint128.
/// @dev This is basically an alias for {unwrap}.
function intoUint256(UD60x18 x) pure returns (uint256 result) {
    result = UD60x18.unwrap(x);
}

/// @notice Casts a UD60x18 number into uint128.
/// @dev Requirements:
/// - x must be less than or equal to `MAX_UINT128`.
function intoUint128(UD60x18 x) pure returns (uint128 result) {
    uint256 xUint = UD60x18.unwrap(x);
    if (xUint > MAX_UINT128) {
        revert CastingErrors.PRBMath_UD60x18_IntoUint128_Overflow(x);
    }
    result = uint128(xUint);
}

/// @notice Casts a UD60x18 number into uint40.
/// @dev Requirements:
/// - x must be less than or equal to `MAX_UINT40`.
function intoUint40(UD60x18 x) pure returns (uint40 result) {
    uint256 xUint = UD60x18.unwrap(x);
    if (xUint > MAX_UINT40) {
        revert CastingErrors.PRBMath_UD60x18_IntoUint40_Overflow(x);
    }
    result = uint40(xUint);
}

/// @notice Alias for {wrap}.
function ud(uint256 x) pure returns (UD60x18 result) {
    result = UD60x18.wrap(x);
}

/// @notice Alias for {wrap}.
function ud60x18(uint256 x) pure returns (UD60x18 result) {
    result = UD60x18.wrap(x);
}

/// @notice Unwraps a UD60x18 number into uint256.
function unwrap(UD60x18 x) pure returns (uint256 result) {
    result = UD60x18.unwrap(x);
}

/// @notice Wraps a uint256 number into the UD60x18 value type.
function wrap(uint256 x) pure returns (UD60x18 result) {
    result = UD60x18.wrap(x);
}

/// @notice Implements the checked addition operation (+) in the UD60x18 type.
function add(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    result = wrap(x.unwrap() + y.unwrap());
}

/// @notice Implements the AND (&) bitwise operation in the UD60x18 type.
function and(UD60x18 x, uint256 bits) pure returns (UD60x18 result) {
    result = wrap(x.unwrap() & bits);
}

/// @notice Implements the AND (&) bitwise operation in the UD60x18 type.
function and2(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    result = wrap(x.unwrap() & y.unwrap());
}

/// @notice Implements the equal operation (==) in the UD60x18 type.
function eq(UD60x18 x, UD60x18 y) pure returns (bool result) {
    result = x.unwrap() == y.unwrap();
}

/// @notice Implements the greater than operation (>) in the UD60x18 type.
function gt(UD60x18 x, UD60x18 y) pure returns (bool result) {
    result = x.unwrap() > y.unwrap();
}

/// @notice Implements the greater than or equal to operation (>=) in the UD60x18 type.
function gte(UD60x18 x, UD60x18 y) pure returns (bool result) {
    result = x.unwrap() >= y.unwrap();
}

/// @notice Implements a zero comparison check function in the UD60x18 type.
function isZero(UD60x18 x) pure returns (bool result) {
    // This wouldn't work if x could be negative.
    result = x.unwrap() == 0;
}

/// @notice Implements the left shift operation (<<) in the UD60x18 type.
function lshift(UD60x18 x, uint256 bits) pure returns (UD60x18 result) {
    result = wrap(x.unwrap() << bits);
}

/// @notice Implements the lower than operation (<) in the UD60x18 type.
function lt(UD60x18 x, UD60x18 y) pure returns (bool result) {
    result = x.unwrap() < y.unwrap();
}

/// @notice Implements the lower than or equal to operation (<=) in the UD60x18 type.
function lte(UD60x18 x, UD60x18 y) pure returns (bool result) {
    result = x.unwrap() <= y.unwrap();
}

/// @notice Implements the checked modulo operation (%) in the UD60x18 type.
function mod(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    result = wrap(x.unwrap() % y.unwrap());
}

/// @notice Implements the not equal operation (!=) in the UD60x18 type.
function neq(UD60x18 x, UD60x18 y) pure returns (bool result) {
    result = x.unwrap() != y.unwrap();
}

/// @notice Implements the NOT (~) bitwise operation in the UD60x18 type.
function not(UD60x18 x) pure returns (UD60x18 result) {
    result = wrap(~x.unwrap());
}

/// @notice Implements the OR (|) bitwise operation in the UD60x18 type.
function or(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    result = wrap(x.unwrap() | y.unwrap());
}

/// @notice Implements the right shift operation (>>) in the UD60x18 type.
function rshift(UD60x18 x, uint256 bits) pure returns (UD60x18 result) {
    result = wrap(x.unwrap() >> bits);
}

/// @notice Implements the checked subtraction operation (-) in the UD60x18 type.
function sub(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    result = wrap(x.unwrap() - y.unwrap());
}

/// @notice Implements the unchecked addition operation (+) in the UD60x18 type.
function uncheckedAdd(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    unchecked {
        result = wrap(x.unwrap() + y.unwrap());
    }
}

/// @notice Implements the unchecked subtraction operation (-) in the UD60x18 type.
function uncheckedSub(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    unchecked {
        result = wrap(x.unwrap() - y.unwrap());
    }
}

/// @notice Implements the XOR (^) bitwise operation in the UD60x18 type.
function xor(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    result = wrap(x.unwrap() ^ y.unwrap());
}

// NOTICE: the "u" prefix stands for "unwrapped".

/// @dev Euler's number as a UD60x18 number.
UD60x18 constant E = UD60x18.wrap(2_718281828459045235);

/// @dev The maximum input permitted in {exp}.
uint256 constant uEXP_MAX_INPUT = 133_084258667509499440;
UD60x18 constant EXP_MAX_INPUT = UD60x18.wrap(uEXP_MAX_INPUT);

/// @dev The maximum input permitted in {exp2}.
uint256 constant uEXP2_MAX_INPUT = 192e18 - 1;
UD60x18 constant EXP2_MAX_INPUT = UD60x18.wrap(uEXP2_MAX_INPUT);

/// @dev Half the UNIT number.
uint256 constant uHALF_UNIT = 0.5e18;
UD60x18 constant HALF_UNIT = UD60x18.wrap(uHALF_UNIT);

/// @dev $log_2(10)$ as a UD60x18 number.
uint256 constant uLOG2_10 = 3_321928094887362347;
UD60x18 constant LOG2_10 = UD60x18.wrap(uLOG2_10);

/// @dev $log_2(e)$ as a UD60x18 number.
uint256 constant uLOG2_E = 1_442695040888963407;
UD60x18 constant LOG2_E = UD60x18.wrap(uLOG2_E);

/// @dev The maximum value a UD60x18 number can have.
uint256 constant uMAX_UD60x18 = 115792089237316195423570985008687907853269984665640564039457_584007913129639935;
UD60x18 constant MAX_UD60x18 = UD60x18.wrap(uMAX_UD60x18);

/// @dev The maximum whole value a UD60x18 number can have.
uint256 constant uMAX_WHOLE_UD60x18 = 115792089237316195423570985008687907853269984665640564039457_000000000000000000;
UD60x18 constant MAX_WHOLE_UD60x18 = UD60x18.wrap(uMAX_WHOLE_UD60x18);

/// @dev PI as a UD60x18 number.
UD60x18 constant PI = UD60x18.wrap(3_141592653589793238);

/// @dev The unit number, which gives the decimal precision of UD60x18.
uint256 constant uUNIT = 1e18;
UD60x18 constant UNIT = UD60x18.wrap(uUNIT);

/// @dev The unit number squared.
uint256 constant uUNIT_SQUARED = 1e36;
UD60x18 constant UNIT_SQUARED = UD60x18.wrap(uUNIT_SQUARED);

/// @dev Zero as a UD60x18 number.
UD60x18 constant ZERO = UD60x18.wrap(0);

/*//////////////////////////////////////////////////////////////////////////
                            MATHEMATICAL FUNCTIONS
//////////////////////////////////////////////////////////////////////////*/

/// @notice Calculates the arithmetic average of x and y using the following formula:
///
/// $$
/// avg(x, y) = (x & y) + ((xUint ^ yUint) / 2)
/// $$
//
/// In English, this is what this formula does:
///
/// 1. AND x and y.
/// 2. Calculate half of XOR x and y.
/// 3. Add the two results together.
///
/// This technique is known as SWAR, which stands for "SIMD within a register". You can read more about it here:
/// https://devblogs.microsoft.com/oldnewthing/20220207-00/?p=106223
///
/// @dev Notes:
/// - The result is rounded toward zero.
///
/// @param x The first operand as a UD60x18 number.
/// @param y The second operand as a UD60x18 number.
/// @return result The arithmetic average as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function avg(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    uint256 xUint = x.unwrap();
    uint256 yUint = y.unwrap();
    unchecked {
        result = wrap((xUint & yUint) + ((xUint ^ yUint) >> 1));
    }
}

/// @notice Yields the smallest whole number greater than or equal to x.
///
/// @dev This is optimized for fractional value inputs, because for every whole value there are (1e18 - 1) fractional
/// counterparts. See https://en.wikipedia.org/wiki/Floor_and_ceiling_functions.
///
/// Requirements:
/// - x must be less than or equal to `MAX_WHOLE_UD60x18`.
///
/// @param x The UD60x18 number to ceil.
/// @param result The smallest whole number greater than or equal to x, as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function ceil(UD60x18 x) pure returns (UD60x18 result) {
    uint256 xUint = x.unwrap();
    if (xUint > uMAX_WHOLE_UD60x18) {
        revert Errors.PRBMath_UD60x18_Ceil_Overflow(x);
    }

    assembly ("memory-safe") {
        // Equivalent to `x % UNIT`.
        let remainder := mod(x, uUNIT)

        // Equivalent to `UNIT - remainder`.
        let delta := sub(uUNIT, remainder)

        // Equivalent to `x + remainder > 0 ? delta : 0`.
        result := add(x, mul(delta, gt(remainder, 0)))
    }
}

/// @notice Divides two UD60x18 numbers, returning a new UD60x18 number.
///
/// @dev Uses {Common.mulDiv} to enable overflow-safe multiplication and division.
///
/// Notes:
/// - Refer to the notes in {Common.mulDiv}.
///
/// Requirements:
/// - Refer to the requirements in {Common.mulDiv}.
///
/// @param x The numerator as a UD60x18 number.
/// @param y The denominator as a UD60x18 number.
/// @param result The quotient as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function div(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    result = wrap(Common.mulDiv(x.unwrap(), uUNIT, y.unwrap()));
}

/// @notice Calculates the natural exponent of x using the following formula:
///
/// $$
/// e^x = 2^{x * log_2{e}}
/// $$
///
/// @dev Requirements:
/// - x must be less than 133_084258667509499441.
///
/// @param x The exponent as a UD60x18 number.
/// @return result The result as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function exp(UD60x18 x) pure returns (UD60x18 result) {
    uint256 xUint = x.unwrap();

    // This check prevents values greater than 192e18 from being passed to {exp2}.
    if (xUint > uEXP_MAX_INPUT) {
        revert Errors.PRBMath_UD60x18_Exp_InputTooBig(x);
    }

    unchecked {
        // Inline the fixed-point multiplication to save gas.
        uint256 doubleUnitProduct = xUint * uLOG2_E;
        result = exp2(wrap(doubleUnitProduct / uUNIT));
    }
}

/// @notice Calculates the binary exponent of x using the binary fraction method.
///
/// @dev See https://ethereum.stackexchange.com/q/79903/24693
///
/// Requirements:
/// - x must be less than 192e18.
/// - The result must fit in UD60x18.
///
/// @param x The exponent as a UD60x18 number.
/// @return result The result as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function exp2(UD60x18 x) pure returns (UD60x18 result) {
    uint256 xUint = x.unwrap();

    // Numbers greater than or equal to 192e18 don't fit in the 192.64-bit format.
    if (xUint > uEXP2_MAX_INPUT) {
        revert Errors.PRBMath_UD60x18_Exp2_InputTooBig(x);
    }

    // Convert x to the 192.64-bit fixed-point format.
    uint256 x_192x64 = (xUint << 64) / uUNIT;

    // Pass x to the {Common.exp2} function, which uses the 192.64-bit fixed-point number representation.
    result = wrap(Common.exp2(x_192x64));
}

/// @notice Yields the greatest whole number less than or equal to x.
/// @dev Optimized for fractional value inputs, because every whole value has (1e18 - 1) fractional counterparts.
/// See https://en.wikipedia.org/wiki/Floor_and_ceiling_functions.
/// @param x The UD60x18 number to floor.
/// @param result The greatest whole number less than or equal to x, as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function floor(UD60x18 x) pure returns (UD60x18 result) {
    assembly ("memory-safe") {
        // Equivalent to `x % UNIT`.
        let remainder := mod(x, uUNIT)

        // Equivalent to `x - remainder > 0 ? remainder : 0)`.
        result := sub(x, mul(remainder, gt(remainder, 0)))
    }
}

/// @notice Yields the excess beyond the floor of x using the odd function definition.
/// @dev See https://en.wikipedia.org/wiki/Fractional_part.
/// @param x The UD60x18 number to get the fractional part of.
/// @param result The fractional part of x as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function frac(UD60x18 x) pure returns (UD60x18 result) {
    assembly ("memory-safe") {
        result := mod(x, uUNIT)
    }
}

/// @notice Calculates the geometric mean of x and y, i.e. $\sqrt{x * y}$, rounding down.
///
/// @dev Requirements:
/// - x * y must fit in UD60x18.
///
/// @param x The first operand as a UD60x18 number.
/// @param y The second operand as a UD60x18 number.
/// @return result The result as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function gm(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    uint256 xUint = x.unwrap();
    uint256 yUint = y.unwrap();
    if (xUint == 0 || yUint == 0) {
        return ZERO;
    }

    unchecked {
        // Checking for overflow this way is faster than letting Solidity do it.
        uint256 xyUint = xUint * yUint;
        if (xyUint / xUint != yUint) {
            revert Errors.PRBMath_UD60x18_Gm_Overflow(x, y);
        }

        // We don't need to multiply the result by `UNIT` here because the x*y product picked up a factor of `UNIT`
        // during multiplication. See the comments in {Common.sqrt}.
        result = wrap(Common.sqrt(xyUint));
    }
}

/// @notice Calculates the inverse of x.
///
/// @dev Notes:
/// - The result is rounded toward zero.
///
/// Requirements:
/// - x must not be zero.
///
/// @param x The UD60x18 number for which to calculate the inverse.
/// @return result The inverse as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function inv(UD60x18 x) pure returns (UD60x18 result) {
    unchecked {
        result = wrap(uUNIT_SQUARED / x.unwrap());
    }
}

/// @notice Calculates the natural logarithm of x using the following formula:
///
/// $$
/// ln{x} = log_2{x} / log_2{e}
/// $$
///
/// @dev Notes:
/// - Refer to the notes in {log2}.
/// - The precision isn't sufficiently fine-grained to return exactly `UNIT` when the input is `E`.
///
/// Requirements:
/// - Refer to the requirements in {log2}.
///
/// @param x The UD60x18 number for which to calculate the natural logarithm.
/// @return result The natural logarithm as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function ln(UD60x18 x) pure returns (UD60x18 result) {
    unchecked {
        // Inline the fixed-point multiplication to save gas. This is overflow-safe because the maximum value that
        // {log2} can return is ~196_205294292027477728.
        result = wrap(log2(x).unwrap() * uUNIT / uLOG2_E);
    }
}

/// @notice Calculates the common logarithm of x using the following formula:
///
/// $$
/// log_{10}{x} = log_2{x} / log_2{10}
/// $$
///
/// However, if x is an exact power of ten, a hard coded value is returned.
///
/// @dev Notes:
/// - Refer to the notes in {log2}.
///
/// Requirements:
/// - Refer to the requirements in {log2}.
///
/// @param x The UD60x18 number for which to calculate the common logarithm.
/// @return result The common logarithm as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function log10(UD60x18 x) pure returns (UD60x18 result) {
    uint256 xUint = x.unwrap();
    if (xUint < uUNIT) {
        revert Errors.PRBMath_UD60x18_Log_InputTooSmall(x);
    }

    // Note that the `mul` in this assembly block is the standard multiplication operation, not {UD60x18.mul}.
    // prettier-ignore
    assembly ("memory-safe") {
        switch x
        case 1 { result := mul(uUNIT, sub(0, 18)) }
        case 10 { result := mul(uUNIT, sub(1, 18)) }
        case 100 { result := mul(uUNIT, sub(2, 18)) }
        case 1000 { result := mul(uUNIT, sub(3, 18)) }
        case 10000 { result := mul(uUNIT, sub(4, 18)) }
        case 100000 { result := mul(uUNIT, sub(5, 18)) }
        case 1000000 { result := mul(uUNIT, sub(6, 18)) }
        case 10000000 { result := mul(uUNIT, sub(7, 18)) }
        case 100000000 { result := mul(uUNIT, sub(8, 18)) }
        case 1000000000 { result := mul(uUNIT, sub(9, 18)) }
        case 10000000000 { result := mul(uUNIT, sub(10, 18)) }
        case 100000000000 { result := mul(uUNIT, sub(11, 18)) }
        case 1000000000000 { result := mul(uUNIT, sub(12, 18)) }
        case 10000000000000 { result := mul(uUNIT, sub(13, 18)) }
        case 100000000000000 { result := mul(uUNIT, sub(14, 18)) }
        case 1000000000000000 { result := mul(uUNIT, sub(15, 18)) }
        case 10000000000000000 { result := mul(uUNIT, sub(16, 18)) }
        case 100000000000000000 { result := mul(uUNIT, sub(17, 18)) }
        case 1000000000000000000 { result := 0 }
        case 10000000000000000000 { result := uUNIT }
        case 100000000000000000000 { result := mul(uUNIT, 2) }
        case 1000000000000000000000 { result := mul(uUNIT, 3) }
        case 10000000000000000000000 { result := mul(uUNIT, 4) }
        case 100000000000000000000000 { result := mul(uUNIT, 5) }
        case 1000000000000000000000000 { result := mul(uUNIT, 6) }
        case 10000000000000000000000000 { result := mul(uUNIT, 7) }
        case 100000000000000000000000000 { result := mul(uUNIT, 8) }
        case 1000000000000000000000000000 { result := mul(uUNIT, 9) }
        case 10000000000000000000000000000 { result := mul(uUNIT, 10) }
        case 100000000000000000000000000000 { result := mul(uUNIT, 11) }
        case 1000000000000000000000000000000 { result := mul(uUNIT, 12) }
        case 10000000000000000000000000000000 { result := mul(uUNIT, 13) }
        case 100000000000000000000000000000000 { result := mul(uUNIT, 14) }
        case 1000000000000000000000000000000000 { result := mul(uUNIT, 15) }
        case 10000000000000000000000000000000000 { result := mul(uUNIT, 16) }
        case 100000000000000000000000000000000000 { result := mul(uUNIT, 17) }
        case 1000000000000000000000000000000000000 { result := mul(uUNIT, 18) }
        case 10000000000000000000000000000000000000 { result := mul(uUNIT, 19) }
        case 100000000000000000000000000000000000000 { result := mul(uUNIT, 20) }
        case 1000000000000000000000000000000000000000 { result := mul(uUNIT, 21) }
        case 10000000000000000000000000000000000000000 { result := mul(uUNIT, 22) }
        case 100000000000000000000000000000000000000000 { result := mul(uUNIT, 23) }
        case 1000000000000000000000000000000000000000000 { result := mul(uUNIT, 24) }
        case 10000000000000000000000000000000000000000000 { result := mul(uUNIT, 25) }
        case 100000000000000000000000000000000000000000000 { result := mul(uUNIT, 26) }
        case 1000000000000000000000000000000000000000000000 { result := mul(uUNIT, 27) }
        case 10000000000000000000000000000000000000000000000 { result := mul(uUNIT, 28) }
        case 100000000000000000000000000000000000000000000000 { result := mul(uUNIT, 29) }
        case 1000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 30) }
        case 10000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 31) }
        case 100000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 32) }
        case 1000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 33) }
        case 10000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 34) }
        case 100000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 35) }
        case 1000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 36) }
        case 10000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 37) }
        case 100000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 38) }
        case 1000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 39) }
        case 10000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 40) }
        case 100000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 41) }
        case 1000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 42) }
        case 10000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 43) }
        case 100000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 44) }
        case 1000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 45) }
        case 10000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 46) }
        case 100000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 47) }
        case 1000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 48) }
        case 10000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 49) }
        case 100000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 50) }
        case 1000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 51) }
        case 10000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 52) }
        case 100000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 53) }
        case 1000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 54) }
        case 10000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 55) }
        case 100000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 56) }
        case 1000000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 57) }
        case 10000000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 58) }
        case 100000000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 59) }
        default { result := uMAX_UD60x18 }
    }

    if (result.unwrap() == uMAX_UD60x18) {
        unchecked {
            // Inline the fixed-point division to save gas.
            result = wrap(log2(x).unwrap() * uUNIT / uLOG2_10);
        }
    }
}

/// @notice Calculates the binary logarithm of x using the iterative approximation algorithm:
///
/// $$
/// log_2{x} = n + log_2{y}, \text{ where } y = x*2^{-n}, \ y \in [1, 2)
/// $$
///
/// For $0 \leq x \lt 1$, the input is inverted:
///
/// $$
/// log_2{x} = -log_2{\frac{1}{x}}
/// $$
///
/// @dev See https://en.wikipedia.org/wiki/Binary_logarithm#Iterative_approximation
///
/// Notes:
/// - Due to the lossy precision of the iterative approximation, the results are not perfectly accurate to the last decimal.
///
/// Requirements:
/// - x must be greater than zero.
///
/// @param x The UD60x18 number for which to calculate the binary logarithm.
/// @return result The binary logarithm as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function log2(UD60x18 x) pure returns (UD60x18 result) {
    uint256 xUint = x.unwrap();

    if (xUint < uUNIT) {
        revert Errors.PRBMath_UD60x18_Log_InputTooSmall(x);
    }

    unchecked {
        // Calculate the integer part of the logarithm.
        uint256 n = Common.msb(xUint / uUNIT);

        // This is the integer part of the logarithm as a UD60x18 number. The operation can't overflow because n
        // n is at most 255 and UNIT is 1e18.
        uint256 resultUint = n * uUNIT;

        // Calculate $y = x * 2^{-n}$.
        uint256 y = xUint >> n;

        // If y is the unit number, the fractional part is zero.
        if (y == uUNIT) {
            return wrap(resultUint);
        }

        // Calculate the fractional part via the iterative approximation.
        // The `delta >>= 1` part is equivalent to `delta /= 2`, but shifting bits is more gas efficient.
        uint256 DOUBLE_UNIT = 2e18;
        for (uint256 delta = uHALF_UNIT; delta > 0; delta >>= 1) {
            y = (y * y) / uUNIT;

            // Is y^2 >= 2e18 and so in the range [2e18, 4e18)?
            if (y >= DOUBLE_UNIT) {
                // Add the 2^{-m} factor to the logarithm.
                resultUint += delta;

                // Halve y, which corresponds to z/2 in the Wikipedia article.
                y >>= 1;
            }
        }
        result = wrap(resultUint);
    }
}

/// @notice Multiplies two UD60x18 numbers together, returning a new UD60x18 number.
///
/// @dev Uses {Common.mulDiv} to enable overflow-safe multiplication and division.
///
/// Notes:
/// - Refer to the notes in {Common.mulDiv}.
///
/// Requirements:
/// - Refer to the requirements in {Common.mulDiv}.
///
/// @dev See the documentation in {Common.mulDiv18}.
/// @param x The multiplicand as a UD60x18 number.
/// @param y The multiplier as a UD60x18 number.
/// @return result The product as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function mul(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    result = wrap(Common.mulDiv18(x.unwrap(), y.unwrap()));
}

/// @notice Raises x to the power of y.
///
/// For $1 \leq x \leq \infty$, the following standard formula is used:
///
/// $$
/// x^y = 2^{log_2{x} * y}
/// $$
///
/// For $0 \leq x \lt 1$, since the unsigned {log2} is undefined, an equivalent formula is used:
///
/// $$
/// i = \frac{1}{x}
/// w = 2^{log_2{i} * y}
/// x^y = \frac{1}{w}
/// $$
///
/// @dev Notes:
/// - Refer to the notes in {log2} and {mul}.
/// - Returns `UNIT` for 0^0.
/// - It may not perform well with very small values of x. Consider using SD59x18 as an alternative.
///
/// Requirements:
/// - Refer to the requirements in {exp2}, {log2}, and {mul}.
///
/// @param x The base as a UD60x18 number.
/// @param y The exponent as a UD60x18 number.
/// @return result The result as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function pow(UD60x18 x, UD60x18 y) pure returns (UD60x18 result) {
    uint256 xUint = x.unwrap();
    uint256 yUint = y.unwrap();

    // If both x and y are zero, the result is `UNIT`. If just x is zero, the result is always zero.
    if (xUint == 0) {
        return yUint == 0 ? UNIT : ZERO;
    }
    // If x is `UNIT`, the result is always `UNIT`.
    else if (xUint == uUNIT) {
        return UNIT;
    }

    // If y is zero, the result is always `UNIT`.
    if (yUint == 0) {
        return UNIT;
    }
    // If y is `UNIT`, the result is always x.
    else if (yUint == uUNIT) {
        return x;
    }

    // If x is greater than `UNIT`, use the standard formula.
    if (xUint > uUNIT) {
        result = exp2(mul(log2(x), y));
    }
    // Conversely, if x is less than `UNIT`, use the equivalent formula.
    else {
        UD60x18 i = wrap(uUNIT_SQUARED / xUint);
        UD60x18 w = exp2(mul(log2(i), y));
        result = wrap(uUNIT_SQUARED / w.unwrap());
    }
}

/// @notice Raises x (a UD60x18 number) to the power y (an unsigned basic integer) using the well-known
/// algorithm "exponentiation by squaring".
///
/// @dev See https://en.wikipedia.org/wiki/Exponentiation_by_squaring.
///
/// Notes:
/// - Refer to the notes in {Common.mulDiv18}.
/// - Returns `UNIT` for 0^0.
///
/// Requirements:
/// - The result must fit in UD60x18.
///
/// @param x The base as a UD60x18 number.
/// @param y The exponent as a uint256.
/// @return result The result as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function powu(UD60x18 x, uint256 y) pure returns (UD60x18 result) {
    // Calculate the first iteration of the loop in advance.
    uint256 xUint = x.unwrap();
    uint256 resultUint = y & 1 > 0 ? xUint : uUNIT;

    // Equivalent to `for(y /= 2; y > 0; y /= 2)`.
    for (y >>= 1; y > 0; y >>= 1) {
        xUint = Common.mulDiv18(xUint, xUint);

        // Equivalent to `y % 2 == 1`.
        if (y & 1 > 0) {
            resultUint = Common.mulDiv18(resultUint, xUint);
        }
    }
    result = wrap(resultUint);
}

/// @notice Calculates the square root of x using the Babylonian method.
///
/// @dev See https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method.
///
/// Notes:
/// - The result is rounded toward zero.
///
/// Requirements:
/// - x must be less than `MAX_UD60x18 / UNIT`.
///
/// @param x The UD60x18 number for which to calculate the square root.
/// @return result The result as a UD60x18 number.
/// @custom:smtchecker abstract-function-nondet
function sqrt(UD60x18 x) pure returns (UD60x18 result) {
    uint256 xUint = x.unwrap();

    unchecked {
        if (xUint > uMAX_UD60x18 / uUNIT) {
            revert Errors.PRBMath_UD60x18_Sqrt_Overflow(x);
        }
        // Multiply x by `UNIT` to account for the factor of `UNIT` picked up when multiplying two UD60x18 numbers.
        // In this case, the two numbers are both the square root.
        result = wrap(Common.sqrt(xUint * uUNIT));
    }
}

/// @notice The unsigned 60.18-decimal fixed-point number representation, which can have up to 60 digits and up to 18
/// decimals. The values of this are bound by the minimum and the maximum values permitted by the Solidity type uint256.
/// @dev The value type is defined here so it can be imported in all other files.
type UD60x18 is uint256;

/*//////////////////////////////////////////////////////////////////////////
                                    CASTING
//////////////////////////////////////////////////////////////////////////*/

using {
    Casting.intoSD1x18,
    Casting.intoUD2x18,
    Casting.intoSD59x18,
    Casting.intoUint128,
    Casting.intoUint256,
    Casting.intoUint40,
    Casting.unwrap
} for UD60x18 global;

/*//////////////////////////////////////////////////////////////////////////
                            MATHEMATICAL FUNCTIONS
//////////////////////////////////////////////////////////////////////////*/

// The global "using for" directive makes the functions in this library callable on the UD60x18 type.
using {
    Math.avg,
    Math.ceil,
    Math.div,
    Math.exp,
    Math.exp2,
    Math.floor,
    Math.frac,
    Math.gm,
    Math.inv,
    Math.ln,
    Math.log10,
    Math.log2,
    Math.mul,
    Math.pow,
    Math.powu,
    Math.sqrt
} for UD60x18 global;

/*//////////////////////////////////////////////////////////////////////////
                                HELPER FUNCTIONS
//////////////////////////////////////////////////////////////////////////*/

// The global "using for" directive makes the functions in this library callable on the UD60x18 type.
using {
    Helpers.add,
    Helpers.and,
    Helpers.eq,
    Helpers.gt,
    Helpers.gte,
    Helpers.isZero,
    Helpers.lshift,
    Helpers.lt,
    Helpers.lte,
    Helpers.mod,
    Helpers.neq,
    Helpers.not,
    Helpers.or,
    Helpers.rshift,
    Helpers.sub,
    Helpers.uncheckedAdd,
    Helpers.uncheckedSub,
    Helpers.xor
} for UD60x18 global;

/*//////////////////////////////////////////////////////////////////////////
                                    OPERATORS
//////////////////////////////////////////////////////////////////////////*/

// The global "using for" directive makes it possible to use these operators on the UD60x18 type.
using {
    Helpers.add as +,
    Helpers.and2 as &,
    Math.div as /,
    Helpers.eq as ==,
    Helpers.gt as >,
    Helpers.gte as >=,
    Helpers.lt as <,
    Helpers.lte as <=,
    Helpers.or as |,
    Helpers.mod as %,
    Math.mul as *,
    Helpers.neq as !=,
    Helpers.not as ~,
    Helpers.sub as -,
    Helpers.xor as ^
} for UD60x18 global;

/// @notice Casts a UD2x18 number into SD1x18.
/// - x must be less than or equal to `uMAX_SD1x18`.
function intoSD1x18(UD2x18 x) pure returns (SD1x18 result) {
    uint64 xUint = UD2x18.unwrap(x);
    if (xUint > uint64(uMAX_SD1x18)) {
        revert Errors.PRBMath_UD2x18_IntoSD1x18_Overflow(x);
    }
    result = SD1x18.wrap(int64(xUint));
}

/// @notice Casts a UD2x18 number into SD59x18.
/// @dev There is no overflow check because the domain of UD2x18 is a subset of SD59x18.
function intoSD59x18(UD2x18 x) pure returns (SD59x18 result) {
    result = SD59x18.wrap(int256(uint256(UD2x18.unwrap(x))));
}

/// @notice Casts a UD2x18 number into UD60x18.
/// @dev There is no overflow check because the domain of UD2x18 is a subset of UD60x18.
function intoUD60x18(UD2x18 x) pure returns (UD60x18 result) {
    result = UD60x18.wrap(UD2x18.unwrap(x));
}

/// @notice Casts a UD2x18 number into uint128.
/// @dev There is no overflow check because the domain of UD2x18 is a subset of uint128.
function intoUint128(UD2x18 x) pure returns (uint128 result) {
    result = uint128(UD2x18.unwrap(x));
}

/// @notice Casts a UD2x18 number into uint256.
/// @dev There is no overflow check because the domain of UD2x18 is a subset of uint256.
function intoUint256(UD2x18 x) pure returns (uint256 result) {
    result = uint256(UD2x18.unwrap(x));
}

/// @notice Casts a UD2x18 number into uint40.
/// @dev Requirements:
/// - x must be less than or equal to `MAX_UINT40`.
function intoUint40(UD2x18 x) pure returns (uint40 result) {
    uint64 xUint = UD2x18.unwrap(x);
    if (xUint > uint64(Common.MAX_UINT40)) {
        revert Errors.PRBMath_UD2x18_IntoUint40_Overflow(x);
    }
    result = uint40(xUint);
}

/// @notice Alias for {wrap}.
function ud2x18(uint64 x) pure returns (UD2x18 result) {
    result = UD2x18.wrap(x);
}

/// @notice Unwrap a UD2x18 number into uint64.
function unwrap(UD2x18 x) pure returns (uint64 result) {
    result = UD2x18.unwrap(x);
}

/// @notice Wraps a uint64 number into UD2x18.
function wrap(uint64 x) pure returns (UD2x18 result) {
    result = UD2x18.wrap(x);
}

/// @notice The unsigned 2.18-decimal fixed-point number representation, which can have up to 2 digits and up to 18
/// decimals. The values of this are bound by the minimum and the maximum values permitted by the underlying Solidity
/// type uint64. This is useful when end users want to use uint64 to save gas, e.g. with tight variable packing in contract
/// storage.
type UD2x18 is uint64;

/*//////////////////////////////////////////////////////////////////////////
                                    CASTING
//////////////////////////////////////////////////////////////////////////*/

using {
    Casting.intoSD1x18,
    Casting.intoSD59x18,
    Casting.intoUD60x18,
    Casting.intoUint256,
    Casting.intoUint128,
    Casting.intoUint40,
    Casting.unwrap
} for UD2x18 global;

/// @dev Euler's number as a UD2x18 number.
UD2x18 constant E = UD2x18.wrap(2_718281828459045235);

/// @dev The maximum value a UD2x18 number can have.
uint64 constant uMAX_UD2x18 = 18_446744073709551615;
UD2x18 constant MAX_UD2x18 = UD2x18.wrap(uMAX_UD2x18);

/// @dev PI as a UD2x18 number.
UD2x18 constant PI = UD2x18.wrap(3_141592653589793238);

/// @dev The unit number, which gives the decimal precision of UD2x18.
uint256 constant uUNIT = 1e18;
UD2x18 constant UNIT = UD2x18.wrap(1e18);

/// @notice Casts an SD59x18 number into int256.
/// @dev This is basically a functional alias for {unwrap}.
function intoInt256(SD59x18 x) pure returns (int256 result) {
    result = SD59x18.unwrap(x);
}

/// @notice Casts an SD59x18 number into SD1x18.
/// @dev Requirements:
/// - x must be greater than or equal to `uMIN_SD1x18`.
/// - x must be less than or equal to `uMAX_SD1x18`.
function intoSD1x18(SD59x18 x) pure returns (SD1x18 result) {
    int256 xInt = SD59x18.unwrap(x);
    if (xInt < uMIN_SD1x18) {
        revert CastingErrors.PRBMath_SD59x18_IntoSD1x18_Underflow(x);
    }
    if (xInt > uMAX_SD1x18) {
        revert CastingErrors.PRBMath_SD59x18_IntoSD1x18_Overflow(x);
    }
    result = SD1x18.wrap(int64(xInt));
}

/// @notice Casts an SD59x18 number into UD2x18.
/// @dev Requirements:
/// - x must be positive.
/// - x must be less than or equal to `uMAX_UD2x18`.
function intoUD2x18(SD59x18 x) pure returns (UD2x18 result) {
    int256 xInt = SD59x18.unwrap(x);
    if (xInt < 0) {
        revert CastingErrors.PRBMath_SD59x18_IntoUD2x18_Underflow(x);
    }
    if (xInt > int256(uint256(uMAX_UD2x18))) {
        revert CastingErrors.PRBMath_SD59x18_IntoUD2x18_Overflow(x);
    }
    result = UD2x18.wrap(uint64(uint256(xInt)));
}

/// @notice Casts an SD59x18 number into UD60x18.
/// @dev Requirements:
/// - x must be positive.
function intoUD60x18(SD59x18 x) pure returns (UD60x18 result) {
    int256 xInt = SD59x18.unwrap(x);
    if (xInt < 0) {
        revert CastingErrors.PRBMath_SD59x18_IntoUD60x18_Underflow(x);
    }
    result = UD60x18.wrap(uint256(xInt));
}

/// @notice Casts an SD59x18 number into uint256.
/// @dev Requirements:
/// - x must be positive.
function intoUint256(SD59x18 x) pure returns (uint256 result) {
    int256 xInt = SD59x18.unwrap(x);
    if (xInt < 0) {
        revert CastingErrors.PRBMath_SD59x18_IntoUint256_Underflow(x);
    }
    result = uint256(xInt);
}

/// @notice Casts an SD59x18 number into uint128.
/// @dev Requirements:
/// - x must be positive.
/// - x must be less than or equal to `uMAX_UINT128`.
function intoUint128(SD59x18 x) pure returns (uint128 result) {
    int256 xInt = SD59x18.unwrap(x);
    if (xInt < 0) {
        revert CastingErrors.PRBMath_SD59x18_IntoUint128_Underflow(x);
    }
    if (xInt > int256(uint256(MAX_UINT128))) {
        revert CastingErrors.PRBMath_SD59x18_IntoUint128_Overflow(x);
    }
    result = uint128(uint256(xInt));
}

/// @notice Casts an SD59x18 number into uint40.
/// @dev Requirements:
/// - x must be positive.
/// - x must be less than or equal to `MAX_UINT40`.
function intoUint40(SD59x18 x) pure returns (uint40 result) {
    int256 xInt = SD59x18.unwrap(x);
    if (xInt < 0) {
        revert CastingErrors.PRBMath_SD59x18_IntoUint40_Underflow(x);
    }
    if (xInt > int256(uint256(MAX_UINT40))) {
        revert CastingErrors.PRBMath_SD59x18_IntoUint40_Overflow(x);
    }
    result = uint40(uint256(xInt));
}

/// @notice Alias for {wrap}.
function sd(int256 x) pure returns (SD59x18 result) {
    result = SD59x18.wrap(x);
}

/// @notice Alias for {wrap}.
function sd59x18(int256 x) pure returns (SD59x18 result) {
    result = SD59x18.wrap(x);
}

/// @notice Unwraps an SD59x18 number into int256.
function unwrap(SD59x18 x) pure returns (int256 result) {
    result = SD59x18.unwrap(x);
}

/// @notice Wraps an int256 number into SD59x18.
function wrap(int256 x) pure returns (SD59x18 result) {
    result = SD59x18.wrap(x);
}

/// @notice Implements the checked addition operation (+) in the SD59x18 type.
function add(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    return wrap(x.unwrap() + y.unwrap());
}

/// @notice Implements the AND (&) bitwise operation in the SD59x18 type.
function and(SD59x18 x, int256 bits) pure returns (SD59x18 result) {
    return wrap(x.unwrap() & bits);
}

/// @notice Implements the AND (&) bitwise operation in the SD59x18 type.
function and2(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    return wrap(x.unwrap() & y.unwrap());
}

/// @notice Implements the equal (=) operation in the SD59x18 type.
function eq(SD59x18 x, SD59x18 y) pure returns (bool result) {
    result = x.unwrap() == y.unwrap();
}

/// @notice Implements the greater than operation (>) in the SD59x18 type.
function gt(SD59x18 x, SD59x18 y) pure returns (bool result) {
    result = x.unwrap() > y.unwrap();
}

/// @notice Implements the greater than or equal to operation (>=) in the SD59x18 type.
function gte(SD59x18 x, SD59x18 y) pure returns (bool result) {
    result = x.unwrap() >= y.unwrap();
}

/// @notice Implements a zero comparison check function in the SD59x18 type.
function isZero(SD59x18 x) pure returns (bool result) {
    result = x.unwrap() == 0;
}

/// @notice Implements the left shift operation (<<) in the SD59x18 type.
function lshift(SD59x18 x, uint256 bits) pure returns (SD59x18 result) {
    result = wrap(x.unwrap() << bits);
}

/// @notice Implements the lower than operation (<) in the SD59x18 type.
function lt(SD59x18 x, SD59x18 y) pure returns (bool result) {
    result = x.unwrap() < y.unwrap();
}

/// @notice Implements the lower than or equal to operation (<=) in the SD59x18 type.
function lte(SD59x18 x, SD59x18 y) pure returns (bool result) {
    result = x.unwrap() <= y.unwrap();
}

/// @notice Implements the unchecked modulo operation (%) in the SD59x18 type.
function mod(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    result = wrap(x.unwrap() % y.unwrap());
}

/// @notice Implements the not equal operation (!=) in the SD59x18 type.
function neq(SD59x18 x, SD59x18 y) pure returns (bool result) {
    result = x.unwrap() != y.unwrap();
}

/// @notice Implements the NOT (~) bitwise operation in the SD59x18 type.
function not(SD59x18 x) pure returns (SD59x18 result) {
    result = wrap(~x.unwrap());
}

/// @notice Implements the OR (|) bitwise operation in the SD59x18 type.
function or(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    result = wrap(x.unwrap() | y.unwrap());
}

/// @notice Implements the right shift operation (>>) in the SD59x18 type.
function rshift(SD59x18 x, uint256 bits) pure returns (SD59x18 result) {
    result = wrap(x.unwrap() >> bits);
}

/// @notice Implements the checked subtraction operation (-) in the SD59x18 type.
function sub(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    result = wrap(x.unwrap() - y.unwrap());
}

/// @notice Implements the checked unary minus operation (-) in the SD59x18 type.
function unary(SD59x18 x) pure returns (SD59x18 result) {
    result = wrap(-x.unwrap());
}

/// @notice Implements the unchecked addition operation (+) in the SD59x18 type.
function uncheckedAdd(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    unchecked {
        result = wrap(x.unwrap() + y.unwrap());
    }
}

/// @notice Implements the unchecked subtraction operation (-) in the SD59x18 type.
function uncheckedSub(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    unchecked {
        result = wrap(x.unwrap() - y.unwrap());
    }
}

/// @notice Implements the unchecked unary minus operation (-) in the SD59x18 type.
function uncheckedUnary(SD59x18 x) pure returns (SD59x18 result) {
    unchecked {
        result = wrap(-x.unwrap());
    }
}

/// @notice Implements the XOR (^) bitwise operation in the SD59x18 type.
function xor(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    result = wrap(x.unwrap() ^ y.unwrap());
}

/// @notice Calculates the absolute value of x.
///
/// @dev Requirements:
/// - x must be greater than `MIN_SD59x18`.
///
/// @param x The SD59x18 number for which to calculate the absolute value.
/// @param result The absolute value of x as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function abs(SD59x18 x) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    if (xInt == uMIN_SD59x18) {
        revert Errors.PRBMath_SD59x18_Abs_MinSD59x18();
    }
    result = xInt < 0 ? wrap(-xInt) : x;
}

/// @notice Calculates the arithmetic average of x and y.
///
/// @dev Notes:
/// - The result is rounded toward zero.
///
/// @param x The first operand as an SD59x18 number.
/// @param y The second operand as an SD59x18 number.
/// @return result The arithmetic average as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function avg(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    int256 yInt = y.unwrap();

    unchecked {
        // This operation is equivalent to `x / 2 +  y / 2`, and it can never overflow.
        int256 sum = (xInt >> 1) + (yInt >> 1);

        if (sum < 0) {
            // If at least one of x and y is odd, add 1 to the result, because shifting negative numbers to the right
            // rounds toward negative infinity. The right part is equivalent to `sum + (x % 2 == 1 || y % 2 == 1)`.
            assembly ("memory-safe") {
                result := add(sum, and(or(xInt, yInt), 1))
            }
        } else {
            // Add 1 if both x and y are odd to account for the double 0.5 remainder truncated after shifting.
            result = wrap(sum + (xInt & yInt & 1));
        }
    }
}

/// @notice Yields the smallest whole number greater than or equal to x.
///
/// @dev Optimized for fractional value inputs, because every whole value has (1e18 - 1) fractional counterparts.
/// See https://en.wikipedia.org/wiki/Floor_and_ceiling_functions.
///
/// Requirements:
/// - x must be less than or equal to `MAX_WHOLE_SD59x18`.
///
/// @param x The SD59x18 number to ceil.
/// @param result The smallest whole number greater than or equal to x, as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function ceil(SD59x18 x) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    if (xInt > uMAX_WHOLE_SD59x18) {
        revert Errors.PRBMath_SD59x18_Ceil_Overflow(x);
    }

    int256 remainder = xInt % uUNIT;
    if (remainder == 0) {
        result = x;
    } else {
        unchecked {
            // Solidity uses C fmod style, which returns a modulus with the same sign as x.
            int256 resultInt = xInt - remainder;
            if (xInt > 0) {
                resultInt += uUNIT;
            }
            result = wrap(resultInt);
        }
    }
}

/// @notice Divides two SD59x18 numbers, returning a new SD59x18 number.
///
/// @dev This is an extension of {Common.mulDiv} for signed numbers, which works by computing the signs and the absolute
/// values separately.
///
/// Notes:
/// - Refer to the notes in {Common.mulDiv}.
/// - The result is rounded toward zero.
///
/// Requirements:
/// - Refer to the requirements in {Common.mulDiv}.
/// - None of the inputs can be `MIN_SD59x18`.
/// - The denominator must not be zero.
/// - The result must fit in SD59x18.
///
/// @param x The numerator as an SD59x18 number.
/// @param y The denominator as an SD59x18 number.
/// @param result The quotient as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function div(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    int256 yInt = y.unwrap();
    if (xInt == uMIN_SD59x18 || yInt == uMIN_SD59x18) {
        revert Errors.PRBMath_SD59x18_Div_InputTooSmall();
    }

    // Get hold of the absolute values of x and y.
    uint256 xAbs;
    uint256 yAbs;
    unchecked {
        xAbs = xInt < 0 ? uint256(-xInt) : uint256(xInt);
        yAbs = yInt < 0 ? uint256(-yInt) : uint256(yInt);
    }

    // Compute the absolute value (x*UNIT÷y). The resulting value must fit in SD59x18.
    uint256 resultAbs = Common.mulDiv(xAbs, uint256(uUNIT), yAbs);
    if (resultAbs > uint256(uMAX_SD59x18)) {
        revert Errors.PRBMath_SD59x18_Div_Overflow(x, y);
    }

    // Check if x and y have the same sign using two's complement representation. The left-most bit represents the sign (1 for
    // negative, 0 for positive or zero).
    bool sameSign = (xInt ^ yInt) > -1;

    // If the inputs have the same sign, the result should be positive. Otherwise, it should be negative.
    unchecked {
        result = wrap(sameSign ? int256(resultAbs) : -int256(resultAbs));
    }
}

/// @notice Calculates the natural exponent of x using the following formula:
///
/// $$
/// e^x = 2^{x * log_2{e}}
/// $$
///
/// @dev Notes:
/// - Refer to the notes in {exp2}.
///
/// Requirements:
/// - Refer to the requirements in {exp2}.
/// - x must be less than 133_084258667509499441.
///
/// @param x The exponent as an SD59x18 number.
/// @return result The result as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function exp(SD59x18 x) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();

    // This check prevents values greater than 192e18 from being passed to {exp2}.
    if (xInt > uEXP_MAX_INPUT) {
        revert Errors.PRBMath_SD59x18_Exp_InputTooBig(x);
    }

    unchecked {
        // Inline the fixed-point multiplication to save gas.
        int256 doubleUnitProduct = xInt * uLOG2_E;
        result = exp2(wrap(doubleUnitProduct / uUNIT));
    }
}

/// @notice Calculates the binary exponent of x using the binary fraction method using the following formula:
///
/// $$
/// 2^{-x} = \frac{1}{2^x}
/// $$
///
/// @dev See https://ethereum.stackexchange.com/q/79903/24693.
///
/// Notes:
/// - If x is less than -59_794705707972522261, the result is zero.
///
/// Requirements:
/// - x must be less than 192e18.
/// - The result must fit in SD59x18.
///
/// @param x The exponent as an SD59x18 number.
/// @return result The result as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function exp2(SD59x18 x) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    if (xInt < 0) {
        // The inverse of any number less than this is truncated to zero.
        if (xInt < -59_794705707972522261) {
            return ZERO;
        }

        unchecked {
            // Inline the fixed-point inversion to save gas.
            result = wrap(uUNIT_SQUARED / exp2(wrap(-xInt)).unwrap());
        }
    } else {
        // Numbers greater than or equal to 192e18 don't fit in the 192.64-bit format.
        if (xInt > uEXP2_MAX_INPUT) {
            revert Errors.PRBMath_SD59x18_Exp2_InputTooBig(x);
        }

        unchecked {
            // Convert x to the 192.64-bit fixed-point format.
            uint256 x_192x64 = uint256((xInt << 64) / uUNIT);

            // It is safe to cast the result to int256 due to the checks above.
            result = wrap(int256(Common.exp2(x_192x64)));
        }
    }
}

/// @notice Yields the greatest whole number less than or equal to x.
///
/// @dev Optimized for fractional value inputs, because for every whole value there are (1e18 - 1) fractional
/// counterparts. See https://en.wikipedia.org/wiki/Floor_and_ceiling_functions.
///
/// Requirements:
/// - x must be greater than or equal to `MIN_WHOLE_SD59x18`.
///
/// @param x The SD59x18 number to floor.
/// @param result The greatest whole number less than or equal to x, as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function floor(SD59x18 x) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    if (xInt < uMIN_WHOLE_SD59x18) {
        revert Errors.PRBMath_SD59x18_Floor_Underflow(x);
    }

    int256 remainder = xInt % uUNIT;
    if (remainder == 0) {
        result = x;
    } else {
        unchecked {
            // Solidity uses C fmod style, which returns a modulus with the same sign as x.
            int256 resultInt = xInt - remainder;
            if (xInt < 0) {
                resultInt -= uUNIT;
            }
            result = wrap(resultInt);
        }
    }
}

/// @notice Yields the excess beyond the floor of x for positive numbers and the part of the number to the right.
/// of the radix point for negative numbers.
/// @dev Based on the odd function definition. https://en.wikipedia.org/wiki/Fractional_part
/// @param x The SD59x18 number to get the fractional part of.
/// @param result The fractional part of x as an SD59x18 number.
function frac(SD59x18 x) pure returns (SD59x18 result) {
    result = wrap(x.unwrap() % uUNIT);
}

/// @notice Calculates the geometric mean of x and y, i.e. $\sqrt{x * y}$.
///
/// @dev Notes:
/// - The result is rounded toward zero.
///
/// Requirements:
/// - x * y must fit in SD59x18.
/// - x * y must not be negative, since complex numbers are not supported.
///
/// @param x The first operand as an SD59x18 number.
/// @param y The second operand as an SD59x18 number.
/// @return result The result as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function gm(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    int256 yInt = y.unwrap();
    if (xInt == 0 || yInt == 0) {
        return ZERO;
    }

    unchecked {
        // Equivalent to `xy / x != y`. Checking for overflow this way is faster than letting Solidity do it.
        int256 xyInt = xInt * yInt;
        if (xyInt / xInt != yInt) {
            revert Errors.PRBMath_SD59x18_Gm_Overflow(x, y);
        }

        // The product must not be negative, since complex numbers are not supported.
        if (xyInt < 0) {
            revert Errors.PRBMath_SD59x18_Gm_NegativeProduct(x, y);
        }

        // We don't need to multiply the result by `UNIT` here because the x*y product picked up a factor of `UNIT`
        // during multiplication. See the comments in {Common.sqrt}.
        uint256 resultUint = Common.sqrt(uint256(xyInt));
        result = wrap(int256(resultUint));
    }
}

/// @notice Calculates the inverse of x.
///
/// @dev Notes:
/// - The result is rounded toward zero.
///
/// Requirements:
/// - x must not be zero.
///
/// @param x The SD59x18 number for which to calculate the inverse.
/// @return result The inverse as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function inv(SD59x18 x) pure returns (SD59x18 result) {
    result = wrap(uUNIT_SQUARED / x.unwrap());
}

/// @notice Calculates the natural logarithm of x using the following formula:
///
/// $$
/// ln{x} = log_2{x} / log_2{e}
/// $$
///
/// @dev Notes:
/// - Refer to the notes in {log2}.
/// - The precision isn't sufficiently fine-grained to return exactly `UNIT` when the input is `E`.
///
/// Requirements:
/// - Refer to the requirements in {log2}.
///
/// @param x The SD59x18 number for which to calculate the natural logarithm.
/// @return result The natural logarithm as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function ln(SD59x18 x) pure returns (SD59x18 result) {
    // Inline the fixed-point multiplication to save gas. This is overflow-safe because the maximum value that
    // {log2} can return is ~195_205294292027477728.
    result = wrap(log2(x).unwrap() * uUNIT / uLOG2_E);
}

/// @notice Calculates the common logarithm of x using the following formula:
///
/// $$
/// log_{10}{x} = log_2{x} / log_2{10}
/// $$
///
/// However, if x is an exact power of ten, a hard coded value is returned.
///
/// @dev Notes:
/// - Refer to the notes in {log2}.
///
/// Requirements:
/// - Refer to the requirements in {log2}.
///
/// @param x The SD59x18 number for which to calculate the common logarithm.
/// @return result The common logarithm as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function log10(SD59x18 x) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    if (xInt < 0) {
        revert Errors.PRBMath_SD59x18_Log_InputTooSmall(x);
    }

    // Note that the `mul` in this block is the standard multiplication operation, not {SD59x18.mul}.
    // prettier-ignore
    assembly ("memory-safe") {
        switch x
        case 1 { result := mul(uUNIT, sub(0, 18)) }
        case 10 { result := mul(uUNIT, sub(1, 18)) }
        case 100 { result := mul(uUNIT, sub(2, 18)) }
        case 1000 { result := mul(uUNIT, sub(3, 18)) }
        case 10000 { result := mul(uUNIT, sub(4, 18)) }
        case 100000 { result := mul(uUNIT, sub(5, 18)) }
        case 1000000 { result := mul(uUNIT, sub(6, 18)) }
        case 10000000 { result := mul(uUNIT, sub(7, 18)) }
        case 100000000 { result := mul(uUNIT, sub(8, 18)) }
        case 1000000000 { result := mul(uUNIT, sub(9, 18)) }
        case 10000000000 { result := mul(uUNIT, sub(10, 18)) }
        case 100000000000 { result := mul(uUNIT, sub(11, 18)) }
        case 1000000000000 { result := mul(uUNIT, sub(12, 18)) }
        case 10000000000000 { result := mul(uUNIT, sub(13, 18)) }
        case 100000000000000 { result := mul(uUNIT, sub(14, 18)) }
        case 1000000000000000 { result := mul(uUNIT, sub(15, 18)) }
        case 10000000000000000 { result := mul(uUNIT, sub(16, 18)) }
        case 100000000000000000 { result := mul(uUNIT, sub(17, 18)) }
        case 1000000000000000000 { result := 0 }
        case 10000000000000000000 { result := uUNIT }
        case 100000000000000000000 { result := mul(uUNIT, 2) }
        case 1000000000000000000000 { result := mul(uUNIT, 3) }
        case 10000000000000000000000 { result := mul(uUNIT, 4) }
        case 100000000000000000000000 { result := mul(uUNIT, 5) }
        case 1000000000000000000000000 { result := mul(uUNIT, 6) }
        case 10000000000000000000000000 { result := mul(uUNIT, 7) }
        case 100000000000000000000000000 { result := mul(uUNIT, 8) }
        case 1000000000000000000000000000 { result := mul(uUNIT, 9) }
        case 10000000000000000000000000000 { result := mul(uUNIT, 10) }
        case 100000000000000000000000000000 { result := mul(uUNIT, 11) }
        case 1000000000000000000000000000000 { result := mul(uUNIT, 12) }
        case 10000000000000000000000000000000 { result := mul(uUNIT, 13) }
        case 100000000000000000000000000000000 { result := mul(uUNIT, 14) }
        case 1000000000000000000000000000000000 { result := mul(uUNIT, 15) }
        case 10000000000000000000000000000000000 { result := mul(uUNIT, 16) }
        case 100000000000000000000000000000000000 { result := mul(uUNIT, 17) }
        case 1000000000000000000000000000000000000 { result := mul(uUNIT, 18) }
        case 10000000000000000000000000000000000000 { result := mul(uUNIT, 19) }
        case 100000000000000000000000000000000000000 { result := mul(uUNIT, 20) }
        case 1000000000000000000000000000000000000000 { result := mul(uUNIT, 21) }
        case 10000000000000000000000000000000000000000 { result := mul(uUNIT, 22) }
        case 100000000000000000000000000000000000000000 { result := mul(uUNIT, 23) }
        case 1000000000000000000000000000000000000000000 { result := mul(uUNIT, 24) }
        case 10000000000000000000000000000000000000000000 { result := mul(uUNIT, 25) }
        case 100000000000000000000000000000000000000000000 { result := mul(uUNIT, 26) }
        case 1000000000000000000000000000000000000000000000 { result := mul(uUNIT, 27) }
        case 10000000000000000000000000000000000000000000000 { result := mul(uUNIT, 28) }
        case 100000000000000000000000000000000000000000000000 { result := mul(uUNIT, 29) }
        case 1000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 30) }
        case 10000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 31) }
        case 100000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 32) }
        case 1000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 33) }
        case 10000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 34) }
        case 100000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 35) }
        case 1000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 36) }
        case 10000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 37) }
        case 100000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 38) }
        case 1000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 39) }
        case 10000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 40) }
        case 100000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 41) }
        case 1000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 42) }
        case 10000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 43) }
        case 100000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 44) }
        case 1000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 45) }
        case 10000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 46) }
        case 100000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 47) }
        case 1000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 48) }
        case 10000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 49) }
        case 100000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 50) }
        case 1000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 51) }
        case 10000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 52) }
        case 100000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 53) }
        case 1000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 54) }
        case 10000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 55) }
        case 100000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 56) }
        case 1000000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 57) }
        case 10000000000000000000000000000000000000000000000000000000000000000000000000000 { result := mul(uUNIT, 58) }
        default { result := uMAX_SD59x18 }
    }

    if (result.unwrap() == uMAX_SD59x18) {
        unchecked {
            // Inline the fixed-point division to save gas.
            result = wrap(log2(x).unwrap() * uUNIT / uLOG2_10);
        }
    }
}

/// @notice Calculates the binary logarithm of x using the iterative approximation algorithm:
///
/// $$
/// log_2{x} = n + log_2{y}, \text{ where } y = x*2^{-n}, \ y \in [1, 2)
/// $$
///
/// For $0 \leq x \lt 1$, the input is inverted:
///
/// $$
/// log_2{x} = -log_2{\frac{1}{x}}
/// $$
///
/// @dev See https://en.wikipedia.org/wiki/Binary_logarithm#Iterative_approximation.
///
/// Notes:
/// - Due to the lossy precision of the iterative approximation, the results are not perfectly accurate to the last decimal.
///
/// Requirements:
/// - x must be greater than zero.
///
/// @param x The SD59x18 number for which to calculate the binary logarithm.
/// @return result The binary logarithm as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function log2(SD59x18 x) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    if (xInt <= 0) {
        revert Errors.PRBMath_SD59x18_Log_InputTooSmall(x);
    }

    unchecked {
        int256 sign;
        if (xInt >= uUNIT) {
            sign = 1;
        } else {
            sign = -1;
            // Inline the fixed-point inversion to save gas.
            xInt = uUNIT_SQUARED / xInt;
        }

        // Calculate the integer part of the logarithm.
        uint256 n = Common.msb(uint256(xInt / uUNIT));

        // This is the integer part of the logarithm as an SD59x18 number. The operation can't overflow
        // because n is at most 255, `UNIT` is 1e18, and the sign is either 1 or -1.
        int256 resultInt = int256(n) * uUNIT;

        // Calculate $y = x * 2^{-n}$.
        int256 y = xInt >> n;

        // If y is the unit number, the fractional part is zero.
        if (y == uUNIT) {
            return wrap(resultInt * sign);
        }

        // Calculate the fractional part via the iterative approximation.
        // The `delta >>= 1` part is equivalent to `delta /= 2`, but shifting bits is more gas efficient.
        int256 DOUBLE_UNIT = 2e18;
        for (int256 delta = uHALF_UNIT; delta > 0; delta >>= 1) {
            y = (y * y) / uUNIT;

            // Is y^2 >= 2e18 and so in the range [2e18, 4e18)?
            if (y >= DOUBLE_UNIT) {
                // Add the 2^{-m} factor to the logarithm.
                resultInt = resultInt + delta;

                // Halve y, which corresponds to z/2 in the Wikipedia article.
                y >>= 1;
            }
        }
        resultInt *= sign;
        result = wrap(resultInt);
    }
}

/// @notice Multiplies two SD59x18 numbers together, returning a new SD59x18 number.
///
/// @dev Notes:
/// - Refer to the notes in {Common.mulDiv18}.
///
/// Requirements:
/// - Refer to the requirements in {Common.mulDiv18}.
/// - None of the inputs can be `MIN_SD59x18`.
/// - The result must fit in SD59x18.
///
/// @param x The multiplicand as an SD59x18 number.
/// @param y The multiplier as an SD59x18 number.
/// @return result The product as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function mul(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    int256 yInt = y.unwrap();
    if (xInt == uMIN_SD59x18 || yInt == uMIN_SD59x18) {
        revert Errors.PRBMath_SD59x18_Mul_InputTooSmall();
    }

    // Get hold of the absolute values of x and y.
    uint256 xAbs;
    uint256 yAbs;
    unchecked {
        xAbs = xInt < 0 ? uint256(-xInt) : uint256(xInt);
        yAbs = yInt < 0 ? uint256(-yInt) : uint256(yInt);
    }

    // Compute the absolute value (x*y÷UNIT). The resulting value must fit in SD59x18.
    uint256 resultAbs = Common.mulDiv18(xAbs, yAbs);
    if (resultAbs > uint256(uMAX_SD59x18)) {
        revert Errors.PRBMath_SD59x18_Mul_Overflow(x, y);
    }

    // Check if x and y have the same sign using two's complement representation. The left-most bit represents the sign (1 for
    // negative, 0 for positive or zero).
    bool sameSign = (xInt ^ yInt) > -1;

    // If the inputs have the same sign, the result should be positive. Otherwise, it should be negative.
    unchecked {
        result = wrap(sameSign ? int256(resultAbs) : -int256(resultAbs));
    }
}

/// @notice Raises x to the power of y using the following formula:
///
/// $$
/// x^y = 2^{log_2{x} * y}
/// $$
///
/// @dev Notes:
/// - Refer to the notes in {exp2}, {log2}, and {mul}.
/// - Returns `UNIT` for 0^0.
///
/// Requirements:
/// - Refer to the requirements in {exp2}, {log2}, and {mul}.
///
/// @param x The base as an SD59x18 number.
/// @param y Exponent to raise x to, as an SD59x18 number
/// @return result x raised to power y, as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function pow(SD59x18 x, SD59x18 y) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    int256 yInt = y.unwrap();

    // If both x and y are zero, the result is `UNIT`. If just x is zero, the result is always zero.
    if (xInt == 0) {
        return yInt == 0 ? UNIT : ZERO;
    }
    // If x is `UNIT`, the result is always `UNIT`.
    else if (xInt == uUNIT) {
        return UNIT;
    }

    // If y is zero, the result is always `UNIT`.
    if (yInt == 0) {
        return UNIT;
    }
    // If y is `UNIT`, the result is always x.
    else if (yInt == uUNIT) {
        return x;
    }

    // Calculate the result using the formula.
    result = exp2(mul(log2(x), y));
}

/// @notice Raises x (an SD59x18 number) to the power y (an unsigned basic integer) using the well-known
/// algorithm "exponentiation by squaring".
///
/// @dev See https://en.wikipedia.org/wiki/Exponentiation_by_squaring.
///
/// Notes:
/// - Refer to the notes in {Common.mulDiv18}.
/// - Returns `UNIT` for 0^0.
///
/// Requirements:
/// - Refer to the requirements in {abs} and {Common.mulDiv18}.
/// - The result must fit in SD59x18.
///
/// @param x The base as an SD59x18 number.
/// @param y The exponent as a uint256.
/// @return result The result as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function powu(SD59x18 x, uint256 y) pure returns (SD59x18 result) {
    uint256 xAbs = uint256(abs(x).unwrap());

    // Calculate the first iteration of the loop in advance.
    uint256 resultAbs = y & 1 > 0 ? xAbs : uint256(uUNIT);

    // Equivalent to `for(y /= 2; y > 0; y /= 2)`.
    uint256 yAux = y;
    for (yAux >>= 1; yAux > 0; yAux >>= 1) {
        xAbs = Common.mulDiv18(xAbs, xAbs);

        // Equivalent to `y % 2 == 1`.
        if (yAux & 1 > 0) {
            resultAbs = Common.mulDiv18(resultAbs, xAbs);
        }
    }

    // The result must fit in SD59x18.
    if (resultAbs > uint256(uMAX_SD59x18)) {
        revert Errors.PRBMath_SD59x18_Powu_Overflow(x, y);
    }

    unchecked {
        // Is the base negative and the exponent odd? If yes, the result should be negative.
        int256 resultInt = int256(resultAbs);
        bool isNegative = x.unwrap() < 0 && y & 1 == 1;
        if (isNegative) {
            resultInt = -resultInt;
        }
        result = wrap(resultInt);
    }
}

/// @notice Calculates the square root of x using the Babylonian method.
///
/// @dev See https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method.
///
/// Notes:
/// - Only the positive root is returned.
/// - The result is rounded toward zero.
///
/// Requirements:
/// - x cannot be negative, since complex numbers are not supported.
/// - x must be less than `MAX_SD59x18 / UNIT`.
///
/// @param x The SD59x18 number for which to calculate the square root.
/// @return result The result as an SD59x18 number.
/// @custom:smtchecker abstract-function-nondet
function sqrt(SD59x18 x) pure returns (SD59x18 result) {
    int256 xInt = x.unwrap();
    if (xInt < 0) {
        revert Errors.PRBMath_SD59x18_Sqrt_NegativeInput(x);
    }
    if (xInt > uMAX_SD59x18 / uUNIT) {
        revert Errors.PRBMath_SD59x18_Sqrt_Overflow(x);
    }

    unchecked {
        // Multiply x by `UNIT` to account for the factor of `UNIT` picked up when multiplying two SD59x18 numbers.
        // In this case, the two numbers are both the square root.
        uint256 resultUint = Common.sqrt(uint256(xInt * uUNIT));
        result = wrap(int256(resultUint));
    }
}

/// @notice The signed 59.18-decimal fixed-point number representation, which can have up to 59 digits and up to 18
/// decimals. The values of this are bound by the minimum and the maximum values permitted by the underlying Solidity
/// type int256.
type SD59x18 is int256;

/*//////////////////////////////////////////////////////////////////////////
                                    CASTING
//////////////////////////////////////////////////////////////////////////*/

using {
    Casting.intoInt256,
    Casting.intoSD1x18,
    Casting.intoUD2x18,
    Casting.intoUD60x18,
    Casting.intoUint256,
    Casting.intoUint128,
    Casting.intoUint40,
    Casting.unwrap
} for SD59x18 global;

/*//////////////////////////////////////////////////////////////////////////
                            MATHEMATICAL FUNCTIONS
//////////////////////////////////////////////////////////////////////////*/

using {
    Math.abs,
    Math.avg,
    Math.ceil,
    Math.div,
    Math.exp,
    Math.exp2,
    Math.floor,
    Math.frac,
    Math.gm,
    Math.inv,
    Math.log10,
    Math.log2,
    Math.ln,
    Math.mul,
    Math.pow,
    Math.powu,
    Math.sqrt
} for SD59x18 global;

/*//////////////////////////////////////////////////////////////////////////
                                HELPER FUNCTIONS
//////////////////////////////////////////////////////////////////////////*/

using {
    Helpers.add,
    Helpers.and,
    Helpers.eq,
    Helpers.gt,
    Helpers.gte,
    Helpers.isZero,
    Helpers.lshift,
    Helpers.lt,
    Helpers.lte,
    Helpers.mod,
    Helpers.neq,
    Helpers.not,
    Helpers.or,
    Helpers.rshift,
    Helpers.sub,
    Helpers.uncheckedAdd,
    Helpers.uncheckedSub,
    Helpers.uncheckedUnary,
    Helpers.xor
} for SD59x18 global;

/*//////////////////////////////////////////////////////////////////////////
                                    OPERATORS
//////////////////////////////////////////////////////////////////////////*/

// The global "using for" directive makes it possible to use these operators on the SD59x18 type.
using {
    Helpers.add as +,
    Helpers.and2 as &,
    Math.div as /,
    Helpers.eq as ==,
    Helpers.gt as >,
    Helpers.gte as >=,
    Helpers.lt as <,
    Helpers.lte as <=,
    Helpers.mod as %,
    Math.mul as *,
    Helpers.neq as !=,
    Helpers.not as ~,
    Helpers.or as |,
    Helpers.sub as -,
    Helpers.unary as -,
    Helpers.xor as ^
} for SD59x18 global;

/// @notice Casts an SD1x18 number into SD59x18.
/// @dev There is no overflow check because the domain of SD1x18 is a subset of SD59x18.
function intoSD59x18(SD1x18 x) pure returns (SD59x18 result) {
    result = SD59x18.wrap(int256(SD1x18.unwrap(x)));
}

/// @notice Casts an SD1x18 number into UD2x18.
/// - x must be positive.
function intoUD2x18(SD1x18 x) pure returns (UD2x18 result) {
    int64 xInt = SD1x18.unwrap(x);
    if (xInt < 0) {
        revert CastingErrors.PRBMath_SD1x18_ToUD2x18_Underflow(x);
    }
    result = UD2x18.wrap(uint64(xInt));
}

/// @notice Casts an SD1x18 number into UD60x18.
/// @dev Requirements:
/// - x must be positive.
function intoUD60x18(SD1x18 x) pure returns (UD60x18 result) {
    int64 xInt = SD1x18.unwrap(x);
    if (xInt < 0) {
        revert CastingErrors.PRBMath_SD1x18_ToUD60x18_Underflow(x);
    }
    result = UD60x18.wrap(uint64(xInt));
}

/// @notice Casts an SD1x18 number into uint256.
/// @dev Requirements:
/// - x must be positive.
function intoUint256(SD1x18 x) pure returns (uint256 result) {
    int64 xInt = SD1x18.unwrap(x);
    if (xInt < 0) {
        revert CastingErrors.PRBMath_SD1x18_ToUint256_Underflow(x);
    }
    result = uint256(uint64(xInt));
}

/// @notice Casts an SD1x18 number into uint128.
/// @dev Requirements:
/// - x must be positive.
function intoUint128(SD1x18 x) pure returns (uint128 result) {
    int64 xInt = SD1x18.unwrap(x);
    if (xInt < 0) {
        revert CastingErrors.PRBMath_SD1x18_ToUint128_Underflow(x);
    }
    result = uint128(uint64(xInt));
}

/// @notice Casts an SD1x18 number into uint40.
/// @dev Requirements:
/// - x must be positive.
/// - x must be less than or equal to `MAX_UINT40`.
function intoUint40(SD1x18 x) pure returns (uint40 result) {
    int64 xInt = SD1x18.unwrap(x);
    if (xInt < 0) {
        revert CastingErrors.PRBMath_SD1x18_ToUint40_Underflow(x);
    }
    if (xInt > int64(uint64(Common.MAX_UINT40))) {
        revert CastingErrors.PRBMath_SD1x18_ToUint40_Overflow(x);
    }
    result = uint40(uint64(xInt));
}

/// @notice Alias for {wrap}.
function sd1x18(int64 x) pure returns (SD1x18 result) {
    result = SD1x18.wrap(x);
}

/// @notice Unwraps an SD1x18 number into int64.
function unwrap(SD1x18 x) pure returns (int64 result) {
    result = SD1x18.unwrap(x);
}

/// @notice Wraps an int64 number into SD1x18.
function wrap(int64 x) pure returns (SD1x18 result) {
    result = SD1x18.wrap(x);
}

/// @notice The signed 1.18-decimal fixed-point number representation, which can have up to 1 digit and up to 18
/// decimals. The values of this are bound by the minimum and the maximum values permitted by the underlying Solidity
/// type int64. This is useful when end users want to use int64 to save gas, e.g. with tight variable packing in contract
/// storage.
type SD1x18 is int64;

/*//////////////////////////////////////////////////////////////////////////
                                    CASTING
//////////////////////////////////////////////////////////////////////////*/

using {
    Casting.intoSD59x18,
    Casting.intoUD2x18,
    Casting.intoUD60x18,
    Casting.intoUint256,
    Casting.intoUint128,
    Casting.intoUint40,
    Casting.unwrap
} for SD1x18 global;

/// @dev Euler's number as an SD1x18 number.
SD1x18 constant E = SD1x18.wrap(2_718281828459045235);

/// @dev The maximum value an SD1x18 number can have.
int64 constant uMAX_SD1x18 = 9_223372036854775807;
SD1x18 constant MAX_SD1x18 = SD1x18.wrap(uMAX_SD1x18);

/// @dev The maximum value an SD1x18 number can have.
int64 constant uMIN_SD1x18 = -9_223372036854775808;
SD1x18 constant MIN_SD1x18 = SD1x18.wrap(uMIN_SD1x18);

/// @dev PI as an SD1x18 number.
SD1x18 constant PI = SD1x18.wrap(3_141592653589793238);

/// @dev The unit number, which gives the decimal precision of SD1x18.
SD1x18 constant UNIT = SD1x18.wrap(1e18);
int256 constant uUNIT = 1e18;

/// @notice Thrown when trying to cast a uint128 that doesn't fit in SD1x18.
error PRBMath_IntoSD1x18_Overflow(uint128 x);

/// @notice Thrown when trying to cast a uint128 that doesn't fit in UD2x18.
error PRBMath_IntoUD2x18_Overflow(uint128 x);

/// @title PRBMathCastingUint128
/// @notice Casting utilities for uint128.
library PRBMathCastingUint128 {
    /// @notice Casts a uint128 number to SD1x18.
    /// @dev Requirements:
    /// - x must be less than or equal to `MAX_SD1x18`.
    function intoSD1x18(uint128 x) internal pure returns (SD1x18 result) {
        if (x > uint256(int256(uMAX_SD1x18))) {
            revert PRBMath_IntoSD1x18_Overflow(x);
        }
        result = SD1x18.wrap(int64(uint64(x)));
    }

    /// @notice Casts a uint128 number to SD59x18.
    /// @dev There is no overflow check because the domain of uint128 is a subset of SD59x18.
    function intoSD59x18(uint128 x) internal pure returns (SD59x18 result) {
        result = SD59x18.wrap(int256(uint256(x)));
    }

    /// @notice Casts a uint128 number to UD2x18.
    /// @dev Requirements:
    /// - x must be less than or equal to `MAX_SD1x18`.
    function intoUD2x18(uint128 x) internal pure returns (UD2x18 result) {
        if (x > uint64(uMAX_UD2x18)) {
            revert PRBMath_IntoUD2x18_Overflow(x);
        }
        result = UD2x18.wrap(uint64(x));
    }

    /// @notice Casts a uint128 number to UD60x18.
    /// @dev There is no overflow check because the domain of uint128 is a subset of UD60x18.
    function intoUD60x18(uint128 x) internal pure returns (UD60x18 result) {
        result = UD60x18.wrap(uint256(x));
    }
}

/// @notice Thrown when trying to cast a uint256 that doesn't fit in SD1x18.
error PRBMath_IntoSD1x18_Overflow(uint256 x);

/// @notice Thrown when trying to cast a uint256 that doesn't fit in SD59x18.
error PRBMath_IntoSD59x18_Overflow(uint256 x);

/// @notice Thrown when trying to cast a uint256 that doesn't fit in UD2x18.
error PRBMath_IntoUD2x18_Overflow(uint256 x);

/// @title PRBMathCastingUint256
/// @notice Casting utilities for uint256.
library PRBMathCastingUint256 {
    /// @notice Casts a uint256 number to SD1x18.
    /// @dev Requirements:
    /// - x must be less than or equal to `MAX_SD1x18`.
    function intoSD1x18(uint256 x) internal pure returns (SD1x18 result) {
        if (x > uint256(int256(uMAX_SD1x18))) {
            revert PRBMath_IntoSD1x18_Overflow(x);
        }
        result = SD1x18.wrap(int64(int256(x)));
    }

    /// @notice Casts a uint256 number to SD59x18.
    /// @dev Requirements:
    /// - x must be less than or equal to `MAX_SD59x18`.
    function intoSD59x18(uint256 x) internal pure returns (SD59x18 result) {
        if (x > uint256(uMAX_SD59x18)) {
            revert PRBMath_IntoSD59x18_Overflow(x);
        }
        result = SD59x18.wrap(int256(x));
    }

    /// @notice Casts a uint256 number to UD2x18.
    function intoUD2x18(uint256 x) internal pure returns (UD2x18 result) {
        if (x > uint256(uMAX_UD2x18)) {
            revert PRBMath_IntoUD2x18_Overflow(x);
        }
        result = UD2x18.wrap(uint64(x));
    }

    /// @notice Casts a uint256 number to UD60x18.
    function intoUD60x18(uint256 x) internal pure returns (UD60x18 result) {
        result = UD60x18.wrap(x);
    }
}

/// @title PRBMathCastingUint40
/// @notice Casting utilities for uint40.
library PRBMathCastingUint40 {
    /// @notice Casts a uint40 number into SD1x18.
    /// @dev There is no overflow check because the domain of uint40 is a subset of SD1x18.
    function intoSD1x18(uint40 x) internal pure returns (SD1x18 result) {
        result = SD1x18.wrap(int64(uint64(x)));
    }

    /// @notice Casts a uint40 number into SD59x18.
    /// @dev There is no overflow check because the domain of uint40 is a subset of SD59x18.
    function intoSD59x18(uint40 x) internal pure returns (SD59x18 result) {
        result = SD59x18.wrap(int256(uint256(x)));
    }

    /// @notice Casts a uint40 number into UD2x18.
    /// @dev There is no overflow check because the domain of uint40 is a subset of UD2x18.
    function intoUD2x18(uint40 x) internal pure returns (UD2x18 result) {
        result = UD2x18.wrap(uint64(x));
    }

    /// @notice Casts a uint40 number into UD60x18.
    /// @dev There is no overflow check because the domain of uint40 is a subset of UD60x18.
    function intoUD60x18(uint40 x) internal pure returns (UD60x18 result) {
        result = UD60x18.wrap(uint256(x));
    }
}

/*

██████╗ ██████╗ ██████╗ ███╗   ███╗ █████╗ ████████╗██╗  ██╗
██╔══██╗██╔══██╗██╔══██╗████╗ ████║██╔══██╗╚══██╔══╝██║  ██║
██████╔╝██████╔╝██████╔╝██╔████╔██║███████║   ██║   ███████║
██╔═══╝ ██╔══██╗██╔══██╗██║╚██╔╝██║██╔══██║   ██║   ██╔══██║
██║     ██║  ██║██████╔╝██║ ╚═╝ ██║██║  ██║   ██║   ██║  ██║
╚═╝     ╚═╝  ╚═╝╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝

███████╗██████╗  ██╗██╗  ██╗ ██╗ █████╗
██╔════╝██╔══██╗███║╚██╗██╔╝███║██╔══██╗
███████╗██║  ██║╚██║ ╚███╔╝ ╚██║╚█████╔╝
╚════██║██║  ██║ ██║ ██╔██╗  ██║██╔══██╗
███████║██████╔╝ ██║██╔╝ ██╗ ██║╚█████╔╝
╚══════╝╚═════╝  ╚═╝╚═╝  ╚═╝ ╚═╝ ╚════╝

*/

/// @notice Converts a simple integer to SD59x18 by multiplying it by `UNIT`.
///
/// @dev Requirements:
/// - x must be greater than or equal to `MIN_SD59x18 / UNIT`.
/// - x must be less than or equal to `MAX_SD59x18 / UNIT`.
///
/// @param x The basic integer to convert.
/// @param result The same number converted to SD59x18.
function convert(int256 x) pure returns (SD59x18 result) {
    if (x < uMIN_SD59x18 / uUNIT) {
        revert PRBMath_SD59x18_Convert_Underflow(x);
    }
    if (x > uMAX_SD59x18 / uUNIT) {
        revert PRBMath_SD59x18_Convert_Overflow(x);
    }
    unchecked {
        result = SD59x18.wrap(x * uUNIT);
    }
}

/// @notice Converts an SD59x18 number to a simple integer by dividing it by `UNIT`.
/// @dev The result is rounded toward zero.
/// @param x The SD59x18 number to convert.
/// @return result The same number as a simple integer.
function convert(SD59x18 x) pure returns (int256 result) {
    result = SD59x18.unwrap(x) / uUNIT;
}

/*

██████╗ ██████╗ ██████╗ ███╗   ███╗ █████╗ ████████╗██╗  ██╗
██╔══██╗██╔══██╗██╔══██╗████╗ ████║██╔══██╗╚══██╔══╝██║  ██║
██████╔╝██████╔╝██████╔╝██╔████╔██║███████║   ██║   ███████║
██╔═══╝ ██╔══██╗██╔══██╗██║╚██╔╝██║██╔══██║   ██║   ██╔══██║
██║     ██║  ██║██████╔╝██║ ╚═╝ ██║██║  ██║   ██║   ██║  ██║
╚═╝     ╚═╝  ╚═╝╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝

███████╗██████╗ ███████╗ █████╗ ██╗  ██╗ ██╗ █████╗
██╔════╝██╔══██╗██╔════╝██╔══██╗╚██╗██╔╝███║██╔══██╗
███████╗██║  ██║███████╗╚██████║ ╚███╔╝ ╚██║╚█████╔╝
╚════██║██║  ██║╚════██║ ╚═══██║ ██╔██╗  ██║██╔══██╗
███████║██████╔╝███████║ █████╔╝██╔╝ ██╗ ██║╚█████╔╝
╚══════╝╚═════╝ ╚══════╝ ╚════╝ ╚═╝  ╚═╝ ╚═╝ ╚════╝

*/

// This file is here for backward compatibility. It will be removed in V5.

// solhint-disable func-visibility
// SPDX-License-Identifier: MIT

/// @dev Calculates the absolute value of `a`.
function abs(int256 a) pure returns (uint256 result) {
    // The unary operator "-" cannot be applied to "type(int256).min".
    if (a == type(int256).min) {
        return uint256(type(int256).min);
    }

    unchecked {
        result = uint256(a > 0 ? a : -a);
    }
}

/// @dev Checks if the `a` address array contains the `b` address.
function contains(address[] memory a, address b) pure returns (bool result) {
    address item;
    uint256 length = a.length;

    for (uint256 i = 0; i < length;) {
        item = a[i];
        if (item == b) {
            return true;
        }
        unchecked {
            i += 1;
        }
    }

    result = false;
}

/// @dev Checks if the `a` address array contains the `b` address.
function containsAddress(address[] memory a, address b) pure returns (bool result) {
    result = contains(a, b);
}

/// @dev Checks if the `a` bytes32 array contains the `b` bytes32.
function contains(bytes32[] memory a, bytes32 b) pure returns (bool result) {
    bytes32 item;
    uint256 length = a.length;

    for (uint256 i = 0; i < length;) {
        item = a[i];
        if (item == b) {
            return true;
        }
        unchecked {
            i += 1;
        }
    }

    result = false;
}

/// @dev Checks if the `a` bytes32 array contains the `b` bytes32.
function containsBytes32(bytes32[] memory a, bytes32 b) pure returns (bool result) {
    result = contains(a, b);
}

/// @dev Checks if the `a` string array contains the `b` string.
function contains(string[] memory a, string memory b) pure returns (bool result) {
    bytes32 bHash = keccak256(abi.encode(b));
    string memory item;
    uint256 length = a.length;

    for (uint256 i = 0; i < length;) {
        item = a[i];
        if (keccak256(abi.encode(item)) == bHash) {
            return true;
        }
        unchecked {
            i += 1;
        }
    }

    result = false;
}

/// @dev Checks if the `a` string array contains the `b` string.
function containsString(string[] memory a, string memory b) pure returns (bool result) {
    result = contains(a, b);
}

/// @dev Checks if the `a` int256 array contains the `b` int256.
function contains(int256[] memory a, int256 b) pure returns (bool result) {
    int256 item;
    uint256 length = a.length;

    for (uint256 i = 0; i < length;) {
        item = a[i];
        if (item == b) {
            return true;
        }
        unchecked {
            i += 1;
        }
    }

    return false;
}

/// @dev Checks if the `a` int256 array contains the `b` int256.
function containsInt256(int256[] memory a, int256 b) pure returns (bool result) {
    result = contains(a, b);
}

/// @dev Checks if the `a` uint256 array contains the `b` uint256.
function contains(uint256[] memory a, uint256 b) pure returns (bool result) {
    uint256 item;
    uint256 length = a.length;

    for (uint256 i = 0; i < length;) {
        item = a[i];
        if (item == b) {
            return true;
        }
        unchecked {
            i += 1;
        }
    }

    result = false;
}

/// @dev Checks if the `a` uint256 array contains the `b` uint256.
function containsUint256(uint256[] memory a, uint256 b) pure returns (bool result) {
    result = contains(a, b);
}

/// @dev Calculates the absolute delta between `a` and `b`.
function delta(int256 a, int256 b) pure returns (uint256 result) {
    // If XOR of a and b is greater than -1, a and b have the same sign. This works due to two's complement.
    // See https://twitter.com/PaulRBerg/status/1546957951579062272.
    if ((a ^ b) > -1) {
        result = delta(abs(a), abs(b));
    } else {
        unchecked {
            result = abs(a) + abs(b);
        }
    }
}

/// @dev Calculates the absolute delta between `a` and `b`.
function delta(uint256 a, uint256 b) pure returns (uint256 result) {
    unchecked {
        result = a > b ? a - b : b - a;
    }
}

/// @dev Checks if the `a` address array equals the `b` address array.
function eq(address[] memory a, address[] memory b) pure returns (bool result) {
    result = keccak256(abi.encode(a)) == keccak256(abi.encode(b));
}

/// @dev Checks if the `a` address array equals the `b` address array.
function eqAddressArr(address[] memory a, address[] memory b) pure returns (bool result) {
    result = eq(a, b);
}

/// @dev Checks if the `a` bool array equals the `b` bool array.
function eq(bool[] memory a, bool[] memory b) pure returns (bool result) {
    result = keccak256(abi.encode(a)) == keccak256(abi.encode(b));
}

/// @dev Checks if the `a` bool array equals the `b` bool array.
function eqBoolArr(bool[] memory a, bool[] memory b) pure returns (bool result) {
    result = eq(a, b);
}

/// @dev Checks if the `a` bytes equals the `b` bytes.
function eq(bytes memory a, bytes memory b) pure returns (bool result) {
    result = keccak256(a) == keccak256(b);
}

/// @dev Checks if the `a` bytes equals the `b` bytes.
function eqBytes(bytes memory a, bytes memory b) pure returns (bool result) {
    result = eq(a, b);
}

/// @dev Checks if the `a` bytes32 equals the `b` bytes32.
function eq(bytes32 a, bytes32 b) pure returns (bool result) {
    result = keccak256(abi.encode(a)) == keccak256(abi.encode(b));
}

/// @dev Checks if the `a` bytes32 equals the `b` bytes32.
function eqBytes32(bytes32 a, bytes32 b) pure returns (bool result) {
    result = eq(a, b);
}

/// @dev Checks if the `a` bytes32 array equals the `b` bytes32 array.
function eq(bytes32[] memory a, bytes32[] memory b) pure returns (bool result) {
    result = keccak256(abi.encode(a)) == keccak256(abi.encode(b));
}

/// @dev Checks if the `a` bytes32 array equals the `b` bytes32 array.
function eqBytes32Arr(bytes32[] memory a, bytes32[] memory b) pure returns (bool result) {
    result = eq(a, b);
}

/// @dev Checks if the `a` string equals the `b` string.
function eq(string memory a, string memory b) pure returns (bool result) {
    result = keccak256(abi.encode(a)) == keccak256(abi.encode(b));
}

/// @dev Checks if the `a` string equals the `b` string.
function eqString(string memory a, string memory b) pure returns (bool result) {
    result = eq(a, b);
}

/// @dev Checks if the `a` string array equals the `b` string array.
function eq(string[] memory a, string[] memory b) pure returns (bool result) {
    result = keccak256(abi.encode(a)) == keccak256(abi.encode(b));
}

/// @dev Checks if the `a` string array equals the `b` string array.
function eqStringArr(string[] memory a, string[] memory b) pure returns (bool result) {
    result = eq(a, b);
}

/// @dev Checks if the `a` int256 array equals the `b` int256 array.
function eq(int256[] memory a, int256[] memory b) pure returns (bool result) {
    result = keccak256(abi.encode(a)) == keccak256(abi.encode(b));
}

/// @dev Checks if the `a` int256 array equals the `b` int256 array.
function eqInt256Arr(int256[] memory a, int256[] memory b) pure returns (bool result) {
    result = eq(a, b);
}

/// @dev Checks if the `a` uint256 array equals the `b` uint256 array.
function eq(uint256[] memory a, uint256[] memory b) pure returns (bool result) {
    result = keccak256(abi.encode(a)) == keccak256(abi.encode(b));
}

/// @dev Checks if the `a` uint256 array equals the `b` uint256 array.
function eqUint256Arr(uint256[] memory a, uint256[] memory b) pure returns (bool result) {
    result = eq(a, b);
}

/// Cheatcodes are marked as view/pure/none using the following rules:
///
///   1. A call's observable behavior includes its return value, logs, reverts and state writes,
///   2. If it can influence a later call's observable behavior, it's neither view nor pure (it is modifying some
///   state be it the EVM, interpreter, filesystem, etc),
///   3. Otherwise, if it can be influenced by an earlier call, or if reading some state, it's view,
///   4. Otherwise, it's pure.

/// @notice An EVM interpreter written with testing and debugging in mind. This is usually either HEVM or REVM.
/// @dev This interface can be safely used in scripts running on a live network, so for example you don't accidentally
/// change the block timestamp and use a fake timestamp as a value somewhere.
interface VmSafe {
    struct DirEntry {
        string errorMessage;
        string path;
        uint64 depth;
        bool isDir;
        bool isSymlink;
    }

    struct FsMetadata {
        bool isDir;
        bool isSymlink;
        uint256 length;
        bool readOnly;
        uint256 modified;
        uint256 accessed;
        uint256 created;
    }

    struct Log {
        bytes32[] topics;
        bytes data;
        address emitter;
    }

    struct Rpc {
        string key;
        string url;
    }

    /// @dev Gets all accessed reads and write slot from a recording session, for a given address.
    function accesses(address target) external returns (bytes32[] memory readSlots, bytes32[] memory writeSlots);

    /// @dev Gets the address for a given private key.
    function addr(uint256 privateKey) external pure returns (address keyAddr);

    /// @dev If the condition is false, discard this run's fuzz inputs and generate new ones.
    function assume(bool condition) external pure;

    /// @dev Writes a breakpoint to jump to in the debugger.
    function breakpoint(string calldata char) external;

    /// @dev Writes a conditional breakpoint to jump to in the debugger.
    function breakpoint(string calldata char, bool value) external;

    /// @dev Using the address that calls the test contract, has the next call (at this call depth only) create a
    /// transaction that can later be signed and sent onchain.
    function broadcast() external;

    /// @dev Has the next call (at this call depth only) create a transaction with the address provided as
    /// the sender that can later be signed and sent onchain.
    function broadcast(address signer) external;

    /// @dev Has the next call (at this call depth only) create a transaction with the private key provided as
    /// the sender that can later be signed and sent onchain
    function broadcast(uint256 privateKey) external;

    /// @dev Closes file for reading, resetting the offset and allowing to read it from beginning with readLine.
    function closeFile(string calldata path) external;

    /// @dev Creates a new, empty directory at the provided path, which is relative ot the project root.
    /// This cheatcode will revert in the following situations, but is not limited to just these cases:
    ///   - User lacks permissions to modify `path`.
    ///   - A parent of the given path doesn't exist and `recursive` is false.
    ///   - `path` already exists and `recursive` is false.
    function createDir(string calldata path, bool recursive) external;

    /// @dev Derive a private key from a provided mnenomic string (or mnenomic file path) at the derivation
    /// path m/44'/60'/0'/0/{index}
    function deriveKey(string calldata mnemonic, uint32 index) external pure returns (uint256 privateKey);

    /// @dev Derive a private key from a provided mnenomic string (or mnenomic file path) at {derivationPath}{index}
    function deriveKey(
        string calldata mnemonic,
        string calldata derivationPath,
        uint32 index
    )
        external
        pure
        returns (uint256 privateKey);

    /// @dev Reads environment variables
    function envAddress(string calldata name) external view returns (address value);

    function envBool(string calldata name) external view returns (bool value);

    function envBytes(string calldata name) external view returns (bytes memory value);

    function envBytes32(string calldata name) external view returns (bytes32 value);

    function envInt(string calldata name) external view returns (int256 value);

    function envString(string calldata name) external view returns (string memory value);

    function envUint(string calldata name) external view returns (uint256 value);

    /// @dev Reads environment variables as arrays.
    function envAddress(string calldata name, string calldata delim) external view returns (address[] memory values);

    function envBool(string calldata name, string calldata delim) external view returns (bool[] memory values);

    function envBytes(string calldata name, string calldata delim) external view returns (bytes[] memory values);

    function envBytes32(string calldata name, string calldata delim) external view returns (bytes32[] memory values);

    function envInt(string calldata name, string calldata delim) external view returns (int256[] memory values);

    function envString(string calldata name, string calldata delim) external view returns (string[] memory values);

    function envUint(string calldata name, string calldata delim) external view returns (uint256[] memory values);

    /// @dev Reads environment variables with a default value.
    function envOr(string calldata name, bool defaultValue) external returns (bool value);

    function envOr(string calldata name, uint256 defaultValue) external returns (uint256 value);

    function envOr(string calldata name, int256 defaultValue) external returns (int256 value);

    function envOr(string calldata name, address defaultValue) external returns (address value);

    function envOr(string calldata name, bytes32 defaultValue) external returns (bytes32 value);

    function envOr(string calldata name, string calldata defaultValue) external returns (string memory value);

    function envOr(string calldata name, bytes calldata defaultValue) external returns (bytes memory value);

    /// @dev Reads environment variables as arrays with default value.
    function envOr(
        string calldata name,
        string calldata,
        bool[] calldata defaultValue
    )
        external
        returns (bool[] memory value);

    function envOr(
        string calldata name,
        string calldata,
        uint256[] calldata defaultValue
    )
        external
        returns (uint256[] memory value);

    function envOr(
        string calldata name,
        string calldata,
        int256[] calldata defaultValue
    )
        external
        returns (int256[] memory value);

    function envOr(
        string calldata name,
        string calldata,
        address[] calldata defaultValue
    )
        external
        returns (address[] memory value);

    function envOr(
        string calldata name,
        string calldata,
        bytes32[] calldata defaultValue
    )
        external
        returns (bytes32[] memory value);

    function envOr(
        string calldata name,
        string calldata,
        string[] calldata defaultValue
    )
        external
        returns (string[] memory value);

    function envOr(
        string calldata name,
        string calldata,
        bytes[] calldata defaultValue
    )
        external
        returns (bytes[] memory value);

    /// @dev Performs a foreign function call via the terminal.
    function ffi(string[] calldata commandInput) external returns (bytes memory result);

    /// @dev Given a path, query the file system to get information about a file, directory, etc.
    function fsMetadata(string calldata fileOrDir) external returns (FsMetadata memory metadata);

    /// @dev Gets the code from an artifact file. Takes in the relative path to the json file.
    function getCode(string calldata artifactPath) external view returns (bytes memory creationBytecode);

    /// @dev Gets the _deployed_ bytecode from an artifact file. Takes in the relative path to the json file.
    function getDeployedCode(string calldata artifactPath) external view returns (bytes memory runtimeBytecode);

    /// @dev Gets the label for the specified address.
    function getLabel(address account) external returns (string memory label);

    /// @dev Gets the nonce of an account.
    function getNonce(address account) external view returns (uint64 nonce);

    /// @dev Gets all the recorded logs.
    function getRecordedLogs() external returns (Log[] memory logs);

    /// @dev Labels an address in call traces.
    function label(address account, string calldata newLabel) external;

    /// @dev Loads a storage slot from an address.
    function load(address target, bytes32 slot) external view returns (bytes32 data);

    /// @dev Convert values from a string
    function parseBytes(string calldata stringifiedValue) external pure returns (bytes memory parsedValue);

    function parseAddress(string calldata stringifiedValue) external pure returns (address parsedValue);

    function parseBool(string calldata stringifiedValue) external pure returns (bool parsedValue);

    function parseBytes32(string calldata stringifiedValue) external pure returns (bytes32 parsedValue);

    function parseInt(string calldata stringifiedValue) external pure returns (int256 parsedValue);

    /// In case the returned value is a JSON object, it's encoded as a ABI-encoded tuple. As JSON objects
    /// don't have the notion of ordered, but tuples do, they JSON object is encoded with it's fields ordered in
    /// ALPHABETICAL order. That means that in order to successfully decode the tuple, we need to define a tuple that
    /// encodes the fields in the same order, which is alphabetical. In the case of Solidity structs, they are encoded
    /// as tuples, with the attributes in the order in which they are defined.
    /// For example: json = { 'a': 1, 'b': 0xa4tb......3xs}
    /// a: uint256
    /// b: address
    /// To decode that json, we need to define a struct or a tuple as follows:
    /// struct json = { uint256 a; address b; }
    /// If we defined a json struct with the opposite order, meaning placing the address b first, it would try to
    /// decode the tuple in that order, and thus fail.
    /// ----
    /// Given a string of JSON, return it as ABI-encoded
    function parseJson(string calldata json) external pure returns (bytes memory abiEncodedData);

    function parseJson(string calldata json, string calldata key) external pure returns (bytes memory abiEncodedData);

    /// The following parseJson cheatcodes will do type coercion, for the type that they indicate.
    /// For example, parseJsonUint will coerce all values to a uint256. That includes stringified numbers "12"
    /// and hex numbers "0xEF".
    /// Type coercion works ONLY for discrete values or arrays. That means that the key must return a value or array,
    /// not a JSON object.
    function parseJsonAddress(string calldata, string calldata) external returns (address);

    function parseJsonAddressArray(string calldata, string calldata) external returns (address[] memory);

    function parseJsonBytes(string calldata, string calldata) external returns (bytes memory);

    function parseJsonBytesArray(string calldata, string calldata) external returns (bytes[] memory);

    function parseJsonBytes32(string calldata, string calldata) external returns (bytes32);

    function parseJsonBytes32Array(string calldata, string calldata) external returns (bytes32[] memory);

    function parseJsonInt(string calldata, string calldata) external returns (int256);

    function parseJsonIntArray(string calldata, string calldata) external returns (int256[] memory);

    function parseJsonBool(string calldata, string calldata) external returns (bool);

    function parseJsonBoolArray(string calldata, string calldata) external returns (bool[] memory);

    function parseJsonString(string calldata, string calldata) external returns (string memory);

    function parseJsonStringArray(string calldata, string calldata) external returns (string[] memory);

    function parseJsonUint(string calldata, string calldata) external returns (uint256);

    function parseJsonUintArray(string calldata, string calldata) external returns (uint256[] memory);

    function parseUint(string calldata value) external pure returns (uint256 parsedValue);

    /// @dev Pauses gas metering (i.e. gas usage is not counted). No-op if already paused.
    function pauseGasMetering() external;

    /// @dev Get the path of the current project root
    function projectRoot() external view returns (string memory path);

    /// @dev Removes a directory at the provided path, which is relative to the project root.
    /// This cheatcode will revert in the following situations, but is not limited to just these cases:
    ///   - `path` doesn't exist.
    ///   - `path` isn't a directory.
    ///   - User lacks permissions to modify `path`.
    ///   - The directory is not empty and `recursive` is false.
    function removeDir(string calldata path, bool recursive) external;

    ///  @dev Reads the directory at the given path recursively, up to `max_depth`.
    /// `max_depth` defaults to 1, meaning only the direct children of the given directory will be returned.
    /// Follows symbolic links if `follow_links` is true.
    function readDir(string calldata path) external view returns (DirEntry[] memory entries);
    function readDir(string calldata path, uint64 maxDepth) external view returns (DirEntry[] memory entries);
    function readDir(
        string calldata path,
        uint64 maxDepth,
        bool followLinks
    )
        external
        view
        returns (DirEntry[] memory entries);

    /// @dev Reads the entire content of file to string. `path` is relative to the project root.
    function readFile(string calldata path) external view returns (string memory data);

    /// @dev Reads the entire content of file as binary. `path` is relative to the project root.
    function readFileBinary(string calldata path) external view returns (bytes memory data);

    /// @dev Reads a symbolic link, returning the path that the link points to.
    /// This cheatcode will revert in the following situations, but is not limited to just these cases:
    ///   - `path` is not a symbolic link.
    ///   - `path` does not exist.
    function readLink(string calldata linkPath) external view returns (string memory targetPath);

    /// @dev Records all storage reads and writes.
    function record() external;

    /// @dev Record all the transaction logs.
    function recordLogs() external;

    /// @dev Adds a private key to the local Forge wallet and returns the address.
    function rememberKey(uint256 privateKey) external returns (address keyAddr);

    /// @dev Resumes gas metering (i.e. gas usage is counted again). No-op if already on.
    function resumeGasMetering() external;

    //// @dev Returns the RPC url for the given alias.
    function rpcUrl(string calldata rpcAlias) external view returns (string memory json);

    //// @dev Returns all rpc urls and their aliases `[alias, url][]`.
    function rpcUrls() external view returns (string[2][] memory urls);

    /// @dev Returns all rpc urls and their aliases as structs.
    function rpcUrlStructs() external view returns (Rpc[] memory urls);

    /// @dev Serializes a key and value to a JSON object stored in-memory that can be later written to a file.
    /// It returns the stringified version of the specific JSON file up to that moment.
    function serializeBool(
        string calldata objectKey,
        string calldata valueKey,
        bool value
    )
        external
        returns (string memory json);

    function serializeUint(
        string calldata objectKey,
        string calldata valueKey,
        uint256 value
    )
        external
        returns (string memory json);

    function serializeInt(
        string calldata objectKey,
        string calldata valueKey,
        int256 value
    )
        external
        returns (string memory json);

    function serializeAddress(
        string calldata objectKey,
        string calldata valueKey,
        address value
    )
        external
        returns (string memory json);

    function serializeBytes32(
        string calldata objectKey,
        string calldata valueKey,
        bytes32 value
    )
        external
        returns (string memory json);

    function serializeString(
        string calldata objectKey,
        string calldata valueKey,
        string calldata value
    )
        external
        returns (string memory json);

    function serializeBytes(
        string calldata objectKey,
        string calldata valueKey,
        bytes calldata value
    )
        external
        returns (string memory json);

    function serializeBool(
        string calldata objectKey,
        string calldata valueKey,
        bool[] calldata values
    )
        external
        returns (string memory json);

    function serializeUint(
        string calldata objectKey,
        string calldata valueKey,
        uint256[] calldata values
    )
        external
        returns (string memory json);

    function serializeInt(
        string calldata objectKey,
        string calldata valueKey,
        int256[] calldata values
    )
        external
        returns (string memory json);

    function serializeAddress(
        string calldata objectKey,
        string calldata valueKey,
        address[] calldata values
    )
        external
        returns (string memory json);

    function serializeBytes32(
        string calldata objectKey,
        string calldata valueKey,
        bytes32[] calldata values
    )
        external
        returns (string memory json);

    function serializeString(
        string calldata objectKey,
        string calldata valueKey,
        string[] calldata values
    )
        external
        returns (string memory json);

    function serializeBytes(
        string calldata objectKey,
        string calldata valueKey,
        bytes[] calldata values
    )
        external
        returns (string memory json);

    /// @dev Sets environment variables.
    function setEnv(string calldata name, string calldata value) external;

    /// @dev Signs data.
    function sign(uint256 privateKey, bytes32 digest) external pure returns (uint8 v, bytes32 r, bytes32 s);

    /// @dev Using the address that calls the test contract, has all subsequent calls (at this call depth only)
    /// create transactions that can later be signed and sent onchain.
    function startBroadcast() external;

    /// @dev Has all subsequent calls (at this call depth only) create transactions that can later be signed and
    /// sent onchain.
    function startBroadcast(address broadcaster) external;

    /// @dev Has all subsequent calls (at this call depth only) create transactions with the private key provided that
    /// can later be signed and sent onchain
    function startBroadcast(uint256 privateKey) external;

    /// @dev Stops collecting onchain transactions.
    function stopBroadcast() external;

    /// Convert values to a string.
    function toString(address value) external pure returns (string memory stringifiedValue);

    function toString(bool value) external pure returns (string memory stringifiedValue);

    function toString(bytes calldata value) external pure returns (string memory stringifiedValue);

    function toString(bytes32 value) external pure returns (string memory stringifiedValue);

    function toString(int256 value) external pure returns (string memory stringifiedValue);

    function toString(uint256 value) external pure returns (string memory stringifiedValue);

    /// @dev Writes data to file, creating a file if it does not exist, and entirely replacing its contents if it does.
    /// `path` is relative to the project root
    function writeFile(string calldata path, string calldata data) external;

    /// @dev Writes binary data to a file, creating a file if it does not exist, and entirely replacing its contents if
    /// it does. `path` is relative to the project root.
    function writeFileBinary(string calldata path, bytes calldata data) external;

    /// @dev Writes line to file, creating a file if it does not exist. `path` is relative to the project root.
    function writeLine(string calldata path, string calldata data) external;

    /// @dev Write a serialized JSON object to a file. If the file exists, it will be overwritten.
    function writeJson(string calldata json, string calldata path) external;

    /// @dev Write a serialized JSON object to an **existing** JSON file, replacing a value with key = <value_key>
    /// This is useful to replace a specific value of a JSON file, without having to parse the entire thing
    function writeJson(string calldata json, string calldata path, string calldata valueKey) external;
}

/// @notice An EVM interpreter written with testing and debugging in mind. This is usually either HEVM or REVM.
/// @dev This interface contains cheatcodes that are potentially unsafe on a live network.
interface Vm is VmSafe {
    //// @dev Returns the identifier of the currently active fork. Reverts if no fork is currently active.
    function activeFork() external returns (uint256 forkId);

    /// @dev In forking mode, explicitly grant the given address cheatcode access
    function allowCheatcodes(address account) external;

    /// @dev Sets block.chainid.
    function chainId(uint256 newChainId) external;

    /// @dev Clears all mocked calls.
    function clearMockedCalls() external;

    /// @dev Sets block.coinbase
    function coinbase(address newCoinbase) external;

    /// @dev Creates a new fork with the given endpoint and block number and returns the identifier of the fork.
    function createFork(string calldata urlOrAlias, uint256 blockNumber) external returns (uint256);

    /// @dev Creates a new fork with the given endpoint and the _latest_ block and returns the identifier of the fork.
    function createFork(string calldata urlOrAlias) external returns (uint256);

    /// @dev Creates _and_ also selects a new fork with the given endpoint and the latest block and returns the
    /// identifier of the fork.
    function createSelectFork(string calldata urlOrAlias) external returns (uint256);

    /// @dev Creates _and_ also selects a new fork with the given endpoint and block number and returns the identifier
    /// of the fork.
    function createSelectFork(string calldata urlOrAlias, uint256 blockNumber) external returns (uint256);

    /// @dev Creates _and_ also selects new fork with the given endpoint and at the block the given transaction was
    /// mined in, replays all transaction mined in the block before the transaction, returns the identifier of the fork
    function createSelectFork(string calldata urlOrAlias, bytes32 txHash) external returns (uint256 forkId);

    /// @dev Sets an account's balance.
    function deal(address account, uint256 newBalance) external;

    /// @dev Sets block.difficulty
    /// Not available on EVM versions from Paris onwards. Use `prevrandao` instead.
    /// If used on unsupported EVM versions, it will revert.
    function difficulty(uint256 newDifficulty) external;

    /// @dev Sets an address' code.
    function etch(address target, bytes calldata newRuntimeBytecode) external;

    /// @dev Expects a call to an address with the specified calldata.
    /// Calldata can be either a strict or a partial match.
    function expectCall(address callee, bytes calldata data) external;

    /// @dev Expects given number of calls to an address with the specified calldata.
    function expectCall(address callee, bytes calldata data, uint64 count) external;

    /// @dev Expects a call to an address with the specified msg.value and calldata.
    function expectCall(address callee, uint256 msgValue, bytes calldata data) external;

    /// @dev Expects given number of calls to an address with the specified msg.value and calldata
    function expectCall(address callee, uint256 msgValue, bytes calldata data, uint64 count) external;

    /// @dev Expects a call to an address with the specified msg.value, gas, and calldata.
    function expectCall(address callee, uint256 msgValue, uint64 gas, bytes calldata data) external;

    /// @dev Expects given number of calls to an address with the specified msg.value, gas, and calldata.
    function expectCall(address callee, uint256 msgValue, uint64 gas, bytes calldata data, uint64 count) external;

    /// @dev Expects a call to an address with the specified msg.value and calldata, and a *minimum* amount of gas.
    function expectCallMinGas(address callee, uint256 msgValue, uint64 minGas, bytes calldata data) external;

    /// @dev Expect given number of calls to an address with the specified msg.value and calldata, and a *minimum*
    /// amount of gas.
    function expectCallMinGas(
        address callee,
        uint256 msgValue,
        uint64 minGas,
        bytes calldata data,
        uint64 count
    )
        external;

    /// @dev Prepare an expected log with all four checks enabled.
    /// Call this function, then emit an event, then call a function. Internally after the call, we check if
    /// logs were emitted in the expected order with the expected topics and data.
    /// Second form also checks supplied address against emitting contract.
    function expectEmit() external;
    function expectEmit(address emitter) external;

    /// @dev Prepare an expected log.
    /// Call this function, then emit an event, then call a function. Internally after the call, we check if
    /// logs were emitted in the expected order with the expected topics and data (as specified by the booleans).
    /// Second form also checks supplied address against emitting contract.
    function expectEmit(bool checkTopic1, bool checkTopic2, bool checkTopic3, bool checkData) external;
    function expectEmit(
        bool checkTopic1,
        bool checkTopic2,
        bool checkTopic3,
        bool checkData,
        address emitter
    )
        external;

    /// @dev Expects an error on next call.
    function expectRevert(bytes calldata revertData) external;

    function expectRevert(bytes4 revertData) external;

    function expectRevert() external;

    /// @dev Only allows memory writes to offsets [0x00, 0x60) ∪ [min, max) in the current subcontext. If any other
    /// memory is written to, the test will fail. Can be called multiple times to add more ranges to the set.
    function expectSafeMemory(uint64 min, uint64 max) external;
    /// @dev Only allows memory writes to offsets [0x00, 0x60) ∪ [min, max) in the next created subcontext.
    /// If any other memory is written to, the test will fail. Can be called multiple times to add more ranges
    /// to the set.
    function expectSafeMemoryCall(uint64 min, uint64 max) external;

    /// @dev Sets block.basefee.
    function fee(uint256 newBasefee) external;

    /// @dev Returns true if the account is marked as persistent.
    function isPersistent(address account) external view returns (bool persistent);

    /// @dev Marks that the account(s) should use persistent storage across fork swaps in a multifork setup.
    // Meaning, changes made to the state of this account will be kept when switching forks
    function makePersistent(address account) external;

    function makePersistent(address account0, address account1) external;

    function makePersistent(address account0, address account1, address account2) external;

    function makePersistent(address[] calldata accounts) external;

    /// @dev Mocks a call to an address, returning specified data.
    /// Calldata can either be strict or a partial match, e.g. if you only pass a Solidity selector to the expected
    /// calldata, then the entire Solidity function will be mocked.
    function mockCall(address callee, bytes calldata data, bytes calldata returnData) external;

    /// @dev Mocks a call to an address with a specific msg.value, returning specified data.
    /// Calldata match takes precedence over msg.value in case of ambiguity.
    function mockCall(address callee, uint256 msgValue, bytes calldata data, bytes calldata returnData) external;

    /// @dev Reverts a call to an address with specified revert data.
    function mockCallRevert(address callee, bytes calldata data, bytes calldata revertData) external;

    /// @dev Reverts a call to an address with a specific msg.value, with specified revert data.
    function mockCallRevert(
        address callee,
        uint256 msgValue,
        bytes calldata data,
        bytes calldata revertData
    )
        external;

    /// @dev Sets the *next* call's msg.sender to be the input address.
    function prank(address msgSender) external;

    /// @dev Sets the *next* call's msg.sender to be the input address, and the tx.origin to be the second input.
    function prank(address msgSender, address txOrigin) external;

    /// @dev Sets block.prevrandao
    /// Not available on EVM versions before Paris. Use `difficulty` instead.
    /// If used on unsupported EVM versions, it will revert.
    function prevrandao(bytes32 newPrevrandao) external;

    /// @dev Removes a file from the filesystem.
    /// This cheatcode will revert in the following situations, but is not limited to just these cases:
    ///   - `path` points to a directory.
    ///   - The file doesn't exist.
    ///   - The user lacks permissions to remove the file.
    /// `path` is relative to the project root.
    function removeFile(string calldata path) external;

    /// @dev Resets the nonce of an account to 0 for EOAs and 1 for contract accounts.
    function resetNonce(address account) external;

    /// @dev Revert the state of the evm to a previous snapshot.
    /// Takes the snapshot id to revert to.
    /// This deletes the snapshot and all snapshots taken after the given snapshot id.
    function revertTo(uint256 snapshotId) external returns (bool result);

    /// @dev Revokes persistent status from the address, previously added via `makePersistent`
    function revokePersistent(address account) external;

    function revokePersistent(address[] calldata accounts) external;

    /// @dev Sets block.height.
    function roll(uint256 newHeight) external;

    /// @dev Updates the currently active fork to given block number. This is similar to `roll` but for the
    /// currently active fork.
    function rollFork(uint256 forkId) external;

    /// @dev Updates the given fork to given block number.
    function rollFork(uint256 forkId, uint256 blockNumber) external;

    /// @dev Updates the currently active fork to given transaction
    /// this will `rollFork` with the number of the block the transaction was mined in and replays all transaction
    /// mined before it in the block
    function rollFork(bytes32 txHash) external;

    /// @dev Updates the given fork to block number of the given transaction and replays all transaction mined before
    /// it in the block
    function rollFork(uint256 forkId, bytes32 txHash) external;

    /// @dev Takes a fork identifier created by `createFork` and sets the corresponding forked state as active.
    function selectFork(uint256 forkId) external;

    /// @dev Sets the nonce of an account; must be higher than the current nonce of the account.
    function setNonce(address account, uint64 newNonce) external;

    /// @dev Sets the nonce of an account to an arbitrary value.
    function setNonceUnsafe(address account, uint64 newNonce) external;

    /// @dev Snapshot the current state of the EVM.
    /// Returns the id of the snapshot that was created.
    /// To revert a snapshot use `revertTo`.
    function snapshot() external returns (uint256 snapshotId);

    /// @dev Sets all subsequent calls' msg.sender to be the input address until `stopPrank` is called.
    function startPrank(address msgSender) external;

    /// @dev Sets all subsequent calls' msg.sender to be the input address until `stopPrank` is called, and
    /// the tx.origin to be the second input.
    function startPrank(address msgSender, address txOrigin) external;

    /// @dev Resets subsequent calls' msg.sender to be `address(this)`.
    function stopPrank() external;

    /// @dev Stores a value to an address' storage slot.
    function store(address target, bytes32 slot, bytes32 value) external;

    /// @dev Fetches the given transaction from the active fork and executes it on the current state
    function transact(bytes32 txHash) external;

    /// @dev Fetches the given transaction from the given fork and executes it on the current state
    function transact(uint256 forkId, bytes32 txHash) external;

    /// @dev Sets tx.gasprice.
    function txGasPrice(uint256 newGasPrice) external;

    /// @dev Sets block.timestamp.
    function warp(uint256 timestamp) external;
}

/*

██████╗ ██████╗ ██████╗ ████████╗███████╗███████╗████████╗
██╔══██╗██╔══██╗██╔══██╗╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝
██████╔╝██████╔╝██████╔╝   ██║   █████╗  ███████╗   ██║
██╔═══╝ ██╔══██╗██╔══██╗   ██║   ██╔══╝  ╚════██║   ██║
██║     ██║  ██║██████╔╝   ██║   ███████╗███████║   ██║
╚═╝     ╚═╝  ╚═╝╚═════╝    ╚═╝   ╚══════╝╚══════╝   ╚═╝

*/

/// @notice Modern collection of testing assertions and logging utilities.
/// @author Paul Razvan Berg
/// @dev Inspired by DSTest.
contract PRBTest {
    /*//////////////////////////////////////////////////////////////////////////
                                    EVENTS
    //////////////////////////////////////////////////////////////////////////*/
    event Log(string err);
    event LogAddress(address value);
    event LogArray(address[] value);
    event LogArray(bool[] value);
    event LogArray(bytes32[] value);
    event LogArray(int256[] value);
    event LogArray(string[] value);
    event LogArray(uint256[] value);
    event LogBytes(bytes value);
    event LogBytes32(bytes32 value);
    event LogString(string value);
    event LogInt256(int256 value);
    event LogUint256(uint256 value);
    event LogNamedAddress(string key, address value);
    event LogNamedArray(string key, address[] value);
    event LogNamedArray(string key, bool[] value);
    event LogNamedArray(string key, bytes32[] value);
    event LogNamedArray(string key, int256[] value);
    event LogNamedArray(string key, string[] value);
    event LogNamedArray(string key, uint256[] value);
    event LogNamedBytes(string key, bytes value);
    event LogNamedBytes32(string key, bytes32 value);
    event LogNamedInt256(string key, int256 value);
    event LogNamedString(string key, string value);
    event LogNamedUint256(string key, uint256 value);

    /*//////////////////////////////////////////////////////////////////////////
                                    CONSTANTS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev A flag to indicate that this is a test contract.
    function IS_TEST() external pure virtual returns (bool) {
        return true;
    }

    /// @dev The maximum value available in the int256 type.
    int256 internal constant MAX_INT256 = type(int256).max;

    /// @dev The maximum value available in the uint256 type.
    uint256 internal constant MAX_UINT256 = type(uint256).max;

    /// @dev The minimum value available in the int256 type.
    int256 internal constant MIN_INT256 = type(int256).min;

    /*//////////////////////////////////////////////////////////////////////////
                                    CHEATCODES
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev The virtual address of the Foundry VM.
    address internal constant VM_ADDRESS = address(uint160(uint256(keccak256("hevm cheat code"))));

    /// @dev An instance of the Foundry VM, which contains cheatcodes for testing.
    Vm internal constant vm = Vm(VM_ADDRESS);

    /*//////////////////////////////////////////////////////////////////////////
                                FAILURE SYSTEM
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev This instance's failure flag.
    bool private _failed;

    /// @dev Checks whether any test has failed so far. In addition to the local failure flag, we look for the global
    /// flag in the HEVM contract at storage slot "failed", because it is possible to run assertions between different
    /// instances of PRBTest.
    /// See https://github.com/dapphub/dapptools/issues/768.
    function failed() public returns (bool) {
        if (_failed) {
            return true;
        }

        // If there is HEVM context, load the global variable "failed".
        if (VM_ADDRESS.code.length > 0) {
            (, bytes memory returndata) = VM_ADDRESS.call(
                abi.encodePacked(bytes4(keccak256("load(address,bytes32)")), abi.encode(VM_ADDRESS, bytes32("failed")))
            );
            bool globalFailed = abi.decode(returndata, (bool));
            return globalFailed;
        } else {
            return false;
        }
    }

    /// @dev Fails the test by setting the private variable `_failed` to `true` and storing "0x01" at the "failed"
    /// storage slot on the HEVM contract. Doing this instead of reverting makes it possible to test multiple
    /// assertions in one test function while also preserving emitted events.
    function fail() internal {
        // If there is no HEVM context, stop here.
        if (VM_ADDRESS.code.length == 0) {
            return;
        }

        // Store "0x01" at the "failed" storage slot on the HEVM contract.
        (bool status,) = VM_ADDRESS.call(
            abi.encodePacked(
                bytes4(keccak256("store(address,bytes32,bytes32)")),
                abi.encode(VM_ADDRESS, bytes32("failed"), bytes32(uint256(0x01)))
            )
        );

        // Dummy statement to silence the compiler warning.
        status;

        // Set this instance's failed flag to `true`.
        _failed = true;
    }

    /// @dev Logs the error message `err` and fails the test.
    function fail(string memory err) internal {
        emit LogNamedString("Error", err);
        fail();
    }

    /*//////////////////////////////////////////////////////////////////////////
                                BOOLEAN ASSERTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Tests that `condition` evaluates to `true`. If it does not, the test fails.
    function assertTrue(bool condition) internal virtual {
        if (!condition) {
            emit Log("Error: Assertion Failed");
            fail();
        }
    }

    /// @dev Tests that `condition` evaluates to `true`. If it does not, the test fails with the error message `err`.
    function assertTrue(bool condition, string memory err) internal virtual {
        if (!condition) {
            emit LogNamedString("Error", err);
            fail();
        }
    }

    /// @dev Tests that `condition` evaluates to `false`. If it does not, the test fails.
    function assertFalse(bool condition) internal virtual {
        assertTrue(!condition);
    }

    /// @dev Tests that `condition` evaluates to `false`. If it does not, the test fails with the error message `err`.
    function assertFalse(bool condition, string memory err) internal virtual {
        assertTrue(!condition, err);
    }

    /*//////////////////////////////////////////////////////////////////////////
                                EQUALITY ASSERTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    function assertEq(address a, address b) internal virtual {
        if (a != b) {
            emit Log("Error: a == b not satisfied [address]");
            emit LogNamedAddress("   Left", a);
            emit LogNamedAddress("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    function assertEq(address a, address b, string memory err) internal virtual {
        if (a != b) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(address[] memory a, address[] memory b) internal virtual {
        if (keccak256(abi.encode(a)) != keccak256(abi.encode(b))) {
            emit Log("Error: a == b not satisfied [address[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(address[] memory a, address[] memory b, string memory err) internal virtual {
        if (keccak256(abi.encode(a)) != keccak256(abi.encode(b))) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    function assertEq(bool a, bool b) internal virtual {
        if (a != b) {
            emit Log("Error: a == b not satisfied [bool]");
            emit LogNamedString("   Left", a ? "true" : "false");
            emit LogNamedString("  Right", b ? "true" : "false");
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    function assertEq(bool a, bool b, string memory err) internal virtual {
        if (a != b) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(bool[] memory a, bool[] memory b) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit Log("Error: a == b not satisfied [bool[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(bool[] memory a, bool[] memory b, string memory err) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(bytes memory a, bytes memory b) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit Log("Error: a == b not satisfied [bytes]");
            emit LogNamedBytes("   Left", a);
            emit LogNamedBytes("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(bytes memory a, bytes memory b, string memory err) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    function assertEq(bytes32 a, bytes32 b) internal virtual {
        if (a != b) {
            emit Log("Error: a == b not satisfied [bytes32]");
            emit LogNamedBytes32("   Left", a);
            emit LogNamedBytes32("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    function assertEq(bytes32 a, bytes32 b, string memory err) internal virtual {
        if (a != b) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(bytes32[] memory a, bytes32[] memory b) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit Log("Error: a == b not satisfied [bytes32[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(bytes32[] memory a, bytes32[] memory b, string memory err) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal.
    function assertEq(int256 a, int256 b) internal virtual {
        if (a != b) {
            emit Log("Error: a == b not satisfied [int256]");
            emit LogNamedInt256("   Left", a);
            emit LogNamedInt256("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    function assertEq(int256 a, int256 b, string memory err) internal virtual {
        if (a != b) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(int256[] memory a, int256[] memory b) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit Log("Error: a == b not satisfied [int256[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(int256[] memory a, int256[] memory b, string memory err) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    /// Works by comparing the `keccak256` hashes of the strings, which is faster than iterating over the elements.
    function assertEq(string memory a, string memory b) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit Log("Error: a == b not satisfied [string]");
            emit LogNamedString("   Left", a);
            emit LogNamedString("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the strings, which is faster than iterating over the elements.
    function assertEq(string memory a, string memory b, string memory err) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(string[] memory a, string[] memory b) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit Log("Error: a == b not satisfied [string[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(string[] memory a, string[] memory b, string memory err) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    function assertEq(uint256 a, uint256 b) internal virtual {
        if (a != b) {
            emit Log("Error: a == b not satisfied [uint256]");
            emit LogNamedUint256("   Left", a);
            emit LogNamedUint256("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    function assertEq(uint256 a, uint256 b, string memory err) internal virtual {
        if (a != b) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(uint256[] memory a, uint256[] memory b) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit Log("Error: a == b not satisfied [uint256[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertEq(uint256[] memory a, uint256[] memory b, string memory err) internal virtual {
        if (!Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertEq(a, b);
        }
    }

    /*//////////////////////////////////////////////////////////////////////////
                                INEQUALITY ASSERTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    function assertNotEq(address a, address b) internal virtual {
        if (a == b) {
            emit Log("Error: a != b not satisfied [address]");
            emit LogNamedAddress("   Left", a);
            emit LogNamedAddress("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    function assertNotEq(address a, address b, string memory err) internal virtual {
        if (a == b) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(address[] memory a, address[] memory b) internal virtual {
        if (Helpers.eq(a, b)) {
            emit Log("Error: a != b not satisfied [address[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(address[] memory a, address[] memory b, string memory err) internal virtual {
        if (Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    function assertNotEq(bool a, bool b) internal virtual {
        if (a == b) {
            emit Log("Error: a != b not satisfied [bool]");
            emit LogNamedString("   Left", a ? "true" : "false");
            emit LogNamedString("  Right", b ? "true" : "false");
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    function assertNotEq(bool a, bool b, string memory err) internal virtual {
        if (a == b) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(bool[] memory a, bool[] memory b) internal virtual {
        if (Helpers.eq(a, b)) {
            emit Log("Error: a != b not satisfied [bool[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(bool[] memory a, bool[] memory b, string memory err) internal virtual {
        if (Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(bytes memory a, bytes memory b) internal virtual {
        if (Helpers.eq(a, b)) {
            emit Log("Error: a != b not satisfied [bytes]");
            emit LogNamedBytes("   Left", a);
            emit LogNamedBytes("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are equal. If they are not, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(bytes memory a, bytes memory b, string memory err) internal virtual {
        if (Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    function assertNotEq(bytes32 a, bytes32 b) internal virtual {
        if (Helpers.eq(a, b)) {
            emit Log("Error: a != b not satisfied [bytes32]");
            emit LogNamedBytes32("   Left", a);
            emit LogNamedBytes32("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    function assertNotEq(bytes32 a, bytes32 b, string memory err) internal virtual {
        if (Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(bytes32[] memory a, bytes32[] memory b) internal virtual {
        if (Helpers.eq(a, b)) {
            emit Log("Error: a != b not satisfied [bytes32[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(bytes32[] memory a, bytes32[] memory b, string memory err) internal virtual {
        if (Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    function assertNotEq(int256 a, int256 b) internal virtual {
        if (a == b) {
            emit Log("Error: a != b not satisfied [int256]");
            emit LogNamedInt256("   Left", a);
            emit LogNamedInt256("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    function assertNotEq(int256 a, int256 b, string memory err) internal virtual {
        if (a == b) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(int256[] memory a, int256[] memory b) internal virtual {
        if (Helpers.eq(a, b)) {
            emit Log("Error: a != b not satisfied [int256[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(int256[] memory a, int256[] memory b, string memory err) internal virtual {
        if (Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    /// Works by comparing the `keccak256` hashes of the strings, which is faster than iterating over the elements.
    function assertNotEq(string memory a, string memory b) internal virtual {
        if (Helpers.eq(a, b)) {
            emit Log("Error: a != b not satisfied [string]");
            emit LogNamedString("   Left", a);
            emit LogNamedString("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the strings, which is faster than iterating over the elements.
    function assertNotEq(string memory a, string memory b, string memory err) internal virtual {
        if (Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(string[] memory a, string[] memory b) internal virtual {
        if (Helpers.eq(a, b)) {
            emit Log("Error: a != b not satisfied [string[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(string[] memory a, string[] memory b, string memory err) internal virtual {
        if (Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    function assertNotEq(uint256 a, uint256 b) internal virtual {
        if (a == b) {
            emit Log("Error: a != b not satisfied [uint256]");
            emit LogNamedUint256("   Left", a);
            emit LogNamedUint256("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    function assertNotEq(uint256 a, uint256 b, string memory err) internal virtual {
        if (a == b) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(uint256[] memory a, uint256[] memory b) internal virtual {
        if (Helpers.eq(a, b)) {
            emit Log("Error: a != b not satisfied [uint256[]]");
            emit LogNamedArray("   Left", a);
            emit LogNamedArray("  Right", b);
            fail();
        }
    }

    /// @dev Tests that `a` and `b` are not equal. If they are, the test fails with the error message `err`.
    /// Works by comparing the `keccak256` hashes of the arrays, which is faster than iterating over the elements.
    function assertNotEq(uint256[] memory a, uint256[] memory b, string memory err) internal virtual {
        if (Helpers.eq(a, b)) {
            emit LogNamedString("Error", err);
            assertNotEq(a, b);
        }
    }

    /*//////////////////////////////////////////////////////////////////////////
                                APPROXIMATE ASSERTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Tests that the absolute difference between `a and `b` is less than or equal to `maxDelta`.
    /// If it is not, the test fails.
    function assertAlmostEq(int256 a, int256 b, uint256 maxDelta) internal virtual {
        uint256 actualDelta = Helpers.delta(a, b);
        if (actualDelta > maxDelta) {
            emit Log("Error: a ~= b not satisfied [int256]");
            emit LogNamedInt256("      Expected", b);
            emit LogNamedInt256("      Right", a);
            emit LogNamedUint256("     Max Delta", maxDelta);
            emit LogNamedUint256("  Actual Delta", actualDelta);
            fail();
        }
    }

    /// @dev Tests that the absolute difference between `a and `b` is less than or equal to `maxDelta`.
    /// If it is not, the test fails with the error message `err`.
    function assertAlmostEq(int256 a, int256 b, uint256 maxDelta, string memory err) internal virtual {
        if (Helpers.delta(a, b) > maxDelta) {
            emit LogNamedString("Error", err);
            assertAlmostEq(a, b, maxDelta);
        }
    }

    /// @dev Tests that the absolute difference between `a and `b` is less than or equal to `maxDelta`.
    /// If it is not, the test fails.
    function assertAlmostEq(uint256 a, uint256 b, uint256 maxDelta) internal virtual {
        uint256 actualDelta = Helpers.delta(a, b);
        if (actualDelta > maxDelta) {
            emit Log("Error: a ~= b not satisfied [uint256]");
            emit LogNamedUint256("      Expected", b);
            emit LogNamedUint256("      Right", a);
            emit LogNamedUint256("     Max Delta", maxDelta);
            emit LogNamedUint256("  Actual Delta", actualDelta);
            fail();
        }
    }

    /// @dev Tests that the absolute difference between `a and `b` is less than or equal to `maxDelta`.
    /// If it is not, the test fails with the error message `err`.
    function assertAlmostEq(uint256 a, uint256 b, uint256 maxDelta, string memory err) internal virtual {
        if (Helpers.delta(a, b) > maxDelta) {
            emit LogNamedString("Error", err);
            assertAlmostEq(a, b, maxDelta);
        }
    }

    /*//////////////////////////////////////////////////////////////////////////
                            NUMERICAL COMPARISON ASSERTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Tests that `a` is greater than `b`. If it is not, the test fails.
    function assertGt(int256 a, int256 b) internal virtual {
        if (a <= b) {
            emit Log("Error: a > b not satisfied [int256]");
            emit LogNamedInt256("  Value a", a);
            emit LogNamedInt256("  Value b", b);
            fail();
        }
    }

    /// @dev Tests that `a` is greater than `b`. If it is not, the test fails with the error message `err`.
    function assertGt(int256 a, int256 b, string memory err) internal virtual {
        if (a <= b) {
            emit LogNamedString("Error", err);
            assertGt(a, b);
        }
    }

    /// @dev Tests that `a` is greater than `b`. If it is not, the test fails.
    function assertGt(uint256 a, uint256 b) internal virtual {
        if (a <= b) {
            emit Log("Error: a > b not satisfied [uint256]");
            emit LogNamedUint256("  Value a", a);
            emit LogNamedUint256("  Value b", b);
            fail();
        }
    }

    /// @dev Tests that `a` is greater than `b`. If it is not, the test fails with the error message `err`.
    function assertGt(uint256 a, uint256 b, string memory err) internal virtual {
        if (a <= b) {
            emit LogNamedString("Error", err);
            assertGt(a, b);
        }
    }

    /// @dev Tests that `a` is greater than or equal to `b`. If it is not, the test fails.
    function assertGte(int256 a, int256 b) internal virtual {
        if (a < b) {
            emit Log("Error: a >= b not satisfied [int256]");
            emit LogNamedInt256("  Value a", a);
            emit LogNamedInt256("  Value b", b);
            fail();
        }
    }

    /// @dev Tests that `a` is greater than or equal to `b`. If it is not, the test fails with the error message `err`.
    function assertGte(int256 a, int256 b, string memory err) internal virtual {
        if (a < b) {
            emit LogNamedString("Error", err);
            assertGte(a, b);
        }
    }

    /// @dev Tests that `a` is greater than or equal to `b`. If it is not, the test fails.
    function assertGte(uint256 a, uint256 b) internal virtual {
        if (a < b) {
            emit Log("Error: a >= b not satisfied [uint256]");
            emit LogNamedUint256("  Value a", a);
            emit LogNamedUint256("  Value b", b);
            fail();
        }
    }

    /// @dev Tests that `a` is greater than or equal to `b`. If it is not, the test fails with the error message `err`.
    function assertGte(uint256 a, uint256 b, string memory err) internal virtual {
        if (a < b) {
            emit LogNamedString("Error", err);
            assertGte(a, b);
        }
    }

    /// @dev Tests that `a` is lower than `b`. If it is not, the test fails.
    function assertLt(int256 a, int256 b) internal virtual {
        if (a >= b) {
            emit Log("Error: a < b not satisfied [int256]");
            emit LogNamedInt256("  Value a", a);
            emit LogNamedInt256("  Value b", b);
            fail();
        }
    }

    /// @dev Tests that `a` is lower than `b`. If it is not, the test fails with the error message `err`.
    function assertLt(int256 a, int256 b, string memory err) internal virtual {
        if (a >= b) {
            emit LogNamedString("Error", err);
            assertLt(a, b);
        }
    }

    /// @dev Tests that `a` is lower than `b`. If it is not, the test fails.
    function assertLt(uint256 a, uint256 b) internal virtual {
        if (a >= b) {
            emit Log("Error: a < b not satisfied [uint256]");
            emit LogNamedUint256("  Value a", a);
            emit LogNamedUint256("  Value b", b);
            fail();
        }
    }

    /// @dev Tests that `a` is lower than `b`. If it is not, the test fails with the error message `err`.
    function assertLt(uint256 a, uint256 b, string memory err) internal virtual {
        if (a >= b) {
            emit LogNamedString("Error", err);
            assertLt(a, b);
        }
    }

    /// @dev Tests that `a` is lower than or equal to `b`. If it is not, the test fails.
    function assertLte(int256 a, int256 b) internal virtual {
        if (a > b) {
            emit Log("Error: a <= b not satisfied [int256]");
            emit LogNamedInt256("  Value a", a);
            emit LogNamedInt256("  Value b", b);
            fail();
        }
    }

    /// @dev Tests that `a` is lower than or equal to `b`. If it is not, the test fails with the error message `err`.
    function assertLte(int256 a, int256 b, string memory err) internal virtual {
        if (a > b) {
            emit LogNamedString("Error", err);
            assertLte(a, b);
        }
    }

    /// @dev Tests that `a` is lower than or equal to `b`. If it is not, the test fails.
    function assertLte(uint256 a, uint256 b) internal virtual {
        if (a > b) {
            emit Log("Error: a <= b not satisfied [uint256]");
            emit LogNamedUint256("  Value a", a);
            emit LogNamedUint256("  Value b", b);
            fail();
        }
    }

    /// @dev Tests that `a` is lower than or equal to `b`. If it is not, the test fails with the error message `err`.
    function assertLte(uint256 a, uint256 b, string memory err) internal virtual {
        if (a > b) {
            emit LogNamedString("Error", err);
            assertLte(a, b);
        }
    }

    /*//////////////////////////////////////////////////////////////////////////
                                CONTAINMENT ASSERTIONS
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Tests that `a` contains `b`. If it does not, the test fails.
    function assertContains(address[] memory a, address b) internal virtual {
        if (!Helpers.contains(a, b)) {
            emit Log("Error: a does not contain b [address[]]");
            emit LogNamedArray("  Array a", a);
            emit LogNamedAddress("   Item b", b);
            fail();
        }
    }

    /// @dev Tests that `a` contains `b`. If it does not, the test fails with the error message `err`.
    function assertContains(address[] memory a, address b, string memory err) internal virtual {
        if (!Helpers.contains(a, b)) {
            emit LogNamedString("Error", err);
            assertContains(a, b);
        }
    }

    /// @dev Tests that `a` contains `b`. If it does not, the test fails.
    function assertContains(bytes32[] memory a, bytes32 b) internal virtual {
        if (!Helpers.contains(a, b)) {
            emit Log("Error: a does not contain b [bytes32[]]");
            emit LogNamedArray("  Array a", a);
            emit LogNamedBytes32("   Item b", b);
            fail();
        }
    }

    /// @dev Tests that `a` contains `b`. If it does not, the test fails with the error message `err`.
    function assertContains(bytes32[] memory a, bytes32 b, string memory err) internal virtual {
        if (!Helpers.contains(a, b)) {
            emit LogNamedString("Error", err);
            assertContains(a, b);
        }
    }

    /// @dev Tests that `a` contains `b`. If it does not, the test fails.
    function assertContains(int256[] memory a, int256 b) internal virtual {
        if (!Helpers.contains(a, b)) {
            emit Log("Error: a does not contain b [int256[]]");
            emit LogNamedArray("  Array a", a);
            emit LogNamedInt256("   Item b", b);
            fail();
        }
    }

    /// @dev Tests that `a` contains `b`. If it does not, the test fails with the error message `err`.
    function assertContains(int256[] memory a, int256 b, string memory err) internal virtual {
        if (!Helpers.contains(a, b)) {
            emit LogNamedString("Error", err);
            assertContains(a, b);
        }
    }

    /// @dev Tests that `a` contains `b`. If it does not, the test fails.
    function assertContains(string[] memory a, string memory b) internal virtual {
        if (!Helpers.contains(a, b)) {
            emit Log("Error: a does not contain b [string[]]");
            emit LogNamedArray("  Array a", a);
            emit LogNamedString("   Item b", b);
            fail();
        }
    }

    /// @dev Tests that `a` contains `b`. If it does not, the test fails with the error message `err`.
    function assertContains(string[] memory a, string memory b, string memory err) internal virtual {
        if (!Helpers.contains(a, b)) {
            emit LogNamedString("Error", err);
            assertContains(a, b);
        }
    }

    /// @dev Tests that `a` contains `b`. If it does not, the test fails.
    function assertContains(uint256[] memory a, uint256 b) internal virtual {
        if (!Helpers.contains(a, b)) {
            emit Log("Error: a does not contain b [uint256[]]");
            emit LogNamedArray("  Array a", a);
            emit LogNamedUint256("   Item b", b);
            fail();
        }
    }

    /// @dev Tests that `a` contains `b`. If it does not, the test fails with the error message `err`.
    function assertContains(uint256[] memory a, uint256 b, string memory err) internal virtual {
        if (!Helpers.contains(a, b)) {
            emit LogNamedString("Error", err);
            assertContains(a, b);
        }
    }
}

contract PRBMathAssertions is PRBTest {
    /*//////////////////////////////////////////////////////////////////////////
                                       SD1X18
    //////////////////////////////////////////////////////////////////////////*/

    function assertEq(SD1x18 a, SD1x18 b) internal {
        assertEq(SD1x18.unwrap(a), SD1x18.unwrap(b));
    }

    function assertEq(SD1x18 a, SD1x18 b, string memory err) internal {
        assertEq(SD1x18.unwrap(a), SD1x18.unwrap(b), err);
    }

    function assertEq(SD1x18 a, int64 b) internal {
        assertEq(SD1x18.unwrap(a), b);
    }

    function assertEq(SD1x18 a, int64 b, string memory err) internal {
        assertEq(SD1x18.unwrap(a), b, err);
    }

    function assertEq(int64 a, SD1x18 b) internal {
        assertEq(a, SD1x18.unwrap(b));
    }

    function assertEq(int64 a, SD1x18 b, string memory err) internal {
        assertEq(a, SD1x18.unwrap(b), err);
    }

    function assertEq(SD1x18[] memory a, SD1x18[] memory b) internal {
        int256[] memory castedA;
        int256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB);
    }

    function assertEq(SD1x18[] memory a, SD1x18[] memory b, string memory err) internal {
        int256[] memory castedA;
        int256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB, err);
    }

    function assertEq(SD1x18[] memory a, int64[] memory b) internal {
        int256[] memory castedA;
        int256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB);
    }

    function assertEq(SD1x18[] memory a, int64[] memory b, string memory err) internal {
        int256[] memory castedA;
        int256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB, err);
    }

    function assertEq(int64[] memory a, SD1x18[] memory b) internal {
        int256[] memory castedA;
        int256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB);
    }

    function assertEq(int64[] memory a, SD1x18[] memory b, string memory err) internal {
        int256[] memory castedA;
        int256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB, err);
    }

    /*//////////////////////////////////////////////////////////////////////////
                                       SD59X18
    //////////////////////////////////////////////////////////////////////////*/

    function assertEq(SD59x18 a, SD59x18 b) internal {
        assertEq(SD59x18.unwrap(a), SD59x18.unwrap(b));
    }

    function assertEq(SD59x18 a, SD59x18 b, string memory err) internal {
        assertEq(SD59x18.unwrap(a), SD59x18.unwrap(b), err);
    }

    function assertEq(SD59x18 a, int256 b) internal {
        assertEq(SD59x18.unwrap(a), b);
    }

    function assertEq(SD59x18 a, int256 b, string memory err) internal {
        assertEq(SD59x18.unwrap(a), b, err);
    }

    function assertEq(int256 a, SD59x18 b) internal {
        assertEq(a, SD59x18.unwrap(b));
    }

    function assertEq(int256 a, SD59x18 b, string memory err) internal {
        assertEq(a, SD59x18.unwrap(b), err);
    }

    function assertEq(SD59x18[] memory a, SD59x18[] memory b) internal {
        int256[] memory castedA;
        int256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB);
    }

    function assertEq(SD59x18[] memory a, SD59x18[] memory b, string memory err) internal {
        int256[] memory castedA;
        int256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB, err);
    }

    function assertEq(SD59x18[] memory a, int256[] memory b) internal {
        int256[] memory castedA;
        assembly {
            castedA := a
        }
        assertEq(castedA, b);
    }

    function assertEq(SD59x18[] memory a, int256[] memory b, string memory err) internal {
        int256[] memory castedA;
        assembly {
            castedA := a
        }
        assertEq(castedA, b, err);
    }

    function assertEq(int256[] memory a, SD59x18[] memory b) internal {
        int256[] memory castedB;
        assembly {
            castedB := b
        }
        assertEq(a, b);
    }

    function assertEq(int256[] memory a, SD59x18[] memory b, string memory err) internal {
        int256[] memory castedB;
        assembly {
            castedB := b
        }
        assertEq(a, b, err);
    }

    /*//////////////////////////////////////////////////////////////////////////
                                       UD2X18
    //////////////////////////////////////////////////////////////////////////*/

    function assertEq(UD2x18 a, UD2x18 b) internal {
        assertEq(UD2x18.unwrap(a), UD2x18.unwrap(b));
    }

    function assertEq(UD2x18 a, UD2x18 b, string memory err) internal {
        assertEq(UD2x18.unwrap(a), UD2x18.unwrap(b), err);
    }

    function assertEq(UD2x18 a, uint64 b) internal {
        assertEq(UD2x18.unwrap(a), uint256(b));
    }

    function assertEq(UD2x18 a, uint64 b, string memory err) internal {
        assertEq(UD2x18.unwrap(a), uint256(b), err);
    }

    function assertEq(uint64 a, UD2x18 b) internal {
        assertEq(uint256(a), UD2x18.unwrap(b));
    }

    function assertEq(uint64 a, UD2x18 b, string memory err) internal {
        assertEq(uint256(a), UD2x18.unwrap(b), err);
    }

    function assertEq(UD2x18[] memory a, UD2x18[] memory b) internal {
        uint256[] memory castedA;
        uint256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB);
    }

    function assertEq(UD2x18[] memory a, UD2x18[] memory b, string memory err) internal {
        uint256[] memory castedA;
        uint256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB, err);
    }

    function assertEq(UD2x18[] memory a, uint64[] memory b) internal {
        uint256[] memory castedA;
        uint256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB);
    }

    function assertEq(UD2x18[] memory a, uint64[] memory b, string memory err) internal {
        uint256[] memory castedA;
        uint256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB, err);
    }

    function assertEq(uint64[] memory a, UD2x18[] memory b) internal {
        uint256[] memory castedA;
        uint256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB);
    }

    function assertEq(uint64[] memory a, UD2x18[] memory b, string memory err) internal {
        uint256[] memory castedA;
        uint256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB, err);
    }

    /*//////////////////////////////////////////////////////////////////////////
                                       UD60X18
    //////////////////////////////////////////////////////////////////////////*/

    function assertEq(UD60x18 a, UD60x18 b) internal {
        assertEq(UD60x18.unwrap(a), UD60x18.unwrap(b));
    }

    function assertEq(UD60x18 a, UD60x18 b, string memory err) internal {
        assertEq(UD60x18.unwrap(a), UD60x18.unwrap(b), err);
    }

    function assertEq(UD60x18 a, uint256 b) internal {
        assertEq(UD60x18.unwrap(a), b);
    }

    function assertEq(UD60x18 a, uint256 b, string memory err) internal {
        assertEq(UD60x18.unwrap(a), b, err);
    }

    function assertEq(uint256 a, UD60x18 b) internal {
        assertEq(a, UD60x18.unwrap(b));
    }

    function assertEq(uint256 a, UD60x18 b, string memory err) internal {
        assertEq(a, UD60x18.unwrap(b), err);
    }

    function assertEq(UD60x18[] memory a, UD60x18[] memory b) internal {
        uint256[] memory castedA;
        uint256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB);
    }

    function assertEq(UD60x18[] memory a, UD60x18[] memory b, string memory err) internal {
        uint256[] memory castedA;
        uint256[] memory castedB;
        assembly {
            castedA := a
            castedB := b
        }
        assertEq(castedA, castedB, err);
    }

    function assertEq(UD60x18[] memory a, uint256[] memory b) internal {
        uint256[] memory castedA;
        assembly {
            castedA := a
        }
        assertEq(castedA, b);
    }

    function assertEq(UD60x18[] memory a, uint256[] memory b, string memory err) internal {
        uint256[] memory castedA;
        assembly {
            castedA := a
        }
        assertEq(castedA, b, err);
    }

    function assertEq(uint256[] memory a, SD59x18[] memory b) internal {
        uint256[] memory castedB;
        assembly {
            castedB := b
        }
        assertEq(a, b);
    }

    function assertEq(uint256[] memory a, SD59x18[] memory b, string memory err) internal {
        uint256[] memory castedB;
        assembly {
            castedB := b
        }
        assertEq(a, b, err);
    }
}

// This file is here for backward compatibility. It will be removed in V5.

interface IMulticall3 {
    struct Call {
        address target;
        bytes callData;
    }

    struct Call3 {
        address target;
        bool allowFailure;
        bytes callData;
    }

    struct Call3Value {
        address target;
        bool allowFailure;
        uint256 value;
        bytes callData;
    }

    struct Result {
        bool success;
        bytes returnData;
    }

    function aggregate(Call[] calldata calls)
        external
        payable
        returns (uint256 blockNumber, bytes[] memory returnData);

    function aggregate3(Call3[] calldata calls) external payable returns (Result[] memory returnData);

    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData);

    function blockAndAggregate(Call[] calldata calls)
        external
        payable
        returns (uint256 blockNumber, bytes32 blockHash, Result[] memory returnData);

    function getBasefee() external view returns (uint256 basefee);

    function getBlockHash(uint256 blockNumber) external view returns (bytes32 blockHash);

    function getBlockNumber() external view returns (uint256 blockNumber);

    function getChainId() external view returns (uint256 chainid);

    function getCurrentBlockCoinbase() external view returns (address coinbase);

    function getCurrentBlockDifficulty() external view returns (uint256 difficulty);

    function getCurrentBlockGasLimit() external view returns (uint256 gaslimit);

    function getCurrentBlockTimestamp() external view returns (uint256 timestamp);

    function getEthBalance(address addr) external view returns (uint256 balance);

    function getLastBlockHash() external view returns (bytes32 blockHash);

    function tryAggregate(bool requireSuccess, Call[] calldata calls)
        external
        payable
        returns (Result[] memory returnData);

    function tryBlockAndAggregate(bool requireSuccess, Call[] calldata calls)
        external
        payable
        returns (uint256 blockNumber, bytes32 blockHash, Result[] memory returnData);
}

// Cheatcodes are marked as view/pure/none using the following rules:
// 0. A call's observable behaviour includes its return value, logs, reverts and state writes,
// 1. If you can influence a later call's observable behaviour, you're neither `view` nor `pure (you are modifying some state be it the EVM, interpreter, filesystem, etc),
// 2. Otherwise if you can be influenced by an earlier call, or if reading some state, you're `view`,
// 3. Otherwise you're `pure`.

interface VmSafe {
    struct Log {
        bytes32[] topics;
        bytes data;
        address emitter;
    }

    struct Rpc {
        string key;
        string url;
    }

    struct DirEntry {
        string errorMessage;
        string path;
        uint64 depth;
        bool isDir;
        bool isSymlink;
    }

    struct FsMetadata {
        bool isDir;
        bool isSymlink;
        uint256 length;
        bool readOnly;
        uint256 modified;
        uint256 accessed;
        uint256 created;
    }

    // Loads a storage slot from an address
    function load(address target, bytes32 slot) external view returns (bytes32 data);
    // Signs data
    function sign(uint256 privateKey, bytes32 digest) external pure returns (uint8 v, bytes32 r, bytes32 s);
    // Gets the address for a given private key
    function addr(uint256 privateKey) external pure returns (address keyAddr);
    // Gets the nonce of an account
    function getNonce(address account) external view returns (uint64 nonce);
    // Performs a foreign function call via the terminal
    function ffi(string[] calldata commandInput) external returns (bytes memory result);
    // Sets environment variables
    function setEnv(string calldata name, string calldata value) external;
    // Reads environment variables, (name) => (value)
    function envBool(string calldata name) external view returns (bool value);
    function envUint(string calldata name) external view returns (uint256 value);
    function envInt(string calldata name) external view returns (int256 value);
    function envAddress(string calldata name) external view returns (address value);
    function envBytes32(string calldata name) external view returns (bytes32 value);
    function envString(string calldata name) external view returns (string memory value);
    function envBytes(string calldata name) external view returns (bytes memory value);
    // Reads environment variables as arrays
    function envBool(string calldata name, string calldata delim) external view returns (bool[] memory value);
    function envUint(string calldata name, string calldata delim) external view returns (uint256[] memory value);
    function envInt(string calldata name, string calldata delim) external view returns (int256[] memory value);
    function envAddress(string calldata name, string calldata delim) external view returns (address[] memory value);
    function envBytes32(string calldata name, string calldata delim) external view returns (bytes32[] memory value);
    function envString(string calldata name, string calldata delim) external view returns (string[] memory value);
    function envBytes(string calldata name, string calldata delim) external view returns (bytes[] memory value);
    // Read environment variables with default value
    function envOr(string calldata name, bool defaultValue) external returns (bool value);
    function envOr(string calldata name, uint256 defaultValue) external returns (uint256 value);
    function envOr(string calldata name, int256 defaultValue) external returns (int256 value);
    function envOr(string calldata name, address defaultValue) external returns (address value);
    function envOr(string calldata name, bytes32 defaultValue) external returns (bytes32 value);
    function envOr(string calldata name, string calldata defaultValue) external returns (string memory value);
    function envOr(string calldata name, bytes calldata defaultValue) external returns (bytes memory value);
    // Read environment variables as arrays with default value
    function envOr(string calldata name, string calldata delim, bool[] calldata defaultValue)
        external
        returns (bool[] memory value);
    function envOr(string calldata name, string calldata delim, uint256[] calldata defaultValue)
        external
        returns (uint256[] memory value);
    function envOr(string calldata name, string calldata delim, int256[] calldata defaultValue)
        external
        returns (int256[] memory value);
    function envOr(string calldata name, string calldata delim, address[] calldata defaultValue)
        external
        returns (address[] memory value);
    function envOr(string calldata name, string calldata delim, bytes32[] calldata defaultValue)
        external
        returns (bytes32[] memory value);
    function envOr(string calldata name, string calldata delim, string[] calldata defaultValue)
        external
        returns (string[] memory value);
    function envOr(string calldata name, string calldata delim, bytes[] calldata defaultValue)
        external
        returns (bytes[] memory value);
    // Records all storage reads and writes
    function record() external;
    // Gets all accessed reads and write slot from a recording session, for a given address
    function accesses(address target) external returns (bytes32[] memory readSlots, bytes32[] memory writeSlots);
    // Gets the _creation_ bytecode from an artifact file. Takes in the relative path to the json file
    function getCode(string calldata artifactPath) external view returns (bytes memory creationBytecode);
    // Gets the _deployed_ bytecode from an artifact file. Takes in the relative path to the json file
    function getDeployedCode(string calldata artifactPath) external view returns (bytes memory runtimeBytecode);
    // Labels an address in call traces
    function label(address account, string calldata newLabel) external;
    // Gets the label for the specified address
    function getLabel(address account) external returns (string memory label);
    // Using the address that calls the test contract, has the next call (at this call depth only) create a transaction that can later be signed and sent onchain
    function broadcast() external;
    // Has the next call (at this call depth only) create a transaction with the address provided as the sender that can later be signed and sent onchain
    function broadcast(address signer) external;
    // Has the next call (at this call depth only) create a transaction with the private key provided as the sender that can later be signed and sent onchain
    function broadcast(uint256 privateKey) external;
    // Using the address that calls the test contract, has all subsequent calls (at this call depth only) create transactions that can later be signed and sent onchain
    function startBroadcast() external;
    // Has all subsequent calls (at this call depth only) create transactions with the address provided that can later be signed and sent onchain
    function startBroadcast(address signer) external;
    // Has all subsequent calls (at this call depth only) create transactions with the private key provided that can later be signed and sent onchain
    function startBroadcast(uint256 privateKey) external;
    // Stops collecting onchain transactions
    function stopBroadcast() external;

    // Get the path of the current project root.
    function projectRoot() external view returns (string memory path);
    // Reads the entire content of file to string. `path` is relative to the project root.
    function readFile(string calldata path) external view returns (string memory data);
    // Reads the entire content of file as binary. `path` is relative to the project root.
    function readFileBinary(string calldata path) external view returns (bytes memory data);
    // Reads next line of file to string.
    function readLine(string calldata path) external view returns (string memory line);
    // Writes data to file, creating a file if it does not exist, and entirely replacing its contents if it does.
    // `path` is relative to the project root.
    function writeFile(string calldata path, string calldata data) external;
    // Writes binary data to a file, creating a file if it does not exist, and entirely replacing its contents if it does.
    // `path` is relative to the project root.
    function writeFileBinary(string calldata path, bytes calldata data) external;
    // Writes line to file, creating a file if it does not exist.
    // `path` is relative to the project root.
    function writeLine(string calldata path, string calldata data) external;
    // Closes file for reading, resetting the offset and allowing to read it from beginning with readLine.
    // `path` is relative to the project root.
    function closeFile(string calldata path) external;
    // Removes a file from the filesystem.
    // This cheatcode will revert in the following situations, but is not limited to just these cases:
    // - `path` points to a directory.
    // - The file doesn't exist.
    // - The user lacks permissions to remove the file.
    // `path` is relative to the project root.
    function removeFile(string calldata path) external;
    // Creates a new, empty directory at the provided path.
    // This cheatcode will revert in the following situations, but is not limited to just these cases:
    // - User lacks permissions to modify `path`.
    // - A parent of the given path doesn't exist and `recursive` is false.
    // - `path` already exists and `recursive` is false.
    // `path` is relative to the project root.
    function createDir(string calldata path, bool recursive) external;
    // Removes a directory at the provided path.
    // This cheatcode will revert in the following situations, but is not limited to just these cases:
    // - `path` doesn't exist.
    // - `path` isn't a directory.
    // - User lacks permissions to modify `path`.
    // - The directory is not empty and `recursive` is false.
    // `path` is relative to the project root.
    function removeDir(string calldata path, bool recursive) external;
    // Reads the directory at the given path recursively, up to `max_depth`.
    // `max_depth` defaults to 1, meaning only the direct children of the given directory will be returned.
    // Follows symbolic links if `follow_links` is true.
    function readDir(string calldata path) external view returns (DirEntry[] memory entries);
    function readDir(string calldata path, uint64 maxDepth) external view returns (DirEntry[] memory entries);
    function readDir(string calldata path, uint64 maxDepth, bool followLinks)
        external
        view
        returns (DirEntry[] memory entries);
    // Reads a symbolic link, returning the path that the link points to.
    // This cheatcode will revert in the following situations, but is not limited to just these cases:
    // - `path` is not a symbolic link.
    // - `path` does not exist.
    function readLink(string calldata linkPath) external view returns (string memory targetPath);
    // Given a path, query the file system to get information about a file, directory, etc.
    function fsMetadata(string calldata path) external view returns (FsMetadata memory metadata);

    // Convert values to a string
    function toString(address value) external pure returns (string memory stringifiedValue);
    function toString(bytes calldata value) external pure returns (string memory stringifiedValue);
    function toString(bytes32 value) external pure returns (string memory stringifiedValue);
    function toString(bool value) external pure returns (string memory stringifiedValue);
    function toString(uint256 value) external pure returns (string memory stringifiedValue);
    function toString(int256 value) external pure returns (string memory stringifiedValue);
    // Convert values from a string
    function parseBytes(string calldata stringifiedValue) external pure returns (bytes memory parsedValue);
    function parseAddress(string calldata stringifiedValue) external pure returns (address parsedValue);
    function parseUint(string calldata stringifiedValue) external pure returns (uint256 parsedValue);
    function parseInt(string calldata stringifiedValue) external pure returns (int256 parsedValue);
    function parseBytes32(string calldata stringifiedValue) external pure returns (bytes32 parsedValue);
    function parseBool(string calldata stringifiedValue) external pure returns (bool parsedValue);
    // Record all the transaction logs
    function recordLogs() external;
    // Gets all the recorded logs
    function getRecordedLogs() external returns (Log[] memory logs);
    // Derive a private key from a provided mnenomic string (or mnenomic file path) at the derivation path m/44'/60'/0'/0/{index}
    function deriveKey(string calldata mnemonic, uint32 index) external pure returns (uint256 privateKey);
    // Derive a private key from a provided mnenomic string (or mnenomic file path) at {derivationPath}{index}
    function deriveKey(string calldata mnemonic, string calldata derivationPath, uint32 index)
        external
        pure
        returns (uint256 privateKey);
    // Adds a private key to the local forge wallet and returns the address
    function rememberKey(uint256 privateKey) external returns (address keyAddr);
    //
    // parseJson
    //
    // ----
    // In case the returned value is a JSON object, it's encoded as a ABI-encoded tuple. As JSON objects
    // don't have the notion of ordered, but tuples do, they JSON object is encoded with it's fields ordered in
    // ALPHABETICAL order. That means that in order to successfully decode the tuple, we need to define a tuple that
    // encodes the fields in the same order, which is alphabetical. In the case of Solidity structs, they are encoded
    // as tuples, with the attributes in the order in which they are defined.
    // For example: json = { 'a': 1, 'b': 0xa4tb......3xs}
    // a: uint256
    // b: address
    // To decode that json, we need to define a struct or a tuple as follows:
    // struct json = { uint256 a; address b; }
    // If we defined a json struct with the opposite order, meaning placing the address b first, it would try to
    // decode the tuple in that order, and thus fail.
    // ----
    // Given a string of JSON, return it as ABI-encoded
    function parseJson(string calldata json, string calldata key) external pure returns (bytes memory abiEncodedData);
    function parseJson(string calldata json) external pure returns (bytes memory abiEncodedData);

    // The following parseJson cheatcodes will do type coercion, for the type that they indicate.
    // For example, parseJsonUint will coerce all values to a uint256. That includes stringified numbers '12'
    // and hex numbers '0xEF'.
    // Type coercion works ONLY for discrete values or arrays. That means that the key must return a value or array, not
    // a JSON object.
    function parseJsonUint(string calldata, string calldata) external returns (uint256);
    function parseJsonUintArray(string calldata, string calldata) external returns (uint256[] memory);
    function parseJsonInt(string calldata, string calldata) external returns (int256);
    function parseJsonIntArray(string calldata, string calldata) external returns (int256[] memory);
    function parseJsonBool(string calldata, string calldata) external returns (bool);
    function parseJsonBoolArray(string calldata, string calldata) external returns (bool[] memory);
    function parseJsonAddress(string calldata, string calldata) external returns (address);
    function parseJsonAddressArray(string calldata, string calldata) external returns (address[] memory);
    function parseJsonString(string calldata, string calldata) external returns (string memory);
    function parseJsonStringArray(string calldata, string calldata) external returns (string[] memory);
    function parseJsonBytes(string calldata, string calldata) external returns (bytes memory);
    function parseJsonBytesArray(string calldata, string calldata) external returns (bytes[] memory);
    function parseJsonBytes32(string calldata, string calldata) external returns (bytes32);
    function parseJsonBytes32Array(string calldata, string calldata) external returns (bytes32[] memory);

    // Serialize a key and value to a JSON object stored in-memory that can be later written to a file
    // It returns the stringified version of the specific JSON file up to that moment.
    function serializeBool(string calldata objectKey, string calldata valueKey, bool value)
        external
        returns (string memory json);
    function serializeUint(string calldata objectKey, string calldata valueKey, uint256 value)
        external
        returns (string memory json);
    function serializeInt(string calldata objectKey, string calldata valueKey, int256 value)
        external
        returns (string memory json);
    function serializeAddress(string calldata objectKey, string calldata valueKey, address value)
        external
        returns (string memory json);
    function serializeBytes32(string calldata objectKey, string calldata valueKey, bytes32 value)
        external
        returns (string memory json);
    function serializeString(string calldata objectKey, string calldata valueKey, string calldata value)
        external
        returns (string memory json);
    function serializeBytes(string calldata objectKey, string calldata valueKey, bytes calldata value)
        external
        returns (string memory json);

    function serializeBool(string calldata objectKey, string calldata valueKey, bool[] calldata values)
        external
        returns (string memory json);
    function serializeUint(string calldata objectKey, string calldata valueKey, uint256[] calldata values)
        external
        returns (string memory json);
    function serializeInt(string calldata objectKey, string calldata valueKey, int256[] calldata values)
        external
        returns (string memory json);
    function serializeAddress(string calldata objectKey, string calldata valueKey, address[] calldata values)
        external
        returns (string memory json);
    function serializeBytes32(string calldata objectKey, string calldata valueKey, bytes32[] calldata values)
        external
        returns (string memory json);
    function serializeString(string calldata objectKey, string calldata valueKey, string[] calldata values)
        external
        returns (string memory json);
    function serializeBytes(string calldata objectKey, string calldata valueKey, bytes[] calldata values)
        external
        returns (string memory json);

    //
    // writeJson
    //
    // ----
    // Write a serialized JSON object to a file. If the file exists, it will be overwritten.
    // Let's assume we want to write the following JSON to a file:
    //
    // { "boolean": true, "number": 342, "object": { "title": "finally json serialization" } }
    //
    // ```
    //  string memory json1 = "some key";
    //  vm.serializeBool(json1, "boolean", true);
    //  vm.serializeBool(json1, "number", uint256(342));
    //  json2 = "some other key";
    //  string memory output = vm.serializeString(json2, "title", "finally json serialization");
    //  string memory finalJson = vm.serialize(json1, "object", output);
    //  vm.writeJson(finalJson, "./output/example.json");
    // ```
    // The critical insight is that every invocation of serialization will return the stringified version of the JSON
    // up to that point. That means we can construct arbitrary JSON objects and then use the return stringified version
    // to serialize them as values to another JSON object.
    //
    // json1 and json2 are simply keys used by the backend to keep track of the objects. So vm.serializeJson(json1,..)
    // will find the object in-memory that is keyed by "some key".
    function writeJson(string calldata json, string calldata path) external;
    // Write a serialized JSON object to an **existing** JSON file, replacing a value with key = <value_key>
    // This is useful to replace a specific value of a JSON file, without having to parse the entire thing
    function writeJson(string calldata json, string calldata path, string calldata valueKey) external;
    // Returns the RPC url for the given alias
    function rpcUrl(string calldata rpcAlias) external view returns (string memory json);
    // Returns all rpc urls and their aliases `[alias, url][]`
    function rpcUrls() external view returns (string[2][] memory urls);
    // Returns all rpc urls and their aliases as structs.
    function rpcUrlStructs() external view returns (Rpc[] memory urls);
    // If the condition is false, discard this run's fuzz inputs and generate new ones.
    function assume(bool condition) external pure;
    // Pauses gas metering (i.e. gas usage is not counted). Noop if already paused.
    function pauseGasMetering() external;
    // Resumes gas metering (i.e. gas usage is counted again). Noop if already on.
    function resumeGasMetering() external;
    // Writes a breakpoint to jump to in the debugger
    function breakpoint(string calldata char) external;
    // Writes a conditional breakpoint to jump to in the debugger
    function breakpoint(string calldata char, bool value) external;
}

interface Vm is VmSafe {
    // Sets block.timestamp
    function warp(uint256 newTimestamp) external;
    // Sets block.height
    function roll(uint256 newHeight) external;
    // Sets block.basefee
    function fee(uint256 newBasefee) external;
    // Sets block.difficulty
    // Not available on EVM versions from Paris onwards. Use `prevrandao` instead.
    // If used on unsupported EVM versions it will revert.
    function difficulty(uint256 newDifficulty) external;
    // Sets block.prevrandao
    // Not available on EVM versions before Paris. Use `difficulty` instead.
    // If used on unsupported EVM versions it will revert.
    function prevrandao(bytes32 newPrevrandao) external;
    // Sets block.chainid
    function chainId(uint256 newChainId) external;
    // Sets tx.gasprice
    function txGasPrice(uint256 newGasPrice) external;
    // Stores a value to an address' storage slot.
    function store(address target, bytes32 slot, bytes32 value) external;
    // Sets the nonce of an account; must be higher than the current nonce of the account
    function setNonce(address account, uint64 newNonce) external;
    // Sets the nonce of an account to an arbitrary value
    function setNonceUnsafe(address account, uint64 newNonce) external;
    // Resets the nonce of an account to 0 for EOAs and 1 for contract accounts
    function resetNonce(address account) external;
    // Sets the *next* call's msg.sender to be the input address
    function prank(address msgSender) external;
    // Sets all subsequent calls' msg.sender to be the input address until `stopPrank` is called
    function startPrank(address msgSender) external;
    // Sets the *next* call's msg.sender to be the input address, and the tx.origin to be the second input
    function prank(address msgSender, address txOrigin) external;
    // Sets all subsequent calls' msg.sender to be the input address until `stopPrank` is called, and the tx.origin to be the second input
    function startPrank(address msgSender, address txOrigin) external;
    // Resets subsequent calls' msg.sender to be `address(this)`
    function stopPrank() external;
    // Sets an address' balance
    function deal(address account, uint256 newBalance) external;
    // Sets an address' code
    function etch(address target, bytes calldata newRuntimeBytecode) external;
    // Expects an error on next call
    function expectRevert(bytes calldata revertData) external;
    function expectRevert(bytes4 revertData) external;
    function expectRevert() external;

    // Prepare an expected log with all four checks enabled.
    // Call this function, then emit an event, then call a function. Internally after the call, we check if
    // logs were emitted in the expected order with the expected topics and data.
    // Second form also checks supplied address against emitting contract.
    function expectEmit() external;
    function expectEmit(address emitter) external;

    // Prepare an expected log with (bool checkTopic1, bool checkTopic2, bool checkTopic3, bool checkData).
    // Call this function, then emit an event, then call a function. Internally after the call, we check if
    // logs were emitted in the expected order with the expected topics and data (as specified by the booleans).
    // Second form also checks supplied address against emitting contract.
    function expectEmit(bool checkTopic1, bool checkTopic2, bool checkTopic3, bool checkData) external;
    function expectEmit(bool checkTopic1, bool checkTopic2, bool checkTopic3, bool checkData, address emitter)
        external;

    // Mocks a call to an address, returning specified data.
    // Calldata can either be strict or a partial match, e.g. if you only
    // pass a Solidity selector to the expected calldata, then the entire Solidity
    // function will be mocked.
    function mockCall(address callee, bytes calldata data, bytes calldata returnData) external;
    // Mocks a call to an address with a specific msg.value, returning specified data.
    // Calldata match takes precedence over msg.value in case of ambiguity.
    function mockCall(address callee, uint256 msgValue, bytes calldata data, bytes calldata returnData) external;
    // Reverts a call to an address with specified revert data.
    function mockCallRevert(address callee, bytes calldata data, bytes calldata revertData) external;
    // Reverts a call to an address with a specific msg.value, with specified revert data.
    function mockCallRevert(address callee, uint256 msgValue, bytes calldata data, bytes calldata revertData)
        external;
    // Clears all mocked calls
    function clearMockedCalls() external;
    // Expects a call to an address with the specified calldata.
    // Calldata can either be a strict or a partial match
    function expectCall(address callee, bytes calldata data) external;
    // Expects given number of calls to an address with the specified calldata.
    function expectCall(address callee, bytes calldata data, uint64 count) external;
    // Expects a call to an address with the specified msg.value and calldata
    function expectCall(address callee, uint256 msgValue, bytes calldata data) external;
    // Expects given number of calls to an address with the specified msg.value and calldata
    function expectCall(address callee, uint256 msgValue, bytes calldata data, uint64 count) external;
    // Expect a call to an address with the specified msg.value, gas, and calldata.
    function expectCall(address callee, uint256 msgValue, uint64 gas, bytes calldata data) external;
    // Expects given number of calls to an address with the specified msg.value, gas, and calldata.
    function expectCall(address callee, uint256 msgValue, uint64 gas, bytes calldata data, uint64 count) external;
    // Expect a call to an address with the specified msg.value and calldata, and a *minimum* amount of gas.
    function expectCallMinGas(address callee, uint256 msgValue, uint64 minGas, bytes calldata data) external;
    // Expect given number of calls to an address with the specified msg.value and calldata, and a *minimum* amount of gas.
    function expectCallMinGas(address callee, uint256 msgValue, uint64 minGas, bytes calldata data, uint64 count)
        external;
    // Only allows memory writes to offsets [0x00, 0x60) ∪ [min, max) in the current subcontext. If any other
    // memory is written to, the test will fail. Can be called multiple times to add more ranges to the set.
    function expectSafeMemory(uint64 min, uint64 max) external;
    // Only allows memory writes to offsets [0x00, 0x60) ∪ [min, max) in the next created subcontext.
    // If any other memory is written to, the test will fail. Can be called multiple times to add more ranges
    // to the set.
    function expectSafeMemoryCall(uint64 min, uint64 max) external;
    // Sets block.coinbase
    function coinbase(address newCoinbase) external;
    // Snapshot the current state of the evm.
    // Returns the id of the snapshot that was created.
    // To revert a snapshot use `revertTo`
    function snapshot() external returns (uint256 snapshotId);
    // Revert the state of the EVM to a previous snapshot
    // Takes the snapshot id to revert to.
    // This deletes the snapshot and all snapshots taken after the given snapshot id.
    function revertTo(uint256 snapshotId) external returns (bool success);
    // Creates a new fork with the given endpoint and block and returns the identifier of the fork
    function createFork(string calldata urlOrAlias, uint256 blockNumber) external returns (uint256 forkId);
    // Creates a new fork with the given endpoint and the _latest_ block and returns the identifier of the fork
    function createFork(string calldata urlOrAlias) external returns (uint256 forkId);
    // Creates a new fork with the given endpoint and at the block the given transaction was mined in, replays all transaction mined in the block before the transaction,
    // and returns the identifier of the fork
    function createFork(string calldata urlOrAlias, bytes32 txHash) external returns (uint256 forkId);
    // Creates _and_ also selects a new fork with the given endpoint and block and returns the identifier of the fork
    function createSelectFork(string calldata urlOrAlias, uint256 blockNumber) external returns (uint256 forkId);
    // Creates _and_ also selects new fork with the given endpoint and at the block the given transaction was mined in, replays all transaction mined in the block before
    // the transaction, returns the identifier of the fork
    function createSelectFork(string calldata urlOrAlias, bytes32 txHash) external returns (uint256 forkId);
    // Creates _and_ also selects a new fork with the given endpoint and the latest block and returns the identifier of the fork
    function createSelectFork(string calldata urlOrAlias) external returns (uint256 forkId);
    // Takes a fork identifier created by `createFork` and sets the corresponding forked state as active.
    function selectFork(uint256 forkId) external;
    /// Returns the identifier of the currently active fork. Reverts if no fork is currently active.
    function activeFork() external view returns (uint256 forkId);
    // Updates the currently active fork to given block number
    // This is similar to `roll` but for the currently active fork
    function rollFork(uint256 blockNumber) external;
    // Updates the currently active fork to given transaction
    // this will `rollFork` with the number of the block the transaction was mined in and replays all transaction mined before it in the block
    function rollFork(bytes32 txHash) external;
    // Updates the given fork to given block number
    function rollFork(uint256 forkId, uint256 blockNumber) external;
    // Updates the given fork to block number of the given transaction and replays all transaction mined before it in the block
    function rollFork(uint256 forkId, bytes32 txHash) external;
    // Marks that the account(s) should use persistent storage across fork swaps in a multifork setup
    // Meaning, changes made to the state of this account will be kept when switching forks
    function makePersistent(address account) external;
    function makePersistent(address account0, address account1) external;
    function makePersistent(address account0, address account1, address account2) external;
    function makePersistent(address[] calldata accounts) external;
    // Revokes persistent status from the address, previously added via `makePersistent`
    function revokePersistent(address account) external;
    function revokePersistent(address[] calldata accounts) external;
    // Returns true if the account is marked as persistent
    function isPersistent(address account) external view returns (bool persistent);
    // In forking mode, explicitly grant the given address cheatcode access
    function allowCheatcodes(address account) external;
    // Fetches the given transaction from the active fork and executes it on the current state
    function transact(bytes32 txHash) external;
    // Fetches the given transaction from the given fork and executes it on the current state
    function transact(uint256 forkId, bytes32 txHash) external;
}

abstract contract StdUtils {
    /*//////////////////////////////////////////////////////////////////////////
                                     CONSTANTS
    //////////////////////////////////////////////////////////////////////////*/

    IMulticall3 private constant multicall = IMulticall3(0xcA11bde05977b3631167028862bE2a173976CA11);
    VmSafe private constant vm = VmSafe(address(uint160(uint256(keccak256("hevm cheat code")))));
    address private constant CONSOLE2_ADDRESS = 0x000000000000000000636F6e736F6c652e6c6f67;
    uint256 private constant INT256_MIN_ABS =
        57896044618658097711785492504343953926634992332820282019728792003956564819968;
    uint256 private constant SECP256K1_ORDER =
        115792089237316195423570985008687907852837564279074904382605163141518161494337;
    uint256 private constant UINT256_MAX =
        115792089237316195423570985008687907853269984665640564039457584007913129639935;

    // Used by default when deploying with create2, https://github.com/Arachnid/deterministic-deployment-proxy.
    address private constant CREATE2_FACTORY = 0x4e59b44847b379578588920cA78FbF26c0B4956C;

    /*//////////////////////////////////////////////////////////////////////////
                                 INTERNAL FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    function _bound(uint256 x, uint256 min, uint256 max) internal pure virtual returns (uint256 result) {
        require(min <= max, "StdUtils bound(uint256,uint256,uint256): Max is less than min.");
        // If x is between min and max, return x directly. This is to ensure that dictionary values
        // do not get shifted if the min is nonzero. More info: https://github.com/foundry-rs/forge-std/issues/188
        if (x >= min && x <= max) return x;

        uint256 size = max - min + 1;

        // If the value is 0, 1, 2, 3, wrap that to min, min+1, min+2, min+3. Similarly for the UINT256_MAX side.
        // This helps ensure coverage of the min/max values.
        if (x <= 3 && size > x) return min + x;
        if (x >= UINT256_MAX - 3 && size > UINT256_MAX - x) return max - (UINT256_MAX - x);

        // Otherwise, wrap x into the range [min, max], i.e. the range is inclusive.
        if (x > max) {
            uint256 diff = x - max;
            uint256 rem = diff % size;
            if (rem == 0) return max;
            result = min + rem - 1;
        } else if (x < min) {
            uint256 diff = min - x;
            uint256 rem = diff % size;
            if (rem == 0) return min;
            result = max - rem + 1;
        }
    }

    function bound(uint256 x, uint256 min, uint256 max) internal view virtual returns (uint256 result) {
        result = _bound(x, min, max);
        console2_log("Bound Result", result);
    }

    function _bound(int256 x, int256 min, int256 max) internal pure virtual returns (int256 result) {
        require(min <= max, "StdUtils bound(int256,int256,int256): Max is less than min.");

        // Shifting all int256 values to uint256 to use _bound function. The range of two types are:
        // int256 : -(2**255) ~ (2**255 - 1)
        // uint256:     0     ~ (2**256 - 1)
        // So, add 2**255, INT256_MIN_ABS to the integer values.
        //
        // If the given integer value is -2**255, we cannot use `-uint256(-x)` because of the overflow.
        // So, use `~uint256(x) + 1` instead.
        uint256 _x = x < 0 ? (INT256_MIN_ABS - ~uint256(x) - 1) : (uint256(x) + INT256_MIN_ABS);
        uint256 _min = min < 0 ? (INT256_MIN_ABS - ~uint256(min) - 1) : (uint256(min) + INT256_MIN_ABS);
        uint256 _max = max < 0 ? (INT256_MIN_ABS - ~uint256(max) - 1) : (uint256(max) + INT256_MIN_ABS);

        uint256 y = _bound(_x, _min, _max);

        // To move it back to int256 value, subtract INT256_MIN_ABS at here.
        result = y < INT256_MIN_ABS ? int256(~(INT256_MIN_ABS - y) + 1) : int256(y - INT256_MIN_ABS);
    }

    function bound(int256 x, int256 min, int256 max) internal view virtual returns (int256 result) {
        result = _bound(x, min, max);
        console2_log("Bound result", vm.toString(result));
    }

    function boundPrivateKey(uint256 privateKey) internal view virtual returns (uint256 result) {
        result = _bound(privateKey, 1, SECP256K1_ORDER - 1);
    }

    function bytesToUint(bytes memory b) internal pure virtual returns (uint256) {
        require(b.length <= 32, "StdUtils bytesToUint(bytes): Bytes length exceeds 32.");
        return abi.decode(abi.encodePacked(new bytes(32 - b.length), b), (uint256));
    }

    /// @dev Compute the address a contract will be deployed at for a given deployer address and nonce
    /// @notice adapted from Solmate implementation (https://github.com/Rari-Capital/solmate/blob/main/src/utils/LibRLP.sol)
    function computeCreateAddress(address deployer, uint256 nonce) internal pure virtual returns (address) {
        // forgefmt: disable-start
        // The integer zero is treated as an empty byte string, and as a result it only has a length prefix, 0x80, computed via 0x80 + 0.
        // A one byte integer uses its own value as its length prefix, there is no additional "0x80 + length" prefix that comes before it.
        if (nonce == 0x00)      return addressFromLast20Bytes(keccak256(abi.encodePacked(bytes1(0xd6), bytes1(0x94), deployer, bytes1(0x80))));
        if (nonce <= 0x7f)      return addressFromLast20Bytes(keccak256(abi.encodePacked(bytes1(0xd6), bytes1(0x94), deployer, uint8(nonce))));

        // Nonces greater than 1 byte all follow a consistent encoding scheme, where each value is preceded by a prefix of 0x80 + length.
        if (nonce <= 2**8 - 1)  return addressFromLast20Bytes(keccak256(abi.encodePacked(bytes1(0xd7), bytes1(0x94), deployer, bytes1(0x81), uint8(nonce))));
        if (nonce <= 2**16 - 1) return addressFromLast20Bytes(keccak256(abi.encodePacked(bytes1(0xd8), bytes1(0x94), deployer, bytes1(0x82), uint16(nonce))));
        if (nonce <= 2**24 - 1) return addressFromLast20Bytes(keccak256(abi.encodePacked(bytes1(0xd9), bytes1(0x94), deployer, bytes1(0x83), uint24(nonce))));
        // forgefmt: disable-end

        // More details about RLP encoding can be found here: https://eth.wiki/fundamentals/rlp
        // 0xda = 0xc0 (short RLP prefix) + 0x16 (length of: 0x94 ++ proxy ++ 0x84 ++ nonce)
        // 0x94 = 0x80 + 0x14 (0x14 = the length of an address, 20 bytes, in hex)
        // 0x84 = 0x80 + 0x04 (0x04 = the bytes length of the nonce, 4 bytes, in hex)
        // We assume nobody can have a nonce large enough to require more than 32 bytes.
        return addressFromLast20Bytes(
            keccak256(abi.encodePacked(bytes1(0xda), bytes1(0x94), deployer, bytes1(0x84), uint32(nonce)))
        );
    }

    function computeCreate2Address(bytes32 salt, bytes32 initcodeHash, address deployer)
        internal
        pure
        virtual
        returns (address)
    {
        return addressFromLast20Bytes(keccak256(abi.encodePacked(bytes1(0xff), deployer, salt, initcodeHash)));
    }

    /// @dev returns the address of a contract created with CREATE2 using the default CREATE2 deployer
    function computeCreate2Address(bytes32 salt, bytes32 initCodeHash) internal pure returns (address) {
        return computeCreate2Address(salt, initCodeHash, CREATE2_FACTORY);
    }

    /// @dev returns the hash of the init code (creation code + no args) used in CREATE2 with no constructor arguments
    /// @param creationCode the creation code of a contract C, as returned by type(C).creationCode
    function hashInitCode(bytes memory creationCode) internal pure returns (bytes32) {
        return hashInitCode(creationCode, "");
    }

    /// @dev returns the hash of the init code (creation code + ABI-encoded args) used in CREATE2
    /// @param creationCode the creation code of a contract C, as returned by type(C).creationCode
    /// @param args the ABI-encoded arguments to the constructor of C
    function hashInitCode(bytes memory creationCode, bytes memory args) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(creationCode, args));
    }

    // Performs a single call with Multicall3 to query the ERC-20 token balances of the given addresses.
    function getTokenBalances(address token, address[] memory addresses)
        internal
        virtual
        returns (uint256[] memory balances)
    {
        uint256 tokenCodeSize;
        assembly {
            tokenCodeSize := extcodesize(token)
        }
        require(tokenCodeSize > 0, "StdUtils getTokenBalances(address,address[]): Token address is not a contract.");

        // ABI encode the aggregate call to Multicall3.
        uint256 length = addresses.length;
        IMulticall3.Call[] memory calls = new IMulticall3.Call[](length);
        for (uint256 i = 0; i < length; ++i) {
            // 0x70a08231 = bytes4("balanceOf(address)"))
            calls[i] = IMulticall3.Call({target: token, callData: abi.encodeWithSelector(0x70a08231, (addresses[i]))});
        }

        // Make the aggregate call.
        (, bytes[] memory returnData) = multicall.aggregate(calls);

        // ABI decode the return data and return the balances.
        balances = new uint256[](length);
        for (uint256 i = 0; i < length; ++i) {
            balances[i] = abi.decode(returnData[i], (uint256));
        }
    }

    /*//////////////////////////////////////////////////////////////////////////
                                 PRIVATE FUNCTIONS
    //////////////////////////////////////////////////////////////////////////*/

    function addressFromLast20Bytes(bytes32 bytesValue) private pure returns (address) {
        return address(uint160(uint256(bytesValue)));
    }

    // Used to prevent the compilation of console, which shortens the compilation time when console is not used elsewhere.

    function console2_log(string memory p0, uint256 p1) private view {
        (bool status,) = address(CONSOLE2_ADDRESS).staticcall(abi.encodeWithSignature("log(string,uint256)", p0, p1));
        status;
    }

    function console2_log(string memory p0, string memory p1) private view {
        (bool status,) = address(CONSOLE2_ADDRESS).staticcall(abi.encodeWithSignature("log(string,string)", p0, p1));
        status;
    }
}

contract PRBMathUtils is StdUtils {
    /*//////////////////////////////////////////////////////////////////////////
                                      SD1x18
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Helper function to bound an SD1x18 number, which console logs the bounded result.
    function bound(SD1x18 x, SD1x18 min, SD1x18 max) internal view returns (SD1x18) {
        return SD1x18.wrap(int64(bound(int256(x.unwrap()), int256(min.unwrap()), int256(max.unwrap()))));
    }

    /// @dev Helper function to bound an SD1x18 number.
    function _bound(SD1x18 x, SD1x18 min, SD1x18 max) internal pure returns (SD1x18) {
        return SD1x18.wrap(int64(_bound(int256(x.unwrap()), int256(min.unwrap()), int256(max.unwrap()))));
    }

    /// @dev Helper function to bound an SD1x18 number, which console logs the bounded result.
    function bound(SD1x18 x, int64 min, SD1x18 max) internal view returns (SD1x18) {
        return SD1x18.wrap(int64(bound(int256(x.unwrap()), int256(min), int256(max.unwrap()))));
    }

    /// @dev Helper function to bound an SD1x18 number.
    function _bound(SD1x18 x, int64 min, SD1x18 max) internal pure returns (SD1x18) {
        return SD1x18.wrap(int64(_bound(int256(x.unwrap()), int256(min), int256(max.unwrap()))));
    }

    /// @dev Helper function to bound an SD1x18 number, which console logs the bounded result.
    function bound(SD1x18 x, SD1x18 min, int64 max) internal view returns (SD1x18) {
        return SD1x18.wrap(int64(bound(int256(x.unwrap()), int256(min.unwrap()), int256(max))));
    }

    /// @dev Helper function to bound an SD1x18 number.
    function _bound(SD1x18 x, SD1x18 min, int64 max) internal pure returns (SD1x18) {
        return SD1x18.wrap(int64(_bound(int256(x.unwrap()), int256(min.unwrap()), int256(max))));
    }

    /// @dev Helper function to bound an SD1x18 number, which console logs the bounded result.
    function bound(SD1x18 x, int64 min, int64 max) internal view returns (SD1x18) {
        return SD1x18.wrap(int64(bound(int256(x.unwrap()), int256(min), int256(max))));
    }

    /// @dev Helper function to bound an SD1x18 number.
    function _bound(SD1x18 x, int64 min, int64 max) internal pure returns (SD1x18) {
        return SD1x18.wrap(int64(_bound(int256(x.unwrap()), int256(min), int256(max))));
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      SD59X18
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Helper function to bound an SD59x18 number, which console logs the bounded result.
    function bound(SD59x18 x, SD59x18 min, SD59x18 max) internal view returns (SD59x18) {
        return SD59x18.wrap(bound(x.unwrap(), min.unwrap(), max.unwrap()));
    }

    /// @dev Helper function to bound an SD59x18 number.
    function _bound(SD59x18 x, SD59x18 min, SD59x18 max) internal pure returns (SD59x18) {
        return SD59x18.wrap(_bound(x.unwrap(), min.unwrap(), max.unwrap()));
    }

    /// @dev Helper function to bound an SD59x18 number, which console logs the bounded result.
    function bound(SD59x18 x, int256 min, SD59x18 max) internal view returns (SD59x18) {
        return SD59x18.wrap(bound(x.unwrap(), min, max.unwrap()));
    }

    /// @dev Helper function to bound an SD59x18 number.
    function _bound(SD59x18 x, int256 min, SD59x18 max) internal pure returns (SD59x18) {
        return SD59x18.wrap(_bound(x.unwrap(), min, max.unwrap()));
    }

    /// @dev Helper function to bound an SD59x18 number, which console logs the bounded result.
    function bound(SD59x18 x, SD59x18 min, int256 max) internal view returns (SD59x18) {
        return SD59x18.wrap(bound(x.unwrap(), min.unwrap(), max));
    }

    /// @dev Helper function to bound an SD59x18 number.
    function _bound(SD59x18 x, SD59x18 min, int256 max) internal pure returns (SD59x18) {
        return SD59x18.wrap(_bound(x.unwrap(), min.unwrap(), max));
    }

    /// @dev Helper function to bound an SD59x18 number, which console logs the bounded result.
    function bound(SD59x18 x, int256 min, int256 max) internal view returns (SD59x18) {
        return SD59x18.wrap(bound(x.unwrap(), min, max));
    }

    /// @dev Helper function to bound an SD59x18 number.
    function _bound(SD59x18 x, int256 min, int256 max) internal pure returns (SD59x18) {
        return SD59x18.wrap(_bound(x.unwrap(), min, max));
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      UD2x18
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Helper function to bound a UD2x18 number, which console logs the bounded result.
    function bound(UD2x18 x, UD2x18 min, UD2x18 max) internal view returns (UD2x18) {
        return UD2x18.wrap(uint64(bound(uint256(x.unwrap()), uint256(min.unwrap()), uint256(max.unwrap()))));
    }

    /// @dev Helper function to bound a UD2x18 number.
    function _bound(UD2x18 x, UD2x18 min, UD2x18 max) internal pure returns (UD2x18) {
        return UD2x18.wrap(uint64(_bound(uint256(x.unwrap()), uint256(min.unwrap()), uint256(max.unwrap()))));
    }

    /// @dev Helper function to bound a UD2x18 number, which console logs the bounded result.
    function bound(UD2x18 x, uint64 min, UD2x18 max) internal view returns (UD2x18) {
        return UD2x18.wrap(uint64(bound(uint256(x.unwrap()), uint256(min), uint256(max.unwrap()))));
    }

    /// @dev Helper function to bound a UD2x18 number.
    function _bound(UD2x18 x, uint64 min, UD2x18 max) internal pure returns (UD2x18) {
        return UD2x18.wrap(uint64(_bound(uint256(x.unwrap()), uint256(min), uint256(max.unwrap()))));
    }

    /// @dev Helper function to bound a UD2x18 number, which console logs the bounded result.
    function bound(UD2x18 x, UD2x18 min, uint64 max) internal view returns (UD2x18) {
        return UD2x18.wrap(uint64(bound(uint256(x.unwrap()), uint256(min.unwrap()), uint256(max))));
    }

    /// @dev Helper function to bound a UD2x18 number.
    function _bound(UD2x18 x, UD2x18 min, uint64 max) internal pure returns (UD2x18) {
        return UD2x18.wrap(uint64(_bound(uint256(x.unwrap()), uint256(min.unwrap()), uint256(max))));
    }

    /// @dev Helper function to bound a UD2x18 number, which console logs the bounded result.
    function bound(UD2x18 x, uint64 min, uint64 max) internal view returns (UD2x18) {
        return UD2x18.wrap(uint64(bound(uint256(x.unwrap()), uint256(min), uint256(max))));
    }

    /// @dev Helper function to bound a UD2x18 number.
    function _bound(UD2x18 x, uint64 min, uint64 max) internal pure returns (UD2x18) {
        return UD2x18.wrap(uint64(_bound(uint256(x.unwrap()), uint256(min), uint256(max))));
    }

    /*//////////////////////////////////////////////////////////////////////////
                                      UD60X18
    //////////////////////////////////////////////////////////////////////////*/

    /// @dev Helper function to bound a UD60x18 number, which console logs the bounded result.
    function bound(UD60x18 x, UD60x18 min, UD60x18 max) internal view returns (UD60x18) {
        return UD60x18.wrap(bound(x.unwrap(), min.unwrap(), max.unwrap()));
    }

    /// @dev Helper function to bound a UD60x18 number.
    function _bound(UD60x18 x, UD60x18 min, UD60x18 max) internal pure returns (UD60x18) {
        return UD60x18.wrap(_bound(x.unwrap(), min.unwrap(), max.unwrap()));
    }

    /// @dev Helper function to bound a UD60x18 number, which console logs the bounded result.
    function bound(UD60x18 x, uint256 min, UD60x18 max) internal view returns (UD60x18) {
        return UD60x18.wrap(bound(x.unwrap(), min, max.unwrap()));
    }

    /// @dev Helper function to bound a UD60x18 number.
    function _bound(UD60x18 x, uint256 min, UD60x18 max) internal pure returns (UD60x18) {
        return UD60x18.wrap(_bound(x.unwrap(), min, max.unwrap()));
    }

    /// @dev Helper function to bound a UD60x18 number, which console logs the bounded result.
    function bound(UD60x18 x, UD60x18 min, uint256 max) internal view returns (UD60x18) {
        return UD60x18.wrap(bound(x.unwrap(), min.unwrap(), max));
    }

    /// @dev Helper function to bound a UD60x18 number.
    function _bound(UD60x18 x, UD60x18 min, uint256 max) internal pure returns (UD60x18) {
        return UD60x18.wrap(_bound(x.unwrap(), min.unwrap(), max));
    }

    /// @dev Helper function to bound a UD60x18 number, which console logs the bounded result.
    function bound(UD60x18 x, uint256 min, uint256 max) internal view returns (UD60x18) {
        return UD60x18.wrap(bound(x.unwrap(), min, max));
    }

    /// @dev Helper function to bound a UD60x18 number.
    function _bound(UD60x18 x, uint256 min, uint256 max) internal pure returns (UD60x18) {
        return UD60x18.wrap(_bound(x.unwrap(), min, max));
    }
}

/*

██████╗ ██████╗ ██████╗ ███╗   ███╗ █████╗ ████████╗██╗  ██╗
██╔══██╗██╔══██╗██╔══██╗████╗ ████║██╔══██╗╚══██╔══╝██║  ██║
██████╔╝██████╔╝██████╔╝██╔████╔██║███████║   ██║   ███████║
██╔═══╝ ██╔══██╗██╔══██╗██║╚██╔╝██║██╔══██║   ██║   ██╔══██║
██║     ██║  ██║██████╔╝██║ ╚═╝ ██║██║  ██║   ██║   ██║  ██║
╚═╝     ╚═╝  ╚═╝╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝

██╗   ██╗██████╗ ██████╗ ██╗  ██╗ ██╗ █████╗
██║   ██║██╔══██╗╚════██╗╚██╗██╔╝███║██╔══██╗
██║   ██║██║  ██║ █████╔╝ ╚███╔╝ ╚██║╚█████╔╝
██║   ██║██║  ██║██╔═══╝  ██╔██╗  ██║██╔══██╗
╚██████╔╝██████╔╝███████╗██╔╝ ██╗ ██║╚█████╔╝
 ╚═════╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝ ╚═╝ ╚════╝

*/

/// @notice Converts a UD60x18 number to a simple integer by dividing it by `UNIT`.
/// @dev The result is rounded toward zero.
/// @param x The UD60x18 number to convert.
/// @return result The same number in basic integer form.
function convert(UD60x18 x) pure returns (uint256 result) {
    result = UD60x18.unwrap(x) / uUNIT;
}

/// @notice Converts a simple integer to UD60x18 by multiplying it by `UNIT`.
///
/// @dev Requirements:
/// - x must be less than or equal to `MAX_UD60x18 / UNIT`.
///
/// @param x The basic integer to convert.
/// @param result The same number converted to UD60x18.
function convert(uint256 x) pure returns (UD60x18 result) {
    if (x > uMAX_UD60x18 / uUNIT) {
        revert PRBMath_UD60x18_Convert_Overflow(x);
    }
    unchecked {
        result = UD60x18.wrap(x * uUNIT);
    }
}

/*

██████╗ ██████╗ ██████╗ ███╗   ███╗ █████╗ ████████╗██╗  ██╗
██╔══██╗██╔══██╗██╔══██╗████╗ ████║██╔══██╗╚══██╔══╝██║  ██║
██████╔╝██████╔╝██████╔╝██╔████╔██║███████║   ██║   ███████║
██╔═══╝ ██╔══██╗██╔══██╗██║╚██╔╝██║██╔══██║   ██║   ██╔══██║
██║     ██║  ██║██████╔╝██║ ╚═╝ ██║██║  ██║   ██║   ██║  ██║
╚═╝     ╚═╝  ╚═╝╚═════╝ ╚═╝     ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝

██╗   ██╗██████╗  ██████╗  ██████╗ ██╗  ██╗ ██╗ █████╗
██║   ██║██╔══██╗██╔════╝ ██╔═████╗╚██╗██╔╝███║██╔══██╗
██║   ██║██║  ██║███████╗ ██║██╔██║ ╚███╔╝ ╚██║╚█████╔╝
██║   ██║██║  ██║██╔═══██╗████╔╝██║ ██╔██╗  ██║██╔══██╗
╚██████╔╝██████╔╝╚██████╔╝╚██████╔╝██╔╝ ██╗ ██║╚█████╔╝
 ╚═════╝ ╚═════╝  ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═╝ ╚════╝

*/

