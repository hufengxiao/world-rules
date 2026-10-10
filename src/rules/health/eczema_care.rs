//! 湿疹护理
//!
//! 湿疹皮肤的保湿、止痒与刺激规避

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EczemaCareRules,
    name: "湿疹护理",
    desc: "湿疹皮肤的保湿、止痒与刺激规避",
    origin: "中国",
    tags: ["皮肤", "慢性", "护理"]
}

impl EczemaCareRules {
    /// 保湿基础
    pub fn moisturizing(&self) -> Vec<&'static str> {
        vec![
            "每日多次涂抹保湿霜",
            "洗澡后三分钟内涂",
            "选无香料温和产品",
            "保湿是护理核心",
        ]
    }

    /// 清洁注意
    pub fn cleaning(&self) -> Vec<&'static str> {
        vec![
            "温水短时淋浴",
            "少用碱性皂类",
            "避免搓澡用力擦",
            "轻拍吸干勿摩擦",
        ]
    }

    /// 止痒防护
    pub fn itch_control(&self) -> Vec<&'static str> {
        vec![
            "剪短指甲防抓伤",
            "冷敷缓解瘙痒",
            "宽松棉质衣物",
            "夜间可戴棉手套",
        ]
    }

    /// 诱因规避
    pub fn avoidance(&self) -> Vec<&'static str> {
        vec![
            "避免过热水与出汗刺激",
            "远离羊毛化纤贴身衣物",
            "留意食物与环境诱因",
            "减少精神紧张",
        ]
    }
}

impl Rule for EczemaCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("eczema_care")
    }

    fn explain(&self) -> String {
        format!(
            "【湿疹护理】\n{}",
            [
                format!(
                    "保湿基础：\\n{}",
                    self.moisturizing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "清洁注意：\\n{}",
                    self.cleaning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "止痒防护：\\n{}",
                    self.itch_control()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "诱因规避：\\n{}",
                    self.avoidance()
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
    fn test_eczemacarerules_basic() {
        let rules = EczemaCareRules::new();
        assert_eq!(rules.metadata().name, "湿疹护理");
        assert!(!rules.moisturizing().is_empty());
        assert!(!rules.cleaning().is_empty());
        assert!(!rules.itch_control().is_empty());
        assert!(!rules.avoidance().is_empty());
    }

    #[test]
    fn test_eczemacarerules_validation() {
        let rules = EczemaCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("eczema_care"));
    }

    #[test]
    fn test_eczemacarerules_explain() {
        let rules = EczemaCareRules::new();
        let e = rules.explain();
        assert!(e.contains("保湿基础"));
        assert!(e.contains("清洁注意"));
        assert!(e.contains("止痒防护"));
    }
}
