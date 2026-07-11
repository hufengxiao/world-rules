//! 奥运会篮球规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballOlympicRules, name: "奥运会篮球规则", desc: "奥运会篮球比赛规则", origin: "国际", tags: ["体育", "篮球", "奥运"] }
impl BasketballOlympicRules {
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
            "男子12支球队参赛",
            "女子12支球队参赛",
            "东道主自动获得参赛资格",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "小组赛分三组循环",
            "每组前两名+两支最好第三名晋级",
            "淘汰赛决出冠军",
        ]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec![
            "FIBA规则体系",
            "球员必须代表国家/地区参赛",
            "允许一名归化球员",
        ]
    }
}
impl Rule for BasketballOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_olympic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "奥运会篮球规则",
            &[
                ("基本规则", &self.section_0()),
                ("参赛球队", &self.section_1()),
                ("赛制", &self.section_2()),
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
        let r = BasketballOlympicRules::new();
        assert!(!r.explain().is_empty());
    }
}
