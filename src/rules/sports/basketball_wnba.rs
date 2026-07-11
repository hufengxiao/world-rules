//! WNBA 女子篮球规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballWnbaRules, name: "WNBA女子篮球规则", desc: "美国女子职业篮球联赛规则", origin: "美国", tags: ["体育", "篮球", "女子"] }
impl BasketballWnbaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "每队5名球员上场",
            "4节各10分钟比赛时长",
            "三分线距离6.75米",
            "24秒进攻时限",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["常规赛36场比赛", "季后赛采用5场3胜制", "总决赛采用5场3胜制"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "选秀三轮36名球员",
            "乐透抽签未进季后赛球队参与",
            "新秀合同4年",
        ]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec!["软工资帽制度", "超级顶薪条款", "核心球员指定条款"]
    }
}
impl Rule for BasketballWnbaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_wnba")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WNBA女子篮球规则",
            &[
                ("基本规则", &self.section_0()),
                ("赛制", &self.section_1()),
                ("选秀", &self.section_2()),
                ("薪资", &self.section_3()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballWnbaRules::new();
        assert!(!r.explain().is_empty());
    }
}
