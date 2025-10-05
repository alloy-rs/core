#!/usr/bin/env python3
"""
Generate Rust's all_the_tuples macro with configurable N.
"""


def generate_tuple_list(n, double=False):
    """Generate tuple list for a given n."""
    if double:
        return ", ".join(f"(T{i} U{i})" for i in range(1, n + 1))
    else:
        return ", ".join(f"T{i}" for i in range(1, n + 1))


def generate_macro(max_n):
    """Generate the complete all_the_tuples macro."""
    lines = [
        "/// Calls the given macro with all the tuples.",
        "#[rustfmt::skip]",
        "macro_rules! all_the_tuples {",
        "    (@double $mac:path) => {",
    ]

    # Generate @double variant
    for i in range(1, max_n + 1):
        tuple_list = generate_tuple_list(i, double=True)
        lines.append(f"        $mac!({i:2} {tuple_list});")

    lines.append("    };")
    lines.append("")
    lines.append("    ($mac:path) => {")

    # Generate single variant
    for i in range(1, max_n + 1):
        tuple_list = generate_tuple_list(i, double=False)
        lines.append(f"        $mac!({i:2} {tuple_list});")

    lines.append("    };")
    lines.append("}")
    lines.append("")

    return "\n".join(lines)


def main():
    import sys

    if len(sys.argv) != 2:
        print("Usage: python all_tuples.py <N>")
        print("Example: python all_tuples.py 60")
        sys.exit(1)

    try:
        n = int(sys.argv[1])
        if n <= 0:
            raise ValueError("N must be positive")
    except ValueError as e:
        print(f"Error: {e}")
        print("N must be a positive integer")
        sys.exit(1)

    macro_code = generate_macro(n)
    print(macro_code)


if __name__ == "__main__":
    main()
