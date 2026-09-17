//! 产后恢复保健
//!
//! 产妇产后身体恢复、营养、情绪与照护规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PostpartumRecoveryRules,
    name: "产后恢复保健",
    desc: "产妇产后身体恢复、营养、情绪与照护规则",
    origin: "医学",
    tags: ["健康", "产后", "恢复", "产妇"]
}

impl PostpartumRecoveryRules {
    /// 身体恢复
    pub fn recovery(&self) -> Vec<&'static str> {
        vec![
            "产后充分休息",
            "循序渐进行活动",
            "留意恶露与伤口情况",
            "避免过早重体力",
        ]
    }

    /// 营养饮食
    pub fn nutrition(&self) -> Vec<&'static str> {
        vec![
            "均衡营养支持恢复",
            "保证蛋白质与水分",
            "适量蔬果防便秘",
            "哺乳者营养充足",
        ]
    }

    /// 情绪支持
    pub fn mood(&self) -> Vec<&'static str> {
        vec![
            "关注产后情绪变化",
            "坦诚表达疲惫感受",
            "家人给予理解陪伴",
            "情绪持续低落及时求助",
        ]
    }

    /// 安全就医
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "异常出血或红肿就医",
            "发热伤口感染及时处理",
            "按时产后复查",
            "遵医嘱恢复运动",
        ]
    }
}

impl Rule for PostpartumRecoveryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("postpartum")
    }

    fn explain(&self) -> String {
        format!(
            "【产后恢复保健】\n{}",
            [
                format!(
                    "身体恢复：\\n{}",
                    self.recovery()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "营养饮食：\\n{}",
                    self.nutrition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "情绪支持：\\n{}",
                    self.mood()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全就医：\\n{}",
                    self.care()
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
    fn test_postpartumrecoveryrules_basic() {
        let rules = PostpartumRecoveryRules::new();
        assert_eq!(rules.metadata().name, "产后恢复保健");
        assert!(!rules.recovery().is_empty());
        assert!(!rules.nutrition().is_empty());
        assert!(!rules.mood().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_postpartumrecoveryrules_validation() {
        let rules = PostpartumRecoveryRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("postpartum"));
    }

    #[test]
    fn test_postpartumrecoveryrules_explain() {
        let rules = PostpartumRecoveryRules::new();
        let e = rules.explain();
        assert!(e.contains("身体恢复"));
        assert!(e.contains("营养饮食"));
        assert!(e.contains("情绪支持"));
    }
}
