//! NBA 全明星赛规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballAllStarRules, name: "NBA全明星赛规则", desc: "NBA全明星周末比赛规则", origin: "美国", tags: ["体育", "篮球", "全明星"] }
impl BasketballAllStarRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "球迷投票选出首发阵容",
            "教练选出替补阵容",
            "东西部对决改为队长选人制",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "正赛4节各12分钟",
            "第四节不设时限先达目标分获胜",
            "致敬得分制",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["技巧挑战赛", "三分球大赛", "扣篮大赛"]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec!["新秀挑战赛", "名人赛", "G联盟展示赛"]
    }
}
impl Rule for BasketballAllStarRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_all_star")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA全明星赛规则",
            &[
                ("球员选拔", &self.section_0()),
                ("正赛规则", &self.section_1()),
                ("单项赛", &self.section_2()),
                ("其他活动", &self.section_3()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballAllStarRules::new();
        assert!(!r.explain().is_empty());
    }
}
