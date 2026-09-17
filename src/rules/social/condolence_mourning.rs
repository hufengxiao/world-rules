//! 吊唁慰问
//!
//! 参加吊唁、致哀与对丧家的慰问礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CondolenceMourningRules,
    name: "吊唁慰问",
    desc: "参加吊唁、致哀与对丧家的慰问礼仪",
    origin: "中国",
    tags: ["社交", "吊唁", "慰问", "丧事"]
}

impl CondolenceMourningRules {
    /// 吊唁礼节
    pub fn rite(&self) -> Vec<&'static str> {
        vec!["着装庄重朴素", "进灵堂肃穆", "依序致哀行礼", "尊重家属"]
    }

    /// 慰问言语
    pub fn words(&self) -> Vec<&'static str> {
        vec!["表达哀思", "说节哀话", "少说客套", "真诚关心"]
    }

    /// 协助帮扶
    pub fn help(&self) -> Vec<&'static str> {
        vec!["帮忙分担料理", "事后书信慰问", "给实际支持", "不添麻烦"]
    }

    /// 场合分寸
    pub fn decorum(&self) -> Vec<&'static str> {
        vec!["不大声喧哗", "面带沉容", "不取笑他人", "尊重氛围"]
    }
}

impl Rule for CondolenceMourningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("condolence")
    }

    fn explain(&self) -> String {
        format!(
            "【吊唁慰问】\n{}",
            [
                format!(
                    "吊唁礼节：\\n{}",
                    self.rite()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "慰问言语：\\n{}",
                    self.words()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "协助帮扶：\\n{}",
                    self.help()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场合分寸：\\n{}",
                    self.decorum()
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
    fn test_condolencemourningrules_basic() {
        let rules = CondolenceMourningRules::new();
        assert_eq!(rules.metadata().name, "吊唁慰问");
        assert!(!rules.rite().is_empty());
        assert!(!rules.words().is_empty());
        assert!(!rules.help().is_empty());
        assert!(!rules.decorum().is_empty());
    }

    #[test]
    fn test_condolencemourningrules_validation() {
        let rules = CondolenceMourningRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("condolence"));
    }

    #[test]
    fn test_condolencemourningrules_explain() {
        let rules = CondolenceMourningRules::new();
        let e = rules.explain();
        assert!(e.contains("吊唁礼节"));
        assert!(e.contains("慰问言语"));
        assert!(e.contains("协助帮扶"));
    }
}
