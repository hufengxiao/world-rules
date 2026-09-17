//! 增强免疫生活方式
//!
//! 通过作息、营养与运动支持免疫功能的健康规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ImmuneSupportLifestyleRules,
    name: "增强免疫生活方式",
    desc: "通过作息、营养与运动支持免疫功能的健康规则",
    origin: "国际",
    tags: ["健康", "免疫", "生活方式", "增强"]
}

impl ImmuneSupportLifestyleRules {
    /// 作息睡眠
    pub fn rest(&self) -> Vec<&'static str> {
        vec![
            "保证充足规律的睡眠",
            "睡眠不足会削弱免疫",
            "规律作息稳定节律",
            "适量小憩不误夜间",
        ]
    }

    /// 营养支持
    pub fn nutrition(&self) -> Vec<&'static str> {
        vec![
            "均衡摄取各类营养素",
            "足量果蔬补充抗氧化",
            "确保蛋白与锌摄入",
            "充足水分",
        ]
    }

    /// 适度运动
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "坚持适度规律运动",
            "不过度训练损害免疫",
            "运动后注意恢复",
            "结合个人体能循序渐进",
        ]
    }

    /// 压力与习惯
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "管理压力与情绪",
            "戒烟少酒",
            "保持良好卫生习惯",
            "必要时接种并遵医嘱",
        ]
    }
}

impl Rule for ImmuneSupportLifestyleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("immune_support")
    }

    fn explain(&self) -> String {
        format!(
            "【增强免疫生活方式】\n{}",
            [
                format!(
                    "作息睡眠：\\n{}",
                    self.rest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "营养支持：\\n{}",
                    self.nutrition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "适度运动：\\n{}",
                    self.exercise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "压力与习惯：\\n{}",
                    self.lifestyle()
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
    fn test_immunesupportlifestylerules_basic() {
        let rules = ImmuneSupportLifestyleRules::new();
        assert_eq!(rules.metadata().name, "增强免疫生活方式");
        assert!(!rules.rest().is_empty());
        assert!(!rules.nutrition().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.lifestyle().is_empty());
    }

    #[test]
    fn test_immunesupportlifestylerules_validation() {
        let rules = ImmuneSupportLifestyleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("immune_support"));
    }

    #[test]
    fn test_immunesupportlifestylerules_explain() {
        let rules = ImmuneSupportLifestyleRules::new();
        let e = rules.explain();
        assert!(e.contains("作息睡眠"));
        assert!(e.contains("营养支持"));
        assert!(e.contains("适度运动"));
    }
}
