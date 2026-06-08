//! 生物统计学定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: BiostatisticsRules, name: "生物统计学定律", desc: "生物统计学定律", origin: "国际", tags: ["科学", "生物"] }
impl BiostatisticsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["t检验", "卡方检验", "ANOVA"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["随机对照", "队列研究", "病例对照"]
    }
}
impl Rule for BiostatisticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("biostatistics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "生物统计学定律",
            &[("方法", &self.section_0()), ("设计", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BiostatisticsRules::new();
        assert!(!r.explain().is_empty());
    }
}
