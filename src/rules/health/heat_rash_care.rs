//! 痱子护理
//!
//! 暑热出汗导致的痱子预防与护理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HeatRashCareRules,
    name: "痱子护理",
    desc: "暑热出汗导致的痱子预防与护理",
    origin: "中国",
    tags: ["皮肤", "暑热", "护理"]
}

impl HeatRashCareRules {
    /// 降温通风
    pub fn cooling(&self) -> Vec<&'static str> {
        vec![
            "保持环境凉爽通风",
            "空调风扇适度降温",
            "避免长时间暴晒",
            "减少剧烈活动出汗",
        ]
    }

    /// 衣着选择
    pub fn clothing(&self) -> Vec<&'static str> {
        vec![
            "选宽松透气棉质衣物",
            "勤换汗湿衣服",
            "避免紧身化纤面料",
            "婴儿勿包裹过厚",
        ]
    }

    /// 皮肤护理
    pub fn skin_care(&self) -> Vec<&'static str> {
        vec![
            "温水清洗后擦干",
            "保持皮肤干爽",
            "褶皱处尤为注意",
            "可用痱子粉薄涂",
        ]
    }

    /// 护理禁忌
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "勿用热水烫洗",
            "勿搔抓防感染",
            "勿厚涂油性护肤品",
            "脓痱须及时就医",
        ]
    }
}

impl Rule for HeatRashCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("heat_rash_care")
    }

    fn explain(&self) -> String {
        format!(
            "【痱子护理】\n{}",
            [
                format!(
                    "降温通风：\\n{}",
                    self.cooling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "衣着选择：\\n{}",
                    self.clothing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "皮肤护理：\\n{}",
                    self.skin_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "护理禁忌：\\n{}",
                    self.avoid()
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
    fn test_heatrashcarerules_basic() {
        let rules = HeatRashCareRules::new();
        assert_eq!(rules.metadata().name, "痱子护理");
        assert!(!rules.cooling().is_empty());
        assert!(!rules.clothing().is_empty());
        assert!(!rules.skin_care().is_empty());
        assert!(!rules.avoid().is_empty());
    }

    #[test]
    fn test_heatrashcarerules_validation() {
        let rules = HeatRashCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("heat_rash_care"));
    }

    #[test]
    fn test_heatrashcarerules_explain() {
        let rules = HeatRashCareRules::new();
        let e = rules.explain();
        assert!(e.contains("降温通风"));
        assert!(e.contains("衣着选择"));
        assert!(e.contains("皮肤护理"));
    }
}
