//! 吊唁慰问礼仪
//!
//! 遭丧事时的吊唁、慰问与表达哀思之礼

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CondolenceVisitRules,
    name: "吊唁慰问礼仪",
    desc: "遭丧事时的吊唁、慰问与表达哀思之礼",
    origin: "中国",
    tags: ["社交", "礼仪", "吊唁", "慰问"]
}

impl CondolenceVisitRules {
    /// 表达心意
    pub fn meaning(&self) -> Vec<&'static str> {
        vec![
            "适时表达诚挚哀悼",
            "尊重家属情绪",
            "以行动支持陪伴",
            "提供需要帮助",
        ]
    }

    /// 吊唁方式
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "依当地礼节致意",
            "穿着庄重素净",
            "言语简洁真挚",
            "不喧哗高声",
        ]
    }

    /// 慰问言辞
    pub fn words(&self) -> Vec<&'static str> {
        vec![
            "多说陪伴与关心",
            "避免空洞安慰与闲谈",
            "少追问具体细节",
            "尊重家属意愿",
        ]
    }

    /// 后续关怀
    pub fn follow(&self) -> Vec<&'static str> {
        vec![
            "事后给予持续支持",
            "帮助处理实际事务",
            "尊重悲伤周期",
            "有需要稳定陪伴",
        ]
    }
}

impl Rule for CondolenceVisitRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("condolence")
    }

    fn explain(&self) -> String {
        format!(
            "【吊唁慰问礼仪】\n{}",
            [
                format!(
                    "表达心意：\\n{}",
                    self.meaning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "吊唁方式：\\n{}",
                    self.manner()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "慰问言辞：\\n{}",
                    self.words()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "后续关怀：\\n{}",
                    self.follow()
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
    fn test_condolencevisitrules_basic() {
        let rules = CondolenceVisitRules::new();
        assert_eq!(rules.metadata().name, "吊唁慰问礼仪");
        assert!(!rules.meaning().is_empty());
        assert!(!rules.manner().is_empty());
        assert!(!rules.words().is_empty());
        assert!(!rules.follow().is_empty());
    }

    #[test]
    fn test_condolencevisitrules_validation() {
        let rules = CondolenceVisitRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("condolence"));
    }

    #[test]
    fn test_condolencevisitrules_explain() {
        let rules = CondolenceVisitRules::new();
        let e = rules.explain();
        assert!(e.contains("表达心意"));
        assert!(e.contains("吊唁方式"));
        assert!(e.contains("慰问言辞"));
    }
}
