//! 企业活动礼仪
//!
//! 涵盖企业各类活动的礼仪规范，包括年会、庆典、发布会等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CorporateEventEtiquetteRules,
    name: "企业活动礼仪",
    desc: "企业各类活动礼仪规范，包括年会、庆典、发布会等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "企业活动", "庆典"]
}

impl CorporateEventEtiquetteRules {
    /// 活动策划礼仪
    pub fn planning(&self) -> Vec<&'static str> {
        vec![
            "明确活动目的和主题",
            "制定详细活动方案",
            "确认受邀嘉宾名单",
            "安排活动时间地点",
            "设计活动流程环节",
            "准备必要的物资设备",
            "安排活动工作人员",
            "制定应急预案",
        ]
    }

    /// 邀请礼仪
    pub fn invitation(&self) -> Vec<&'static str> {
        vec![
            "提前发送正式邀请",
            "邀请函内容准确完整",
            "确认嘉宾出席情况",
            "提供活动详细信息",
            "安排VIP嘉宾接待",
            "准备签到和座位安排",
            "发送活动提醒通知",
            "感谢嘉宾接受邀请",
        ]
    }

    /// 活动现场礼仪
    pub fn on_site(&self) -> Vec<&'static str> {
        vec![
            "工作人员着装统一整洁",
            "热情迎接来访嘉宾",
            "引导嘉宾签到入场",
            "安排座位合理有序",
            "提供必要的服务支持",
            "保持活动现场整洁",
            "及时处理突发情况",
            "确保活动流程顺畅",
        ]
    }

    /// 年会礼仪
    pub fn annual_meeting(&self) -> Vec<&'static str> {
        vec![
            "表彰优秀员工和团队",
            "领导致辞感谢员工",
            "安排精彩的节目表演",
            "提供丰盛的餐饮服务",
            "组织互动游戏环节",
            "发放年终奖励礼品",
            "营造欢乐庆祝氛围",
            "总结年度工作成就",
        ]
    }

    /// 产品发布会礼仪
    pub fn product_launch(&self) -> Vec<&'static str> {
        vec![
            "设计震撼的发布形式",
            "演示产品核心特点",
            "邀请媒体和合作伙伴",
            "提供产品体验机会",
            "准备问答环节",
            "安排后续洽谈时间",
            "发放产品宣传资料",
            "感谢参与者光临",
        ]
    }

    /// 企业庆典礼仪
    pub fn celebration(&self) -> Vec<&'static str> {
        vec![
            "选择合适庆典形式",
            "回顾企业发展历程",
            "感谢员工和合作伙伴",
            "展示企业成就荣誉",
            "安排庆祝活动环节",
            "邀请重要嘉宾参与",
            "准备纪念品和礼品",
            "营造喜庆庆祝氛围",
        ]
    }

    /// 媒体应对礼仪
    pub fn media_handling(&self) -> Vec<&'static str> {
        vec![
            "安排媒体签到接待",
            "提供媒体工作区域",
            "准备新闻发布材料",
            "安排采访时间窗口",
            "发言人专业应对提问",
            "保持积极正面形象",
            "感谢媒体报道支持",
            "跟进媒体发布内容",
        ]
    }

    /// 活动结束礼仪
    pub fn closing(&self) -> Vec<&'static str> {
        vec![
            "感谢嘉宾参与活动",
            "安排嘉宾有序离场",
            "发送感谢邮件或信息",
            "跟进活动后续事项",
            "收集活动反馈意见",
            "总结活动效果评估",
            "归档活动相关资料",
            "维护嘉宾关系联系",
        ]
    }
}

impl Rule for CorporateEventEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【企业活动礼仪】\n\n\
            活动策划礼仪：\n{}\n\n\
            邀请礼仪：\n{}\n\n\
            活动现场礼仪：\n{}\n\n\
            年会礼仪：\n{}\n\n\
            产品发布会礼仪：\n{}\n\n\
            企业庆典礼仪：\n{}\n\n\
            媒体应对礼仪：\n{}\n\n\
            活动结束礼仪：\n{}",
            self.planning()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.invitation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.on_site()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.annual_meeting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.product_launch()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.celebration()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.media_handling()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.closing()
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
    fn test_corporate_event_rules() {
        let rules = CorporateEventEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "企业活动礼仪");
        assert!(!rules.planning().is_empty());
        assert!(!rules.invitation().is_empty());
        assert!(!rules.on_site().is_empty());
        assert!(!rules.annual_meeting().is_empty());
        assert!(!rules.product_launch().is_empty());
        assert!(!rules.celebration().is_empty());
        assert!(!rules.media_handling().is_empty());
        assert!(!rules.closing().is_empty());
    }

    #[test]
    fn test_corporate_event_validation() {
        let rules = CorporateEventEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_corporate_event_explain() {
        let rules = CorporateEventEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("活动策划礼仪"));
        assert!(explanation.contains("年会礼仪"));
        assert!(explanation.contains("产品发布会礼仪"));
        assert!(explanation.contains("企业庆典礼仪"));
    }
}
