//! 微分几何定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: DifferentialGeometryRules, name: "微分几何定律", desc: "微分几何定律", origin: "国际", tags: ["科学", "数学"] }
impl DifferentialGeometryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["曲率", "挠率", "Frenet公式"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["高斯曲率", "测地线"]
    }
}
impl Rule for DifferentialGeometryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("differential_geometry")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "微分几何定律",
            &[("曲线", &self.section_0()), ("曲面", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DifferentialGeometryRules::new();
        assert!(!r.explain().is_empty());
    }
}
