//! 3x3 奥运会篮球规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: Basketball3x3OlympicRules, name: "3x3奥运篮球规则", desc: "奥运会三人篮球比赛规则", origin: "国际", tags: ["体育", "篮球", "奥运", "三人篮球"] }
impl Basketball3x3OlympicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "每队3名球员上场1名替补",
            "10分钟比赛时长或先得21分",
            "三分线距离6.75米(2分) 圆内1分",
            "12秒进攻时限",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "男女各8支球队参赛",
            "循环赛阶段每队7场比赛",
            "前两名直接晋级半决赛",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["单场淘汰制", "半决赛3-6名交叉淘汰", "决赛决出金牌"]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec![
            "FIBA三人篮球积分排名",
            "东道主自动获得参赛资格",
            "各大洲名额分配",
        ]
    }
}
impl Rule for Basketball3x3OlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_3x3_olympic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "3x3奥运篮球规则",
            &[
                ("基本规则", &self.section_0()),
                ("参赛球队", &self.section_1()),
                ("淘汰赛", &self.section_2()),
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
        let r = Basketball3x3OlympicRules::new();
        assert!(!r.explain().is_empty());
    }
}
