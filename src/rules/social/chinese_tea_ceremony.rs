//! 中国茶道礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseTeaCeremonyRules, name: "中国茶道礼仪", desc: "中国茶道礼仪", origin: "中国", tags: ["社交", "茶道"] }
impl ChineseTeaCeremonyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["茶壶紫砂壶最佳", "公道杯品茗杯", "盖碗万能茶具适合所有茶类"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "温壶温杯用热水冲洗茶具",
            "投茶3-5克/150ml",
            "洗茶第一泡倒掉不喝",
            "水温根据茶类调整绿茶80度红茶95度",
            "出汤控制浸泡时间",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "闻香先闻杯盖香再闻杯底香",
            "品饮小口慢品感受回甘",
            "扣指礼长辈倒茶时用手指轻扣桌面",
            "续茶主人应及时续茶",
        ]
    }
}
impl Rule for ChineseTeaCeremonyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_tea_ceremony")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国茶道礼仪",
            &[
                ("茶具", &self.section_0()),
                ("泡茶步骤", &self.section_1()),
                ("品茶礼仪", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseTeaCeremonyRules::new();
        assert!(!r.explain().is_empty());
    }
}
