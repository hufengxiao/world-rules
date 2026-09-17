//! 捐赠与义卖礼仪
//!
//! 公益捐赠、义卖、募捐中的诚意与规范礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CharityDonationEtiquetteRules,
    name: "捐赠与义卖礼仪",
    desc: "公益捐赠、义卖、募捐中的诚意与规范礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "捐赠", "公益", "义卖"]
}

impl CharityDonationEtiquetteRules {
    /// 诚意捐赠
    pub fn sincerity(&self) -> Vec<&'static str> {
        vec![
            "出于善意量力而为",
            "选正规的受助渠道",
            "不借捐赠抬高炫耀",
            "尊重受助者的尊严",
        ]
    }

    /// 物捐清洁
    pub fn goods(&self) -> Vec<&'static str> {
        vec![
            "收拾干净的用旧衣物",
            "破损严重的物件预览说明",
            "食品注意保质期",
            "写明物品用途与原状",
        ]
    }

    /// 参与活动
    pub fn event(&self) -> Vec<&'static str> {
        vec![
            "遵守义卖集会的秩序",
            "自愿参与不强制摊派",
            "关注善款用途透明",
            "传播公益理性不夸大",
        ]
    }

    /// 善用资源
    pub fn accountability(&self) -> Vec<&'static str> {
        vec![
            "了解款项去向",
            "合理分配有限的精力",
            "尊重自愿参与者的选择",
            "不道德绑架他人捐助",
        ]
    }
}

impl Rule for CharityDonationEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("charity_donation")
    }

    fn explain(&self) -> String {
        format!(
            "【捐赠与义卖礼仪】\n{}",
            [
                format!(
                    "诚意捐赠：\\n{}",
                    self.sincerity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "物捐清洁：\\n{}",
                    self.goods()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "参与活动：\\n{}",
                    self.event()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "善用资源：\\n{}",
                    self.accountability()
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
    fn test_charitydonationetiquetterules_basic() {
        let rules = CharityDonationEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "捐赠与义卖礼仪");
        assert!(!rules.sincerity().is_empty());
        assert!(!rules.goods().is_empty());
        assert!(!rules.event().is_empty());
        assert!(!rules.accountability().is_empty());
    }

    #[test]
    fn test_charitydonationetiquetterules_validation() {
        let rules = CharityDonationEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("charity_donation"));
    }

    #[test]
    fn test_charitydonationetiquetterules_explain() {
        let rules = CharityDonationEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("诚意捐赠"));
        assert!(e.contains("物捐清洁"));
        assert!(e.contains("参与活动"));
    }
}
