//! 大满贯规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TennisGrandSlamRules, name: "大满贯规则", desc: "网球大满贯赛事规则", origin: "国际", tags: ["体育", "球类"] }
impl TennisGrandSlamRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "澳网1月硬地",
            "法网5-6月红土",
            "温网6-7月草地",
            "美网8-9月硬地",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "男子单打五盘三胜制",
            "女子单打三盘两胜制",
            "每盘6局6-6时抢七",
            "发球每局轮换每分两次发球机会",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "鹰眼挑战每盘3次机会",
            "医疗暂停每盘一次3分钟",
            "温网要求全白着装",
        ]
    }
}
impl Rule for TennisGrandSlamRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("tennis_grand_slam")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "大满贯规则",
            &[
                ("四大满贯", &self.section_0()),
                ("比赛规则", &self.section_1()),
                ("特殊规则", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TennisGrandSlamRules::new();
        assert!(!r.explain().is_empty());
    }
}
