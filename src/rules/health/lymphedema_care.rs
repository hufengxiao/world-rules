//! 淋巴水肿护理
//!
//! 淋巴水肿的日常护理规则，包括抬高患肢、皮肤保护、压力治疗与适度活动，帮助减轻肿胀并预防感染。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LymphedemaCareRules,
    name: "淋巴水肿护理",
    desc: "淋巴水肿的日常护理规则，包括抬高患肢、皮肤保护、压力治疗与适度活动，帮助减轻肿胀并预防感染。",
    origin: "康复医学与淋巴水肿专科护理建议",
    tags: ["健康", "淋巴", "水肿", "护理"]
}

impl LymphedemaCareRules {
    /// 肢体护理
    pub fn limb_care(&self) -> Vec<&'static str> {
        vec![
            "抬高患肢促进回流",
            "避免患肢长时间下垂",
            "穿宽松衣物不勒紧",
            "避免患肢测量血压或抽血",
        ]
    }

    /// 皮肤保护
    pub fn skin_care(&self) -> Vec<&'static str> {
        vec![
            "保持皮肤清洁滋润",
            "防止蚊虫叮咬与划伤",
            "伤口及时消毒处理",
            "留意红肿热痛感染信号",
        ]
    }

    /// 压力治疗
    pub fn compression(&self) -> Vec<&'static str> {
        vec![
            "遵医嘱穿戴压力袖套或袜",
            "白天佩戴夜间按需取下",
            "定期更换保持弹力",
            "配合专业淋巴引流按摩",
        ]
    }

    /// 活动与就医
    pub fn activity(&self) -> Vec<&'static str> {
        vec![
            "适度活动促进淋巴循环",
            "循序渐进不突然剧烈运动",
            "肿胀加重须及时就诊",
            "定期复查调整方案",
        ]
    }
}

impl Rule for LymphedemaCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("lymphedema")
    }

    fn explain(&self) -> String {
        format!(
            "【淋巴水肿护理】\n{}",
            [
                format!(
                    "肢体护理：\\n{}",
                    self.limb_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "皮肤保护：\\n{}",
                    self.skin_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "压力治疗：\\n{}",
                    self.compression()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "活动与就医：\\n{}",
                    self.activity()
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
    fn test_lymphedemacarerules_basic() {
        let rules = LymphedemaCareRules::new();
        assert_eq!(rules.metadata().name, "淋巴水肿护理");
        assert!(!rules.limb_care().is_empty());
        assert!(!rules.skin_care().is_empty());
        assert!(!rules.compression().is_empty());
        assert!(!rules.activity().is_empty());
    }

    #[test]
    fn test_lymphedemacarerules_validation() {
        let rules = LymphedemaCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("lymphedema"));
    }

    #[test]
    fn test_lymphedemacarerules_explain() {
        let rules = LymphedemaCareRules::new();
        let e = rules.explain();
        assert!(e.contains("肢体护理"));
        assert!(e.contains("皮肤保护"));
        assert!(e.contains("压力治疗"));
    }
}
