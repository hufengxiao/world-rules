//! FIBA 世界杯篮球规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballFibaWorldCupRules, name: "FIBA世界杯篮球规则", desc: "国际篮联篮球世界杯规则", origin: "国际", tags: ["体育", "篮球", "世界杯"] }
impl BasketballFibaWorldCupRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "每队5名球员上场",
            "4节各10分钟比赛时长",
            "三分线距离6.75米",
            "24秒进攻时限",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["预选赛分四个赛区", "32支球队晋级正赛", "小组赛+淘汰赛制"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["小组赛采用循环赛制", "16强交叉淘汰", "四强赛决出冠军"]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec!["奥运会参赛资格分配", "各大洲名额分配", "东道主自动晋级"]
    }
}
impl Rule for BasketballFibaWorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_fiba_world_cup")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "FIBA世界杯篮球规则",
            &[
                ("基本规则", &self.section_0()),
                ("预选赛", &self.section_1()),
                ("正赛", &self.section_2()),
                ("奥运资格", &self.section_3()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballFibaWorldCupRules::new();
        assert!(!r.explain().is_empty());
    }
}
