//! 儿童节礼仪 - 国际儿童节礼仪规范
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! { struct: ChildrensDayRules, name: "儿童节礼仪", desc: "国际儿童节礼仪规范", origin: "国际", tags: ["社交", "节日", "儿童"] }

impl ChildrensDayRules {
    /// 节日意义
    pub fn significance(&self) -> Vec<&'static str> {
        vec![
            "保障儿童权利 - 保护儿童的合法权益",
            "促进儿童成长 - 为儿童创造良好的成长环境",
            "关爱儿童健康 - 关注儿童身心健康",
            "重视儿童教育 - 强调儿童教育的重要性",
            "反对虐待儿童 - 反对任何形式的儿童虐待",
        ]
    }

    /// 家长礼仪
    pub fn parent_etiquette(&self) -> Vec<&'static str> {
        vec![
            "陪伴孩子 - 放下工作陪伴孩子",
            "准备礼物 - 为孩子准备节日礼物",
            "组织活动 - 带孩子参加庆祝活动",
            "倾听孩子 - 倾听孩子的心声和想法",
            "表达爱意 - 向孩子表达关爱和支持",
            "鼓励成长 - 鼓励孩子追求梦想",
        ]
    }

    /// 学校礼仪
    pub fn school_etiquette(&self) -> Vec<&'static str> {
        vec![
            "文艺演出 - 组织儿童文艺表演",
            "游戏活动 - 安排丰富多彩的游戏",
            "表彰奖励 - 表彰优秀学生",
            "礼物发放 - 为学生发放节日礼物",
            "家长参与 - 邀请家长参与活动",
            "安全教育 - 进行安全知识教育",
        ]
    }

    /// 社会礼仪
    pub fn social_etiquette(&self) -> Vec<&'static str> {
        vec![
            "关爱儿童 - 社会各界关爱儿童成长",
            "保护儿童 - 保护儿童不受伤害",
            "尊重儿童 - 尊重儿童的人格和权利",
            "帮助困难儿童 - 帮助需要帮助的儿童",
            "创造友好环境 - 为儿童创造友好的社会环境",
            "不以成人标准要求儿童 - 理解儿童的特点",
        ]
    }

    /// 祝福用语
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "儿童节快乐 - 最常用的节日祝福",
            "健康成长 - 祝愿健康快乐成长",
            "学业进步 - 祝愿学习进步",
            "天天开心 - 祝愿每天快乐",
            "梦想成真 - 祝愿梦想实现",
            "童心永存 - 祝愿保持童心",
        ]
    }
}

impl Rule for ChildrensDayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("childrens_day")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "儿童节礼仪",
            &[
                ("节日意义", &self.significance()),
                ("家长礼仪", &self.parent_etiquette()),
                ("学校礼仪", &self.school_etiquette()),
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
    fn test_childrens_day_rules() {
        let rules = ChildrensDayRules::new();
        assert_eq!(rules.metadata().name, "儿童节礼仪");
        assert!(!rules.explain().is_empty());
        assert!(rules.significance().len() >= 5);
        assert!(rules.parent_etiquette().len() >= 5);
        assert!(rules.school_etiquette().len() >= 5);
        assert!(rules.social_etiquette().len() >= 5);
    }
}
