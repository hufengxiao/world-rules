//! 华容道滑块
//!
//! 华容道移块、出口与滑动的解法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: KlotskiHuarongRules,
    name: "华容道滑块",
    desc: "华容道移块、出口与滑动的解法",
    origin: "中国",
    tags: ["游戏", "华容道", "滑块", "益智"]
}

impl KlotskiHuarongRules {
    /// 棋盘布局
    pub fn board(&self) -> Vec<&'static str> {
        vec!["格子有大小狭块", "大将块占多格", "士兵小格", "有出口"]
    }

    /// 滑动规则
    pub fn slide(&self) -> Vec<&'static str> {
        vec!["每次移一块", "沿空格滑动", "块不重叠", "不可跨格"]
    }

    /// 目标解出
    pub fn goal(&self) -> Vec<&'static str> {
        vec!["把主将移出口", "借空位腾挪", "层层拆解", "完成最优步"]
    }

    /// 解谜心态
    pub fn patience(&self) -> Vec<&'static str> {
        vec!["失败重来不急", "观察全局局面", "先易后难", "乐在动脑"]
    }
}

impl Rule for KlotskiHuarongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("klotski")
    }

    fn explain(&self) -> String {
        format!(
            "【华容道滑块】\n{}",
            [
                format!(
                    "棋盘布局：\\n{}",
                    self.board()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "滑动规则：\\n{}",
                    self.slide()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "目标解出：\\n{}",
                    self.goal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "解谜心态：\\n{}",
                    self.patience()
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
    fn test_klotskihuarongrules_basic() {
        let rules = KlotskiHuarongRules::new();
        assert_eq!(rules.metadata().name, "华容道滑块");
        assert!(!rules.board().is_empty());
        assert!(!rules.slide().is_empty());
        assert!(!rules.goal().is_empty());
        assert!(!rules.patience().is_empty());
    }

    #[test]
    fn test_klotskihuarongrules_validation() {
        let rules = KlotskiHuarongRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("klotski"));
    }

    #[test]
    fn test_klotskihuarongrules_explain() {
        let rules = KlotskiHuarongRules::new();
        let e = rules.explain();
        assert!(e.contains("棋盘布局"));
        assert!(e.contains("滑动规则"));
        assert!(e.contains("目标解出"));
    }
}
