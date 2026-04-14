//! 证券法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 证券法规则
pub struct SecuritiesLawRules {
    metadata: RuleMetadata,
}

impl SecuritiesLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "证券法规则",
                "中国证券法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "证券法".into()]),
        }
    }

    /// 证券类型
    pub fn securities_types(&self) -> Vec<&'static str> {
        vec![
            "股票: 公司股权凭证",
            "债券: 债务凭证",
            "证券投资基金: 集合投资",
            "存托凭证: 境外证券凭证",
            "资产支持证券: ABS",
            "期货合约: 标准化合约",
            "期权合约: 选择权凭证",
            "其他衍生品",
        ]
    }

    /// 证券发行规则
    pub fn securities_issuance(&self) -> Vec<&'static str> {
        vec![
            "公开发行: 向不特定对象发行",
            "非公开发行: 向特定对象发行",
            "IPO: 首次公开发行",
            "增发: 增加发行新股",
            "配股: 向原股东配售",
            "发行注册制: 注册审核",
            "信息披露要求: 发行信息公开",
            "发行条件: 财务合规等条件",
        ]
    }

    /// 证券交易规则
    pub fn securities_trading(&self) -> Vec<&'static str> {
        vec![
            "证券交易所: 集中交易场所",
            "场外交易: 柜台交易",
            "集中竞价: 撮合成交",
            "大宗交易: 大额交易",
            "融资融券: 信用交易",
            "转融通: 借券机制",
            "交易时间: 规定时段",
            "T+1制度: 次日交割",
        ]
    }

    /// 信息披露规则
    pub fn information_disclosure(&self) -> Vec<&'static str> {
        vec![
            "定期报告: 年报半年报季报",
            "临时报告: 重大事项公告",
            "招股说明书: 发行文件",
            "募集说明书: 债券发行文件",
            "真实准确完整原则",
            "及时披露义务",
            "公平披露原则",
            "内幕信息保密义务",
        ]
    }

    /// 内幕交易规制
    pub fn insider_trading_rules(&self) -> Vec<&'static str> {
        vec![
            "内幕信息定义: 未公开重大信息",
            "内幕人员: 知悉内幕信息者",
            "禁止内幕交易",
            "禁止泄露内幕信息",
            "禁止建议他人买卖",
            "内幕信息敏感期",
            "违法所得认定",
            "法律责任: 行政民事刑事",
        ]
    }

    /// 操纵市场规制
    pub fn market_manipulation_rules(&self) -> Vec<&'static str> {
        vec![
            "连续买卖操纵: 连续买卖影响价格",
            "约定交易操纵: 对倒对敲",
            "洗售操纵: 自买自卖",
            "蛊惑交易操纵: 虚假信息",
            "抢帽子交易: 先买后荐",
            "虚假申报操纵: 虚假报单",
            "跨市场操纵: 关联市场操纵",
            "法律责任认定",
        ]
    }

    /// 上市公司治理
    pub fn corporate_governance(&self) -> Vec<&'static str> {
        vec![
            "独立董事制度: 独立董事比例",
            "董事会专门委员会",
            "股东大会规则",
            "关联交易披露",
            "重大事项决策程序",
            "投资者保护机制",
            "分红政策披露",
            "退市制度规定",
        ]
    }

    /// 投资者保护
    pub fn investor_protection(&self) -> Vec<&'static str> {
        vec![
            "投资者适当性管理",
            "风险揭示义务",
            "投资者教育",
            "投诉举报渠道",
            "先行赔付制度",
            "集体诉讼制度",
            "代表人诉讼制度",
            "投资者赔偿基金",
        ]
    }

    /// 证券监管机构
    pub fn regulatory_authorities(&self) -> Vec<&'static str> {
        vec![
            "证监会: 证券监管机构",
            "证券交易所: 交易场所监管",
            "证券业协会: 行业自律",
            "中证中小投资者服务中心",
            "证监会派出机构",
            "稽查执法部门",
            "行政处罚委员会",
            "行政复议机构",
        ]
    }
}

impl Default for SecuritiesLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SecuritiesLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("securities")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【证券法规则】\n\n证券类型:\n{}\n\n信息披露:\n{}\n\n内幕交易规制:\n{}\n",
            self.securities_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.information_disclosure().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.insider_trading_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_securities_law_rules() {
        let rules = SecuritiesLawRules::new();
        assert!(!rules.securities_types().is_empty());
        assert!(!rules.information_disclosure().is_empty());
    }
}