//! 生态学详细定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: EcologyDetailedRules, name: "生态学详细定律", desc: "生态学详细定律", origin: "国际", tags: ["科学", "生物"] }
impl EcologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["逻辑斯谛增长", "竞争排斥"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["能量流动", "物质循环"]
    }
}
impl Rule for EcologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("ecology_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "生态学详细定律",
            &[("种群", &self.section_0()), ("生态系统", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EcologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
