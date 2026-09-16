//! 宴会敬酒与祝词礼仪
//!
//! 宴席中举杯祝酒、致辞与敬酒次序的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BanquetToastEtiquetteRules,
    name: "宴会敬酒与祝词礼仪",
    desc: "宴席中举杯祝酒、致辞与敬酒次序的礼仪",
    origin: "中国",
    tags: ["社交", "礼仪", "宴会", "敬酒", "祝酒", "餐饮"]
}

impl BanquetToastEtiquetteRules {
    /// 举杯致辞
    pub fn toast(&self) -> Vec<&'static str> {
        vec![
            "举杯时面带微笑起身致意",
            "祝词简短得体不冗长",
            "照顾在场所有宾客",
            "不勉强不能喝酒者",
        ]
    }

    /// 敬酒次序
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "先敬长辈与主宾",
            "再依次敬在场各位",
            "受敬者举杯回敬",
            "不宜频繁离席互敬",
        ]
    }

    /// 饮酒分寸
    pub fn moderation(&self) -> Vec<&'static str> {
        vec![
            "饮酒适量不贪杯",
            "不劝酒不强行拼酒",
            "尊重不能饮酒者以茶代酒",
            "忌失态失言坏气氛",
        ]
    }

    /// 回应祝词
    pub fn respond(&self) -> Vec<&'static str> {
        vec![
            "被敬酒时起身回应",
            "表达对主人的谢意",
            "敬完主动回敬感谢",
            "宴毕向主人致谢",
        ]
    }
}

impl Rule for BanquetToastEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("banquet_toast")
    }

    fn explain(&self) -> String {
        format!(
            "【宴会敬酒与祝词礼仪】\n{}",
            [
                format!(
                    "举杯致辞：\\n{}",
                    self.toast()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "敬酒次序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮酒分寸：\\n{}",
                    self.moderation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回应祝词：\\n{}",
                    self.respond()
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
    fn test_banquettoastetiquetterules_basic() {
        let rules = BanquetToastEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "宴会敬酒与祝词礼仪");
        assert!(!rules.toast().is_empty());
        assert!(!rules.order().is_empty());
        assert!(!rules.moderation().is_empty());
        assert!(!rules.respond().is_empty());
    }

    #[test]
    fn test_banquettoastetiquetterules_validation() {
        let rules = BanquetToastEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("banquet_toast"));
    }

    #[test]
    fn test_banquettoastetiquetterules_explain() {
        let rules = BanquetToastEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("举杯致辞"));
        assert!(e.contains("敬酒次序"));
        assert!(e.contains("饮酒分寸"));
    }
}
