# e9571_str_lib Usage Examples

This document provides examples of using the `e9571_str_lib` module functions in a Rust program, tailored for casino-related scenarios such as sorting addresses, generating hashes, and string manipulation.

## Source Code Example

Below is a Rust program demonstrating the usage of various functions from the `e9571_str_lib` module.

```rust
use e9571_str_lib::e9571_str_lib::*;

fn main() {
    // Example 1: SortPackage
    println!("=== SortPackage ===");
    let data = vec![
        "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c".to_string(),
        "0x55d398326f99059ff775485246999027b3197955".to_string(),
    ];
    println!("Ascending: {:?}", sort_package(&data, 0));
    println!("Descending: {:?}", sort_package(&data, 1));

    // Example 2: SortPackageStr
    println!("\n=== SortPackageStr ===");
    let (str_tmp, md5) = sort_package_str(&data, 0);
    println!("Concatenated: {}", str_tmp);
    println!("MD5: {}", md5);

    // Example 3: Str_Count
    println!("\n=== Str_Count ===");
    println!("Count 'abc' in 'ABCabcABC': {}", str_count("ABCabcABC", "abc"));
    println!("Count 'xyz' in 'ABC': {}", str_count("ABC", "xyz"));

    // Example 4: Str_Count_int
    println!("\n=== Str_Count_int ===");
    println!("Find '2' in '1,2,3': {}", str_count_int("1,2,3", "2"));
    println!("Find '4' in '1,2,3': {}", str_count_int("1,2,3", "4"));

    // Example 5: HashAddressNTokenId
    println!("\n=== HashAddressNTokenId ===");
    let addr = "0x1234567890abcdef1234567890abcdef12345678";
    let type_value = "abcd";
    println!("Hash (addr: {}, type: {}): {}", addr, type_value, hash_address_n_token_id(addr, type_value));
    println!("Test case (type: xyz): {}", hash_address_n_token_id(addr, "xyz"));
    println!("Test case (type: 1234): {}", hash_address_n_token_id(addr, "1234"));

    // Example 6: Str_ascii
    println!("\n=== Str_ascii ===");
    println!("ASCII sum of 'abc': {}", str_ascii("abc"));
    println!("ASCII sum of 'xyz': {}", str_ascii("xyz"));
}
```

## Explanation of Functions

The `e9571_str_lib` module provides utility functions for string manipulation, sorting, and hashing, which are useful in casino applications for handling addresses, tokens, and data processing.

1. **`SortPackage`**:
   - Sorts a vector of strings (e.g., blockchain addresses) in ascending (`0`) or descending (`1`) order.
   - **Use Case**: Sorting user wallet addresses or transaction IDs in casino systems.

2. **`SortPackageStr`**:
   - Concatenates a vector of strings and computes an MD5 hash of the concatenated string.
   - **Use Case**: Generating a unique hash for a set of betting addresses or records.

3. **`Str_Count`**:
   - Counts occurrences of a substring in a string (case-sensitive).
   - **Use Case**: Counting specific patterns in transaction logs or user inputs.

4. **`Str_Count_int`**:
   - Counts occurrences of a specific integer (as a string) in a comma-separated string.
   - **Use Case**: Analyzing betting options or game results stored as comma-separated values.

5. **`HashAddressNTokenId`**:
   - Generates a hash from a blockchain address and a token type identifier.
   - **Use Case**: Creating unique identifiers for casino tokens or user transactions.

6. **`Str_ascii`**:
   - Calculates the sum of ASCII values of characters in a string.
   - **Use Case**: Generating checksums or simple validation codes for casino data.

## Casino Scenario Usage

These functions are particularly useful in casino applications for:
- Sorting and validating blockchain addresses for deposits/withdrawals (`SortPackage`, `SortPackageStr`).
- Counting specific patterns or values in game logs or user inputs (`Str_Count`, `Str_Count_int`).
- Generating unique hashes for transactions or tokens (`HashAddressNTokenId`).
- Computing checksums for data integrity (`Str_ascii`).

## Example Output

The output depends on the implementation of `e9571_str_lib`. An example output might look like:

```
=== SortPackage ===
Ascending: ["0x4f1960E29b2cA581a38c5c474e123f420F8092db", "0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c"]
Descending: ["0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c", "0x4f1960E29b2cA581a38c5c474e123f420F8092db"]

=== SortPackageStr ===
Concatenated: 0x4f1960E29b2cA581a38c5c474e123f420F8092db0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c
MD5: <computed_md5_hash>

=== Str_Count ===
Count 'abc' in 'ABCabcABC': 1
Count 'xyz' in 'ABC': 0

=== Str_Count_int ===
Find '2' in '1,2,3': 1
Find '4' in '1,2,3': 0

=== HashAddressNTokenId ===
Hash (addr: 0x1234567890abcdef1234567890abcdef12345678, type: abcd): <computed_hash>
Test case (type: xyz): <computed_hash>
Test case (type: 1234): <computed_hash>

=== Str_ascii ===
ASCII sum of 'abc': 294
ASCII sum of 'xyz': 363
```

## Notes
- The `e9571_str_lib` module is assumed to be available and correctly implemented.
- Ensure proper error handling for production use (e.g., invalid addresses or empty inputs).
- The example addresses are valid Ethereum-style addresses for demonstration purposes.
- For GitHub rendering, this Markdown uses clear headings, code blocks, and structured explanations.
