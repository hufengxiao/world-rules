//! 亲子宴与宝宝宴礼仪
//!
//! 迎接新生儿的庆祝聚会中送礼、观礼与参与礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BabyShowerEtiquetteRules,
    name: "亲子宴与宝宝宴礼仪",
    desc: "迎接新生儿的庆祝聚会中送礼、观礼与参与礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "宝宝", "亲子", "聚会"]
}

impl BabyShowerEtiquetteRules {
    /// 受邀请参与
    pub fn invitation(&self) -> Vec<&'static str> {
        vec![
            "收到邀请及时回复是否出席",
            "尊重活动形式与主题",
            "准时到场不影响流程",
            "了解是否需要带礼物",
        ]
    }

    /// 礼物与心意
    pub fn gifts(&self) -> Vec<&'static str> {
        vec![
            "可按家庭需要挑选礼物",
            "实用体贴为佳",
            "金额量力不必攀比",
            "写上祝福留言表达心意",
        ]
    }

    /// 与宝宝互动
    pub fn baby(&self) -> Vec<&'static str> {
        vec![
            "抱宝宝前保卫生并询问",
            "不强抱不亲吻",
            "不逗弄太过或惊吓",
            "照顾刚做父母者的疲劳",
        ]
    }

    /// 气氛与祝福
    pub fn blessing(&self) -> Vec<&'static str> {
        vec![
            "送上真诚的祝福",
            "拍照尊重隐私",
            "礼让主场的新手父母",
            "乐在工作氛围不喧宾夺主",
        ]
    }
}

impl Rule for BabyShowerEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("baby_shower")
    }

    fn explain(&self) -> String {
        format!(
            "【亲子宴与宝宝宴礼仪】\n{}",
            [
                format!(
                    "受邀请参与：\\n{}",
                    self.invitation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼物与心意：\\n{}",
                    self.gifts()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "与宝宝互动：\\n{}",
                    self.baby()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "气氛与祝福：\\n{}",
                    self.blessing()
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
    fn test_babyshoweretiquetterules_basic() {
        let rules = BabyShowerEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "亲子宴与宝宝宴礼仪");
        assert!(!rules.invitation().is_empty());
        assert!(!rules.gifts().is_empty());
        assert!(!rules.baby().is_empty());
        assert!(!rules.blessing().is_empty());
    }

    #[test]
    fn test_babyshoweretiquetterules_validation() {
        let rules = BabyShowerEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("baby_shower"));
    }

    #[test]
    fn test_babyshoweretiquetterules_explain() {
        let rules = BabyShowerEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("受邀请参与"));
        assert!(e.contains("礼物与心意"));
        assert!(e.contains("与宝宝互动"));
    }
}
