//! 空手道详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: KarateDetailedRules, name: "空手道详细规则", desc: "空手道详细比赛规则", origin: "WKF", tags: ["体育", "格斗"] }
impl KarateDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["得分区域", "犯规"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["指定型", "评分标准"]
    }
}
impl Rule for KarateDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("karate_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "空手道详细规则",
            &[("组手", &self.section_0()), ("型", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = KarateDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
