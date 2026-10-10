//! 荨麻疹护理
//!
//! 风团瘙痒发作时的缓解与诱因规避

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HivesUrticariaCareRules,
    name: "荨麻疹护理",
    desc: "风团瘙痒发作时的缓解与诱因规避",
    origin: "中国",
    tags: ["皮肤", "过敏", "护理"]
}

impl HivesUrticariaCareRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "皮肤突发红色风团",
            "剧烈瘙痒时起时消",
            "单个风团多不超过一天",
            "可伴眼睑口唇肿胀",
        ]
    }

    /// 止痒处理
    pub fn relief(&self) -> Vec<&'static str> {
        vec![
            "冷敷缓解瘙痒",
            "避免热水烫洗",
            "勿抓挠防皮肤破损",
            "宽松棉质衣物减少摩擦",
        ]
    }

    /// 诱因规避
    pub fn avoidance(&self) -> Vec<&'static str> {
        vec![
            "回想近期可疑食物药物",
            "避免接触过敏原",
            "慎用新化妆品洗剂",
            "注意冷热与情绪诱因",
        ]
    }

    /// 就医警示
    pub fn warning(&self) -> Vec<&'static str> {
        vec![
            "呼吸困难须立即急诊",
            "喉头水肿有窒息风险",
            "反复发作应查过敏原",
            "遵医嘱规律用药",
        ]
    }
}

impl Rule for HivesUrticariaCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hives_urticaria_care")
    }

    fn explain(&self) -> String {
        format!(
            "【荨麻疹护理】\n{}",
            [
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "止痒处理：\\n{}",
                    self.relief()
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
                format!(
                    "就医警示：\\n{}",
                    self.warning()
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
    fn test_hivesurticariacarerules_basic() {
        let rules = HivesUrticariaCareRules::new();
        assert_eq!(rules.metadata().name, "荨麻疹护理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.relief().is_empty());
        assert!(!rules.avoidance().is_empty());
        assert!(!rules.warning().is_empty());
    }

    #[test]
    fn test_hivesurticariacarerules_validation() {
        let rules = HivesUrticariaCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("hives_urticaria_care")
        );
    }

    #[test]
    fn test_hivesurticariacarerules_explain() {
        let rules = HivesUrticariaCareRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("止痒处理"));
        assert!(e.contains("诱因规避"));
    }
}
