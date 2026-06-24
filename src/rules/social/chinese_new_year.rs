//! 春节礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseNewYearRules, name: "春节礼仪", desc: "中国春节礼仪", origin: "中国", tags: ["社交", "节日"] }
impl ChineseNewYearRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "农历正月初一除夕夜守岁",
            "腊月二十三小年祭灶",
            "贴春联福字窗花",
            "准备年货打扫房屋",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "年夜饭全家团聚菜品寓意吉祥",
            "守岁除夕夜不睡觉迎接新年",
            "拜年初一给长辈拜年说吉利话",
            "红包长辈给晚辈用新钞双数金额",
            "放鞭炮烟花驱邪迎新",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "初一不扫地扫走财运",
            "不说不吉利的话",
            "红包不能当面拆开",
            "打碎碗要说碎碎平安",
        ]
    }
}
impl Rule for ChineseNewYearRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_new_year")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "春节礼仪",
            &[
                ("时间与准备", &self.section_0()),
                ("传统习俗", &self.section_1()),
                ("禁忌", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseNewYearRules::new();
        assert!(!r.explain().is_empty());
    }
}
