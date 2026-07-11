//! 项目管理礼仪
//!
//! 涵盖项目管理活动相关的礼仪规范，包括项目启动、团队协作、项目交付等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ProjectManagementEtiquetteRules,
    name: "项目管理礼仪",
    desc: "项目管理活动礼仪规范，包括项目启动、团队协作、项目交付等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "项目管理", "团队"]
}

impl ProjectManagementEtiquetteRules {
    /// 项目启动礼仪
    pub fn project_launch(&self) -> Vec<&'static str> {
        vec![
            "召开项目启动会议",
            "明确项目目标和范围",
            "介绍团队成员和角色",
            "制定项目计划时间表",
            "确认资源分配安排",
            "建立沟通协作机制",
            "明确责任分工界限",
            "激发团队工作热情",
        ]
    }

    /// 项目规划礼仪
    pub fn planning(&self) -> Vec<&'static str> {
        vec![
            "充分调研项目需求",
            "听取各方意见建议",
            "制定详细可行计划",
            "合理评估项目风险",
            "预留必要缓冲时间",
            "明确里程碑节点",
            "分配责任到具体人员",
            "获得各方认可确认",
        ]
    }

    /// 团队协作礼仪
    pub fn team_collaboration(&self) -> Vec<&'static str> {
        vec![
            "尊重每个团队成员",
            "公平分配工作任务",
            "及时沟通项目进展",
            "公开分享项目信息",
            "鼓励团队积极参与",
            "认可成员贡献成果",
            "帮助解决协作问题",
            "促进团队学习成长",
        ]
    }

    /// 项目会议礼仪
    pub fn meeting(&self) -> Vec<&'static str> {
        vec![
            "会议目的明确清晰",
            "提前发送会议议程",
            "准时开始和结束会议",
            "鼓励全员参与发言",
            "记录会议决策要点",
            "跟进会议行动事项",
            "避免冗长无效会议",
            "会后及时发送纪要",
        ]
    }

    /// 进度报告礼仪
    pub fn progress_reporting(&self) -> Vec<&'static str> {
        vec![
            "定期汇报项目进展",
            "诚实报告进度状况",
            "不隐瞒问题和风险",
            "提出解决方案建议",
            "使用清晰的数据展示",
            "及时预警潜在问题",
            "尊重各方信息需求",
            "保持报告格式规范",
        ]
    }

    /// 问题解决礼仪
    pub fn problem_solving(&self) -> Vec<&'static str> {
        vec![
            "及时识别和报告问题",
            "客观分析问题原因",
            "避免指责个人责任",
            "集体讨论解决方案",
            "快速执行解决措施",
            "跟踪问题处理效果",
            "总结预防类似问题",
            "分享问题解决经验",
        ]
    }

    /// 项目交付礼仪
    pub fn delivery(&self) -> Vec<&'static str> {
        vec![
            "按时完成项目交付",
            "确保交付质量达标",
            "提供完整交付文档",
            "组织交付验收会议",
            "演示交付成果功能",
            "解答客户验收问题",
            "处理交付遗留事项",
            "感谢团队辛勤工作",
        ]
    }

    /// 项目收尾礼仪
    pub fn project_closure(&self) -> Vec<&'static str> {
        vec![
            "召开项目总结会议",
            "总结项目经验教训",
            "表彰优秀团队成员",
            "整理归档项目文档",
            "移交项目维护责任",
            "释放项目资源人员",
            "评估项目整体效果",
            "感谢各方支持配合",
        ]
    }
}

impl Rule for ProjectManagementEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【项目管理礼仪】\n\n\
            项目启动礼仪：\n{}\n\n\
            项目规划礼仪：\n{}\n\n\
            团队协作礼仪：\n{}\n\n\
            项目会议礼仪：\n{}\n\n\
            进度报告礼仪：\n{}\n\n\
            问题解决礼仪：\n{}\n\n\
            项目交付礼仪：\n{}\n\n\
            项目收尾礼仪：\n{}",
            self.project_launch()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.planning()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.team_collaboration()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.meeting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.progress_reporting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.problem_solving()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.delivery()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.project_closure()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_project_management_rules() {
        let rules = ProjectManagementEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "项目管理礼仪");
        assert!(!rules.project_launch().is_empty());
        assert!(!rules.planning().is_empty());
        assert!(!rules.team_collaboration().is_empty());
        assert!(!rules.meeting().is_empty());
        assert!(!rules.progress_reporting().is_empty());
        assert!(!rules.problem_solving().is_empty());
        assert!(!rules.delivery().is_empty());
        assert!(!rules.project_closure().is_empty());
    }

    #[test]
    fn test_project_management_validation() {
        let rules = ProjectManagementEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_project_management_explain() {
        let rules = ProjectManagementEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("项目启动礼仪"));
        assert!(explanation.contains("团队协作礼仪"));
        assert!(explanation.contains("项目交付礼仪"));
    }
}
