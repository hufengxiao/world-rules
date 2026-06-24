//! 地理信息系统定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GisRules, name: "地理信息系统定律", desc: "GIS定律", origin: "国际", tags: ["科学", "地球"] }
impl GisRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["矢量栅格"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["空间分析"]
    }
}
impl Rule for GisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("gis")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "地理信息系统定律",
            &[("数据", &self.section_0()), ("分析", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GisRules::new();
        assert!(!r.explain().is_empty());
    }
}
