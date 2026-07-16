//! 保险法深度规则 - 合同、理赔、争议

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: InsuranceLawDeepRules,
    name: "保险法深度规则",
    desc: "保险法的详细规则解析，涵盖合同、理赔、争议",
    origin: "中国",
    tags: ["法律", "商法", "保险法"]
}

impl InsuranceLawDeepRules {
    /// 保险合同深度规则
    pub fn contract_deep(&self) -> Vec<&'static str> {
        vec![
            "合同主体: 投保人、保险人、被保险人、受益人",
            "合同订立: 投保人提出保险要求，经保险人同意承保，合同成立",
            "保险标的: 财产保险的保险标的为财产及其有关利益，人身保险为人的寿命和身体",
            "保险利益: 投保人对保险标的应当具有保险利益，否则合同无效",
            "如实告知: 投保人应如实告知保险标的或被保险人的有关情况",
            "说明义务: 保险人应说明合同条款，对免责条款应作出提示和明确说明",
            "保险金额: 不超过保险价值，超过部分无效",
            "保险期间: 合同约定的保险责任起讫时间",
            "保险责任: 发生保险事故时保险人承担赔偿或给付保险金责任",
            "责任免除: 合同约定的保险人不承担责任的范围",
        ]
    }

    /// 保险理赔深度规则
    pub fn claims_deep(&self) -> Vec<&'static str> {
        vec![
            "报案义务: 投保人、被保险人或受益人知道保险事故发生后应及时通知保险人",
            "报案时限: 一般应在10日内通知，具体以合同约定为准",
            "证明材料: 提供保险事故的性质、原因、损失程度等证明材料",
            "核定期限: 保险人收到赔偿请求后应及时核定，情形复杂的30日内核定",
            "拒赔通知: 不属于保险责任的，应自作出核定之日起3日内发出拒赔通知书",
            "理赔时限: 达成赔偿协议后10日内履行赔偿或给付保险金义务",
            "先行赔付: 保险人自收到赔偿请求60日内不能确定数额的，应先支付可以确定的部分",
            "理赔争议: 当事人可以协商解决，也可以申请仲裁或提起诉讼",
        ]
    }

    /// 财产保险深度规则
    pub fn property_insurance_deep(&self) -> Vec<&'static str> {
        vec![
            "保险价值: 保险标的的实际价值，可以约定保险价值",
            "足额保险: 保险金额等于保险价值的，按照实际损失赔偿",
            "不足额保险: 保险金额低于保险价值的，按比例赔偿",
            "超额保险: 保险金额超过保险价值的，超过部分无效",
            "重复保险: 同一保险标的向两个以上保险人投保的，各保险人赔偿金额总和不超过保险价值",
            "代位求偿: 保险人赔偿后，在赔偿金额范围内代位行使被保险人对第三者的请求权",
            "委付制度: 保险事故发生后，保险标的全部损失时，被保险人可以将标的物权利转移给保险人",
            "损余处理: 保险事故发生后，保险人支付全部保险金额的，保险标的权利归保险人",
        ]
    }

    /// 人身保险深度规则
    pub fn life_insurance_deep(&self) -> Vec<&'static str> {
        vec![
            "年龄限制: 投保人不得为无民事行为能力人投保以死亡为给付保险金条件的保险",
            "同意原则: 投保以死亡为给付保险金条件的保险，应经被保险人同意并认可保险金额",
            "受益人指定: 被保险人或投保人可以指定一人或数人为受益人",
            "受益顺序: 指定数人为受益人的，可以确定受益顺序和受益份额",
            "受益变更: 投保人或被保险人可以变更受益人，但应书面通知保险人",
            "自杀条款: 以死亡为给付条件的合同，自成立之日起2年内被保险人自杀的，保险人不承担责任",
            "年龄误告: 投保人申报的被保险人年龄不真实，可以更正或解除合同",
            "合同效力: 人身保险合同不存在代位求偿权，保险人支付保险金后不享有向第三者追偿的权利",
        ]
    }

    /// 保险监管深度规则
    pub fn regulation_deep(&self) -> Vec<&'static str> {
        vec![
            "监管机构: 国家金融监督管理总局",
            "机构监管: 保险公司设立、变更、终止需经批准",
            "偿付能力: 保险公司应具有与其业务规模和风险程度相适应的偿付能力",
            "准备金提取: 保险公司应提取各项责任准备金",
            "保险资金运用: 银行存款、债券、股票、证券投资基金、不动产等",
            "关联交易管理: 保险公司与关联方交易应遵守监管规定",
            "信息披露: 保险公司应按规定披露财务会计报告、偿付能力报告等信息",
            "违规处罚: 保险公司违反法律法规的，监管机构可以责令改正、罚款、限制业务范围等",
        ]
    }

    /// 保险争议深度规则
    pub fn dispute_deep(&self) -> Vec<&'static str> {
        vec![
            "协商解决: 保险合同纠纷当事人可以自行协商解决",
            "调解解决: 可以向保险行业协会或保险调解中心申请调解",
            "仲裁解决: 合同约定仲裁条款的，可以向仲裁机构申请仲裁",
            "诉讼解决: 向人民法院提起诉讼",
            "举证责任: 主张权利的一方承担举证责任",
            "不利解释: 保险合同条款有争议的，应作出有利于被保险人和受益人的解释",
            "时效规定: 人寿保险的请求权时效为5年，其他保险为2年",
            "管辖法院: 被告住所地或保险标的物所在地人民法院管辖",
        ]
    }
}

impl Rule for InsuranceLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("insurance_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "保险法深度规则",
            &[
                ("保险合同规则", &self.contract_deep()),
                ("保险理赔规则", &self.claims_deep()),
                ("财产保险规则", &self.property_insurance_deep()),
                ("人身保险规则", &self.life_insurance_deep()),
                ("保险监管规则", &self.regulation_deep()),
                ("保险争议规则", &self.dispute_deep()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insurance_law_deep_rules() {
        let rules = InsuranceLawDeepRules::new();
        assert_eq!(rules.metadata().name, "保险法深度规则");
        assert!(!rules.contract_deep().is_empty());
        assert!(!rules.claims_deep().is_empty());
        assert!(!rules.property_insurance_deep().is_empty());
        assert!(!rules.life_insurance_deep().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_contract_count() {
        let rules = InsuranceLawDeepRules::new();
        assert_eq!(rules.contract_deep().len(), 10);
    }

    #[test]
    fn test_claims_count() {
        let rules = InsuranceLawDeepRules::new();
        assert_eq!(rules.claims_deep().len(), 8);
    }

    #[test]
    fn test_category() {
        let rules = InsuranceLawDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("insurance_law_deep"));
    }
}