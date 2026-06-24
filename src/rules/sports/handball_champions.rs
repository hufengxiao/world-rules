//! 手球冠军联赛
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HandballChampionsRules, name: "手球冠军联赛", desc: "手球冠军联赛规则", origin: "欧洲", tags: ["体育", "球类"] }
impl HandballChampionsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["小组赛淘汰赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["7米球"]
    }
}
impl Rule for HandballChampionsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("handball_champions")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "手球冠军联赛",
            &[("赛制", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HandballChampionsRules::new();
        assert!(!r.explain().is_empty());
    }
}
