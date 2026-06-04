//! 反洗钱法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: AntiMoneyLaunderingRules,
    name: "反洗钱法",
    desc: "反洗钱法律规则",
    origin: "中国",
    tags: ["法律", "金融"]
}

impl AntiMoneyLaunderingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["金融机构", "特定非金融机构"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["客户身份识别", "大额交易报告", "可疑交易报告"]
    }
}

impl Rule for AntiMoneyLaunderingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("anti_money_laundering")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "反洗钱法",
            &[("义务主体", &self.section_0()), ("措施", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_anti_money_laundering_rules() {
        let r = AntiMoneyLaunderingRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
