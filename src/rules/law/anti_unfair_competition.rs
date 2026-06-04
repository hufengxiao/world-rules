//! 反不正当竞争法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: AntiUnfairCompetitionRules,
    name: "反不正当竞争法",
    desc: "反不正当竞争法律规则",
    origin: "中国",
    tags: ["法律", "商业"]
}

impl AntiUnfairCompetitionRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["混淆行为", "商业贿赂", "虚假宣传", "侵犯商业秘密"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["流量劫持", "恶意不兼容", "数据爬取"]
    }
}

impl Rule for AntiUnfairCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("anti_unfair_competition")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "反不正当竞争法",
            &[
                ("不正当行为", &self.section_0()),
                ("互联网专条", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_anti_unfair_competition_rules() {
        let r = AntiUnfairCompetitionRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
