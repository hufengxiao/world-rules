//! WT跆拳道详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TaekwondoWtDetailedRules, name: "WT跆拳道详细规则", desc: "WT跆拳道详细规则", origin: "韩国", tags: ["体育", "格斗"] }
impl TaekwondoWtDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["电子头盔"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["旋转踢加分"]
    }
}
impl Rule for TaekwondoWtDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("taekwondo_wt_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WT跆拳道详细规则",
            &[("电子护具", &self.section_0()), ("得分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TaekwondoWtDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
