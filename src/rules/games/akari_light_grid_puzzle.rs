//! 点灯益智
//!
//! 在黑格数字限制下放置灯泡照亮全盘的点灯谜题

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AkariLightGridPuzzleRules,
    name: "点灯益智",
    desc: "在黑格数字限制下放置灯泡照亮全盘的点灯谜题",
    origin: "日本",
    tags: ["游戏", "点灯", "逻辑", "益智"]
}

impl AkariLightGridPuzzleRules {
    /// 规则如础
    pub fn basic(&self) -> Vec<&'static str> {
        vec![
            "白格可放灯泡",
            "光沿行列放射",
            "照到黑格为止",
            "全格点亮清朗胜",
        ]
    }

    /// 黑格约束
    pub fn blackbox(&self) -> Vec<&'static str> {
        vec![
            "黑格数字相邻灯泡数",
            "零邻格无灯",
            "一至四限邻格数",
            "余黑格无限制",
        ]
    }

    /// 放置原则
    pub fn place(&self) -> Vec<&'static str> {
        vec![
            "灯泡不能互相照见",
            "同行列隔黑不干扰",
            "数字格旁先定",
            "逐步点亮所有格",
        ]
    }

    /// 收尾检查
    pub fn finish(&self) -> Vec<&'static str> {
        vec![
            "每白格被照亮",
            "约束数字全满足",
            "灯不相照验证",
            "全盘亮起完成",
        ]
    }
}

impl Rule for AkariLightGridPuzzleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("akari_light")
    }

    fn explain(&self) -> String {
        format!(
            "【点灯益智】\n{}",
            [
                format!(
                    "规则如础：\\n{}",
                    self.basic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "黑格约束：\\n{}",
                    self.blackbox()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "放置原则：\\n{}",
                    self.place()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "收尾检查：\\n{}",
                    self.finish()
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
    fn test_akarilightgridpuzzlerules_basic() {
        let rules = AkariLightGridPuzzleRules::new();
        assert_eq!(rules.metadata().name, "点灯益智");
        assert!(!rules.basic().is_empty());
        assert!(!rules.blackbox().is_empty());
        assert!(!rules.place().is_empty());
        assert!(!rules.finish().is_empty());
    }

    #[test]
    fn test_akarilightgridpuzzlerules_validation() {
        let rules = AkariLightGridPuzzleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("akari_light"));
    }

    #[test]
    fn test_akarilightgridpuzzlerules_explain() {
        let rules = AkariLightGridPuzzleRules::new();
        let e = rules.explain();
        assert!(e.contains("规则如础"));
        assert!(e.contains("黑格约束"));
        assert!(e.contains("放置原则"));
    }
}
