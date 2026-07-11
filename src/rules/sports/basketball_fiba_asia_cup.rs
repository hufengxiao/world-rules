//! FIBA 亚洲杯篮球规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballFibaAsiaCupRules, name: "FIBA亚洲杯篮球规则", desc: "国际篮联亚洲杯篮球比赛规则", origin: "亚洲", tags: ["体育", "篮球", "亚洲杯"] }
impl BasketballFibaAsiaCupRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "每队5名球员上场",
            "4节各10分钟比赛时长",
            "三分线距离6.75米",
            "24秒进攻时限",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "预选赛分六个小组",
            "16支球队晋级正赛",
            "东道主自动获得参赛资格",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "正赛分四个小组循环赛",
            "每组前三名晋级淘汰赛",
            "四分之一决赛交叉淘汰",
        ]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec![
            "前四名获得世界杯参赛资格",
            "归化球员限一人上场",
            "球员需持亚洲护照或父母亚洲血统",
        ]
    }
}
impl Rule for BasketballFibaAsiaCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_fiba_asia_cup")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "FIBA亚洲杯篮球规则",
            &[
                ("基本规则", &self.section_0()),
                ("预选赛", &self.section_1()),
                ("正赛", &self.section_2()),
                ("参赛资格", &self.section_3()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballFibaAsiaCupRules::new();
        assert!(!r.explain().is_empty());
    }
}
