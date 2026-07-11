//! 元旦礼仪 - 中国元旦节传统礼仪规范
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! { struct: NewYearDayRules, name: "元旦礼仪", desc: "中国元旦节礼仪规范", origin: "中国", tags: ["社交", "节日"] }

impl NewYearDayRules {
    /// 元旦庆祝方式
    pub fn celebration_methods(&self) -> Vec<&'static str> {
        vec![
            "升旗仪式 - 参加天安门升旗或本地升旗仪式",
            "联欢晚会 - 观看元旦跨年晚会",
            "新年倒计时 - 跨年倒计时迎接新年",
            "新年祝福 - 向亲友发送新年祝福",
            "新年聚餐 - 与家人朋友聚餐庆祝",
        ]
    }

    /// 公务礼仪
    pub fn official_etiquette(&self) -> Vec<&'static str> {
        vec![
            "新年茶话会 - 单位组织新年茶话会",
            "新年致辞 - 领导发表新年致辞",
            "新年慰问 - 慰问困难群众和一线工作者",
            "新年升旗 - 政府机关举行升旗仪式",
            "新年团拜 - 各级政府举行团拜活动",
        ]
    }

    /// 祝福用语
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "新年快乐 - 最常用的元旦祝福",
            "万事如意 - 祝愿一切顺利",
            "心想事成 - 祝愿愿望实现",
            "步步高升 - 祝愿事业进步",
            "合家幸福 - 祝愿家庭美满",
            "新年新气象 - 祝愿新年有新面貌",
        ]
    }

    /// 禁忌事项
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不说丧气话 - 新年第一天避免消极言论",
            "不打碎器物 - 避免打碎物品，不吉利",
            "不借钱讨债 - 元旦不宜借贷或讨债",
            "不打扫卫生 - 传统上新年不扫地",
            "不穿破旧衣服 - 应穿整洁新衣",
        ]
    }
}

impl Rule for NewYearDayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("new_year_day")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "元旦礼仪",
            &[
                ("庆祝方式", &self.celebration_methods()),
                ("公务礼仪", &self.official_etiquette()),
                ("祝福用语", &self.greetings()),
                ("禁忌事项", &self.taboos()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_year_day_rules() {
        let rules = NewYearDayRules::new();
        assert_eq!(rules.metadata().name, "元旦礼仪");
        assert!(!rules.explain().is_empty());
        assert!(rules.celebration_methods().len() >= 5);
        assert!(rules.official_etiquette().len() >= 5);
        assert!(rules.greetings().len() >= 5);
        assert!(rules.taboos().len() >= 5);
    }
}
