//! 算法设计定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AlgorithmsRules, name: "算法设计定律", desc: "算法设计定律", origin: "国际", tags: ["科学", "计算机"] }
impl AlgorithmsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "冒泡排序:O(n^2)相邻比较交换",
            "快速排序:O(n log n)平均分治法选基准",
            "归并排序:O(n log n)稳定分治法",
            "堆排序:O(n log n)原地不稳定",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "BFS广度优先搜索:O(V+E)队列实现",
            "DFS深度优先搜索:O(V+E)栈/递归实现",
            "Dijkstra最短路径:O((V+E)logV)非负权重",
            "Bellman-Ford:O(VE)可处理负权重",
            "Floyd-Warshall:O(V^3)所有点对最短路径",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "最优子结构:问题的最优解包含子问题的最优解",
            "重叠子问题:子问题被重复计算",
            "状态转移方程:定义子问题之间的关系",
            "背包问题:0-1背包/完全背包/多重背包",
            "最长公共子序列LCS",
        ]
    }
}
impl Rule for AlgorithmsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("algorithms")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "算法设计定律",
            &[
                ("排序算法", &self.section_0()),
                ("图算法", &self.section_1()),
                ("动态规划", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AlgorithmsRules::new();
        assert!(!r.explain().is_empty());
    }
}
