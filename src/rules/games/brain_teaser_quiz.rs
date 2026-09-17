//! 脑筋急转弯
//!
//! 脑筋急转弯的思维反常识与问答乐趣

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BrainTeaserQuizRules,
    name: "脑筋急转弯",
    desc: "脑筋急转弯的思维反常识与问答乐趣",
    origin: "中国",
    tags: ["游戏", "脑筋急转弯", "益智"]
}

impl BrainTeaserQuizRules {
    /// 出题
    pub fn ask(&self) -> Vec<&'static str> {
        vec!["题目清晰不绕弄", "答案合理有趣", "难度适中", "不冷门刁钻"]
    }

    /// 换思维
    pub fn think(&self) -> Vec<&'static str> {
        vec!["跳出常规联想", "多角度想", "注意字面陷阱", "放松头脑灵活"]
    }

    /// 答问规则
    pub fn answer(&self) -> Vec<&'static str> {
        vec!["猜中即答对", "给提示再答", "无人猜由揭晓", "不抢不催"]
    }

    /// 趣味氛围
    pub fn fun(&self) -> Vec<&'static str> {
        vec!["天真轻松快乐", "不骂笨", "一起开怀", "智商不指责"]
    }
}

impl Rule for BrainTeaserQuizRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("brain_teaser")
    }

    fn explain(&self) -> String {
        format!(
            "【脑筋急转弯】\n{}",
            [
                format!(
                    "出题：\\n{}",
                    self.ask()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "换思维：\\n{}",
                    self.think()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "答问规则：\\n{}",
                    self.answer()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "趣味氛围：\\n{}",
                    self.fun()
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
    fn test_brainteaserquizrules_basic() {
        let rules = BrainTeaserQuizRules::new();
        assert_eq!(rules.metadata().name, "脑筋急转弯");
        assert!(!rules.ask().is_empty());
        assert!(!rules.think().is_empty());
        assert!(!rules.answer().is_empty());
        assert!(!rules.fun().is_empty());
    }

    #[test]
    fn test_brainteaserquizrules_validation() {
        let rules = BrainTeaserQuizRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("brain_teaser"));
    }

    #[test]
    fn test_brainteaserquizrules_explain() {
        let rules = BrainTeaserQuizRules::new();
        let e = rules.explain();
        assert!(e.contains("出题"));
        assert!(e.contains("换思维"));
        assert!(e.contains("答问规则"));
    }
}
