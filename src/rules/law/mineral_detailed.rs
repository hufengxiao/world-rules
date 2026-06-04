//! 矿产法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: MineralDetailedRules, name: "矿产法详解", desc: "矿产资源法详解", origin: "中国", tags: ["法律", "资源"] }
impl MineralDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["探矿权", "勘查许可"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["采矿权", "矿山安全"]
    }
}
impl Rule for MineralDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("mineral_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "矿产法详解",
            &[("勘查", &self.section_0()), ("开采", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MineralDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
