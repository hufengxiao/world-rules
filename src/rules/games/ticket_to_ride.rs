//! 车票之旅规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TicketToRideRules, name: "车票之旅规则", desc: "车票之旅桌游规则", origin: "美国", tags: ["游戏", "桌游"] }
impl TicketToRideRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["收集火车卡", "占领路线"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["路线分", "车票分"]
    }
}
impl Rule for TicketToRideRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("ticket_to_ride")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "车票之旅规则",
            &[("基本", &self.section_0()), ("计分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TicketToRideRules::new();
        assert!(!r.explain().is_empty());
    }
}
