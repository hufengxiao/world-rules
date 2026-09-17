//! 与物业相处
//!
//! 交纳物业费、报修与配合物业的礼貌

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PropertyManagementServiceRules,
    name: "与物业相处",
    desc: "交纳物业费、报修与配合物业的礼貌",
    origin: "中国",
    tags: ["社交", "物业", "服务", "社区"]
}

impl PropertyManagementServiceRules {
    /// 友善沟通
    pub fn communicate(&self) -> Vec<&'static str> {
        vec!["找物业态度好", "说明问题清楚", "不颐指气使", "客气协商"]
    }

    /// 合理报修
    pub fn repair(&self) -> Vec<&'static str> {
        vec!["报修详细描述", "约定时间上门", "配合检修", "不满意可反馈"]
    }

    /// 依规缴费
    pub fn fee(&self) -> Vec<&'static str> {
        vec!["按规定交物业费", "清楚费用标准", "有异议协商办", "合法主张"]
    }

    /// 配合管理
    pub fn cooperate(&self) -> Vec<&'static str> {
        vec!["遵守社区规章", "配合安全维护", "楼道不堆积", "共建文明小区"]
    }
}

impl Rule for PropertyManagementServiceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("property")
    }

    fn explain(&self) -> String {
        format!(
            "【与物业相处】\n{}",
            [
                format!(
                    "友善沟通：\\n{}",
                    self.communicate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "合理报修：\\n{}",
                    self.repair()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "依规缴费：\\n{}",
                    self.fee()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "配合管理：\\n{}",
                    self.cooperate()
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
    fn test_propertymanagementservicerules_basic() {
        let rules = PropertyManagementServiceRules::new();
        assert_eq!(rules.metadata().name, "与物业相处");
        assert!(!rules.communicate().is_empty());
        assert!(!rules.repair().is_empty());
        assert!(!rules.fee().is_empty());
        assert!(!rules.cooperate().is_empty());
    }

    #[test]
    fn test_propertymanagementservicerules_validation() {
        let rules = PropertyManagementServiceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("property"));
    }

    #[test]
    fn test_propertymanagementservicerules_explain() {
        let rules = PropertyManagementServiceRules::new();
        let e = rules.explain();
        assert!(e.contains("友善沟通"));
        assert!(e.contains("合理报修"));
        assert!(e.contains("依规缴费"));
    }
}
