//! 刑法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CriminalDetailedRules, name: "刑法详解", desc: "刑法罪名详解", origin: "中国", tags: ["法律", "刑法"] }
impl CriminalDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["故意杀人", "故意伤害"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["盗窃", "诈骗", "抢劫"]
    }
}
impl Rule for CriminalDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "刑法详解",
            &[
                ("侵犯人身", &self.section_0()),
                ("侵犯财产", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CriminalDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
