//! NBA G联盟规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballGLeagueRules, name: "NBA G联盟规则", desc: "NBA发展联盟比赛规则", origin: "美国", tags: ["体育", "篮球", "发展联盟"] }
impl BasketballGLeagueRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "每队5名球员上场",
            "4节各12分钟比赛时长",
            "三分线距离7.24米",
            "24秒进攻时限",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["30支球队分东西部", "常规赛36场比赛", "季后赛前8名晋级"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "双向合同球员可上下调配",
            "NBA球队可下放球员锻炼",
            "G联盟球员可签约NBA球队",
        ]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec!["展示赛制度", "年终锦标赛", "季后赛单场淘汰制"]
    }
}
impl Rule for BasketballGLeagueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_g_league")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA G联盟规则",
            &[
                ("基本规则", &self.section_0()),
                ("赛制", &self.section_1()),
                ("球员调配", &self.section_2()),
                ("特色赛制", &self.section_3()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballGLeagueRules::new();
        assert!(!r.explain().is_empty());
    }
}
