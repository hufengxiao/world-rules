//! 媒体关系礼仪
//!
//! 涵盖企业媒体关系管理相关的礼仪规范，包括媒体接待、采访应对、公关活动等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MediaRelationsEtiquetteRules,
    name: "媒体关系礼仪",
    desc: "企业媒体关系管理礼仪规范，包括媒体接待、采访应对、公关活动等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "媒体", "公关"]
}

impl MediaRelationsEtiquetteRules {
    /// 媒体接待礼仪
    pub fn reception(&self) -> Vec<&'static str> {
        vec![
            "安排专门的媒体接待人员",
            "提供媒体工作便利条件",
            "准备好媒体所需的资料",
            "设置媒体采访区域",
            "提供必要的设备支持",
            "安排采访时间窗口",
            "尊重媒体工作时间",
            "感谢媒体采访报道",
        ]
    }

    /// 采访应对礼仪
    pub fn interview(&self) -> Vec<&'static str> {
        vec![
            "提前了解采访主题",
            "准备相关资料和数据",
            "指定合适的发言人",
            "保持专业镇定态度",
            "回答问题真实准确",
            "避免争议性言论",
            "不回避敏感问题",
            "感谢记者采访机会",
        ]
    }

    /// 新闻发布礼仪
    pub fn news_release(&self) -> Vec<&'static str> {
        vec![
            "撰写清晰准确的新闻稿",
            "选择合适的发布时机",
            "遵循新闻发布规范",
            "提供完整的背景信息",
            "安排媒体联系人",
            "预留媒体询问时间",
            "跟踪发布效果反馈",
            "回应媒体后续问题",
        ]
    }

    /// 公关活动礼仪
    pub fn pr_activities(&self) -> Vec<&'static str> {
        vec![
            "策划有价值的公关活动",
            "邀请相关媒体参与",
            "提供活动详细信息",
            "安排媒体采访机会",
            "准备活动素材材料",
            "协调媒体报道角度",
            "感谢媒体参与支持",
            "跟进媒体报道效果",
        ]
    }

    /// 媒体关系维护
    pub fn relationship_maintenance(&self) -> Vec<&'static str> {
        vec![
            "定期与媒体保持沟通",
            "建立媒体联系人档案",
            "尊重媒体独立性",
            "提供有价值的新闻线索",
            "及时回应媒体询问",
            "维护良好的合作关系",
            "避免过度干预报道",
            "感谢媒体长期支持",
        ]
    }

    /// 危机公关礼仪
    pub fn crisis_handling(&self) -> Vec<&'static str> {
        vec![
            "快速响应媒体关切",
            "诚实披露事实信息",
            "指定统一发言人",
            "避免隐瞒或拖延",
            "积极提供解决方案",
            "控制舆论导向",
            "持续更新处理进展",
            "事后总结改进措施",
        ]
    }

    /// 社交媒体礼仪
    pub fn social_media(&self) -> Vec<&'static str> {
        vec![
            "维护官方社交媒体形象",
            "发布内容专业规范",
            "及时回应网友评论",
            "避免争议性话题",
            "保护企业品牌声誉",
            "遵守平台使用规则",
            "监控舆情动态",
            "与粉丝友好互动",
        ]
    }

    /// 媒体禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要试图贿赂或影响媒体",
            "不要对媒体发表虚假信息",
            "不要回避负面报道",
            "不要对记者态度恶劣",
            "不要泄露未公开信息",
            "不要过度要求报道角度",
            "不要在危机时沉默",
            "不要与媒体发生冲突",
        ]
    }
}

impl Rule for MediaRelationsEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【媒体关系礼仪】\n\n\
            媒体接待礼仪：\n{}\n\n\
            采访应对礼仪：\n{}\n\n\
            新闻发布礼仪：\n{}\n\n\
            公关活动礼仪：\n{}\n\n\
            媒体关系维护：\n{}\n\n\
            危机公关礼仪：\n{}\n\n\
            社交媒体礼仪：\n{}\n\n\
            媒体禁忌：\n{}",
            self.reception()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.interview()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.news_release()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pr_activities()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.relationship_maintenance()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.crisis_handling()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.social_media()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
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
    fn test_media_relations_rules() {
        let rules = MediaRelationsEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "媒体关系礼仪");
        assert!(!rules.reception().is_empty());
        assert!(!rules.interview().is_empty());
        assert!(!rules.news_release().is_empty());
        assert!(!rules.pr_activities().is_empty());
        assert!(!rules.relationship_maintenance().is_empty());
        assert!(!rules.crisis_handling().is_empty());
        assert!(!rules.social_media().is_empty());
        assert!(!rules.taboos().is_empty());
    }

    #[test]
    fn test_media_relations_validation() {
        let rules = MediaRelationsEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_media_relations_explain() {
        let rules = MediaRelationsEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("媒体接待礼仪"));
        assert!(explanation.contains("采访应对礼仪"));
        assert!(explanation.contains("危机公关礼仪"));
    }
}
