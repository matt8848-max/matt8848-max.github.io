# ST 表

ST表（sparse table）用于**解决可重复贡献问题**

此方法基于[倍增思想](../算法基础/倍增.md)，可以做到\\(\Theta(n\log n)\\)预处理，\\(\Theta(1)\\)的回答查询，但不能修改

基于倍增思想，我们考虑如何求出区间最大值．可以发现，如果按照一般的倍增流程，每次跳 
\\(2^i\\) 步的话，询问时的复杂度仍旧是 \\(\Theta(\log n)\\)，并没有比线段树更优，反而预处理一步还比线段树慢。

我们发现 \\(\max(x,x)=x\\)，也就是说，区间最大值是一个具有「可重复贡献」性质的问题
> 即使用来求解的预处理区间有重叠部分，只要这些区间的并是所求的区间，最终计算出的答案就是正确的

如果手动模拟一下，可以发现我们能使用至多两个预处理过的区间来覆盖询问区间，也就是说询问时的时间复杂度可以被降至 \\(\Theta(1)\\)，在处理有大量询问的题目时十分有效。

预处理实现如下：
+ 令\\(f(j,i)\\)为\\(\[j,j+2^i-1\]\\)区间上的最大值（\\(j\\) 为区间左端点，\\(i\\) 为倍增的层数）。
+ \\(f(j,0)=a_j\\)（a是序列）
+ 显然 \\(f(j,i)=\max(f(j,i-1), f(j+2^{i-1}, i-1))\\)：把长度 \\(2^i\\) 的区间拆成左右两段长度 \\(2^{i-1}\\) 的区间。

查询如下：
+ 对于查询\\(\[l,r)\\)，可分为\\(\[l,l+2^s)\\)与\\(\[r-2^s,r)\\)：两段允许重叠，只要并起来盖住\\(\[l,r)\\)即可
+ \\(s=\lfloor\log_2(r-l)\rfloor\\)

模板代码如下：
```rust
/// 稀疏表：预处理 O(n log n)，查询 O(1)，不支持修改。
///
/// 用 `st[i][j]` 表示「从 `j` 开始、长度为 `2^i` 的区间」上 `op` 的结果，
/// 即把倍增层数 `i` 放在第一维——这正是公式 `f(j, i)` 的写法（先左端点、后层数），
/// 同层的下标连续，遍历时内存访问也是连续的。
struct SparseTable<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    st: Vec<Vec<T>>,
    op: F,
}

impl<T: Copy, F: Fn(T, T) -> T> SparseTable<T, F> {
    /// 建表：`st[0]` 是原数组，之后每层都由下一层的两个半段拼出来。
    fn new(nums: &[T], op: F) -> Self {
        let n = nums.len();
        // 区间长度 2^i 不能超过 n，故层数上界为 floor(log2(n)) + 1
        let levels = n.ilog2() as usize + 1;
        let mut st = vec![vec![nums[0]; n]; levels];
        st[0].clone_from_slice(nums);
        for i in 1..levels {
            let half = 1 << (i - 1); // 半段长度 2^(i-1)
            let len = 1 << i; // 本层区间长度 2^i
            // 只有起点 j 满足 j + 2^i <= n 的区间才有值，其余位置留空不用
            for j in 0..=n - len {
                st[i][j] = op(st[i - 1][j], st[i - 1][j + half]);
            }
        }
        Self { st, op }
    }

    /// 查询 `[l, r)` 上的结果；调用方需保证 `l < r` 且 `r <= n`。
    fn query(&self, l: usize, r: usize) -> T {
        // 取不超过区间长度的最大 2 的幂：两段长 2^s 的区间一定能盖满整个查询区间
        let s = (r - l).ilog2() as usize;
        (self.op)(self.st[s][l], self.st[s][r - (1 << s)])
    }
}

fn main() {
    let nums = [3, 1, 4, 1, 5, 9, 2, 6];
    let st = SparseTable::new(&nums, |a, b| a.max(b));
    // nums[0..5] = [3, 1, 4, 1, 5]，最大值为 5
    println!("{}", st.query(0, 5));
    // 注意必须保证 l < r：空区间会让 ilog2(0) panic
}
```

[3691. 最大子数组总值 II](https://leetcode.cn/problems/maximum-total-subarray-value-ii/)

```rust
use std::collections::BinaryHeap;

/// 稀疏表：结构同上一节，`st[i][j]` 表示区间 `[j, j + 2^i)` 上 `op` 的结果。
struct SparseTable<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    st: Vec<Vec<T>>,
    op: F,
}

impl<T: Copy, F: Fn(T, T) -> T> SparseTable<T, F> {
    fn new(nums: &[T], op: F) -> Self {
        let n = nums.len();
        let levels = n.ilog2() as usize + 1;
        let mut st = vec![vec![nums[0]; n]; levels];
        st[0].clone_from_slice(nums);
        for i in 1..levels {
            let half = 1 << (i - 1);
            let len = 1 << i;
            for j in 0..=n - len {
                st[i][j] = op(st[i - 1][j], st[i - 1][j + half]);
            }
        }
        Self { st, op }
    }

    /// 查询 `[l, r)` 上的结果。
    fn query(&self, l: usize, r: usize) -> T {
        let s = (r - l).ilog2() as usize;
        (self.op)(self.st[s][l], self.st[s][r - (1 << s)])
    }
}

struct Solution;

impl Solution {
    /// 贪心：堆里放的是「子数组总值 = 最大值 − 最小值」，
    /// 每次取出最大的一个累加，再把它去掉左端、去掉右端的两个邻居入堆。
    fn max_total_value(nums: Vec<i64>, k: i32) -> i64 {
        let n = nums.len();
        // 最大值、最小值各建一张表，区间最值都变成 O(1) 查询
        let max_st = SparseTable::new(&nums, |a, b| a.max(b));
        let min_st = SparseTable::new(&nums, |a, b| a.min(b));

        // 大根堆：(子数组总值, 左端点, 右端点开区间)
        let mut heap: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
        heap.push((max_st.query(0, n) - min_st.query(0, n), 0, n));

        let mut ans = 0;
        for _ in 0..k {
            let (value, l, r) = heap.pop().expect("k 不超过子数组总数，堆中必有元素");
            ans += value;

            // 只在当前区间是从 n 收缩来的情况下补左侧邻居，避免同一个区间被重复计数
            if r == n && l + 1 < n {
                heap.push((max_st.query(l + 1, r) - min_st.query(l + 1, r), l + 1, r));
            }
            // 去掉右端：区间长度至少为 2 才有意义
            if r - 1 > l {
                heap.push((max_st.query(l, r - 1) - min_st.query(l, r - 1), l, r - 1));
            }
        }
        ans
    }
}

fn main() {
    // [1, 2, 3] 的各子数组总值为 2、1、1、0、0、0，取最大的 2 个：2 + 1
    println!("{}", Solution::max_total_value(vec![1, 2, 3], 2)); // 3
}
```