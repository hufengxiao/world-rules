//! 反垄断法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: AntimonopolyDetailedRules, name: "反垄断法详解", desc: "反垄断法详解", origin: "中国", tags: ["法律", "竞争"] }
impl AntimonopolyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["横向垄断", "纵向垄断"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["申报标准", "审查程序"]
    }
}
impl Rule for AntimonopolyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("antimonopoly_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "反垄断法详解",
            &[
                ("垄断协议", &self.section_0()),
                ("经营者集中", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AntimonopolyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
