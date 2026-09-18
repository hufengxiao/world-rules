//! 数织逻辑涂格
//!
//! 按行列数字提示涂色格子还原图案的逻辑益智

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NonogramLogicArtRules,
    name: "数织逻辑涂格",
    desc: "按行列数字提示涂色格子还原图案的逻辑益智",
    origin: "日本",
    tags: ["游戏", "数织", "逻辑", "益智"]
}

impl NonogramLogicArtRules {
    /// 规则如础
    pub fn basic(&self) -> Vec<&'static str> {
        vec![
            "网格行列提示数",
            "数字表示连续涂格",
            "空格分隔各段",
            "涂满隐性图案胜",
        ]
    }

    /// 推断方法
    pub fn deduce(&self) -> Vec<&'static str> {
        vec![
            "大数先定位中心",
            "行列交叉锁定",
            "数目对格数算",
            "空白标记排除",
        ]
    }

    /// 求解技巧
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "从数字满行入手",
            "逐步交叉填涂",
            "先定必然格",
            "再处理不确定",
        ]
    }

    /// 收尾检查
    pub fn finish(&self) -> Vec<&'static str> {
        vec![
            "行列计数相符",
            "无矛盾冲突",
            "校验每段连续",
            "完成后图案显现",
        ]
    }
}

impl Rule for NonogramLogicArtRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("nonogram")
    }

    fn explain(&self) -> String {
        format!(
            "【数织逻辑涂格】\n{}",
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
                    "推断方法：\\n{}",
                    self.deduce()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "求解技巧：\\n{}",
                    self.technique()
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
    fn test_nonogramlogicartrules_basic() {
        let rules = NonogramLogicArtRules::new();
        assert_eq!(rules.metadata().name, "数织逻辑涂格");
        assert!(!rules.basic().is_empty());
        assert!(!rules.deduce().is_empty());
        assert!(!rules.technique().is_empty());
        assert!(!rules.finish().is_empty());
    }

    #[test]
    fn test_nonogramlogicartrules_validation() {
        let rules = NonogramLogicArtRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("nonogram"));
    }

    #[test]
    fn test_nonogramlogicartrules_explain() {
        let rules = NonogramLogicArtRules::new();
        let e = rules.explain();
        assert!(e.contains("规则如础"));
        assert!(e.contains("推断方法"));
        assert!(e.contains("求解技巧"));
    }
}
