//! 行政复议流程
//!
//! 申请行政复议的条件、期限与流程要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AdministrativeReconsiderationRules,
    name: "行政复议流程",
    desc: "申请行政复议的条件、期限与流程要点",
    origin: "中国",
    tags: ["法律", "复议", "行政", "维权"]
}

impl AdministrativeReconsiderationRules {
    /// 复议前提
    pub fn premise(&self) -> Vec<&'static str> {
        vec![
            "对行政行为不服可复议",
            "有明确的请求理由",
            "在法定期限内申请",
            "主体适格",
        ]
    }

    /// 申请材料
    pub fn apply(&self) -> Vec<&'static str> {
        vec![
            "提交复议申请书",
            "附事实与理由",
            "提供初步证据",
            "写明被申请机关",
        ]
    }

    /// 程序时限
    pub fn deadline(&self) -> Vec<&'static str> {
        vec![
            "规定内提交",
            "复议有一定期限",
            "期间依法计算",
            "及时跟进结果",
        ]
    }

    /// 结果处理
    pub fn outcome(&self) -> Vec<&'static str> {
        vec![
            "服从复议决定",
            "不服可再诉讼",
            "尊重复议程序",
            "维护自身合法权利",
        ]
    }
}

impl Rule for AdministrativeReconsiderationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("reconsideration")
    }

    fn explain(&self) -> String {
        format!(
            "【行政复议流程】\n{}",
            [
                format!(
                    "复议前提：\\n{}",
                    self.premise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "申请材料：\\n{}",
                    self.apply()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "程序时限：\\n{}",
                    self.deadline()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结果处理：\\n{}",
                    self.outcome()
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
    fn test_administrativereconsiderationrules_basic() {
        let rules = AdministrativeReconsiderationRules::new();
        assert_eq!(rules.metadata().name, "行政复议流程");
        assert!(!rules.premise().is_empty());
        assert!(!rules.apply().is_empty());
        assert!(!rules.deadline().is_empty());
        assert!(!rules.outcome().is_empty());
    }

    #[test]
    fn test_administrativereconsiderationrules_validation() {
        let rules = AdministrativeReconsiderationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("reconsideration"));
    }

    #[test]
    fn test_administrativereconsiderationrules_explain() {
        let rules = AdministrativeReconsiderationRules::new();
        let e = rules.explain();
        assert!(e.contains("复议前提"));
        assert!(e.contains("申请材料"));
        assert!(e.contains("程序时限"));
    }
}
