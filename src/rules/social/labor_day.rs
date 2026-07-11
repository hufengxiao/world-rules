//! 劳动节礼仪 - 国际劳动节礼仪规范
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! { struct: LaborDayRules, name: "劳动节礼仪", desc: "国际劳动节礼仪规范", origin: "国际", tags: ["社交", "节日", "劳动"] }

impl LaborDayRules {
    /// 节日意义
    pub fn significance(&self) -> Vec<&'static str> {
        vec![
            "纪念劳动人民 - 纪念劳动人民争取权益的历史",
            "弘扬劳动精神 - 倡导尊重劳动、崇尚劳动",
            "肯定劳动价值 - 肯定劳动创造财富的价值",
            "保护劳动者权益 - 维护劳动者合法权益",
            "表彰劳动模范 - 表彰在各行业做出贡献的劳动者",
        ]
    }

    /// 庆祝活动
    pub fn celebrations(&self) -> Vec<&'static str> {
        vec![
            "表彰大会 - 召开劳动模范表彰大会",
            "文艺演出 - 举办庆祝劳动节文艺演出",
            "劳动竞赛 - 组织各行各业劳动技能竞赛",
            "公益活动 - 组织志愿者公益活动",
            "职工活动 - 企业组织职工文体活动",
            "出游踏青 - 劳动节假期外出旅游",
        ]
    }

    /// 职场礼仪
    pub fn workplace_etiquette(&self) -> Vec<&'static str> {
        vec![
            "劳动表彰 - 表彰优秀员工和先进集体",
            "发放福利 - 向员工发放节日福利",
            "假期安排 - 合理安排员工假期",
            "慰问一线 - 慰问坚守岗位的一线员工",
            "感谢付出 - 感谢员工的辛勤劳动",
            "安全提示 - 提醒假期安全注意事项",
        ]
    }

    /// 社交礼仪
    pub fn social_etiquette(&self) -> Vec<&'static str> {
        vec![
            "尊重劳动者 - 尊重各行各业的劳动者",
            "文明旅游 - 假期旅游注意文明礼貌",
            "遵守秩序 - 公共场合遵守秩序",
            "感谢服务 - 对服务人员表示感谢",
            "帮助他人 - 志愿帮助需要帮助的人",
        ]
    }

    /// 祝福用语
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "劳动节快乐 - 最常用的节日祝福",
            "劳动最光荣 - 弘扬劳动精神",
            "工作顺利 - 祝愿工作顺利",
            "身体健康 - 祝愿身心健康",
            "阖家幸福 - 祝愿家庭幸福",
            "假期愉快 - 祝愿假期快乐",
        ]
    }
}

impl Rule for LaborDayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("labor_day")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "劳动节礼仪",
            &[
                ("节日意义", &self.significance()),
                ("庆祝活动", &self.celebrations()),
                ("职场礼仪", &self.workplace_etiquette()),
                ("社交礼仪", &self.social_etiquette()),
                ("祝福用语", &self.greetings()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_labor_day_rules() {
        let rules = LaborDayRules::new();
        assert_eq!(rules.metadata().name, "劳动节礼仪");
        assert!(!rules.explain().is_empty());
        assert!(rules.significance().len() >= 5);
        assert!(rules.celebrations().len() >= 5);
        assert!(rules.workplace_etiquette().len() >= 5);
        assert!(rules.social_etiquette().len() >= 5);
    }
}
