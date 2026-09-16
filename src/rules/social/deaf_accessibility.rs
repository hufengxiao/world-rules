//! 与听障人士沟通礼仪
//!
//! 与听力障碍者交流时尊重、清晰与辅助的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DeafAccessibilityEtiquetteRules,
    name: "与听障人士沟通礼仪",
    desc: "与听力障碍者交流时尊重、清晰与辅助的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "听障", "无障碍", "沟通"]
}

impl DeafAccessibilityEtiquetteRules {
    /// 发起沟通
    pub fn initiating(&self) -> Vec<&'static str> {
        vec![
            "进入对方视野再打招呼",
            "轻拍肩膀或挥动作引起注意",
            "确认对方注意后再开口",
            "不低估对方理解能力",
        ]
    }

    /// 表达清晰
    pub fn speaking(&self) -> Vec<&'static str> {
        vec![
            "面向对方便于读唇",
            "讲话口型自然语速适中",
            "必要用手势或书面辅助",
            "不明示意时用笔写文字",
        ]
    }

    /// 借助工具
    pub fn tools(&self) -> Vec<&'static str> {
        vec![
            "用手语时请专业翻译",
            "善用文字信息与字幕",
            "尊重对方偏好的交流方式",
            "遇到误读耐心重说",
        ]
    }

    /// 尊重包容
    pub fn respect(&self) -> Vec<&'static str> {
        vec![
            "不称呼歧视性词汇",
            "交谈时保持眼神与尊重",
            "不把对方当作无能力者",
            "经意间始终平等相待",
        ]
    }
}

impl Rule for DeafAccessibilityEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("deaf_access")
    }

    fn explain(&self) -> String {
        format!(
            "【与听障人士沟通礼仪】\n{}",
            [
                format!(
                    "发起沟通：\\n{}",
                    self.initiating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "表达清晰：\\n{}",
                    self.speaking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "借助工具：\\n{}",
                    self.tools()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "尊重包容：\\n{}",
                    self.respect()
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
    fn test_deafaccessibilityetiquetterules_basic() {
        let rules = DeafAccessibilityEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "与听障人士沟通礼仪");
        assert!(!rules.initiating().is_empty());
        assert!(!rules.speaking().is_empty());
        assert!(!rules.tools().is_empty());
        assert!(!rules.respect().is_empty());
    }

    #[test]
    fn test_deafaccessibilityetiquetterules_validation() {
        let rules = DeafAccessibilityEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("deaf_access"));
    }

    #[test]
    fn test_deafaccessibilityetiquetterules_explain() {
        let rules = DeafAccessibilityEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("发起沟通"));
        assert!(e.contains("表达清晰"));
        assert!(e.contains("借助工具"));
    }
}
