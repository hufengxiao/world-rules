//! 尿路感染护理
//!
//! 尿路感染的日常护理与预防规则，包括多饮水、及时排尿、注意卫生与识别就医信号，帮助减轻症状并防止复发。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: UrinaryTractInfectionCareRules,
    name: "尿路感染护理",
    desc: "尿路感染的日常护理与预防规则，包括多饮水、及时排尿、注意卫生与识别就医信号，帮助减轻症状并防止复发。",
    origin: "泌尿科临床护理与公共卫生建议",
    tags: ["健康", "泌尿", "感染", "护理"]
}

impl UrinaryTractInfectionCareRules {
    /// 日常饮水
    pub fn hydration(&self) -> Vec<&'static str> {
        vec![
            "每日足量饮水稀释尿液",
            "勤排尿不憋尿",
            "排尿有助冲刷尿道细菌",
            "避免长时间缺水",
        ]
    }

    /// 卫生习惯
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec![
            "如厕后由前向后擦拭",
            "保持会阴清洁干燥",
            "勤换内裤选棉质透气",
            "性生活前后注意清洁",
        ]
    }

    /// 生活调整
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "避免久坐与过度劳累",
            "少喝浓茶咖啡与酒",
            "注意下身保暖",
            "规律作息增强免疫",
        ]
    }

    /// 就医信号
    pub fn medical(&self) -> Vec<&'static str> {
        vec![
            "尿频尿急尿痛明显",
            "尿液浑浊或有血",
            "腰痛发热寒战",
            "症状反复或加重须就医",
        ]
    }
}

impl Rule for UrinaryTractInfectionCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("uti_care")
    }

    fn explain(&self) -> String {
        format!(
            "【尿路感染护理】\n{}",
            [
                format!(
                    "日常饮水：\\n{}",
                    self.hydration()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "卫生习惯：\\n{}",
                    self.hygiene()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活调整：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医信号：\\n{}",
                    self.medical()
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
    fn test_urinarytractinfectioncarerules_basic() {
        let rules = UrinaryTractInfectionCareRules::new();
        assert_eq!(rules.metadata().name, "尿路感染护理");
        assert!(!rules.hydration().is_empty());
        assert!(!rules.hygiene().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.medical().is_empty());
    }

    #[test]
    fn test_urinarytractinfectioncarerules_validation() {
        let rules = UrinaryTractInfectionCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("uti_care"));
    }

    #[test]
    fn test_urinarytractinfectioncarerules_explain() {
        let rules = UrinaryTractInfectionCareRules::new();
        let e = rules.explain();
        assert!(e.contains("日常饮水"));
        assert!(e.contains("卫生习惯"));
        assert!(e.contains("生活调整"));
    }
}
