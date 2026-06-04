//! 跆拳道详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: TaekwondoDetailedRules, name: "跆拳道详细规则", desc: "跆拳道详细比赛规则", origin: "WTF", tags: ["体育", "格斗"] }
impl TaekwondoDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["1分踢躯干", "3分旋转踢头"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["感应区域"]
    }
}
impl Rule for TaekwondoDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("taekwondo_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "跆拳道详细规则",
            &[("得分", &self.section_0()), ("电子护具", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TaekwondoDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
