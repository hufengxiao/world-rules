//! 消费者权益保护法规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 消费者权益保护法规则
pub struct ConsumerLawRules {
    metadata: RuleMetadata,
}

impl ConsumerLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "消费者权益保护法",
                "中国消费者权益保护基本规则"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "消费者".into()]),
        }
    }

    /// 消费者权利
    pub fn consumer_rights(&self) -> Vec<&'static str> {
        vec![
            "安全权: 人身、财产安全不受损害",
            "知情权: 了解商品/服务真实情况",
            "选择权: 自主选择商品/服务",
            "公平交易权: 公平交易条件",
            "索赔权: 受损时获得赔偿",
            "结社权: 成立社会团体",
            "受教育权: 消费知识教育",
            "受尊重权: 人格尊严受尊重",
            "监督权: 监督商品/服务",
        ]
    }

    /// 经营者义务
    pub fn business_obligations(&self) -> Vec<&'static str> {
        vec![
            "保证商品/服务质量",
            "明码标价",
            "出具购货凭证",
            "不得强制交易",
            "保证人身财产安全",
            "提供真实信息",
            "承担售后责任",
        ]
    }

    /// 退换货规则
    pub fn return_rules(&self) -> Vec<&'static str> {
        vec![
            "7天无理由退货 (网购)",
            "质量问题15天内可换货",
            "三包有效期: 修理、更换、退货",
            "食品、定制商品等除外",
        ]
    }

    /// 赔偿标准
    pub fn compensation_rules(&self) -> Vec<&'static str> {
        vec![
            "欺诈行为: 退一赔三 (最低500元)",
            "食品安全: 退一赔十 (最低1000元)",
            "人身伤害: 医疗费、误工费、精神损害抚慰金",
            "财产损失: 按实际损失赔偿",
        ]
    }

    /// 投诉渠道
    pub fn complaint_channels(&self) -> Vec<&'static str> {
        vec![
            "12315 消费者投诉热线",
            "12315 网站/APP",
            "消费者协会",
            "市场监督管理部门",
            "人民法院起诉",
        ]
    }

    /// 不适用7天无理由退货的商品
    pub fn no_return_exceptions(&self) -> Vec<&'static str> {
        vec![
            "消费者定作的商品",
            "鲜活易腐的商品",
            "在线下载或拆封的音像制品",
            "交付的报纸、期刊",
            "拆封的化妆品",
        ]
    }

    /// 维权时效
    pub fn statute_of_limitations(&self) -> Vec<&'static str> {
        vec![
            "一般诉讼时效: 3年",
            "产品缺陷损害: 2年 (知道或应当知道时起)",
            "最长保护期: 10年 (从交付时起)",
        ]
    }
}

impl Default for ConsumerLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ConsumerLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("consumer")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【消费者权益保护法】\n\n\
            消费者权利:\n{}\n\n\
            经营者义务:\n{}\n\n\
            退换货规则:\n{}\n\n\
            赔偿标准:\n{}\n\n\
            投诉渠道:\n{}\n\n\
            不适用7天退货的商品:\n{}\n",
            self.consumer_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.business_obligations().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.return_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.compensation_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.complaint_channels().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.no_return_exceptions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_law() {
        let rules = ConsumerLawRules::new();
        assert!(rules.consumer_rights().contains(&"安全权: 人身、财产安全不受损害"));
    }
}