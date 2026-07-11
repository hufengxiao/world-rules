//! 供应商关系礼仪
//!
//! 涵盖供应商关系管理相关的礼仪规范，包括供应商选择、合作沟通、履约管理等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SupplierRelationsEtiquetteRules,
    name: "供应商关系礼仪",
    desc: "供应商关系管理礼仪规范，包括供应商选择、合作沟通、履约管理等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "供应商", "采购"]
}

impl SupplierRelationsEtiquetteRules {
    /// 供应商选择礼仪
    pub fn selection(&self) -> Vec<&'static str> {
        vec![
            "公平公正评估供应商",
            "公开招标或采购流程",
            "避免利益冲突",
            "评估供应商资质能力",
            "考虑长期合作潜力",
            "尊重供应商商业机密",
            "及时通知选择结果",
            "对未选中供应商表示感谢",
        ]
    }

    /// 合作沟通礼仪
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "保持开放诚实沟通",
            "定期召开供应商会议",
            "明确需求和期望",
            "倾听供应商建议意见",
            "及时反馈问题解决",
            "分享相关信息和数据",
            "建立有效沟通渠道",
            "尊重供应商工作时间",
        ]
    }

    /// 履约管理礼仪
    pub fn contract_management(&self) -> Vec<&'static str> {
        vec![
            "明确合同条款要求",
            "公平执行合同约定",
            "按时支付货款",
            "合理处理质量问题",
            "协商解决争议分歧",
            "及时通知变更需求",
            "记录履约情况档案",
            "定期评估履约表现",
        ]
    }

    /// 供应商考核礼仪
    pub fn evaluation(&self) -> Vec<&'static str> {
        vec![
            "建立客观评估标准",
            "定期进行供应商评估",
            "公平公正评分评级",
            "提供改进建议反馈",
            "认可优秀供应商表现",
            "帮助供应商改进提升",
            "透明评估过程结果",
            "尊重供应商申诉权利",
        ]
    }

    /// 供应商扶持礼仪
    pub fn support(&self) -> Vec<&'static str> {
        vec![
            "提供技术和管理培训",
            "分享行业最新信息",
            "帮助提升生产能力",
            "共同改进产品质量",
            "提供合理的付款条件",
            "协助解决经营困难",
            "建立长期合作信心",
            "分享市场机会信息",
        ]
    }

    /// 争议处理礼仪
    pub fn dispute_handling(&self) -> Vec<&'static str> {
        vec![
            "冷静理性处理分歧",
            "倾听供应商立场诉求",
            "寻求双方可接受方案",
            "避免情绪化指责",
            "记录争议处理过程",
            "必要时寻求第三方调解",
            "维护合作关系基础",
            "总结预防类似争议",
        ]
    }

    /// 供应商退出礼仪
    pub fn termination(&self) -> Vec<&'static str> {
        vec![
            "提前通知终止合作",
            "说明终止原因理由",
            "妥善处理未完订单",
            "结算应付款项",
            "归还供应商资料财产",
            "保护供应商商业机密",
            "感谢过去的合作支持",
            "保持基本商务礼仪",
        ]
    }

    /// 供应商禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要索要或接受回扣",
            "不要泄露供应商机密",
            "不要无故拖欠货款",
            "不要过度压榨供应商",
            "不要单方面变更合同",
            "不要区别对待同类供应商",
            "不要在争议中采取极端措施",
            "不要损害供应商声誉",
        ]
    }
}

impl Rule for SupplierRelationsEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【供应商关系礼仪】\n\n\
            供应商选择礼仪：\n{}\n\n\
            合作沟通礼仪：\n{}\n\n\
            履约管理礼仪：\n{}\n\n\
            供应商考核礼仪：\n{}\n\n\
            供应商扶持礼仪：\n{}\n\n\
            争议处理礼仪：\n{}\n\n\
            供应商退出礼仪：\n{}\n\n\
            供应商禁忌：\n{}",
            self.selection()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.communication()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.contract_management()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.evaluation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.support()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dispute_handling()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.termination()
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
    fn test_supplier_relations_rules() {
        let rules = SupplierRelationsEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "供应商关系礼仪");
        assert!(!rules.selection().is_empty());
        assert!(!rules.communication().is_empty());
        assert!(!rules.contract_management().is_empty());
        assert!(!rules.evaluation().is_empty());
        assert!(!rules.support().is_empty());
        assert!(!rules.dispute_handling().is_empty());
        assert!(!rules.termination().is_empty());
        assert!(!rules.taboos().is_empty());
    }

    #[test]
    fn test_supplier_relations_validation() {
        let rules = SupplierRelationsEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_supplier_relations_explain() {
        let rules = SupplierRelationsEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("供应商选择礼仪"));
        assert!(explanation.contains("合作沟通礼仪"));
        assert!(explanation.contains("供应商禁忌"));
    }
}