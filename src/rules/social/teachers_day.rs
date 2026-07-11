//! 教师节礼仪 - 中国教师节礼仪规范
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! { struct: TeachersDayRules, name: "教师节礼仪", desc: "中国教师节礼仪规范", origin: "中国", tags: ["社交", "节日", "教育"] }

impl TeachersDayRules {
    /// 节日意义
    pub fn significance(&self) -> Vec<&'static str> {
        vec![
            "尊师重教 - 弘扬中华民族尊师重教的优良传统",
            "感谢师恩 - 感谢教师的辛勤付出和培育",
            "尊重教师职业 - 肯定教师职业的重要价值",
            "关心教师待遇 - 改善教师的工作和生活条件",
            "传承教育精神 - 传承教育的优良传统",
        ]
    }

    /// 学生礼仪
    pub fn student_etiquette(&self) -> Vec<&'static str> {
        vec![
            "送上祝福 - 向老师表达节日祝福",
            "准备礼物 - 准备贺卡或小礼物",
            "表达感谢 - 真诚感谢老师的教导",
            "遵守纪律 - 上课认真听讲，遵守纪律",
            "完成作业 - 按时完成老师布置的作业",
            "尊重课堂 - 尊重老师的课堂教学",
        ]
    }

    /// 家长礼仪
    pub fn parent_etiquette(&self) -> Vec<&'static str> {
        vec![
            "配合教育 - 积极配合老师的教育工作",
            "表达感谢 - 向老师表达感谢之情",
            "尊重老师 - 尊重老师的教育方式",
            "沟通交流 - 与老师保持良好沟通",
            "理解支持 - 理解并支持老师的工作",
            "不送礼金 - 不送贵重礼品或礼金",
        ]
    }

    /// 学校礼仪
    pub fn school_etiquette(&self) -> Vec<&'static str> {
        vec![
            "表彰大会 - 召开优秀教师表彰大会",
            "慰问教师 - 慰问教师职工",
            "组织活动 - 组织教师节庆祝活动",
            "发放福利 - 为教师发放节日福利",
            "营造氛围 - 营造尊师重教的校园氛围",
            "关心教师 - 关心教师的工作和生活",
        ]
    }

    /// 社会礼仪
    pub fn social_etiquette(&self) -> Vec<&'static str> {
        vec![
            "尊重教师 - 全社会尊重教师职业",
            "支持教育 - 支持教育事业的发展",
            "关爱教师 - 关心教师的身心健康",
            "弘扬传统 - 弘扬尊师重教的传统",
            "宣传典型 - 宣传优秀教师典型",
            "营造氛围 - 营造尊师重教的社会氛围",
        ]
    }

    /// 祝福用语
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "教师节快乐 - 最常用的节日祝福",
            "桃李满天下 - 赞美老师育人成果",
            "春蚕到死丝方尽 - 赞美老师奉献精神",
            "蜡炬成灰泪始干 - 赞美老师无私奉献",
            "一日为师终身为父 - 表达对老师的敬重",
            "桃李芬芳 - 祝愿学生遍布天下",
        ]
    }
}

impl Rule for TeachersDayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("teachers_day")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "教师节礼仪",
            &[
                ("节日意义", &self.significance()),
                ("学生礼仪", &self.student_etiquette()),
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
    fn test_teachers_day_rules() {
        let rules = TeachersDayRules::new();
        assert_eq!(rules.metadata().name, "教师节礼仪");
        assert!(!rules.explain().is_empty());
        assert!(rules.significance().len() >= 5);
        assert!(rules.student_etiquette().len() >= 5);
        assert!(rules.parent_etiquette().len() >= 5);
        assert!(rules.school_etiquette().len() >= 5);
    }
}
