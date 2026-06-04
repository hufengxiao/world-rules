//! 水法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: WaterDetailedRules, name: "水法详解", desc: "水法详解", origin: "中国", tags: ["法律", "资源"] }
impl WaterDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["取水许可", "水权交易"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["水功能区", "饮用水源"]
    }
}
impl Rule for WaterDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("water_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "水法详解",
            &[("管理", &self.section_0()), ("保护", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WaterDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
