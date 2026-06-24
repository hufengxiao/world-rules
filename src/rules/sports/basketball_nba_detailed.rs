//! NBA详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballNbaDetailedRules, name: "NBA详细规则", desc: "NBA详细比赛规则", origin: "美国", tags: ["体育", "球类"] }
impl BasketballNbaDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "5人对5人比赛4节各12分钟",
            "24秒进攻时限",
            "8秒过半场3秒区限制",
            "三分线NBA7.24米FIBA6.75米",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "NBA选秀两轮60个选秀权",
            "乐透抽签未进季后赛14支球队参与",
            "新秀合同首轮4年",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "软工资帽限制球队薪资总额",
            "奢侈税超过奢侈税线需缴纳罚款",
            "伯德条款允许超工资帽续约自己的球员",
        ]
    }
}
impl Rule for BasketballNbaDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_nba_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA详细规则",
            &[
                ("基本规则", &self.section_0()),
                ("选秀规则", &self.section_1()),
                ("工资帽", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballNbaDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
