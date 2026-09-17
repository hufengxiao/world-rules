//! 向上汇报
//!
//! 向领导汇报工作的条理、时机与表达

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ReportToSuperiorRules,
    name: "向上汇报",
    desc: "向领导汇报工作的条理、时机与表达",
    origin: "国际",
    tags: ["职场", "汇报", "工作"]
}

impl ReportToSuperiorRules {
    /// 汇报时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "按节点做汇报",
            "重大事项及时说",
            "遇阻尽快沟通",
            "不刻意隐瞒",
        ]
    }

    /// 条理清楚
    pub fn structure(&self) -> Vec<&'static str> {
        vec!["先结论后细节", "讲重点和进展", "数字事实说话", "简明扼要"]
    }

    /// 听取反馈
    pub fn feedback(&self) -> Vec<&'static str> {
        vec!["认真听指示", "不懂就问清", "接受合理建议", "不辩不顶撞"]
    }

    /// 跟进落实
    pub fn follow(&self) -> Vec<&'static str> {
        vec!["把指示记下来", "按时推进", "完成情况反馈", "担当尽责"]
    }
}

impl Rule for ReportToSuperiorRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("report")
    }

    fn explain(&self) -> String {
        format!(
            "【向上汇报】\n{}",
            [
                format!(
                    "汇报时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "条理清楚：\\n{}",
                    self.structure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "听取反馈：\\n{}",
                    self.feedback()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "跟进落实：\\n{}",
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
    fn test_reporttosuperiorrules_basic() {
        let rules = ReportToSuperiorRules::new();
        assert_eq!(rules.metadata().name, "向上汇报");
        assert!(!rules.timing().is_empty());
        assert!(!rules.structure().is_empty());
        assert!(!rules.feedback().is_empty());
        assert!(!rules.follow().is_empty());
    }

    #[test]
    fn test_reporttosuperiorrules_validation() {
        let rules = ReportToSuperiorRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("report"));
    }

    #[test]
    fn test_reporttosuperiorrules_explain() {
        let rules = ReportToSuperiorRules::new();
        let e = rules.explain();
        assert!(e.contains("汇报时机"));
        assert!(e.contains("条理清楚"));
        assert!(e.contains("听取反馈"));
    }
}
