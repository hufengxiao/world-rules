//! 票据法深度规则 - 汇票、本票、支票

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: NegotiableInstrumentsLawDeepRules,
    name: "票据法深度规则",
    desc: "票据法的详细规则解析，涵盖汇票、本票、支票",
    origin: "中国",
    tags: ["法律", "商法", "票据法"]
}

impl NegotiableInstrumentsLawDeepRules {
    /// 汇票深度规则
    pub fn bill_of_exchange_deep(&self) -> Vec<&'static str> {
        vec![
            "汇票定义: 出票人签发的，委托付款人在见票时或在指定日期无条件支付确定金额给收款人或持票人的票据",
            "出票: 签发票据并将其交付给收款人的票据行为",
            "记载事项: 表明汇票的字样、无条件支付的委托、确定的金额、付款人名称、收款人名称、出票日期、出票人签章",
            "背书: 持票人将票据权利转让给他人或将票据权利授予他人行使的票据行为",
            "背书连续: 票据上记载的背书人与被背书人在票据签章的先后顺序相互衔接",
            "承兑: 汇票付款人承诺在汇票到期日支付汇票金额的票据行为",
            "承兑期限: 见票后定期付款的汇票，持票人应当自出票日起1个月内向付款人提示承兑",
            "付款期限: 见票即付的汇票，自出票日起1个月内付款；定日付款的汇票，自到期日起10日内提示付款",
            "追索权: 汇票到期被拒绝付款的，持票人可以对背书人、出票人以及汇票的其他债务人行使追索权",
            "追索期限: 持票人对前手的追索权，自被拒绝承兑或被拒绝付款之日起6个月",
        ]
    }

    /// 本票深度规则
    pub fn promissory_note_deep(&self) -> Vec<&'static str> {
        vec![
            "本票定义: 出票人签发的，承诺自己在见票时无条件支付确定金额给收款人或持票人的票据",
            "本票特征: 自付票据，出票人自己为付款人",
            "银行本票: 银行签发的，承诺在见票时无条件支付确定金额给收款人或持票人的票据",
            "本票金额: 本票的出票人必须具有支付本票金额的可靠资金来源",
            "付款提示: 本票的持票人未按照规定期限提示见票的，丧失对出票人以外的前手的追索权",
            "本票期限: 本票自出票日起，付款期限最长不得超过2个月",
            "记载事项: 表明本票的字样、无条件支付的承诺、确定的金额、收款人名称、出票日期、出票人签章",
            "本票追索: 持票人对前手的追索权，自被拒绝付款之日起6个月",
        ]
    }

    /// 支票深度规则
    pub fn check_deep(&self) -> Vec<&'static str> {
        vec![
            "支票定义: 出票人签发的，委托办理支票存款业务的银行或其他金融机构在见票时无条件支付确定金额给收款人或持票人的票据",
            "支票类型: 现金支票、转账支票、普通支票",
            "支票金额: 支票的出票人所签发的金额不得超过其付款时在付款人处实有的存款金额，禁止签发空头支票",
            "支票期限: 支票的持票人应当自出票日起10日内提示付款",
            "支票记载: 表明支票的字样、无条件支付的委托、确定的金额、付款人名称、出票日期、出票人签章",
            "支票补记: 支票上的金额可以由出票人授权补记，未补记前的支票不得使用",
            "禁止转让: 出票人在支票上记载不得转让的，支票不得转让",
            "空头支票: 出票人签发空头支票或签发与其预留印鉴不符的支票的，银行应予以退票并按票面金额处以5%但不低于1000元的罚款",
        ]
    }

    /// 票据行为深度规则
    pub fn instrument_acts_deep(&self) -> Vec<&'static str> {
        vec![
            "出票行为: 签发票据并将其交付给收款人的基本票据行为",
            "背书行为: 持票人将票据权利转让给他人的附属票据行为",
            "承兑行为: 汇票付款人承诺支付票据金额的附属票据行为",
            "保证行为: 票据债务人以外的人为担保票据债务履行而在票据上签章的行为",
            "付款行为: 付款人或承兑人向持票人支付票据金额的行为",
            "票据签章: 自然人签名或盖章，单位盖章并加法定代表人或授权代理人签章",
            "票据代理: 代理人应在票据上表明代理关系并签章",
            "票据变造: 无权更改票据内容的人篡改票据上除签章外其他事项的行为",
            "票据伪造: 假冒他人名义在票据上签章的行为",
        ]
    }

    /// 票据权利深度规则
    pub fn instrument_rights_deep(&self) -> Vec<&'static str> {
        vec![
            "票据权利: 付款请求权和追索权",
            "付款请求权: 持票人向票据主债务人请求支付票据金额的权利",
            "追索权: 票据到期被拒绝付款或因其他法定原因，持票人向票据债务人请求支付票据金额的权利",
            "权利取得: 从票据权利人处受让票据并取得票据权利",
            "权利善意取得: 依票据法规定转让票据权利时，受让人善意且无重大过失支付对价取得票据的，取得票据权利",
            "权利消灭: 因付款、票据权利时效届满、票据权利人放弃等原因消灭",
            "利益返还: 持票人因超过票据权利时效或票据记载事项欠缺而丧失票据权利的，仍享有民事权利",
            "票据抗辩: 票据债务人根据票据法规定提出抗辩事由拒绝履行票据义务",
        ]
    }

    /// 票据丧失与补救深度规则
    pub fn instrument_loss_deep(&self) -> Vec<&'static str> {
        vec![
            "挂失止付: 失票人及时通知票据付款人挂失止付，付款人应暂停支付",
            "挂失时限: 失票人应当在通知挂失止付后3日内，依法向法院申请公示催告或提起诉讼",
            "公示催告: 法院受理后通知付款人停止支付，并公告催促利害关系人申报权利",
            "公告期限: 公示催告的期间由法院根据情况决定，不得少于60日",
            "除权判决: 公示催告期间无人申报权利的，法院应根据申请人的申请作出判决，宣告票据无效",
            "票据复权: 在公示催告期间申报权利并提供票据的，法院应裁定终结公示催告程序",
            "票据恢复: 失票人向法院提起诉讼，请求债务人支付票据金额",
            "善意取得保护: 善意持票人取得票据的，原持票人不得请求返还票据",
        ]
    }
}

impl Rule for NegotiableInstrumentsLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("negotiable_instruments_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "票据法深度规则",
            &[
                ("汇票规则", &self.bill_of_exchange_deep()),
                ("本票规则", &self.promissory_note_deep()),
                ("支票规则", &self.check_deep()),
                ("票据行为规则", &self.instrument_acts_deep()),
                ("票据权利规则", &self.instrument_rights_deep()),
                ("票据丧失与补救规则", &self.instrument_loss_deep()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negotiable_instruments_law_deep_rules() {
        let rules = NegotiableInstrumentsLawDeepRules::new();
        assert_eq!(rules.metadata().name, "票据法深度规则");
        assert!(!rules.bill_of_exchange_deep().is_empty());
        assert!(!rules.promissory_note_deep().is_empty());
        assert!(!rules.check_deep().is_empty());
        assert!(!rules.instrument_acts_deep().is_empty());
        assert!(!rules.instrument_rights_deep().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_bill_of_exchange_count() {
        let rules = NegotiableInstrumentsLawDeepRules::new();
        assert_eq!(rules.bill_of_exchange_deep().len(), 10);
    }

    #[test]
    fn test_check_count() {
        let rules = NegotiableInstrumentsLawDeepRules::new();
        assert_eq!(rules.check_deep().len(), 8);
    }

    #[test]
    fn test_category() {
        let rules = NegotiableInstrumentsLawDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("negotiable_instruments_law_deep")
        );
    }
}
