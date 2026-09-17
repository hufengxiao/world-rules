//! 规范洗手卫生
//!
//! 正确洗手、除菌与传染病预防的卫生习惯

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HandHygieneWashRules,
    name: "规范洗手卫生",
    desc: "正确洗手、除菌与传染病预防的卫生习惯",
    origin: "医学",
    tags: ["健康", "洗手", "卫生", "预防"]
}

impl HandHygieneWashRules {
    /// 洗手时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "饭前便后洗手",
            "接触脏物后洗手",
            "外出回家洗手",
            "生病前后洗手",
        ]
    }

    /// 正确手法
    pub fn washing(&self) -> Vec<&'static str> {
        vec!["流水冲湿加皂", "搓洗指缝掌心", "揉搓指甲指背", "冲洗后擦干"]
    }

    /// 消毒辅助
    pub fn sanitize(&self) -> Vec<&'static str> {
        vec!["无水处用免洗", "消毒液搓揉", "接触公共物后", "肥皂最稳"]
    }

    /// 习惯培养
    pub fn habit(&self) -> Vec<&'static str> {
        vec!["减少触摸面部", "不共用毛巾", "勤洗护健康", "全家养成习惯"]
    }
}

impl Rule for HandHygieneWashRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hand_hygiene")
    }

    fn explain(&self) -> String {
        format!(
            "【规范洗手卫生】\n{}",
            [
                format!(
                    "洗手时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确手法：\\n{}",
                    self.washing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "消毒辅助：\\n{}",
                    self.sanitize()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "习惯培养：\\n{}",
                    self.habit()
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
    fn test_handhygienewashrules_basic() {
        let rules = HandHygieneWashRules::new();
        assert_eq!(rules.metadata().name, "规范洗手卫生");
        assert!(!rules.timing().is_empty());
        assert!(!rules.washing().is_empty());
        assert!(!rules.sanitize().is_empty());
        assert!(!rules.habit().is_empty());
    }

    #[test]
    fn test_handhygienewashrules_validation() {
        let rules = HandHygieneWashRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("hand_hygiene"));
    }

    #[test]
    fn test_handhygienewashrules_explain() {
        let rules = HandHygieneWashRules::new();
        let e = rules.explain();
        assert!(e.contains("洗手时机"));
        assert!(e.contains("正确手法"));
        assert!(e.contains("消毒辅助"));
    }
}
