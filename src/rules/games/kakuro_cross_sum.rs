//! 数谜（Kakuro）
//!
//! 数谜是一种数字填空解谜游戏，在空白格中填入1-9的数字，使每个横向或纵向连续区块的数字之和等于其左侧或上方的提示数，且同一区块内数字不重复。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: KakuroCrossSumRules,
    name: "数谜（Kakuro）",
    desc: "数谜是一种数字填空解谜游戏，在空白格中填入1-9的数字，使每个横向或纵向连续区块的数字之和等于其左侧或上方的提示数，且同一区块内数字不重复。",
    origin: "源自《Cross Sums》谜题，流行于日本与欧美解谜界",
    tags: ["解谜", "数字", "逻辑", "桌面游戏"]
}

impl KakuroCrossSumRules {
    /// 基本盘面
    pub fn board(&self) -> Vec<&'static str> {
        vec![
            "棋盘由黑格与白格组成",
            "黑格印有提示数字或斜线分隔",
            "白格为待填的数字格",
            "提示数分横向与纵向两类",
        ]
    }

    /// 填写规则
    pub fn filling(&self) -> Vec<&'static str> {
        vec![
            "每格只能填1到9的整数",
            "同一连续区块内数字不得重复",
            "区块数字之和须等于对应提示数",
            "不可跳过白格或跨黑格连接",
        ]
    }

    /// 解题技巧
    pub fn solving(&self) -> Vec<&'static str> {
        vec![
            "优先破解唯一组合的短区块",
            "利用交叉格同时满足横纵约束",
            "记录候选数字逐步排除",
            "遇矛盾时回溯检查前一步",
        ]
    }

    /// 完成判定
    pub fn verify(&self) -> Vec<&'static str> {
        vec![
            "所有白格均已填满",
            "每个区块之和与提示一致",
            "所有区块内无重复数字",
            "无违反规则的冲突格",
        ]
    }
}

impl Rule for KakuroCrossSumRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("kakuro")
    }

    fn explain(&self) -> String {
        format!(
            "【数谜（Kakuro）】\n{}",
            [
                format!(
                    "基本盘面：\\n{}",
                    self.board()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "填写规则：\\n{}",
                    self.filling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "解题技巧：\\n{}",
                    self.solving()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "完成判定：\\n{}",
                    self.verify()
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
    fn test_kakurocrosssumrules_basic() {
        let rules = KakuroCrossSumRules::new();
        assert_eq!(rules.metadata().name, "数谜（Kakuro）");
        assert!(!rules.board().is_empty());
        assert!(!rules.filling().is_empty());
        assert!(!rules.solving().is_empty());
        assert!(!rules.verify().is_empty());
    }

    #[test]
    fn test_kakurocrosssumrules_validation() {
        let rules = KakuroCrossSumRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("kakuro"));
    }

    #[test]
    fn test_kakurocrosssumrules_explain() {
        let rules = KakuroCrossSumRules::new();
        let e = rules.explain();
        assert!(e.contains("基本盘面"));
        assert!(e.contains("填写规则"));
        assert!(e.contains("解题技巧"));
    }
}
