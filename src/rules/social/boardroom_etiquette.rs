//! 董事会礼仪
//!
//! 涵盖董事会会议相关的礼仪规范，包括参会准备、会议行为、决策礼仪等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BoardroomEtiquetteRules,
    name: "董事会礼仪",
    desc: "董事会会议礼仪规范，包括参会准备、会议行为、决策礼仪等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "董事会", "高层"]
}

impl BoardroomEtiquetteRules {
    /// 参会准备礼仪
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前审阅会议材料",
            "准备意见和建议",
            "确认参会方式（现场或远程）",
            "了解会议议程和议题",
            "检查投票权限",
            "准备必要的文件和资料",
            "确认会议时间和地点",
            "安排行程准时到达",
        ]
    }

    /// 会议入场礼仪
    pub fn entering(&self) -> Vec<&'static str> {
        vec![
            "准时到达会议室",
            "着装正式得体",
            "向其他董事问候致意",
            "按座位安排入座",
            "关闭或静音手机",
            "避免携带无关物品",
            "保持安静等待会议开始",
            "准备好笔记本电脑或纸笔",
        ]
    }

    /// 发言礼仪
    pub fn speaking(&self) -> Vec<&'static str> {
        vec![
            "等待主持人示意发言",
            "发言前说明身份和立场",
            "表达清晰简洁",
            "避免冗长重复发言",
            "尊重他人发言权利",
            "不打断他人发言",
            "使用专业和礼貌的语言",
            "提出建设性意见",
        ]
    }

    /// 决策投票礼仪
    pub fn voting(&self) -> Vec<&'static str> {
        vec![
            "充分理解投票议题",
            "独立行使投票权",
            "明确表达投票立场",
            "尊重投票结果",
            "不泄露投票细节（保密议题）",
            "记录个人投票决定",
            "理解弃权规则",
            "遵循法定投票程序",
        ]
    }

    /// 保密礼仪
    pub fn confidentiality(&self) -> Vec<&'static str> {
        vec![
            "严格保密会议内容",
            "不向外界透露讨论细节",
            "妥善保管会议文件",
            "不在公共场所讨论议题",
            "遵守保密协议条款",
            "离职后继续履行保密义务",
            "正确处理机密文件销毁",
            "报告任何泄密风险",
        ]
    }

    /// 远程参会礼仪
    pub fn remote_attendance(&self) -> Vec<&'static str> {
        vec![
            "提前测试视频会议系统",
            "确保网络连接稳定",
            "选择安静背景环境",
            "保持摄像头开启",
            "发言时靠近麦克风",
            "避免背景噪音干扰",
            "按时登录参会",
            "远程投票遵循特定程序",
        ]
    }

    /// 会议结束礼仪
    pub fn closing(&self) -> Vec<&'static str> {
        vec![
            "确认会议决议记录",
            "归还机密文件材料",
            "与董事成员告别",
            "离开时保持安静",
            "跟进后续行动事项",
            "签署会议纪要",
            "执行会议决定",
            "反馈会议效果建议",
        ]
    }

    /// 董事行为规范
    pub fn director_behavior(&self) -> Vec<&'static str> {
        vec![
            "忠诚履行董事职责",
            "勤勉尽责参与决策",
            "避免利益冲突",
            "披露关联关系",
            "不滥用董事权力",
            "遵守公司章程",
            "维护公司利益",
            "依法承担董事责任",
        ]
    }
}

impl Rule for BoardroomEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【董事会礼仪】\n\n\
            参会准备礼仪：\n{}\n\n\
            会议入场礼仪：\n{}\n\n\
            发言礼仪：\n{}\n\n\
            决策投票礼仪：\n{}\n\n\
            保密礼仪：\n{}\n\n\
            远程参会礼仪：\n{}\n\n\
            会议结束礼仪：\n{}\n\n\
            董事行为规范：\n{}",
            self.preparation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.entering()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.speaking()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.voting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.confidentiality()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.remote_attendance()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.closing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.director_behavior()
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
    fn test_boardroom_rules() {
        let rules = BoardroomEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "董事会礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.entering().is_empty());
        assert!(!rules.speaking().is_empty());
        assert!(!rules.voting().is_empty());
        assert!(!rules.confidentiality().is_empty());
        assert!(!rules.remote_attendance().is_empty());
        assert!(!rules.closing().is_empty());
        assert!(!rules.director_behavior().is_empty());
    }

    #[test]
    fn test_boardroom_validation() {
        let rules = BoardroomEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_boardroom_explain() {
        let rules = BoardroomEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("参会准备礼仪"));
        assert!(explanation.contains("发言礼仪"));
        assert!(explanation.contains("决策投票礼仪"));
        assert!(explanation.contains("保密礼仪"));
    }
}
