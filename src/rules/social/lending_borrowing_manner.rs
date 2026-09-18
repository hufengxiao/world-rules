//! 借物借钱的分寸
//!
//! 向人借钱借物以及被借时的讲究与边界

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LendingBorrowingMannerRules,
    name: "借物借钱的分寸",
    desc: "向人借钱借物以及被借时的讲究与边界",
    origin: "中国",
    tags: ["社交", "借钱", "借物", "人情"]
}

impl LendingBorrowingMannerRules {
    /// 开口借钱
    pub fn borrow(&self) -> Vec<&'static str> {
        vec![
            "先说清数额用途",
            "讲明归还期限",
            "写借条留凭证",
            "救急不救穷",
        ]
    }

    /// 借物规矩
    pub fn borrow_item(&self) -> Vec<&'static str> {
        vec!["借前说明用途", "按时归还", "原样奉还", "损坏主动赔偿"]
    }

    /// 被借回应
    pub fn lend(&self) -> Vec<&'static str> {
        vec!["量力而行", "借出防赖账留条", "不想借委婉拒", "不伤和气"]
    }

    /// 还钱礼貌
    pub fn repay(&self) -> Vec<&'static str> {
        vec!["到期主动还", "归还时道谢", "人情记心底", "有借有还再借不难"]
    }
}

impl Rule for LendingBorrowingMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("lending_borrowing")
    }

    fn explain(&self) -> String {
        format!(
            "【借物借钱的分寸】\n{}",
            [
                format!(
                    "开口借钱：\\n{}",
                    self.borrow()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "借物规矩：\\n{}",
                    self.borrow_item()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "被借回应：\\n{}",
                    self.lend()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "还钱礼貌：\\n{}",
                    self.repay()
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
    fn test_lendingborrowingmannerrules_basic() {
        let rules = LendingBorrowingMannerRules::new();
        assert_eq!(rules.metadata().name, "借物借钱的分寸");
        assert!(!rules.borrow().is_empty());
        assert!(!rules.borrow_item().is_empty());
        assert!(!rules.lend().is_empty());
        assert!(!rules.repay().is_empty());
    }

    #[test]
    fn test_lendingborrowingmannerrules_validation() {
        let rules = LendingBorrowingMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("lending_borrowing"));
    }

    #[test]
    fn test_lendingborrowingmannerrules_explain() {
        let rules = LendingBorrowingMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("开口借钱"));
        assert!(e.contains("借物规矩"));
        assert!(e.contains("被借回应"));
    }
}
