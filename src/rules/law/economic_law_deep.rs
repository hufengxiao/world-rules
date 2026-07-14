//! 经济法深度规则
//!
//! 涵盖经济法核心领域的详细内容，包括：
//! - 竞争法深度规则
//! - 消费者权益保护深度规则
//! - 产品质量深度规则
//! - 金融监管深度规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: EconomicLawDeepRules,
    name: "经济法深度规则",
    desc: "经济法核心领域的详细规则解析",
    origin: "中国",
    tags: ["法律", "经济法", "竞争法", "消费者权益"]
}

impl EconomicLawDeepRules {
    /// 竞争法深度规则
    pub fn competition_law_detailed(&self) -> Vec<&'static str> {
        vec![
            "垄断协议禁止: 禁止经营者达成垄断协议，排除、限制竞争",
            "横向垄断协议: 禁止具有竞争关系的经营者达成固定价格、限制产量、分割市场等协议",
            "纵向垄断协议: 禁止经营者与交易相对人达成固定转售价格、限定最低转售价格等协议",
            "滥用市场支配地位: 禁止具有市场支配地位的经营者滥用市场支配地位",
            "市场支配地位认定: 依据经营者在相关市场的市场份额、控制市场的能力等因素认定",
            "经营者集中申报: 经营者集中达到国务院规定的申报标准的，应当事先申报",
            "经营者集中审查: 国务院反垄断执法机构应当对申报的经营者集中进行审查",
            "行政性垄断禁止: 行政机关和法律授权的具有管理公共事务职能的组织不得滥用行政权力排除、限制竞争",
            "反垄断执法: 反垄断执法机构依法对涉嫌垄断行为进行调查",
            "法律责任: 经营者达成垄断协议、滥用市场支配地位的，处以上一年度销售额1%-10%的罚款",
        ]
    }

    /// 消费者权益保护深度规则
    pub fn consumer_protection_detailed(&self) -> Vec<&'static str> {
        vec![
            "消费者权利: 消费者享有安全权、知情权、选择权、公平交易权、求偿权等权利",
            "安全保障义务: 经营者应当保证其提供的商品或服务符合保障人身、财产安全的要求",
            "真实信息告知: 经营者应当向消费者提供商品或服务的真实信息，不得作虚假或引人误解的宣传",
            "质量保证义务: 经营者应当保证在正常使用商品或提供服务的情况下其提供的商品或服务应当具有的质量",
            "退货换货义务: 经营者提供的商品或服务不符合质量要求的，消费者有权要求退货、换货",
            "惩罚性赔偿: 经营者提供商品或服务有欺诈行为的，应当按照消费者的要求增加赔偿三倍",
            "个人信息保护: 经营者收集、使用消费者个人信息，应当遵循合法、正当、必要原则",
            "消费争议解决: 消费者和经营者发生消费者权益争议的，可以通过协商、调解、投诉、仲裁、诉讼解决",
            "消费者组织: 消费者协会和其他消费者组织是依法成立的对商品和服务进行社会监督的保护消费者合法权益的社会组织",
            "举证责任倒置: 经营者提供的耐用商品或服务，消费者自接受之日起六个月内发现瑕疵发生争议的，由经营者承担有关瑕疵的举证责任",
        ]
    }

    /// 产品质量深度规则
    pub fn product_quality_detailed(&self) -> Vec<&'static str> {
        vec![
            "产品质量要求: 产品应当符合国家标准、行业标准，不存在危及人身、财产安全的不合理危险",
            "生产者责任: 生产者应当对其生产的产品质量负责，产品存在缺陷造成损害的应当承担侵权责任",
            "销售者责任: 销售者应当建立并执行进货检查验收制度，验明产品合格证明和其他标识",
            "产品标识: 产品或其包装上的标识必须真实，有产品质量检验合格证明",
            "缺陷产品召回: 生产者发现其生产的产品存在缺陷的，应当及时采取停止生产、销售、警示、召回等措施",
            "产品质量监督: 国务院市场监督管理部门主管全国产品质量监督工作",
            "质量检验: 产品质量应当检验合格，不得以不合格产品冒充合格产品",
            "产品质量责任: 因产品存在缺陷造成人身、缺陷产品以外的其他财产损害的，生产者应当承担赔偿责任",
            "诉讼时效: 因产品存在缺陷造成损害要求赔偿的诉讼时效期间为两年",
            "免责情形: 生产者能够证明未将产品投入流通、产品投入流通时引起损害的缺陷尚不存在等情形的，不承担赔偿责任",
        ]
    }

    /// 金融监管深度规则
    pub fn financial_regulation_detailed(&self) -> Vec<&'static str> {
        vec![
            "银行业监管: 银行业金融机构应当严格遵守审慎经营规则，建立健全风险管理和内部控制制度",
            "资本充足率: 商业银行资本充足率不得低于8%，核心一级资本充足率不得低于5%",
            "流动性要求: 商业银行流动性覆盖率不得低于100%，流动性比例不得低于25%",
            "贷款集中度: 商业银行对同一借款人的贷款余额与商业银行资本余额的比例不得超过10%",
            "证券业监管: 证券公司应当建立健全内部控制制度，采取有效隔离措施防范利益冲突",
            "保险业监管: 保险公司应当具有与其业务规模和风险水平相适应的偿付能力",
            "支付结算监管: 支付机构应当遵守支付结算管理规定，保障客户资金安全",
            "金融消费者保护: 金融机构应当建立金融消费者权益保护机制，妥善处理金融消费者投诉",
            "反洗钱义务: 金融机构应当履行客户身份识别、客户身份资料和交易记录保存、大额和可疑交易报告义务",
            "跨境监管合作: 金融监督管理机构应当加强跨境监管合作，防范跨境金融风险",
        ]
    }
}

impl Rule for EconomicLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("economic_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "经济法深度规则",
            &[
                ("竞争法深度规则", &self.competition_law_detailed()),
                (
                    "消费者权益保护深度规则",
                    &self.consumer_protection_detailed(),
                ),
                ("产品质量深度规则", &self.product_quality_detailed()),
                ("金融监管深度规则", &self.financial_regulation_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_economic_law_deep_rules() {
        let rules = EconomicLawDeepRules::new();
        assert_eq!(rules.metadata().name, "经济法深度规则");
        assert!(!rules.competition_law_detailed().is_empty());
        assert!(!rules.consumer_protection_detailed().is_empty());
        assert!(!rules.product_quality_detailed().is_empty());
        assert!(!rules.financial_regulation_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_competition_law_count() {
        let rules = EconomicLawDeepRules::new();
        assert_eq!(rules.competition_law_detailed().len(), 10);
    }

    #[test]
    fn test_consumer_protection_count() {
        let rules = EconomicLawDeepRules::new();
        assert_eq!(rules.consumer_protection_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = EconomicLawDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("economic_law_deep"));
    }
}
