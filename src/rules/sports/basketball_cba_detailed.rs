//! CBA 中国篮球详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballCbaDetailedRules, name: "CBA详细规则", desc: "中国男子篮球职业联赛详细规则", origin: "中国", tags: ["体育", "篮球"] }
impl BasketballCbaDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "每队5名球员上场",
            "4节各12分钟比赛时长",
            "三分线距离6.75米",
            "24秒进攻时限",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["常规赛20支球队", "分南北两个赛区", "每队46场常规赛"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "前12名进入季后赛",
            "季后赛采用5场3胜制",
            "总决赛采用7场4胜制",
        ]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec![
            "外援注册4人同时上场2人",
            "亚洲球员不占外援名额",
            "国内球员选秀制度",
        ]
    }
}
impl Rule for BasketballCbaDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_cba_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "CBA详细规则",
            &[
                ("基本规则", &self.section_0()),
                ("常规赛", &self.section_1()),
                ("季后赛", &self.section_2()),
                ("外援政策", &self.section_3()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballCbaDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
