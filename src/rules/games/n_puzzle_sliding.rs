//! 数码拼图
//!
//! 滑动空格将乱序数字牌归位的华容道式拼图

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NPuzzleSlidingRules,
    name: "数码拼图",
    desc: "滑动空格将乱序数字牌归位的华容道式拼图",
    origin: "中国",
    tags: ["游戏", "拼图", "益智", "华容道"]
}

impl NPuzzleSlidingRules {
    /// 基本规则
    pub fn basic(&self) -> Vec<&'static str> {
        vec![
            "方格数字牌加空格",
            "四乘四方15数字",
            "滑动空格相邻牌",
            "按序归位完成",
        ]
    }

    /// 滑动操作
    pub fn slide(&self) -> Vec<&'static str> {
        vec![
            "空格旁牌可移入空位",
            "一次滑动一张",
            "空格移动位置",
            "逐步调整布局",
        ]
    }

    /// 目标状态
    pub fn goal(&self) -> Vec<&'static str> {
        vec!["左上起顺序递增", "末位留空格", "1至15顺序排列", "左上角为1"]
    }

    /// 求解思路
    pub fn solve(&self) -> Vec<&'static str> {
        vec![
            "从上到下逐行归位",
            "最后两行循环交换",
            "逆序奇偶可解性",
            "耐心整行整列推",
        ]
    }
}

impl Rule for NPuzzleSlidingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("n_puzzle")
    }

    fn explain(&self) -> String {
        format!(
            "【数码拼图】\n{}",
            [
                format!(
                    "基本规则：\\n{}",
                    self.basic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "滑动操作：\\n{}",
                    self.slide()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "目标状态：\\n{}",
                    self.goal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "求解思路：\\n{}",
                    self.solve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_npuzzleslidingrules_basic() {
        let rules = NPuzzleSlidingRules::new();
        assert_eq!(rules.metadata().name, "数码拼图");
        assert!(!rules.basic().is_empty());
        assert!(!rules.slide().is_empty());
        assert!(!rules.goal().is_empty());
        assert!(!rules.solve().is_empty());
    }

    #[test]
    fn test_npuzzleslidingrules_validation() {
        let rules = NPuzzleSlidingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("n_puzzle"));
    }

    #[test]
    fn test_npuzzleslidingrules_explain() {
        let rules = NPuzzleSlidingRules::new();
        let e = rules.explain();
        assert!(e.contains("基本规则"));
        assert!(e.contains("滑动操作"));
        assert!(e.contains("目标状态"));
    }
}
