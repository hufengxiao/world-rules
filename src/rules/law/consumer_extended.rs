//! 消费者权益保护法扩展规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 消费者权益保护法扩展规则
pub struct ConsumerLawExtendedRules {
    metadata: RuleMetadata,
}

impl ConsumerLawExtendedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "消费者权益保护法扩展规则",
                "中国消费者权益保护法详细知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "消费者".into()]),
        }
    }

    /// 消费者权利
    pub fn consumer_rights(&self) -> Vec<&'static str> {
        vec![
            "安全权: 人身财产安全权",
            "知情权: 了解商品真实情况",
            "选择权: 自主选择商品服务",
            "公平交易权: 公平交易条件",
            "索赔权: 损害赔偿权利",
            "结社权: 成立社会组织权利",
            "受教育权: 消费知识教育",
            "受尊重权: 人格尊严尊重",
            "监督权: 监督批评权利",
            "个人信息保护权: 信息安全权利",
        ]
    }

    /// 经营者义务
    pub fn operator_obligations(&self) -> Vec<&'static str> {
        vec![
            "法定义务: 遵守法律法规",
            "质量保证义务: 保证商品质量",
            "信息披露义务: 真实信息提供",
            "公平交易义务: 公平交易条件",
            "售后服务义务: 售后服务保障",
            "安全保障义务: 安全防护措施",
            "不得强制交易: 禁止强制买卖",
            "不得虚假宣传: 禁止虚假广告",
        ]
    }

    /// 产品质量规则
    pub fn product_quality(&self) -> Vec<&'static str> {
        vec![
            "产品质量要求: 质量合格要求",
            "产品标识要求: 标识规范要求",
            "产品警示说明: 安全警示标识",
            "产品检验制度: 质量检验制度",
            "产品召回制度: 缺陷产品召回",
            "产品保修制度: 保修责任规定",
            "产品质量认证: 质量认证制度",
            "质量责任追究: 质量责任处罚",
        ]
    }

    /// 产品三包规则
    pub fn warranty_rules(&self) -> Vec<&'static str> {
        vec![
            "包修: 免费维修服务",
            "包换: 更换产品服务",
            "包退: 退货退款服务",
            "三包期限: 各类产品期限",
            "三包凭证: 三包凭证要求",
            "三包条件: 三包适用条件",
            "三包免除: 三包免责情形",
            "维修记录: 维修记录保存",
        ]
    }

    /// 消费争议解决
    pub fn dispute_resolution(&self) -> Vec<&'static str> {
        vec![
            "协商和解: 双方协商处理",
            "调解解决: 消费调解处理",
            "申诉解决: 行政申诉处理",
            "仲裁解决: 仲裁机构仲裁",
            "诉讼解决: 法院诉讼程序",
            "举证责任: 经营者举证",
            "小额诉讼: 小额诉讼程序",
            "公益诉讼: 消费公益诉讼",
        ]
    }

    /// 消费欺诈赔偿
    pub fn fraud_compensation(&self) -> Vec<&'static str> {
        vec![
            "欺诈认定: 欺诈行为认定",
            "三倍赔偿: 欺诈惩罚性赔偿",
            "最低赔偿500元",
            "赔偿范围: 损失赔偿范围",
            "精神损害赔偿: 精神损害赔偿",
            "举证责任: 经营者举证欺诈",
            "赔偿时效: 赔偿请求时效",
            "赔偿执行: 赔偿执行程序",
        ]
    }

    /// 网络消费规则
    pub fn online_consumption(&self) -> Vec<&'static str> {
        vec![
            "网购退货权: 七日无理由退货",
            "网购信息披露: 网络信息要求",
            "网购格式条款: 格式合同规范",
            "网购支付安全: 支付安全保障",
            "网购个人信息: 信息安全保护",
            "网购物流规则: 配送物流规则",
            "网购评价权利: 评价权利保护",
            "网购投诉渠道: 网购投诉处理",
        ]
    }

    /// 消费者组织
    pub fn consumer_organizations(&self) -> Vec<&'static str> {
        vec![
            "消费者协会: 消协组织职能",
            "消费者权益保护委员会",
            "行业消费者组织",
            "组织职责: 维权服务职能",
            "投诉受理: 投诉处理职能",
            "消费调查: 调查监督职能",
            "消费教育: 教育宣传职能",
            "公益诉讼: 公益诉讼职能",
        ]
    }
}

impl Default for ConsumerLawExtendedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ConsumerLawExtendedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("consumer_extended")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【消费者权益保护法扩展规则】\n\n消费者权利:\n{}\n\n经营者义务:\n{}\n\n三包规则:\n{}\n",
            self.consumer_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.operator_obligations().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.warranty_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_law_extended_rules() {
        let rules = ConsumerLawExtendedRules::new();
        assert!(!rules.consumer_rights().is_empty());
        assert!(!rules.warranty_rules().is_empty());
    }
}