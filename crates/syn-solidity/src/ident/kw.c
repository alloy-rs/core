#if 0
/usr/bin/env cc -o kw "$0" || exit 1
dir=$(dirname $0)
./kw --expr difference > $dir/difference.expr
./kw --expr intersection > $dir/intersection.expr
[ -n "$1" ] && ./kw "$@";
rm -f ./kw
exit 0
#endif

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Copy-pasted from Solc:
// https://github.com/ethereum/solidity/blob/58811f134ac369b20c2ec1120907321edf08fff1/liblangutil/Token.h#L66C1-L278C1
#define TOKEN_LIST(T, K)												\
	/* End of source indicator. */										\
	T(EOS, "EOS", 0)													\
																		\
	/* Punctuators (ECMA-262, section 7.7, page 15). */				\
	T(LParen, "(", 0)                                                   \
	T(RParen, ")", 0)                                                   \
	T(LBrack, "[", 0)                                                   \
	T(RBrack, "]", 0)                                                   \
	T(LBrace, "{", 0)                                                   \
	T(RBrace, "}", 0)                                                   \
	T(Colon, ":", 0)                                                    \
	T(Semicolon, ";", 0)                                                \
	T(Period, ".", 0)                                                   \
	T(Conditional, "?", 3)                                              \
	T(DoubleArrow, "=>", 0)                                             \
	T(RightArrow, "->", 0)                                              \
	\
	/* Assignment operators. */										\
	/* IsAssignmentOp() relies on this block of enum values being */	\
	/* contiguous and sorted in the same order!*/						\
	T(Assign, "=", 2)                                                   \
	/* The following have to be in exactly the same order as the simple binary operators*/ \
	T(AssignBitOr, "|=", 2)                                           \
	T(AssignBitXor, "^=", 2)                                          \
	T(AssignBitAnd, "&=", 2)                                          \
	T(AssignShl, "<<=", 2)                                            \
	T(AssignSar, ">>=", 2)                                            \
	T(AssignShr, ">>>=", 2)                                           \
	T(AssignAdd, "+=", 2)                                             \
	T(AssignSub, "-=", 2)                                             \
	T(AssignMul, "*=", 2)                                             \
	T(AssignDiv, "/=", 2)                                             \
	T(AssignMod, "%=", 2)                                             \
	\
	/* Binary operators sorted by precedence. */                       \
	/* IsBinaryOp() relies on this block of enum values */             \
	/* being contiguous and sorted in the same order! */               \
	T(Comma, ",", 1)                                                   \
	T(Or, "||", 4)                                                     \
	T(And, "&&", 5)                                                    \
	T(BitOr, "|", 8)                                                   \
	T(BitXor, "^", 9)                                                  \
	T(BitAnd, "&", 10)                                                 \
	T(SHL, "<<", 11)                                                   \
	T(SAR, ">>", 11)                                                   \
	T(SHR, ">>>", 11)                                                  \
	T(Add, "+", 12)                                                    \
	T(Sub, "-", 12)                                                    \
	T(Mul, "*", 13)                                                    \
	T(Div, "/", 13)                                                    \
	T(Mod, "%", 13)                                                    \
	T(Exp, "**", 14)                                                   \
	\
	/* Compare operators sorted by precedence. */                      \
	/* IsCompareOp() relies on this block of enum values */            \
	/* being contiguous and sorted in the same order! */               \
	T(Equal, "==", 6)                                                  \
	T(NotEqual, "!=", 6)                                               \
	T(LessThan, "<", 7)                                                \
	T(GreaterThan, ">", 7)                                             \
	T(LessThanOrEqual, "<=", 7)                                        \
	T(GreaterThanOrEqual, ">=", 7)                                     \
	\
	/* Unary operators. */                                             \
	/* IsUnaryOp() relies on this block of enum values */              \
	/* being contiguous and sorted in the same order! */               \
	T(Not, "!", 0)                                                     \
	T(BitNot, "~", 0)                                                  \
	T(Inc, "++", 0)                                                    \
	T(Dec, "--", 0)                                                    \
	K(Delete, "delete", 0)                                             \
	\
	/* Inline Assembly Operators */                                    \
	T(AssemblyAssign, ":=", 2)                                         \
	/* Keywords */                                                     \
	K(Abstract, "abstract", 0)                                         \
	K(Anonymous, "anonymous", 0)                                       \
	K(As, "as", 0)                                                     \
	K(Assembly, "assembly", 0)                                         \
	K(Break, "break", 0)                                               \
	K(Catch, "catch", 0)                                               \
	K(Constant, "constant", 0)                                         \
	K(Constructor, "constructor", 0)                                   \
	K(Continue, "continue", 0)                                         \
	K(Contract, "contract", 0)                                         \
	K(Do, "do", 0)                                                     \
	K(Else, "else", 0)                                                 \
	K(Enum, "enum", 0)                                                 \
	K(Emit, "emit", 0)                                                 \
	K(Event, "event", 0)                                               \
	K(External, "external", 0)                                         \
	K(Fallback, "fallback", 0)                                         \
	K(For, "for", 0)                                                   \
	K(Function, "function", 0)                                         \
	K(Hex, "hex", 0)                                                   \
	K(If, "if", 0)                                                     \
	K(Indexed, "indexed", 0)                                           \
	K(Interface, "interface", 0)                                       \
	K(Internal, "internal", 0)                                         \
	K(Immutable, "immutable", 0)                                       \
	K(Import, "import", 0)                                             \
	K(Is, "is", 0)                                                     \
	K(Library, "library", 0)                                           \
	K(Mapping, "mapping", 0)                                           \
	K(Memory, "memory", 0)                                             \
	K(Modifier, "modifier", 0)                                         \
	K(New, "new", 0)                                                   \
	K(Override, "override", 0)                                         \
	K(Payable, "payable", 0)                                           \
	K(Public, "public", 0)                                             \
	K(Pragma, "pragma", 0)                                             \
	K(Private, "private", 0)                                           \
	K(Pure, "pure", 0)                                                 \
	K(Receive, "receive", 0)                                           \
	K(Return, "return", 0)                                             \
	K(Returns, "returns", 0)                                           \
	K(Storage, "storage", 0)                                           \
	K(CallData, "calldata", 0)                                         \
	K(Struct, "struct", 0)                                             \
	K(Throw, "throw", 0)                                               \
	K(Try, "try", 0)                                                   \
	K(Type, "type", 0)                                                 \
	K(Unchecked, "unchecked", 0)                                       \
	K(Unicode, "unicode", 0)                                           \
	K(Using, "using", 0)                                               \
	K(View, "view", 0)                                                 \
	K(Virtual, "virtual", 0)                                           \
	K(While, "while", 0)                                               \
	\
	/* Ether subdenominations */                                       \
	K(SubWei, "wei", 0)                                                \
	K(SubGwei, "gwei", 0)                                              \
	K(SubEther, "ether", 0)                                            \
	K(SubSecond, "seconds", 0)                                         \
	K(SubMinute, "minutes", 0)                                         \
	K(SubHour, "hours", 0)                                             \
	K(SubDay, "days", 0)                                               \
	K(SubWeek, "weeks", 0)                                             \
	K(SubYear, "years", 0)                                             \
	/* type keywords*/                                                 \
	K(Int, "int", 0)                                                   \
	K(UInt, "uint", 0)                                                 \
	K(Bytes, "bytes", 0)                                               \
	K(String, "string", 0)                                             \
	K(Address, "address", 0)                                           \
	K(Bool, "bool", 0)                                                 \
	K(Fixed, "fixed", 0)                                               \
	K(UFixed, "ufixed", 0)                                             \
	T(IntM, "intM", 0)                                                 \
	T(UIntM, "uintM", 0)                                               \
	T(BytesM, "bytesM", 0)                                             \
	T(FixedMxN, "fixedMxN", 0)                                         \
	T(UFixedMxN, "ufixedMxN", 0)                                       \
	T(TypesEnd, nullptr, 0) /* used as type enum end marker */         \
	\
	/* Literals */                                                     \
	K(TrueLiteral, "true", 0)                                          \
	K(FalseLiteral, "false", 0)                                        \
	T(Number, nullptr, 0)                                              \
	T(StringLiteral, nullptr, 0)                                       \
	T(UnicodeStringLiteral, nullptr, 0)                                \
	T(HexStringLiteral, nullptr, 0)                                    \
	T(CommentLiteral, nullptr, 0)                                      \
	\
	/* Identifiers (not keywords or future reserved words). */         \
	T(Identifier, nullptr, 0)                                          \
	\
	/* Keywords reserved for future use. */                            \
	K(After, "after", 0)                                               \
	K(Alias, "alias", 0)                                               \
	K(Apply, "apply", 0)                                               \
	K(Auto, "auto", 0)                                                 \
	K(Byte, "byte", 0)                                                 \
	K(Case, "case", 0)                                                 \
	K(CopyOf, "copyof", 0)                                             \
	K(Default, "default", 0)                                           \
	K(Define, "define", 0)                                             \
	K(Final, "final", 0)                                               \
	K(Implements, "implements", 0)                                     \
	K(In, "in", 0)                                                     \
	K(Inline, "inline", 0)                                             \
	K(Let, "let", 0)                                                   \
	K(Macro, "macro", 0)                                               \
	K(Match, "match", 0)                                               \
	K(Mutable, "mutable", 0)                                           \
	K(NullLiteral, "null", 0)                                          \
	K(Of, "of", 0)                                                     \
	K(Partial, "partial", 0)                                           \
	K(Promise, "promise", 0)                                           \
	K(Reference, "reference", 0)                                       \
	K(Relocatable, "relocatable", 0)                                   \
	K(Sealed, "sealed", 0)                                             \
	K(Sizeof, "sizeof", 0)                                             \
	K(Static, "static", 0)                                             \
	K(Supports, "supports", 0)                                         \
	K(Switch, "switch", 0)                                             \
	K(Typedef, "typedef", 0)                                           \
	K(TypeOf, "typeof", 0)                                             \
	K(Var, "var", 0)                                                   \
	\
	/* Yul-specific tokens, but not keywords. */                       \
	T(Leave, "leave", 0)                                               \
	\
	T(NonExperimentalEnd, nullptr, 0) /* used as non-experimental enum end marker */ \
	T(ExperimentalEnd, nullptr, 0) /* used as experimental enum end marker */ \
	/* Illegal token - not able to scan. */                            \
	T(Illegal, "ILLEGAL", 0)                                           \
	\
	/* Scanner-internal use only. */                                   \
	T(Whitespace, nullptr, 0)

// https://doc.rust-lang.org/reference/keywords.html
char* rust[] = {
	// strict
	"as",
	"break",
	"const",
	"continue",
	"crate",
	"else",
	"enum",
	"extern",
	"false",
	"fn",
	"for",
	"if",
	"impl",
	"in",
	"let",
	"loop",
	"match",
	"mod",
	"move",
	"mut",
	"pub",
	"ref",
	"return",
	"self",
	"Self",
	"static",
	"struct",
	"super",
	"trait",
	"true",
	"type",
	"unsafe",
	"use",
	"where",
	"while",
	// strict, >=2018
	"async",
	"await",
	"dyn",
	// reserved
	"abstract",
	"become",
	"box",
	"do",
	"final",
	"macro",
	"override",
	"priv",
	"typeof",
	"unsized",
	"virtual",
	"yield",
	// reserved, >=2018
	"try",
};

// Cannot ever be made into raw identifiers, so just ignore these.
char* rust_disallowed[] = {
	"crate",
	"self",
	"Self",
	"super",
};

char* solidity[] = {
	#define NOOP(...)
	#define KW(name, string, precedence) string,
	TOKEN_LIST(NOOP, KW)
};

int print_array(char* strings[], int len, int as_expr) {
	if (as_expr) printf("[");
	for (int i = 0; i < len; i++) {
		if (as_expr) {
			printf("\"%s\",", strings[i]);
		} else {
			printf("%s\n", strings[i]);
		}
	}
	if (as_expr) printf("]");
	return 0;
}

int get_set(int difference, int as_expr) {
	char** out = malloc(200);
	int out_i = 0;
    for (int i = 0; i < sizeof(rust) / sizeof(char*); i++) {
        char* s_rust = rust[i];

		for (int j = 0; j < sizeof(rust_disallowed) / sizeof(char*); j++) {
			if (strcmp(s_rust, rust_disallowed[j]) == 0) {
				// https://xkcd.com/292
				goto cont;
			}
		}

		int any = 0;
        for (int j = 0; j < sizeof(solidity) / sizeof(char*); j++) {
			if (strcmp(s_rust, solidity[j]) == 0) {
				any = 1;
				break;
			}
        }
		if ((!any) == difference) {
			out[out_i++] = s_rust;
		}

		cont:
    }
    return print_array(out, out_i, as_expr);
}

// Arguments:
// - "rust" prints the set of keywords that are reserved in Rust.
// - "solidity" prints the set of keywords that are reserved in Solidity.
// - "intersection" prints the set of keywords that are reserved in both Rust and Solidity.
// - "difference" prints the set of keywords that are reserved in Rust but not in Solidity.
// Flags:
// - "--expr" prints the selected set as a Rust expression instead of one-per-line.
int main(int argc, char* argv[]) {
	if (argc < 2) return 1;

	int current_arg = 1;
	int as_expr = 0;
	for (; current_arg < argc; current_arg++) {
		if (argv[current_arg][0] != '-') break;
		if (strcmp(argv[current_arg], "--expr") == 0) as_expr = 1;
	}

	char* posarg = argv[current_arg];

	int ret;
	if (strcmp(posarg, "rust") == 0) ret = print_array(rust, sizeof(rust) / sizeof(char*), as_expr);
	else if (strcmp(posarg, "solidity") == 0) ret = print_array(solidity, sizeof(solidity) / sizeof(char*), as_expr);
	else if (strcmp(posarg, "intersection") == 0) ret = get_set(0, as_expr);
	else if (strcmp(posarg, "difference") == 0) ret = get_set(1, as_expr);
	else {
		fprintf(stderr, "Unknown argument: %s\n", posarg);
		ret = 1;
	}
	return ret;
}
