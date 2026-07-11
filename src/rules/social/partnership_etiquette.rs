//! 合作伙伴礼仪
//!
//! 涵盖企业合作伙伴关系管理相关的礼仪规范，包括合作洽谈、关系维护、合作终止等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PartnershipEtiquetteRules,
    name: "合作伙伴礼仪",
    desc: "企业合作伙伴关系管理礼仪规范，包括合作洽谈、关系维护、合作终止等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "合作伙伴", "联盟"]
}

impl PartnershipEtiquetteRules {
    /// 合作洽谈礼仪
    pub fn negotiation(&self) -> Vec<&'static str> {
        vec![
            "明确合作目标和期望",
            "评估双方合作契合度",
            "公平协商合作条款",
            "保护各自核心利益",
            "寻求双赢合作方案",
            "建立信任合作基础",
            "尊重对方决策流程",
            "记录协商过程要点",
        ]
    }

    /// 合作协议礼仪
    pub fn agreement(&self) -> Vec<&'static str> {
        vec![
            "签订正式合作协议",
            "明确双方权利义务",
            "界定责任分工范围",
            "设定合作期限条件",
            "约定争议解决机制",
            "保护各自知识产权",
            "确定沟通协调机制",
            "双方高层签字见证",
        ]
    }

    /// 合作启动礼仪
    pub fn launch(&self) -> Vec<&'static str> {
        vec![
            "举行合作启动仪式",
            "双方团队相互介绍",
            "明确项目对接人员",
            "建立沟通协作机制",
            "安排首次合作会议",
            "制定工作计划时间表",
            "共享必要资源信息",
            "表达合作诚意信心",
        ]
    }

    /// 合作执行礼仪
    pub fn execution(&self) -> Vec<&'static str> {
        vec![
            "按时履行合作承诺",
            "及时沟通进展情况",
            "协调解决合作问题",
            "分享合作成果收益",
            "尊重对方工作方式",
            "避免单方擅自决策",
            "定期评估合作效果",
            "记录合作过程档案",
        ]
    }

    /// 关系维护礼仪
    pub fn relationship_maintenance(&self) -> Vec<&'static str> {
        vec![
            "保持定期沟通联系",
            "安排高层定期会面",
            "分享行业最新动态",
            "提供合作支持帮助",
            "认可对方贡献价值",
            "及时处理合作分歧",
            "共同拓展合作机会",
            "维护长期合作信心",
        ]
    }

    /// 合作终止礼仪
    pub fn termination(&self) -> Vec<&'static str> {
        vec![
            "提前协商终止时机",
            "明确终止原因理由",
            "妥善处理遗留事项",
            "保护各自合法权益",
            "交接合作相关资料",
            "感谢过去的合作支持",
            "维护基本商务关系",
            "避免损害对方声誉",
        ]
    }

    /// 战略联盟礼仪
    pub fn strategic_alliance(&self) -> Vec<&'static str> {
        vec![
            "明确战略联盟目标",
            "高层定期战略沟通",
            "共同制定联盟规划",
            "资源共享优势互补",
            "联合开发市场机会",
            "共同应对竞争挑战",
            "维护联盟整体利益",
            "定期评估联盟效果",
        ]
    }

    /// 合作禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要单方面违约",
            "不要泄露合作机密",
            "不要过度主导合作",
            "不要损害对方利益",
            "不要隐瞒重要信息",
            "不要在合作中作弊",
            "不要中途无故退出",
            "不要破坏合作关系",
        ]
    }
}

impl Rule for PartnershipEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【合作伙伴礼仪】\n\n\
            合作洽谈礼仪：\n{}\n\n\
            合作协议礼仪：\n{}\n\n\
            合作启动礼仪：\n{}\n\n\
            合作执行礼仪：\n{}\n\n\
            关系维护礼仪：\n{}\n\n\
            合作终止礼仪：\n{}\n\n\
            战略联盟礼仪：\n{}\n\n\
            合作禁忌：\n{}",
            self.negotiation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.agreement()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.launch()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.execution()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.relationship_maintenance()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.termination()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.strategic_alliance()
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
    fn test_partnership_rules() {
        let rules = PartnershipEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "合作伙伴礼仪");
        assert!(!rules.negotiation().is_empty());
        assert!(!rules.agreement().is_empty());
        assert!(!rules.launch().is_empty());
        assert!(!rules.execution().is_empty());
        assert!(!rules.relationship_maintenance().is_empty());
        assert!(!rules.termination().is_empty());
        assert!(!rules.strategic_alliance().is_empty());
        assert!(!rules.taboos().is_empty());
    }

    #[test]
    fn test_partnership_validation() {
        let rules = PartnershipEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_partnership_explain() {
        let rules = PartnershipEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("合作洽谈礼仪"));
        assert!(explanation.contains("合作执行礼仪"));
        assert!(explanation.contains("合作禁忌"));
    }
}
