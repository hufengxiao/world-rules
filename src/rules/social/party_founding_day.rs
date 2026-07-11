//! 建党节礼仪 - 中国共产党建党纪念日礼仪规范
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! { struct: PartyFoundingDayRules, name: "建党节礼仪", desc: "中国共产党建党纪念日礼仪规范", origin: "中国", tags: ["社交", "节日", "政治"] }

impl PartyFoundingDayRules {
    /// 节日意义
    pub fn significance(&self) -> Vec<&'static str> {
        vec![
            "纪念建党 - 纪念中国共产党成立的伟大历史",
            "缅怀先烈 - 缅怀革命先烈的英勇牺牲",
            "不忘初心 - 铭记党的初心和使命",
            "传承精神 - 传承党的优良传统和作风",
            "继续前进 - 为实现中华民族伟大复兴而奋斗",
        ]
    }

    /// 党员礼仪
    pub fn member_etiquette(&self) -> Vec<&'static str> {
        vec![
            "重温入党誓词 - 在党旗下重温入党誓词",
            "学习党史 - 学习党的光辉历史",
            "缴纳党费 - 按规定缴纳党费",
            "参加组织生活 - 积极参加党的组织生活",
            "志愿服务 - 参加党员志愿服务活动",
            "表彰先进 - 表彰优秀共产党员",
        ]
    }

    /// 组织活动
    pub fn organizational_activities(&self) -> Vec<&'static str> {
        vec![
            "主题党日 - 组织开展主题党日活动",
            "党史学习 - 开展党史学习教育活动",
            "红色教育 - 参观红色教育基地",
            "表彰大会 - 召开表彰优秀党员大会",
            "座谈会 - 举办党员座谈会",
            "文艺演出 - 组织庆祝文艺演出",
        ]
    }

    /// 社会礼仪
    pub fn social_etiquette(&self) -> Vec<&'static str> {
        vec![
            "尊重历史 - 尊重党的历史和贡献",
            "学习精神 - 学习党的优秀精神品质",
            "爱国爱党 - 坚持爱国爱党的统一",
            "铭记历史 - 不忘党的奋斗历程",
            "支持党的工作 - 支持党和国家的各项工作",
            "传承红色基因 - 传承革命传统和精神",
        ]
    }

    /// 祝福用语
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "建党节快乐 - 节日祝福",
            "不忘初心牢记使命 - 铭记初心使命",
            "永远跟党走 - 表达坚定信念",
            "祝党生日快乐 - 祝福党的生日",
            "党的事业兴旺发达 - 祝愿党的事业发展",
            "为人民服务 - 铭记党的宗旨",
        ]
    }
}

impl Rule for PartyFoundingDayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("party_founding_day")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "建党节礼仪",
            &[
                ("节日意义", &self.significance()),
                ("党员礼仪", &self.member_etiquette()),
                ("组织活动", &self.organizational_activities()),
                ("社会礼仪", &self.social_etiquette()),
                ("祝福用语", &self.greetings()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_party_founding_day_rules() {
        let rules = PartyFoundingDayRules::new();
        assert_eq!(rules.metadata().name, "建党节礼仪");
        assert!(!rules.explain().is_empty());
        assert!(rules.significance().len() >= 5);
        assert!(rules.member_etiquette().len() >= 5);
        assert!(rules.organizational_activities().len() >= 5);
        assert!(rules.social_etiquette().len() >= 5);
    }
}
