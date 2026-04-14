//! 招投标法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 招投标法规则
pub struct BiddingLawRules {
    metadata: RuleMetadata,
}

impl BiddingLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "招投标法规则",
                "中国招投标法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "招投标".into()]),
        }
    }

    /// 招投标原则
    pub fn bidding_principles(&self) -> Vec<&'static str> {
        vec![
            "公开原则: 公开招标",
            "公平原则: 公平竞争",
            "公正原则: 公正评标",
            "诚实信用原则",
            "竞争原则: 自由竞争",
            "择优原则:择优中标",
            "效率原则: 高效招标",
            "廉洁原则: 廉洁招标",
        ]
    }

    /// 招标方式
    pub fn bidding_methods(&self) -> Vec<&'static str> {
        vec![
            "公开招标: 公开竞争招标",
            "邀请招标: 邀请投标招标",
            "竞争性谈判",
            "询价采购方式",
            "单一来源采购",
            "框架协议采购",
            "电子招标投标",
            "两阶段招标",
        ]
    }

    /// 招标程序
    pub fn bidding_procedure(&self) -> Vec<&'static str> {
        vec![
            "招标公告发布",
            "招标文件编制",
            "投标人资格要求",
            "投标文件编制",
            "投标文件提交",
            "开标程序: 开标流程",
            "评标程序: 评标流程",
            "中标结果公示",
        ]
    }

    /// 投标规则
    pub fn tender_rules(&self) -> Vec<&'static str> {
        vec![
            "投标资格条件",
            "投标文件要求",
            "投标保证金",
            "投标截止时间",
            "投标报价要求",
            "投标技术方案",
            "投标承诺要求",
            "联合体投标规则",
        ]
    }

    /// 评标规则
    pub fn evaluation_rules(&self) -> Vec<&'static str> {
        vec![
            "评标委员会组成",
            "评标标准制定",
            "评标方法选择",
            "综合评分法",
            "最低评标价法",
            "技术评分法",
            "评标保密要求",
            "评标回避制度",
        ]
    }

    /// 中标规则
    pub fn winning_rules(&self) -> Vec<&'static str> {
        vec![
            "中标条件确定",
            "中标结果公示",
            "中标通知书发出",
            "中标合同签订",
            "中标保证金处理",
            "中标履约保证",
            "中标变更处理",
            "中标无效情形",
        ]
    }

    /// 招投标禁止行为
    pub fn prohibited_behaviors(&self) -> Vec<&'static str> {
        vec![
            "串通投标禁止",
            "围标串标禁止",
            "虚假投标禁止",
            "行贿投标禁止",
            "歧视性招标禁止",
            "限制竞争禁止",
            "泄露标底禁止",
            "违规评标禁止",
        ]
    }

    /// 招投标监督
    pub fn bidding_supervision(&self) -> Vec<&'static str> {
        vec![
            "招标备案监督",
            "招标过程监督",
            "招标投诉处理",
            "招标异议处理",
            "招标违法行为查处",
            "招标行政处罚",
            "招标信用管理",
            "招标信息公开",
        ]
    }
}

impl Default for BiddingLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BiddingLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("bidding")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【招投标法规则】\n\n招标原则:\n{}\n\n招标方式:\n{}\n\n招标程序:\n{}\n",
            self.bidding_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.bidding_methods().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.bidding_procedure().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bidding_law_rules() {
        let rules = BiddingLawRules::new();
        assert!(!rules.bidding_principles().is_empty());
        assert!(!rules.bidding_procedure().is_empty());
    }
}