//! 找不同看图
//!
//! 在两幅近似图中找出细微差异的观察游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SpotDifferenceVisualRules,
    name: "找不同看图",
    desc: "在两幅近似图中找出细微差异的观察游戏",
    origin: "中国",
    tags: ["游戏", "观察", "找不同", "益智"]
}

impl SpotDifferenceVisualRules {
    /// 玩法目标
    pub fn goal(&self) -> Vec<&'static str> {
        vec![
            "比较两幅相近图",
            "找出所有不同点",
            "逐格对照",
            "全部找出即胜",
        ]
    }

    /// 观察方法
    pub fn observe(&self) -> Vec<&'static str> {
        vec![
            "从上到下逐行比",
            "注意颜色形状",
            "物体位置数量",
            "冷眼对比不马虎",
        ]
    }

    /// 技巧提示
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "重点看边缘细节",
            "大小角度差异",
            "局部放大细看",
            "快速扫再仔细核",
        ]
    }

    /// 收尾确认
    pub fn finish(&self) -> Vec<&'static str> {
        vec!["数量对得上", "圈出每处差异", "验收不漏点", "完成从容收手"]
    }
}

impl Rule for SpotDifferenceVisualRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("spot_difference")
    }

    fn explain(&self) -> String {
        format!(
            "【找不同看图】\n{}",
            [
                format!(
                    "玩法目标：\\n{}",
                    self.goal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观察方法：\\n{}",
                    self.observe()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "技巧提示：\\n{}",
                    self.technique()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "收尾确认：\\n{}",
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
    fn test_spotdifferencevisualrules_basic() {
        let rules = SpotDifferenceVisualRules::new();
        assert_eq!(rules.metadata().name, "找不同看图");
        assert!(!rules.goal().is_empty());
        assert!(!rules.observe().is_empty());
        assert!(!rules.technique().is_empty());
        assert!(!rules.finish().is_empty());
    }

    #[test]
    fn test_spotdifferencevisualrules_validation() {
        let rules = SpotDifferenceVisualRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("spot_difference"));
    }

    #[test]
    fn test_spotdifferencevisualrules_explain() {
        let rules = SpotDifferenceVisualRules::new();
        let e = rules.explain();
        assert!(e.contains("玩法目标"));
        assert!(e.contains("观察方法"));
        assert!(e.contains("技巧提示"));
    }
}
