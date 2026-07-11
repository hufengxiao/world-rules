//! NBA 季后赛规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballPlayoffRules, name: "NBA季后赛规则", desc: "NBA季后赛比赛规则", origin: "美国", tags: ["体育", "篮球", "季后赛"] }
impl BasketballPlayoffRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "东西部各8支球队晋级",
            "附加赛决出最后两个名额",
            "按战绩排名决定对阵",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "首轮7场4胜制",
            "次轮7场4胜制",
            "分区决赛7场4胜制",
            "总决赛7场4胜制",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "第1名vs第8名",
            "第2名vs第7名",
            "第3名vs第6名",
            "第4名vs第5名",
        ]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec![
            "战绩优者多主场优势",
            "总决赛主场优势由战绩决定",
            "抢七决胜在主场优势方举行",
        ]
    }
}
impl Rule for BasketballPlayoffRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_playoff")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NBA季后赛规则",
            &[
                ("晋级规则", &self.section_0()),
                ("赛制", &self.section_1()),
                ("对阵安排", &self.section_2()),
                ("主场优势", &self.section_3()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballPlayoffRules::new();
        assert!(!r.explain().is_empty());
    }
}
