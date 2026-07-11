//! 妇女节礼仪 - 国际妇女节礼仪规范
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! { struct: WomensDayRules, name: "妇女节礼仪", desc: "国际妇女节礼仪规范", origin: "国际", tags: ["社交", "节日", "性别平等"] }

impl WomensDayRules {
    /// 节日意义
    pub fn significance(&self) -> Vec<&'static str> {
        vec![
            "纪念妇女争取权利 - 纪念妇女争取平等权利的历史",
            "庆祝女性成就 - 庆祝女性在各领域的成就",
            "促进性别平等 - 推动性别平等和社会进步",
            "关爱女性权益 - 关注和保护女性的合法权益",
            "倡导尊重女性 - 倡导全社会尊重和关爱女性",
        ]
    }

    /// 职场礼仪
    pub fn workplace_etiquette(&self) -> Vec<&'static str> {
        vec![
            "领导慰问 - 领导向女性员工表示慰问",
            "发放福利 - 为女性员工发放节日福利",
            "组织活动 - 组织女性员工参加活动",
            "半天假期 - 女性员工享受半天假期",
            "表彰先进 - 表彰优秀女性员工",
            "尊重理解 - 尊重女性员工的工作付出",
        ]
    }

    /// 家庭礼仪
    pub fn family_etiquette(&self) -> Vec<&'static str> {
        vec![
            "送花祝福 - 向母亲、妻子送花表达祝福",
            "分担家务 - 男性主动分担家务",
            "准备礼物 - 为女性家人准备节日礼物",
            "表达感谢 - 感谢女性的辛勤付出",
            "家庭聚餐 - 全家一起庆祝节日",
        ]
    }

    /// 社交礼仪
    pub fn social_etiquette(&self) -> Vec<&'static str> {
        vec![
            "发送祝福 - 向女性朋友发送祝福信息",
            "赠送鲜花 - 送康乃馨等表达敬意",
            "真诚赞美 - 真诚赞美女性的优秀品质",
            "平等尊重 - 以平等尊重的态度对待女性",
            "支持女性 - 支持女性的事业和发展",
        ]
    }

    /// 祝福用语
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "妇女节快乐 - 最常用的节日祝福",
            "永远年轻美丽 - 祝愿青春永驻",
            "事业有成 - 祝愿事业顺利",
            "家庭幸福 - 祝愿家庭美满",
            "健康快乐 - 祝愿身心健康",
            "活出精彩 - 祝愿人生精彩",
        ]
    }
}

impl Rule for WomensDayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("womens_day")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "妇女节礼仪",
            &[
                ("节日意义", &self.significance()),
                ("职场礼仪", &self.workplace_etiquette()),
                ("家庭礼仪", &self.family_etiquette()),
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
    fn test_womens_day_rules() {
        let rules = WomensDayRules::new();
        assert_eq!(rules.metadata().name, "妇女节礼仪");
        assert!(!rules.explain().is_empty());
        assert!(rules.significance().len() >= 5);
        assert!(rules.workplace_etiquette().len() >= 5);
        assert!(rules.family_etiquette().len() >= 5);
        assert!(rules.social_etiquette().len() >= 5);
    }
}
