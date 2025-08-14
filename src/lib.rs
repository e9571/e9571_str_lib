use sha3::{Digest, Keccak256};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::collections::HashMap;
use e9571_lib1::e9571_lib1::{parse_int, word_split, default_encode_md5};

pub mod e9571_str_lib {
    use super::*;

    /// 定义序列类型，用于字符串列表排序 (StringList)
    #[derive(Clone)]
    pub struct StringList(Vec<String>);

    impl StringList {
        /// 创建新的 StringList
        pub fn new(data: &[String]) -> Self {
            StringList(data.to_vec())
        }

        /// 获取元素个数 (Len)
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// 排序模式 (SortPackage)
        /// Type: 0 for ascending, non-zero for descending
        pub fn sort_package(&mut self, sort_type: i32) -> Vec<String> {
            if sort_type == 0 {
                // 升序
                self.0.sort();
            } else {
                // 降序
                self.0.sort_by(|a, b| b.cmp(a));
            }
            self.0.clone()
        }
    }

    /// 字符串排序 (SortPackage)
    /// Sorts a list of strings, Type: 0 for ascending, non-zero for descending
    pub fn sort_package(data: &[String], sort_type: i32) -> Vec<String> {
        let mut list = StringList::new(data);
        list.sort_package(sort_type)
    }

    /// 字符串模式排序并返回拼接字符串及MD5 (SortPackageStr)
    /// Returns (concatenated string with '@', MD5 hash)
    pub fn sort_package_str(data: &[String], sort_type: i32) -> (String, String) {
        let list = sort_package(data, sort_type);
        let str_tmp = list.join("@");
        let md5_hash = default_encode_md5(&str_tmp);
        (str_tmp, md5_hash)
    }

    /// 无差别对比字符串，返回匹配次数 (Str_Count)
    /// Counts occurrences of data2 in data1 (case-insensitive)
    pub fn str_count(data1: &str, data2: &str) -> i32 {
        let data1_lower = data1.to_lowercase();
        let data2_lower = data2.to_lowercase();
        data1_lower.matches(&data2_lower).count() as i32
    }

    /// 数组整数对比，返回是否匹配 (Str_Count_int)
    /// Checks if data2 (as int) exists in comma-separated data1
    pub fn str_count_int(data1: &str, data2: &str) -> i32 {
        let list = word_split(data1, ",");
        for item in list {
            if parse_int(&item) == parse_int(data2) {
                return 1;
            }
        }
        0
    }

    /// 智能合约哈希数值函数模拟 (HashAddressNTokenId)
    /// Computes Keccak256 hash of concatenated addr and typeValue, returns first 8 bytes as u64
    pub fn hash_address_n_token_id(addr: &str, type_value: &str) -> u64 {
        let data = format!("{}{}", addr, type_value);
        let mut hasher = Keccak256::new();
        hasher.update(data.as_bytes());
        let hash_bytes = hasher.finalize();
        let result = BigUint::from_bytes_be(&hash_bytes[..8]);
        result.to_u64().unwrap_or(0)
    }

    /// 快速编码模式，ASCII 之和 (Str_ascii)
    /// Sums ASCII values of characters, returns as u64
    pub fn str_ascii(str: &str) -> u64 {
        str.chars().map(|c| c as u64).sum()
    }
}