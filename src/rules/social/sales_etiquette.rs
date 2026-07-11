//! 销售礼仪
//!
//! 涵盖商务销售活动相关的礼仪规范，包括客户拜访、产品演示、成交谈判等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SalesEtiquetteRules,
    name: "销售礼仪",
    desc: "商务销售活动礼仪规范，包括客户拜访、产品演示、成交谈判等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "销售", "客户"]
}

impl SalesEtiquetteRules {
    /// 客户拜访礼仪
    pub fn client_visit(&self) -> Vec<&'static str> {
        vec![
            "提前预约拜访时间",
            "确认拜访目的和对象",
            "准备拜访相关资料",
            "准时到达约定地点",
            "着装整洁专业",
            "礼貌问候和自我介绍",
            "尊重客户时间安排",
            "拜访结束表示感谢",
        ]
    }

    /// 产品演示礼仪
    pub fn product_demo(&self) -> Vec<&'static str> {
        vec![
            "提前测试演示设备",
            "了解客户具体需求",
            "针对需求调整演示",
            "演示过程中保持专业",
            "清晰介绍产品特点",
            "邀请客户参与体验",
            "耐心回答客户问题",
            "收集客户反馈意见",
        ]
    }

    /// 销售沟通礼仪
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "倾听客户需求问题",
            "理解客户真实需求",
            "提供专业解决方案",
            "避免过度推销",
            "诚实介绍产品能力",
            "不夸大产品效果",
            "尊重客户决策权",
            "保持友好专业态度",
        ]
    }

    /// 价格谈判礼仪
    pub fn price_negotiation(&self) -> Vec<&'static str> {
        vec![
            "了解客户预算范围",
            "提供合理报价方案",
            "解释定价依据理由",
            "灵活处理折扣请求",
            "不轻易暴露底价",
            "寻求双方接受方案",
            "明确付款条件要求",
            "记录协商过程细节",
        ]
    }

    /// 成成交礼仪
    pub fn closing(&self) -> Vec<&'static str> {
        vec![
            "确认成交条件细节",
            "感谢客户信任选择",
            "及时签署合同文件",
            "明确后续服务安排",
            "提供完整的交接材料",
            "承诺服务质量保障",
            "安排后续跟进服务",
            "保持长期客户联系",
        ]
    }

    /// 客户服务礼仪
    pub fn customer_service(&self) -> Vec<&'static str> {
        vec![
            "快速响应客户需求",
            "耐心解答客户疑问",
            "主动提供使用指导",
            "及时处理售后问题",
            "定期回访客户满意度",
            "收集客户改进建议",
            "维护客户关系档案",
            "提供增值服务支持",
        ]
    }

    /// 拒绝处理礼仪
    pub fn handling_rejection(&self) -> Vec<&'static str> {
        vec![
            "理解客户拒绝原因",
            "不强迫客户接受",
            "保持专业态度",
            "留下联系方式",
            "感谢客户的时间",
            "保持未来合作机会",
            "反思销售过程改进",
            "不因拒绝而消极",
        ]
    }

    /// 销售禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要欺骗或误导客户",
            "不要贬低竞争对手",
            "不要过度施压客户",
            "不要泄露客户隐私",
            "不要在客户面前争辩",
            "不要推销不适合的产品",
            "不要违反公司销售政策",
            "不要忽视客户售后服务",
        ]
    }
}

impl Rule for SalesEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【销售礼仪】\n\n\
            客户拜访礼仪：\n{}\n\n\
            产品演示礼仪：\n{}\n\n\
            销售沟通礼仪：\n{}\n\n\
            价格谈判礼仪：\n{}\n\n\
            成成交礼仪：\n{}\n\n\
            客户服务礼仪：\n{}\n\n\
            拒绝处理礼仪：\n{}\n\n\
            销售禁忌：\n{}",
            self.client_visit()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.product_demo()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.communication()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.price_negotiation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.closing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.customer_service()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.handling_rejection()
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
    fn test_sales_rules() {
        let rules = SalesEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "销售礼仪");
        assert!(!rules.client_visit().is_empty());
        assert!(!rules.product_demo().is_empty());
        assert!(!rules.communication().is_empty());
        assert!(!rules.price_negotiation().is_empty());
        assert!(!rules.closing().is_empty());
        assert!(!rules.customer_service().is_empty());
        assert!(!rules.handling_rejection().is_empty());
        assert!(!rules.taboos().is_empty());
    }

    #[test]
    fn test_sales_validation() {
        let rules = SalesEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_sales_explain() {
        let rules = SalesEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("客户拜访礼仪"));
        assert!(explanation.contains("产品演示礼仪"));
        assert!(explanation.contains("销售禁忌"));
    }
}