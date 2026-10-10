//! 献血礼仪
//!
//! 参与无偿献血的礼节与配合规则，包括现场秩序、如实告知健康状况、配合采血与献血后休养，营造友善的公益氛围。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BloodDonationEtiquetteRules,
    name: "献血礼仪",
    desc: "参与无偿献血的礼节与配合规则，包括现场秩序、如实告知健康状况、配合采血与献血后休养，营造友善的公益氛围。",
    origin: "无偿献血公益活动中的常见场景",
    tags: ["社交", "公益", "献血", "礼仪"]
}

impl BloodDonationEtiquetteRules {
    /// 现场秩序
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "按号排队耐心等候",
            "保持安静不喧哗",
            "听从工作人员指引",
            "不插队不催促",
        ]
    }

    /// 健康告知
    pub fn disclosure(&self) -> Vec<&'static str> {
        vec![
            "如实填写健康状况",
            "不隐瞒疾病与用药史",
            "近期不适主动说明",
            "对自身与他人负责",
        ]
    }

    /// 采血配合
    pub fn cooperation(&self) -> Vec<&'static str> {
        vec![
            "放松心情配合操作",
            "穿刺时保持手臂不动",
            "不适及时告知医护",
            "献完按压止血片刻",
        ]
    }

    /// 献血之后
    pub fn aftercare(&self) -> Vec<&'static str> {
        vec![
            "静坐休息再起身",
            "当日避免剧烈运动",
            "适当补充水分营养",
            "对医护人员道谢",
        ]
    }
}

impl Rule for BloodDonationEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("blood_donation")
    }

    fn explain(&self) -> String {
        format!(
            "【献血礼仪】\n{}",
            [
                format!(
                    "现场秩序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健康告知：\\n{}",
                    self.disclosure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "采血配合：\\n{}",
                    self.cooperation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "献血之后：\\n{}",
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
    fn test_blooddonationetiquetterules_basic() {
        let rules = BloodDonationEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "献血礼仪");
        assert!(!rules.order().is_empty());
        assert!(!rules.disclosure().is_empty());
        assert!(!rules.cooperation().is_empty());
        assert!(!rules.aftercare().is_empty());
    }

    #[test]
    fn test_blooddonationetiquetterules_validation() {
        let rules = BloodDonationEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("blood_donation"));
    }

    #[test]
    fn test_blooddonationetiquetterules_explain() {
        let rules = BloodDonationEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("现场秩序"));
        assert!(e.contains("健康告知"));
        assert!(e.contains("采血配合"));
    }
}
