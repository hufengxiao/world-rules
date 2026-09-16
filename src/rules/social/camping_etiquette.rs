//! 露营礼仪
//!
//! 露营营地选址、篝火、噪音与环保的公共礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CampingEtiquetteRules,
    name: "露营礼仪",
    desc: "露营营地选址、篝火、噪音与环保的公共礼仪",
    origin: "国际",
    tags: ["社交", "户外", "露营", "礼仪"]
}

impl CampingEtiquetteRules {
    /// 营地选址
    pub fn site(&self) -> Vec<&'static str> {
        vec![
            "在指定营地扎营",
            "避开陡坡与低洼易涝地",
            "与邻居保持合理距离",
            "不在禁露营区过夜",
        ]
    }

    /// 篝火使用
    pub fn fire(&self) -> Vec<&'static str> {
        vec![
            "只在允许的地方用火",
            "用火期间有人看守",
            "离营前彻底熄灭",
            "遵守防火期规定",
        ]
    }

    /// 噪音管理
    pub fn noise(&self) -> Vec<&'static str> {
        vec![
            "夜间降低说话音量",
            "避免过早过晚喧闹",
            "音乐音量不越营地范围",
            "尊重他人安静休息",
        ]
    }

    /// 营地整洁
    pub fn cleanliness(&self) -> Vec<&'static str> {
        vec![
            "离营带走全部物品",
            "厨余垃圾独立收理",
            "不惊扰野生动物与植被",
        ]
    }
}

impl Rule for CampingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("camping")
    }

    fn explain(&self) -> String {
        format!(
            "【露营礼仪】\n{}",
            [
                format!(
                    "营地选址：\\n{}",
                    self.site()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "篝火使用：\\n{}",
                    self.fire()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "噪音管理：\\n{}",
                    self.noise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "营地整洁：\\n{}",
                    self.cleanliness()
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
    fn test_campingetiquetterules_basic() {
        let rules = CampingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "露营礼仪");
        assert!(!rules.site().is_empty());
        assert!(!rules.fire().is_empty());
        assert!(!rules.noise().is_empty());
        assert!(!rules.cleanliness().is_empty());
    }

    #[test]
    fn test_campingetiquetterules_validation() {
        let rules = CampingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("camping"));
    }

    #[test]
    fn test_campingetiquetterules_explain() {
        let rules = CampingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("营地选址"));
        assert!(e.contains("篝火使用"));
        assert!(e.contains("噪音管理"));
    }
}
