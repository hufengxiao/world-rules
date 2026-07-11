//! 投资者关系礼仪
//!
//! 涵盖投资者关系管理相关的礼仪规范，包括信息披露、投资者沟通、股东服务等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InvestorRelationsEtiquetteRules,
    name: "投资者关系礼仪",
    desc: "投资者关系管理礼仪规范，包括信息披露、投资者沟通、股东服务等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "投资者", "股东"]
}

impl InvestorRelationsEtiquetteRules {
    /// 信息披露礼仪
    pub fn disclosure(&self) -> Vec<&'static str> {
        vec![
            "及时准确披露重要信息",
            "遵守信息披露法规",
            "公平对待所有投资者",
            "避免选择性披露",
            "使用清晰易懂的语言",
            "确保信息渠道畅通",
            "定期发布财务报告",
            "及时回应投资者询问",
        ]
    }

    /// 投资者沟通礼仪
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "保持开放透明的态度",
            "定期举行投资者会议",
            "认真倾听投资者关切",
            "专业解答投资者问题",
            "避免误导性陈述",
            "谨慎处理前瞻性信息",
            "尊重投资者隐私",
            "记录沟通内容备查",
        ]
    }

    /// 股东大会礼仪
    pub fn shareholder_meeting(&self) -> Vec<&'static str> {
        vec![
            "依法召开股东大会",
            "提前通知会议信息",
            "提供完整会议材料",
            "安排合理的会议流程",
            "尊重股东发言权利",
            "公平处理股东提案",
            "确保投票程序公正",
            "及时公布会议决议",
        ]
    }

    /// 股东服务礼仪
    pub fn shareholder_service(&self) -> Vec<&'static str> {
        vec![
            "设立股东服务热线",
            "提供便捷的查询服务",
            "及时处理股东请求",
            "维护股东权益利益",
            "发送股东通讯刊物",
            "组织股东参观活动",
            "提供股东专属服务",
            "定期更新股东信息",
        ]
    }

    /// 投资者路演礼仪
    pub fn roadshow(&self) -> Vec<&'static str> {
        vec![
            "精心准备路演材料",
            "选择合适的路演地点",
            "邀请重要投资者参与",
            "展示公司战略前景",
            "回答投资者问题",
            "避免过度承诺",
            "安排一对一会谈",
            "跟进路演后续联系",
        ]
    }

    /// 分析师关系礼仪
    pub fn analyst_relations(&self) -> Vec<&'static str> {
        vec![
            "公平对待所有分析师",
            "提供准确的公司信息",
            "协助分析师理解业务",
            "定期举行分析师会议",
            "尊重分析师独立判断",
            "不试图影响评级结果",
            "及时回应分析师询问",
            "维护良好的行业关系",
        ]
    }

    /// 机构投资者礼仪
    pub fn institutional_investors(&self) -> Vec<&'static str> {
        vec![
            "重视机构投资者关系",
            "提供深度的公司信息",
            "安排高层会面机会",
            "理解机构投资策略",
            "定期沟通公司进展",
            "响应机构投资需求",
            "维护长期合作关系",
            "尊重机构决策独立性",
        ]
    }

    /// 危机沟通礼仪
    pub fn crisis_communication(&self) -> Vec<&'static str> {
        vec![
            "及时承认和解释问题",
            "诚实披露影响范围",
            "提出解决方案措施",
            "保持投资者信心",
            "指定专人负责沟通",
            "控制信息传播范围",
            "避免恐慌性言论",
            "持续更新处理进展",
        ]
    }
}

impl Rule for InvestorRelationsEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【投资者关系礼仪】\n\n\
            信息披露礼仪：\n{}\n\n\
            投资者沟通礼仪：\n{}\n\n\
            股东大会礼仪：\n{}\n\n\
            股东服务礼仪：\n{}\n\n\
            投资者路演礼仪：\n{}\n\n\
            分析师关系礼仪：\n{}\n\n\
            机构投资者礼仪：\n{}\n\n\
            危机沟通礼仪：\n{}",
            self.disclosure()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.communication()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.shareholder_meeting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.shareholder_service()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.roadshow()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.analyst_relations()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.institutional_investors()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.crisis_communication()
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
    fn test_investor_relations_rules() {
        let rules = InvestorRelationsEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "投资者关系礼仪");
        assert!(!rules.disclosure().is_empty());
        assert!(!rules.communication().is_empty());
        assert!(!rules.shareholder_meeting().is_empty());
        assert!(!rules.shareholder_service().is_empty());
        assert!(!rules.roadshow().is_empty());
        assert!(!rules.analyst_relations().is_empty());
        assert!(!rules.institutional_investors().is_empty());
        assert!(!rules.crisis_communication().is_empty());
    }

    #[test]
    fn test_investor_relations_validation() {
        let rules = InvestorRelationsEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_investor_relations_explain() {
        let rules = InvestorRelationsEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("信息披露礼仪"));
        assert!(explanation.contains("股东大会礼仪"));
        assert!(explanation.contains("危机沟通礼仪"));
    }
}
