//! 养老保险
//!
//! 养老保险缴纳、领取与补缴的基本要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PensionInsuranceRules,
    name: "养老保险",
    desc: "养老保险缴纳、领取与补缴的基本要点",
    origin: "中国",
    tags: ["社保", "养老", "保险"]
}

impl PensionInsuranceRules {
    /// 参保缴费
    pub fn contribute(&self) -> Vec<&'static str> {
        vec![
            "在职单位代缴",
            "按比例单位个人缴费",
            "缴满年限可领",
            "个人账户积累",
        ]
    }

    /// 账户积累
    pub fn account(&self) -> Vec<&'static str> {
        vec![
            "个人账户随缴累计",
            "缴费基数核定",
            "跨地可转移",
            "查对每年记录",
        ]
    }

    /// 领取条件
    pub fn receive(&self) -> Vec<&'static str> {
        vec![
            "达到法定退休年龄",
            "缴费满规定年限",
            "按月领取养老金",
            "多缴多得",
        ]
    }

    /// 权益关注
    pub fn care(&self) -> Vec<&'static str> {
        vec!["核对自己参保", "单位漏缴主张", "领取有保障", "合理规划晚年"]
    }
}

impl Rule for PensionInsuranceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("pension")
    }

    fn explain(&self) -> String {
        format!(
            "【养老保险】\n{}",
            [
                format!(
                    "参保缴费：\\n{}",
                    self.contribute()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "账户积累：\\n{}",
                    self.account()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "领取条件：\\n{}",
                    self.receive()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "权益关注：\\n{}",
                    self.care()
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
    fn test_pensioninsurancerules_basic() {
        let rules = PensionInsuranceRules::new();
        assert_eq!(rules.metadata().name, "养老保险");
        assert!(!rules.contribute().is_empty());
        assert!(!rules.account().is_empty());
        assert!(!rules.receive().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_pensioninsurancerules_validation() {
        let rules = PensionInsuranceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("pension"));
    }

    #[test]
    fn test_pensioninsurancerules_explain() {
        let rules = PensionInsuranceRules::new();
        let e = rules.explain();
        assert!(e.contains("参保缴费"));
        assert!(e.contains("账户积累"));
        assert!(e.contains("领取条件"));
    }
}
