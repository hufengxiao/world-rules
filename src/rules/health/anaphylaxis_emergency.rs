//! 过敏性休克急救
//!
//! 严重过敏反应的识别与紧急处置

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AnaphylaxisEmergencyRules,
    name: "过敏性休克急救",
    desc: "严重过敏反应的识别与紧急处置",
    origin: "中国",
    tags: ["急救", "过敏", "应急"]
}

impl AnaphylaxisEmergencyRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "皮肤荨麻疹瘙痒",
            "喉头水肿呼吸困难",
            "血压骤降头晕",
            "恶心呕吐腹痛",
        ]
    }

    /// 紧急处置
    pub fn emergency(&self) -> Vec<&'static str> {
        vec![
            "立即拨打一二零",
            "有肾上腺素笔立即使用",
            "让患者平卧抬腿",
            "保持气道通畅",
        ]
    }

    /// 体位处理
    pub fn position(&self) -> Vec<&'static str> {
        vec![
            "呼吸困难者半坐位",
            "血压低者平卧抬高下肢",
            "呕吐者侧卧防窒息",
            "不可让患者站立行走",
        ]
    }

    /// 后续注意
    pub fn aftercare(&self) -> Vec<&'static str> {
        vec![
            "症状缓解仍须就医",
            "记录过敏原",
            "随身携带急救药物",
            "告知亲友过敏史",
        ]
    }
}

impl Rule for AnaphylaxisEmergencyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("anaphylaxis_emergency")
    }

    fn explain(&self) -> String {
        format!(
            "【过敏性休克急救】\n{}",
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
                    "紧急处置：\\n{}",
                    self.emergency()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体位处理：\\n{}",
                    self.position()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "后续注意：\\n{}",
                    self.aftercare()
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
    fn test_anaphylaxisemergencyrules_basic() {
        let rules = AnaphylaxisEmergencyRules::new();
        assert_eq!(rules.metadata().name, "过敏性休克急救");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.emergency().is_empty());
        assert!(!rules.position().is_empty());
        assert!(!rules.aftercare().is_empty());
    }

    #[test]
    fn test_anaphylaxisemergencyrules_validation() {
        let rules = AnaphylaxisEmergencyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("anaphylaxis_emergency")
        );
    }

    #[test]
    fn test_anaphylaxisemergencyrules_explain() {
        let rules = AnaphylaxisEmergencyRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("紧急处置"));
        assert!(e.contains("体位处理"));
    }
}
