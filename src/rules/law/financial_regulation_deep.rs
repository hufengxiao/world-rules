//! 金融监管法深度规则
//!
//! 涵盖金融监管核心领域的详细内容，包括：
//! - 银行业监管规则
//! - 证券业监管规则
//! - 保险业监管规则
//!
//! # 法律依据
//!
//! 主要依据：
//! - 《中华人民共和国银行业监督管理法》
//! - 《中华人民共和国证券法》（2019年修订）
//! - 《中华人民共和国保险法》（2015年修正）
//! - 《中华人民共和国商业银行法》
//! - 《巴塞尔协议III》

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
use serde::{Deserialize, Serialize};

/// 金融机构类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinancialInstitution {
    /// 商业银行
    CommercialBank,
    /// 政策性银行
    PolicyBank,
    /// 证券公司
    SecuritiesCompany,
    /// 保险公司
    InsuranceCompany,
    /// 基金管理公司
    FundManagementCompany,
    /// 信托公司
    TrustCompany,
    /// 金融资产管理公司
    FinancialAssetManagementCompany,
    /// 金融租赁公司
    FinancialLeasingCompany,
}

/// 风险类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskType {
    /// 信用风险
    Credit,
    /// 市场风险
    Market,
    /// 操作风险
    Operational,
    /// 流动性风险
    Liquidity,
    /// 声誉风险
    Reputation,
    /// 法律风险
    Legal,
    /// 信息科技风险
    InformationTechnology,
}

/// 监管指标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulatoryIndicator {
    /// 资本充足率
    CapitalAdequacyRatio,
    /// 核心一级资本充足率
    CoreTier1CapitalRatio,
    /// 流动性覆盖率
    LiquidityCoverageRatio,
    /// 流动性比例
    LiquidityRatio,
    /// 拨备覆盖率
    ProvisionCoverageRatio,
    /// 贷款拨备率
    LoanProvisionRatio,
    /// 不良贷款率
    NonPerformingLoanRatio,
    /// 单一客户贷款集中度
    SingleCustomerLoanConcentration,
}

/// 监管措施类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupervisionMeasure {
    /// 责令改正
    OrderCorrection,
    /// 限制业务活动
    RestrictBusiness,
    /// 责令暂停部分业务
    SuspendBusiness,
    /// 限制分配红利
    RestrictDividend,
    /// 责令调整董事高管
    AdjustDirectors,
    /// 责令调整业务规模
    AdjustBusinessScale,
    /// 责令停止批准新业务
    StopNewBusiness,
    /// 撤销有关业务许可
    RevokeBusinessLicense,
}

/// 银行业监管参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankingRegulationParams {
    /// 资本充足率（%）
    pub capital_adequacy_ratio: f64,
    /// 核心一级资本充足率（%）
    pub core_tier1_capital_ratio: f64,
    /// 流动性覆盖率（%）
    pub liquidity_coverage_ratio: f64,
    /// 流动性比例（%）
    pub liquidity_ratio: f64,
    /// 不良贷款率（%）
    pub non_performing_loan_ratio: f64,
    /// 单一客户贷款集中度（%）
    pub single_customer_concentration: f64,
}

simple_rule! {
    struct: FinancialRegulationDeepRules,
    name: "金融监管法深度规则",
    desc: "金融监管法的详细规则解析",
    origin: "中国",
    tags: ["法律", "经济法", "金融监管"]
}

impl FinancialRegulationDeepRules {
    /// 验证银行业监管指标合规性
    ///
    /// # 参数
    /// - `params`: 银行业监管参数
    ///
    /// # 返回
    /// (是否合规, 不合规项列表)
    pub fn validate_banking_indicators(
        &self,
        params: &BankingRegulationParams,
    ) -> (bool, Vec<String>) {
        let mut issues = Vec::new();

        // 资本充足率 >= 8%
        if params.capital_adequacy_ratio < 8.0 {
            issues.push(format!(
                "资本充足率不足：{}% < 8%",
                params.capital_adequacy_ratio
            ));
        }

        // 核心一级资本充足率 >= 5%
        if params.core_tier1_capital_ratio < 5.0 {
            issues.push(format!(
                "核心一级资本充足率不足：{}% < 5%",
                params.core_tier1_capital_ratio
            ));
        }

        // 流动性覆盖率 >= 100%
        if params.liquidity_coverage_ratio < 100.0 {
            issues.push(format!(
                "流动性覆盖率不足：{}% < 100%",
                params.liquidity_coverage_ratio
            ));
        }

        // 流动性比例 >= 25%
        if params.liquidity_ratio < 25.0 {
            issues.push(format!("流动性比例不足：{}% < 25%", params.liquidity_ratio));
        }

        // 不良贷款率预警阈值（示例：5%）
        if params.non_performing_loan_ratio > 5.0 {
            issues.push(format!(
                "不良贷款率过高：{}% > 5%",
                params.non_performing_loan_ratio
            ));
        }

        // 单一客户贷款集中度 <= 10%
        if params.single_customer_concentration > 10.0 {
            issues.push(format!(
                "单一客户贷款集中度过高：{}% > 10%",
                params.single_customer_concentration
            ));
        }

        (issues.is_empty(), issues)
    }

    /// 确定风险等级
    ///
    /// # 参数
    /// - `risk_type`: 风险类型
    /// - `severity`: 严重程度（1-5）
    ///
    /// # 返回
    /// 风险等级描述
    pub fn determine_risk_level(&self, _risk_type: RiskType, severity: u32) -> &'static str {
        match severity {
            1 => "低风险：需要关注并持续监测",
            2 => "中低风险：需要加强管控措施",
            3 => "中风险：需要采取纠正措施",
            4 => "中高风险：需要立即采取行动",
            5 => "高风险：需要紧急处置",
            _ => "未知风险等级",
        }
    }

    /// 推荐监管措施
    ///
    /// # 参数
    /// - `non_compliant_count`: 不合规指标数量
    /// - `severity`: 问题严重程度（1-5）
    ///
    /// # 返回
    /// 推荐的监管措施
    pub fn recommend_supervision_measure(
        &self,
        non_compliant_count: usize,
        severity: u32,
    ) -> SupervisionMeasure {
        if non_compliant_count == 0 {
            return SupervisionMeasure::OrderCorrection; // 无需特殊措施
        }

        match severity {
            1 => SupervisionMeasure::OrderCorrection,
            2 => SupervisionMeasure::RestrictBusiness,
            3 => SupervisionMeasure::SuspendBusiness,
            4 => SupervisionMeasure::AdjustDirectors,
            5 => SupervisionMeasure::RevokeBusinessLicense,
            _ => SupervisionMeasure::OrderCorrection,
        }
    }

    /// 银行业监管规则
    pub fn banking_regulation_rules(&self) -> Vec<&'static str> {
        vec![
            "审慎经营原则: 银行业金融机构应当严格遵守审慎经营规则建立健全风险管理和内部控制制度",
            "资本充足率要求: 商业银行资本充足率不得低于8%核心一级资本充足率不得低于5%",
            "流动性要求: 商业银行流动性覆盖率不得低于100%流动性比例不得低于25%",
            "贷款集中度限制: 商业银行对同一借款人的贷款余额与商业银行资本余额的比例不得超过10%",
            "关联交易管理: 银行业金融机构应当建立关联交易管理制度防范关联交易风险",
            "信息披露义务: 银行业金融机构应当真实准确完整及时地披露财务会计报告风险管理状况等信息",
            "内部控制制度: 银行业金融机构应当建立完善的内部控制制度确保业务活动合法合规",
            "风险管理体系: 银行业金融机构应当建立全面的风险管理体系识别计量监测和控制各类风险",
            "资产质量分类: 银行业金融机构应当按照规定对资产进行风险分类真实反映资产质量",
            "准备金计提: 银行业金融机构应当按照规定计提贷款损失准备金和其他资产减值准备",
        ]
    }

    /// 证券业监管规则
    pub fn securities_regulation_rules(&self) -> Vec<&'static str> {
        vec![
            "内部控制制度: 证券公司应当建立健全内部控制制度采取有效隔离措施防范利益冲突",
            "客户资金管理: 证券公司应当建立健全客户资金管理制度保障客户资金安全",
            "风险准备金: 证券公司应当按规定提取风险准备金用于弥补证券交易损失",
            "净资本要求: 证券公司的净资本等风险控制指标应当符合国务院证券监督管理机构的规定",
            "投资者适当性: 证券公司应当了解客户的情况向客户销售与其风险识别能力和承担能力相适应的证券产品",
            "信息披露义务: 证券公司应当真实准确完整及时地披露财务会计报告风险管理状况等信息",
            "禁止欺诈行为: 证券公司及其从业人员不得在证券交易活动中作出虚假陈述或者信息误导",
            "禁止内幕交易: 证券交易内幕信息的知情人和非法获取内幕信息的人在内幕信息公开前不得买卖该证券",
            "禁止操纵市场: 禁止任何人操纵证券市场操纵证券市场行为给投资者造成损失的应当依法承担赔偿责任",
            "投资者保护: 国家设立证券投资者保护基金用于证券公司撤销关闭和破产时保护投资者合法权益",
        ]
    }

    /// 保险业监管规则
    pub fn insurance_regulation_rules(&self) -> Vec<&'static str> {
        vec![
            "偿付能力要求: 保险公司应当具有与其业务规模和风险水平相适应的偿付能力",
            "偿付能力充足率: 保险公司偿付能力充足率不得低于100%",
            "保险保障基金: 保险公司应当按规定提取保险保障基金用于在保险公司被撤销关闭和破产时保障保单持有人的利益",
            "准备金计提: 保险公司应当按照规定提取各项责任准备金真实反映保险公司承担的保险责任",
            "资金运用管理: 保险公司的资金运用必须稳健遵循安全性原则",
            "资金运用范围: 保险公司资金运用限于银行存款买卖债券股票证券投资基金份额等",
            "关联交易管理: 保险公司应当建立关联交易管理制度防范关联交易风险",
            "信息披露义务: 保险公司应当真实准确完整及时地披露财务会计报告风险管理状况等信息",
            "再保险管理: 保险公司应当按照规定办理再保险业务分散风险",
            "保险条款审批: 关系社会公众利益的保险险种依法实行强制保险的险种等的保险条款应当经保险监督管理机构审批",
        ]
    }

    /// 反洗钱规则
    pub fn anti_money_laundering_rules(&self) -> Vec<&'static str> {
        vec![
            "客户身份识别: 金融机构应当履行客户身份识别义务了解客户及其交易目的",
            "客户身份资料保存: 金融机构应当保存客户身份资料和交易记录至少保存五年",
            "大额交易报告: 金融机构发现大额交易应当向反洗钱行政主管部门报告",
            "可疑交易报告: 金融机构发现可疑交易应当向反洗钱行政主管部门报告",
            "客户风险分类: 金融机构应当对客户进行风险分类采取相应的风险控制措施",
            "持续监控义务: 金融机构应当对客户交易进行持续监控发现异常及时报告",
            "内部制度: 金融机构应当建立健全反洗钱内部控制制度",
            "培训义务: 金融机构应当对员工进行反洗钱培训和宣传",
            "配合调查义务: 金融机构应当配合反洗钱行政主管部门的调查",
            "法律责任: 金融机构未履行反洗钱义务的由反洗钱行政主管部门责令限期改正给予警告处以罚款",
        ]
    }

    /// 金融消费者保护规则
    pub fn consumer_protection_rules(&self) -> Vec<&'static str> {
        vec![
            "适当性义务: 金融机构向金融消费者推荐产品时应当履行适当性义务将合适的产品提供给合适的金融消费者",
            "信息披露义务: 金融机构应当以显著方式向金融消费者披露产品风险收益费用等关键信息",
            "告知说明义务: 金融机构应当以通俗易懂的语言向金融消费者说明产品的重要条款和风险",
            "销售禁止行为: 金融机构不得欺诈误导诱导金融消费者购买与其风险承受能力不匹配的产品",
            "投诉处理机制: 金融机构应当建立健全金融消费者投诉处理机制及时妥善处理投诉",
            "个人信息保护: 金融机构应当保护金融消费者的个人信息安全不得非法收集使用泄露个人信息",
            "销售行为规范: 金融机构销售人员应当具备相应的专业素质和职业操守",
            "理财业务规范: 金融机构开展理财业务应当做到卖者尽责买者自负",
            "信用卡业务规范: 金融机构开展信用卡业务应当审慎经营规范营销行为",
            "法律责任: 金融机构侵害金融消费者合法权益的依法承担民事责任和行政责任",
        ]
    }

    /// 跨境金融监管规则
    pub fn cross_border_rules(&self) -> Vec<&'static str> {
        vec![
            "跨境监管合作: 金融监督管理机构应当加强跨境监管合作防范跨境金融风险",
            "信息交换: 金融监督管理机构应当与境外监管机构建立信息交换机制",
            "跨境金融机构监管: 在中国境内设立的外资金融机构应当遵守中国法律法规接受中国监管机构的监管",
            "跨境业务风险管理: 金融机构开展跨境业务应当建立健全风险管理制度",
            "外汇管理: 金融机构开展跨境业务应当遵守外汇管理规定",
            "反洗钱跨境合作: 金融机构应当配合跨境反洗钱监管合作",
            "跨境数据流动: 金融机构跨境传输数据应当遵守数据安全法律法规",
            "海外分支机构监管: 中国金融机构的海外分支机构应当接受当地监管机构的监管并向国内监管机构报告",
            "跨境风险处置: 金融监督管理机构应当参与跨境金融机构风险处置的国际合作",
            "国际标准接轨: 金融监管制度应当与国际标准接轨促进金融市场开放",
        ]
    }
}

impl Rule for FinancialRegulationDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("financial_regulation_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "金融监管法深度规则",
            &[
                ("银行业监管规则", &self.banking_regulation_rules()),
                ("证券业监管规则", &self.securities_regulation_rules()),
                ("保险业监管规则", &self.insurance_regulation_rules()),
                ("反洗钱规则", &self.anti_money_laundering_rules()),
                ("消费者保护规则", &self.consumer_protection_rules()),
                ("跨境监管规则", &self.cross_border_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_financial_regulation_deep_rules() {
        let rules = FinancialRegulationDeepRules::new();
        assert_eq!(rules.metadata().name, "金融监管法深度规则");
        assert!(!rules.banking_regulation_rules().is_empty());
        assert!(!rules.securities_regulation_rules().is_empty());
        assert!(!rules.insurance_regulation_rules().is_empty());
        assert!(!rules.anti_money_laundering_rules().is_empty());
        assert!(!rules.consumer_protection_rules().is_empty());
        assert!(!rules.cross_border_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_banking_regulation_count() {
        let rules = FinancialRegulationDeepRules::new();
        assert_eq!(rules.banking_regulation_rules().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = FinancialRegulationDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("financial_regulation_deep")
        );
    }

    #[test]
    fn test_validate_banking_indicators_compliant() {
        let rules = FinancialRegulationDeepRules::new();
        let params = BankingRegulationParams {
            capital_adequacy_ratio: 12.0,
            core_tier1_capital_ratio: 8.0,
            liquidity_coverage_ratio: 110.0,
            liquidity_ratio: 30.0,
            non_performing_loan_ratio: 2.0,
            single_customer_concentration: 8.0,
        };
        let (compliant, issues) = rules.validate_banking_indicators(&params);
        assert!(compliant);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_validate_banking_indicators_non_compliant() {
        let rules = FinancialRegulationDeepRules::new();
        let params = BankingRegulationParams {
            capital_adequacy_ratio: 6.0,         // 低于8%
            core_tier1_capital_ratio: 3.0,       // 低于5%
            liquidity_coverage_ratio: 90.0,      // 低于100%
            liquidity_ratio: 20.0,               // 低于25%
            non_performing_loan_ratio: 8.0,      // 高于5%
            single_customer_concentration: 15.0, // 高于10%
        };
        let (compliant, issues) = rules.validate_banking_indicators(&params);
        assert!(!compliant);
        assert_eq!(issues.len(), 6);
    }

    #[test]
    fn test_determine_risk_level() {
        let rules = FinancialRegulationDeepRules::new();
        assert!(rules
            .determine_risk_level(RiskType::Credit, 1)
            .contains("低风险"));
        assert!(rules
            .determine_risk_level(RiskType::Credit, 5)
            .contains("高风险"));
    }

    #[test]
    fn test_recommend_supervision_measure_low() {
        let rules = FinancialRegulationDeepRules::new();
        let measure = rules.recommend_supervision_measure(1, 1);
        assert_eq!(measure, SupervisionMeasure::OrderCorrection);
    }

    #[test]
    fn test_recommend_supervision_measure_high() {
        let rules = FinancialRegulationDeepRules::new();
        let measure = rules.recommend_supervision_measure(5, 5);
        assert_eq!(measure, SupervisionMeasure::RevokeBusinessLicense);
    }

    #[test]
    fn test_recommend_supervision_measure_no_issue() {
        let rules = FinancialRegulationDeepRules::new();
        let measure = rules.recommend_supervision_measure(0, 1);
        assert_eq!(measure, SupervisionMeasure::OrderCorrection);
    }
}
