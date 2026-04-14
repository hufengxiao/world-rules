//! 公司法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 公司法规则
pub struct CompanyLawRules {
    metadata: RuleMetadata,
}

impl CompanyLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "公司法规则",
                "中国公司法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "公司法".into()]),
        }
    }

    /// 公司类型
    pub fn company_types(&self) -> Vec<&'static str> {
        vec![
            "有限责任公司: 50人以下股东",
            "股份有限公司: 发起人2-200人",
            "一人有限责任公司: 单一股东",
            "国有独资公司: 国家单独出资",
            "外商投资企业: 外资参与",
            "集团公司: 多公司组合",
        ]
    }

    /// 公司设立条件
    pub fn establishment_conditions(&self) -> Vec<&'static str> {
        vec![
            "有符合法定人数的股东",
            "有符合规定的公司章程",
            "有公司名称和组织机构",
            "有公司住所",
            "有符合规定的资本",
            "依法进行登记注册",
        ]
    }

    /// 股东权利
    pub fn shareholder_rights(&self) -> Vec<&'static str> {
        vec![
            "资产收益权: 分红、剩余财产分配",
            "参与决策权: 参加股东会、表决",
            "选择管理者权: 选举董事监事",
            "知情权: 查阅公司文件",
            "股份转让权: 转让股权",
            "优先购买权: 新股优先认购",
            "诉讼权: 提起股东诉讼",
        ]
    }

    /// 股东义务
    pub fn shareholder_obligations(&self) -> Vec<&'static str> {
        vec![
            "缴纳出资义务",
            "不得抽逃出资",
            "遵守公司章程",
            "不得滥用股东权利",
            "对公司债务承担有限责任",
        ]
    }

    /// 公司治理结构
    pub fn governance_structure(&self) -> Vec<&'static str> {
        vec![
            "股东会/股东大会: 最高权力机构",
            "董事会: 经营决策机构",
            "监事会: 监督机构",
            "经理层: 执行机构",
            "法定代表人: 对外代表公司",
        ]
    }

    /// 公司资本规则
    pub fn capital_rules(&self) -> Vec<&'static str> {
        vec![
            "注册资本: 股东认缴出资总额",
            "实缴资本: 实际缴纳的出资",
            "资本公积: 超出注册资本部分",
            "盈余公积: 法定盈余公积10%",
            "法定公积金提取比例限制",
            "资本维持原则: 保持资本完整",
        ]
    }

    /// 公司合并分立
    pub fn merger_division(&self) -> Vec<&'static str> {
        vec![
            "吸收合并: 一公司存续",
            "新设合并: 成立新公司",
            "派生分立: 原公司存续",
            "新设分立: 成立新公司",
            "编制资产负债表财产清单",
            "通知债权人并公告",
            "债权人可要求清偿或担保",
        ]
    }

    /// 公司解散清算
    pub fn dissolution_liquidation(&self) -> Vec<&'static str> {
        vec![
            "解散原因: 章程规定、股东决议、破产",
            "成立清算组: 清算公司财产",
            "通知公告债权人",
            "清偿债务分配剩余财产",
            "注销登记: 终止公司",
            "清算组责任: 妥善清算",
        ]
    }
}

impl Default for CompanyLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CompanyLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("company")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【公司法规则】\n\n公司类型:\n{}\n\n股东权利:\n{}\n\n治理结构:\n{}\n",
            self.company_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.shareholder_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.governance_structure().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_law_rules() {
        let rules = CompanyLawRules::new();
        assert!(!rules.company_types().is_empty());
        assert!(!rules.shareholder_rights().is_empty());
    }
}