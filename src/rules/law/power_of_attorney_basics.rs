//! 授权委托常识
//!
//! 出具授权委托书、代理权限与授权风险的要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PowerOfAttorneyBasicsRules,
    name: "授权委托常识",
    desc: "出具授权委托书、代理权限与授权风险的要点",
    origin: "中国",
    tags: ["法律", "授权", "委托", "代理"]
}

impl PowerOfAttorneyBasicsRules {
    /// 授权内容
    pub fn content(&self) -> Vec<&'static str> {
        vec![
            "明确授权事项与范围",
            "写明代理人信息",
            "界定权限与时限",
            "书面形式更稳妥",
        ]
    }

    /// 权限谨慎
    pub fn scope(&self) -> Vec<&'static str> {
        vec![
            "不授予无限权限",
            "重大事项签署审慎",
            "可限定金额与事项",
            "遇变更及时调整",
        ]
    }

    /// 风险控制
    pub fn risk(&self) -> Vec<&'static str> {
        vec![
            "选择可信赖的代理人",
            "用过的授权及时撤回",
            "妥善保管授权文件",
            "遇滥用及时处理",
        ]
    }

    /// 终止代理
    pub fn terminate(&self) -> Vec<&'static str> {
        vec![
            "委托结束书面告知",
            "撤销授权须通知相关方",
            "核对代理有无遗留",
            "依规结束代理关系",
        ]
    }
}

impl Rule for PowerOfAttorneyBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("power_of_attorney")
    }

    fn explain(&self) -> String {
        format!(
            "【授权委托常识】\n{}",
            [
                format!(
                    "授权内容：\\n{}",
                    self.content()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "权限谨慎：\\n{}",
                    self.scope()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "风险控制：\\n{}",
                    self.risk()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "终止代理：\\n{}",
                    self.terminate()
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
    fn test_powerofattorneybasicsrules_basic() {
        let rules = PowerOfAttorneyBasicsRules::new();
        assert_eq!(rules.metadata().name, "授权委托常识");
        assert!(!rules.content().is_empty());
        assert!(!rules.scope().is_empty());
        assert!(!rules.risk().is_empty());
        assert!(!rules.terminate().is_empty());
    }

    #[test]
    fn test_powerofattorneybasicsrules_validation() {
        let rules = PowerOfAttorneyBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("power_of_attorney"));
    }

    #[test]
    fn test_powerofattorneybasicsrules_explain() {
        let rules = PowerOfAttorneyBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("授权内容"));
        assert!(e.contains("权限谨慎"));
        assert!(e.contains("风险控制"));
    }
}
