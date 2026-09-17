//! 经济补偿与解雇
//!
//! 解除劳动关系时的经济补偿与工龄计算

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WorkerDismissalPayRules,
    name: "经济补偿与解雇",
    desc: "解除劳动关系时的经济补偿与工龄计算",
    origin: "中国",
    tags: ["劳动", "补偿", "解雇", "赔偿金"]
}

impl WorkerDismissalPayRules {
    /// 补偿情形
    pub fn condition(&self) -> Vec<&'static str> {
        vec![
            "非本人过错被解雇有补偿",
            "协商解除依法补偿",
            "单位违法解除需赔偿",
            "问清解除理由",
        ]
    }

    /// 补偿计算
    pub fn formula(&self) -> Vec<&'static str> {
        vec![
            "按工作年限计",
            "每年约一个月工资",
            "月薪按前十二月平均",
            "按法定上限计算",
        ]
    }

    /// 程序权利
    pub fn right(&self) -> Vec<&'static str> {
        vec![
            "要求书面解除说明",
            "核对补偿数目",
            "一次性结清",
            "保留工资记录",
        ]
    }

    /// 争议处理
    pub fn dispute(&self) -> Vec<&'static str> {
        vec![
            "协商不成仲裁",
            "及时工伤等主张",
            "法定期限内维权",
            "懂法不慌",
        ]
    }
}

impl Rule for WorkerDismissalPayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("dismissal_pay")
    }

    fn explain(&self) -> String {
        format!(
            "【经济补偿与解雇】\n{}",
            [
                format!(
                    "补偿情形：\\n{}",
                    self.condition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "补偿计算：\\n{}",
                    self.formula()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "程序权利：\\n{}",
                    self.right()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "争议处理：\\n{}",
                    self.dispute()
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
    fn test_workerdismissalpayrules_basic() {
        let rules = WorkerDismissalPayRules::new();
        assert_eq!(rules.metadata().name, "经济补偿与解雇");
        assert!(!rules.condition().is_empty());
        assert!(!rules.formula().is_empty());
        assert!(!rules.right().is_empty());
        assert!(!rules.dispute().is_empty());
    }

    #[test]
    fn test_workerdismissalpayrules_validation() {
        let rules = WorkerDismissalPayRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("dismissal_pay"));
    }

    #[test]
    fn test_workerdismissalpayrules_explain() {
        let rules = WorkerDismissalPayRules::new();
        let e = rules.explain();
        assert!(e.contains("补偿情形"));
        assert!(e.contains("补偿计算"));
        assert!(e.contains("程序权利"));
    }
}
