//! 教育法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 教育法规则
pub struct EducationLawRules {
    metadata: RuleMetadata,
}

impl EducationLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "教育法规则",
                "中国教育法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "教育".into()]),
        }
    }

    /// 教育类型
    pub fn education_types(&self) -> Vec<&'static str> {
        vec![
            "学前教育: 幼儿教育阶段",
            "义务教育: 小学初中九年",
            "高中教育: 高中阶段教育",
            "职业教育: 中职高职教育",
            "高等教育: 大学研究生教育",
            "成人教育: 成人继续教育",
            "特殊教育: 残疾人教育",
            "民办教育: 民办学校教育",
        ]
    }

    /// 义务教育规则
    pub fn compulsory_education(&self) -> Vec<&'static str> {
        vec![
            "九年义务教育: 小学六年初中三年",
            "免学费制度: 免除学杂费",
            "就近入学: 划片就近入学",
            "禁止辍学: 禁止学生辍学",
            "义务教育均衡发展",
            "禁止开除学生: 不得开除",
            "入学年龄: 6周岁入学",
            "义务教育完成证书",
        ]
    }

    /// 学校设立规则
    pub fn school_establishment(&self) -> Vec<&'static str> {
        vec![
            "设立条件: 场所师资经费",
            "审批程序: 教育部门审批",
            "分级管理: 各级教育管理",
            "学校章程: 制定学校章程",
            "办学许可证: 办学许可制度",
            "学校变更: 变更审批程序",
            "学校终止: 终止注销程序",
            "民办学校设立: 民办学校审批",
        ]
    }

    /// 教师权利义务
    pub fn teacher_rights(&self) -> Vec<&'static str> {
        vec![
            "教育教学权: 开展教学活动",
            "学术研究权: 从事科学研究",
            "指导评价权: 评价学生学业",
            "参与管理权: 参与学校管理",
            "进修培训权: 培训进修权利",
            "教育教学义务: 执行教学任务",
            "思想教育义务: 教育引导义务",
            "尊重学生义务: 尊重人格尊严",
        ]
    }

    /// 学生权利保护
    pub fn student_rights(&self) -> Vec<&'static str> {
        vec![
            "受教育权: 平等接受教育",
            "获得资助权: 获得资助资助",
            "获得评价权: 公正学业评价",
            "申诉权利: 对处理结果申诉",
            "人身权利: 人身安全保护",
            "隐私权利: 个人隐私保护",
            "财产权利: 个人财产保护",
            "知识产权: 创新成果保护",
        ]
    }

    /// 招生考试规则
    pub fn admission_rules(&self) -> Vec<&'static str> {
        vec![
            "高考制度: 全国统一高考",
            "中考制度: 初中升学考试",
            "研究生考试: 研招考试",
            "招生计划: 招生名额管理",
            "录取规则: 录取标准程序",
            "公平招生: 禁止招生歧视",
            "招生信息公开: 信息公开要求",
            "招生违规处理: 违规处罚规定",
        ]
    }

    /// 教育经费规则
    pub fn education_funding(&self) -> Vec<&'static str> {
        vec![
            "财政投入: 政府教育投入",
            "经费增长: 经费逐年增长",
            "经费分配: 各级经费分配",
            "学费收取: 学费收取规定",
            "资助制度: 学生资助体系",
            "奖学金制度: 奖学金设立",
            "助学贷款: 贷款助学制度",
            "经费监督: 经费使用监督",
        ]
    }

    /// 教育法律责任
    pub fn education_liability(&self) -> Vec<&'static str> {
        vec![
            "行政责任: 教育行政处罚",
            "民事责任: 损害赔偿责任",
            "刑事责任: 严重违法入刑",
            "违规招生处罚: 招生违规处罚",
            "违规办校处罚: 办学违规处罚",
            "违规收费处罚: 收费违规处罚",
            "考试作弊处罚: 考试违规处罚",
            "学生伤害处理: 学生伤害责任",
        ]
    }
}

impl Default for EducationLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EducationLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("education")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【教育法规则】\n\n教育类型:\n{}\n\n义务教育:\n{}\n\n学生权利:\n{}\n",
            self.education_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.compulsory_education().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.student_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_education_law_rules() {
        let rules = EducationLawRules::new();
        assert!(!rules.education_types().is_empty());
        assert!(!rules.compulsory_education().is_empty());
    }
}