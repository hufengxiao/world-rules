//! 税法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 税法规则
pub struct TaxLawRules {
    metadata: RuleMetadata,
}

impl TaxLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "税法规则",
                "中国税法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "税法".into()]),
        }
    }

    /// 税种分类
    pub fn tax_categories(&self) -> Vec<&'static str> {
        vec![
            "流转税: 增值税、消费税、关税",
            "所得税: 企业所得税、个人所得税",
            "财产税: 房产税、契税、车船税",
            "行为税: 印花税、土地增值税",
            "资源税: 资源税、环境保护税",
            "中央税: 消费税、关税",
            "地方税: 房产税、契税",
            "共享税: 增值税、所得税",
        ]
    }

    /// 增值税规则
    pub fn vat_rules(&self) -> Vec<&'static str> {
        vec![
            "基本税率: 13%",
            "低税率: 9%(农产品等)",
            "小规模纳税人征收率: 3%",
            "进项税额抵扣制度",
            "销项税额计算方法",
            "增值税专用发票管理",
            "免税项目规定",
            "出口退税政策",
        ]
    }

    /// 企业所得税规则
    pub fn corporate_income_tax(&self) -> Vec<&'static str> {
        vec![
            "法定税率: 25%",
            "小型微利企业优惠税率",
            "高新技术企业优惠税率: 15%",
            "应纳税所得额计算",
            "税前扣除项目",
            "不得扣除项目规定",
            "亏损弥补年限: 5年",
            "境外所得抵免",
        ]
    }

    /// 个人所得税规则
    pub fn individual_income_tax(&self) -> Vec<&'static str> {
        vec![
            "综合所得: 工资薪金等合并计税",
            "综合所得税率: 3%-45%超额累进",
            "经营所得税率: 5%-35%超额累进",
            "分类所得: 利息股息20%",
            "基本减除费用: 5000元/月",
            "专项扣除: 社保公积金",
            "专项附加扣除: 教育、医疗、住房等",
            "年度汇算清缴制度",
        ]
    }

    /// 税收征收管理
    pub fn tax_administration(&self) -> Vec<&'static str> {
        vec![
            "税务登记: 纳税人登记管理",
            "账簿凭证管理: 建账建制",
            "发票管理: 发票领购使用",
            "纳税申报: 定期申报义务",
            "税款征收: 征收方式与期限",
            "税务检查: 检查权限与程序",
            "税务行政复议: 争议解决",
            "税收保全措施: 保全纳税人财产",
        ]
    }

    /// 税收法律责任
    pub fn tax_liability(&self) -> Vec<&'static str> {
        vec![
            "偷税: 逃避缴纳税款",
            "抗税: 拒不缴纳税款",
            "骗税: 骗取出口退税",
            "欠税: 拖欠税款",
            "罚款: 0.5倍至5倍",
            "滞纳金: 日万分之五",
            "刑事责任: 严重违法入刑",
            "补税追征期限: 3年或无限期",
        ]
    }

    /// 税收优惠类型
    pub fn tax_preferences(&self) -> Vec<&'static str> {
        vec![
            "减免税: 直接减免应纳税额",
            "免税: 免予征税",
            "税率优惠: 降低税率",
            "加计扣除: 扣除额增加",
            "加速折旧: 折旧年限缩短",
            "税收抵免: 抵减应纳税额",
            "延期纳税: 缓缴税款",
            "退税: 退还已缴税款",
        ]
    }

    /// 纳税人权利
    pub fn taxpayer_rights(&self) -> Vec<&'static str> {
        vec![
            "知情权: 了解税收政策",
            "保密权: 商业秘密保护",
            "申请减免税权",
            "申请退税权",
            "陈述申辩权",
            "行政复议权",
            "行政诉讼权",
            "请求赔偿权",
        ]
    }
}

impl Default for TaxLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TaxLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("tax")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【税法规则】\n\n税种分类:\n{}\n\n增值税规则:\n{}\n\n个人所得税:\n{}\n",
            self.tax_categories().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.vat_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.individual_income_tax().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_law_rules() {
        let rules = TaxLawRules::new();
        assert!(!rules.tax_categories().is_empty());
        assert!(!rules.vat_rules().is_empty());
    }
}