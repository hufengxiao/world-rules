//! 中暑处理
//!
//! 中暑识别、降温处理与预防

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HeatstrokeResponseRules,
    name: "中暑处理",
    desc: "中暑识别、降温处理与预防",
    origin: "医学",
    tags: ["健康", "中暑", "降温", "处理"]
}

impl HeatstrokeResponseRules {
    /// 识别症状
    pub fn symptom(&self) -> Vec<&'static str> {
        vec!["头昏恶心心慌", "大汗面色潮红", "体温升高", "意识模糊警惕"]
    }

    /// 就地处理
    pub fn treat(&self) -> Vec<&'static str> {
        vec![
            "移到阴凉通风处",
            "解开衣物散热",
            "补水适量少盐",
            "湿毛巾敷冷",
        ]
    }

    /// 重度就医
    pub fn visit(&self) -> Vec<&'static str> {
        vec!["高热不退就医", "意识不清急送", "抽搐及时处置", "不拖延"]
    }

    /// 预防中暑
    pub fn prevent(&self) -> Vec<&'static str> {
        vec!["高温少出门", "外出遮阳喝水", "老人儿童注意", "热天避暴晒"]
    }
}

impl Rule for HeatstrokeResponseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("heatstroke")
    }

    fn explain(&self) -> String {
        format!(
            "【中暑处理】\n{}",
            [
                format!(
                    "识别症状：\\n{}",
                    self.symptom()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就地处理：\\n{}",
                    self.treat()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "重度就医：\\n{}",
                    self.visit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防中暑：\\n{}",
                    self.prevent()
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
    fn test_heatstrokeresponserules_basic() {
        let rules = HeatstrokeResponseRules::new();
        assert_eq!(rules.metadata().name, "中暑处理");
        assert!(!rules.symptom().is_empty());
        assert!(!rules.treat().is_empty());
        assert!(!rules.visit().is_empty());
        assert!(!rules.prevent().is_empty());
    }

    #[test]
    fn test_heatstrokeresponserules_validation() {
        let rules = HeatstrokeResponseRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("heatstroke"));
    }

    #[test]
    fn test_heatstrokeresponserules_explain() {
        let rules = HeatstrokeResponseRules::new();
        let e = rules.explain();
        assert!(e.contains("识别症状"));
        assert!(e.contains("就地处理"));
        assert!(e.contains("重度就医"));
    }
}
