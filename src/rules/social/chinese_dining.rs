//! 中餐礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseDiningRules, name: "中餐礼仪", desc: "中国传统餐桌礼仪", origin: "中国", tags: ["社交", "餐桌"] }
impl ChineseDiningRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "主位面对门口主人或最尊贵的客人",
            "主宾主人右手边最重要的客人",
            "以右为尊以远为上",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "等主人或长辈先动筷",
            "不翻拣菜肴",
            "不把筷子插在饭上像祭祀",
            "喝汤不出声嘴中有食物不说话",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "晚辈敬酒杯沿低于长辈杯沿",
            "敬酒时双手持杯",
            "先敬主宾再按顺序",
            "主人应先敬酒客人回敬",
        ]
    }
}
impl Rule for ChineseDiningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_dining")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中餐礼仪",
            &[
                ("座次安排", &self.section_0()),
                ("用餐礼仪", &self.section_1()),
                ("敬酒礼仪", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseDiningRules::new();
        assert!(!r.explain().is_empty());
    }
}
