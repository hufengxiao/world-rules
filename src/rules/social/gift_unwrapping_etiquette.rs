//! 拆收礼物礼仪
//!
//! 收受礼物时开启、致谢与体面的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GiftUnwrappingEtiquetteRules,
    name: "拆收礼物礼仪",
    desc: "收受礼物时开启、致谢与体面的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "礼物", "致谢"]
}

impl GiftUnwrappingEtiquetteRules {
    /// 致谢表达
    pub fn thanks(&self) -> Vec<&'static str> {
        vec![
            "收到礼物当场致谢",
            "由衷感谢对方的用心",
            "提及具体的喜爱之处",
            "赞美可选真诚不夸张",
        ]
    }

    /// 拆礼方式
    pub fn open(&self) -> Vec<&'static str> {
        vec![
            "视场合当场或会后拆",
            "小心拆开不撕坏包装",
            "贴好卡片与包装",
            "开封后妥善收纳礼物",
        ]
    }

    /// 体谅回应
    pub fn respond(&self) -> Vec<&'static str> {
        vec![
            "不合适礼物仍婉言致谢",
            "不直接表达失望",
            "后续可礼貌沟通偏好",
            "不辜负赠送者的心意",
        ]
    }

    /// 礼尚往来
    pub fn reciprocity(&self) -> Vec<&'static str> {
        vec![
            "记得回礼或答谢",
            "心意重于价值",
            "把握时机回赠",
            "珍惜彼此情谊",
        ]
    }
}

impl Rule for GiftUnwrappingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("gift_unwrapping")
    }

    fn explain(&self) -> String {
        format!(
            "【拆收礼物礼仪】\n{}",
            [
                format!(
                    "致谢表达：\\n{}",
                    self.thanks()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "拆礼方式：\\n{}",
                    self.open()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体谅回应：\\n{}",
                    self.respond()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼尚往来：\\n{}",
                    self.reciprocity()
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
    fn test_giftunwrappingetiquetterules_basic() {
        let rules = GiftUnwrappingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "拆收礼物礼仪");
        assert!(!rules.thanks().is_empty());
        assert!(!rules.open().is_empty());
        assert!(!rules.respond().is_empty());
        assert!(!rules.reciprocity().is_empty());
    }

    #[test]
    fn test_giftunwrappingetiquetterules_validation() {
        let rules = GiftUnwrappingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("gift_unwrapping"));
    }

    #[test]
    fn test_giftunwrappingetiquetterules_explain() {
        let rules = GiftUnwrappingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("致谢表达"));
        assert!(e.contains("拆礼方式"));
        assert!(e.contains("体谅回应"));
    }
}
