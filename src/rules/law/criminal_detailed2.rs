//! 刑法详解2
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: CriminalDetailed2Rules, name: "刑法详解2", desc: "刑法详解2", origin: "中国", tags: ["法律", "刑法"] }
impl CriminalDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["管制", "拘役", "有期徒刑", "无期", "死刑"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["从轻", "减轻", "从重"]
    }
}
impl Rule for CriminalDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_detailed2")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "刑法详解2",
            &[("刑罚", &self.section_0()), ("量刑", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CriminalDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
