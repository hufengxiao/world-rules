//! 耳痛与护耳
//!
//! 耳痛耳炎的识别、止护与就医护理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EarAcheCareRules,
    name: "耳痛与护耳",
    desc: "耳痛耳炎的识别、止护与就医护理规则",
    origin: "医学",
    tags: ["健康", "耳朵", "耳痛", "护理"]
}

impl EarAcheCareRules {
    /// 识别表症
    pub fn signs(&self) -> Vec<&'static str> {
        vec![
            "关注耳痛耳闷不适",
            "留意是否有分泌物",
            "注意听力变化",
            "儿童耳痛及时留意",
        ]
    }

    /// 居家安抚
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "避免用硬物掏耳",
            "保持耳道干燥清洁",
            "洗头游泳防进水",
            "疼痛可温热敷外耳",
        ]
    }

    /// 用药谨慎
    pub fn medication(&self) -> Vec<&'static str> {
        vec![
            "不擅自滴入不明药水",
            "耳部用药遵医嘱",
            "勿用力擤鼻加重",
            "症状明显就医",
        ]
    }

    /// 就医判断
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "耳痛持续或加重就医",
            "流脓血速就医",
            "听力明显下降检查",
            "儿童或老人耳痛重视",
        ]
    }
}

impl Rule for EarAcheCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("ear_ache")
    }

    fn explain(&self) -> String {
        format!(
            "【耳痛与护耳】\n{}",
            [
                format!(
                    "识别表症：\\n{}",
                    self.signs()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "居家安抚：\\n{}",
                    self.relief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用药谨慎：\\n{}",
                    self.medication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医判断：\\n{}",
                    self.seek()
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
    fn test_earachecarerules_basic() {
        let rules = EarAcheCareRules::new();
        assert_eq!(rules.metadata().name, "耳痛与护耳");
        assert!(!rules.signs().is_empty());
        assert!(!rules.relief().is_empty());
        assert!(!rules.medication().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_earachecarerules_validation() {
        let rules = EarAcheCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("ear_ache"));
    }

    #[test]
    fn test_earachecarerules_explain() {
        let rules = EarAcheCareRules::new();
        let e = rules.explain();
        assert!(e.contains("识别表症"));
        assert!(e.contains("居家安抚"));
        assert!(e.contains("用药谨慎"));
    }
}
