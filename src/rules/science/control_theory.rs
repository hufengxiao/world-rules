//! 控制理论定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ControlTheoryRules, name: "控制理论定律", desc: "控制理论定律", origin: "国际", tags: ["科学", "工程"] }
impl ControlTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["PID控制", "根轨迹"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["状态空间", "最优控制"]
    }
}
impl Rule for ControlTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("control_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "控制理论定律",
            &[("经典", &self.section_0()), ("现代", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ControlTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
