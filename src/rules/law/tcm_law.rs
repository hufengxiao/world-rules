//! 中医药法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: TcmLawRules,
    name: "中医药法",
    desc: "中医药法律规则",
    origin: "中国",
    tags: ["法律", "医疗"]
}

impl TcmLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["中医诊所备案", "中医医疗机构", "师承教育"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["中药材种植", "中药饮片", "中成药审批"]
    }
}

impl Rule for TcmLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("tcm_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "中医药法",
            &[("服务", &self.section_0()), ("中药", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tcm_law_rules() {
        let r = TcmLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
