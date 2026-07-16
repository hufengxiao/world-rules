//! 破产法深度规则 - 申请、清算、重整、和解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BankruptcyLawDeepRules,
    name: "破产法深度规则",
    desc: "破产法的详细规则解析，涵盖申请、清算、重整、和解",
    origin: "中国",
    tags: ["法律", "商法", "破产法"]
}

impl BankruptcyLawDeepRules {
    /// 破产申请深度规则
    pub fn application_deep(&self) -> Vec<&'static str> {
        vec![
            "破产原因: 不能清偿到期债务，并且资产不足以清偿全部债务或明显缺乏清偿能力",
            "申请主体: 债务人、债权人、清算责任人",
            "申请材料: 破产申请书、财产状况说明、债务清册、债权清册、财务会计报告等",
            "管辖法院: 债务人住所地人民法院管辖",
            "受理审查: 法院收到申请后15日内裁定是否受理，特殊情况可延长15日",
            "破产受理效力: 债务人对个别债权人的债务清偿无效，执行程序中止",
            "管理人指定: 法院裁定受理破产申请的，应同时指定管理人",
            "债权人会议: 第一次债权人会议由法院召集，在债权申报期限届满后15日内召开",
        ]
    }

    /// 破产清算深度规则
    pub fn liquidation_deep(&self) -> Vec<&'static str> {
        vec![
            "管理人职责: 接管债务人财产、调查财产状况、决定内部管理事务、管理和处分财产",
            "财产追回: 管理人有权追回破产申请前1年内无偿转让财产、不合理价格交易等行为",
            "抵销权: 债权人在破产申请受理前对债务人负有债务的，可以向管理人主张抵销",
            "撤销权: 管理人有权请求法院撤销破产申请前1年内的无偿转让财产、提前清偿等行为",
            "财产变价: 管理人拟定破产财产变价方案，提交债权人会议讨论",
            "变价方式: 拍卖、招标出售、协议转让等方式",
            "清偿顺序: 破产费用和共益债务→职工债权→社会保险和税款→普通债权",
            "清偿比例: 同一顺序债权按比例清偿",
            "破产终结: 破产财产分配完毕后，管理人请求法院裁定终结破产程序",
        ]
    }

    /// 破产重整深度规则
    pub fn reorganization_deep(&self) -> Vec<&'static str> {
        vec![
            "重整申请: 债务人或债权人可以直接申请重整，法院受理破产申请后宣告破产前也可以转换",
            "重整期间: 自法院裁定债务人重整之日起不超过6个月，经批准可延长3个月",
            "财产管理: 经法院批准，债务人可以在管理人监督下自行管理财产和营业事务",
            "限制担保权: 重整期间对债务人的特定财产享有的担保权暂停行使",
            "出资人权益调整: 按照重整计划草案，可以调整出资人权益",
            "重整计划制定: 管理人或债务人应在6个月内提交重整计划草案",
            "计划批准: 债权人会议表决通过，法院批准后执行",
            "计划执行: 由债务人负责执行，管理人监督",
            "重整失败: 债务人不能执行或不执行重整计划的，法院裁定终止重整并宣告破产",
        ]
    }

    /// 破产和解深度规则
    pub fn reconciliation_deep(&self) -> Vec<&'static str> {
        vec![
            "和解申请: 债务人可以申请和解，提出和解协议草案",
            "和解条件: 债务人有正当理由且债权人会议通过",
            "和解协议: 债权人会议通过和解协议的决议，由出席会议的有表决权的债权人过半数同意",
            "表决权比例: 其所代表的债权额占无财产担保债权总额三分之二以上",
            "协议生效: 法院认可和解协议后公告，和解协议具有法律约束力",
            "协议履行: 债务人按照和解协议规定的条件清偿债务",
            "和解失败: 债务人不执行和解协议的，法院裁定终止和解并宣告破产",
            "无效和解: 因债务人的欺诈行为成立的和解协议，法院应当裁定无效并宣告破产",
        ]
    }

    /// 破产债权深度规则
    pub fn claims_deep(&self) -> Vec<&'static str> {
        vec![
            "申报期限: 法院发布受理破产申请公告之日起最短30日，最长3个月",
            "债权登记: 管理人收到债权申报材料后登记造册，编制债权表",
            "债权审查: 管理人审查债权的真实性、合法性和数额",
            "债权确认: 债权表由第一次债权人会议核查，无异议的法院确认",
            "异议处理: 债务人、债权人对债权表记载有异议的，可以向受理破产申请的法院提起诉讼",
            "劳动债权: 债务人所欠职工的工资和医疗、伤残补助、抚恤费用优先清偿",
            "税收债权: 债务人欠缴的除前项规定以外的社会保险费用和税款",
            "普通债权: 上述债权之外的债权",
        ]
    }

    /// 破产费用与共益债务深度规则
    pub fn expenses_deep(&self) -> Vec<&'static str> {
        vec![
            "破产费用: 破产案件的诉讼费用，管理、变价和分配债务人财产的费用，管理人执行职务的费用",
            "共益债务: 因管理人请求履行合同产生的债务、债务人财产受无因管理产生的债务、因不当得利产生的债务等",
            "清偿顺序: 破产费用和共益债务由债务人财产随时清偿",
            "清偿比例: 债务人财产不足以清偿所有破产费用和共益债务的，按比例清偿",
            "费用不足: 债务人财产不足以清偿破产费用的，管理人应提请法院终结破产程序",
            "管理报酬: 管理人执行职务的报酬由法院确定",
            "审计评估: 破产程序中必要的审计、评估费用",
            "公告费用: 破产程序中的公告费用",
        ]
    }
}

impl Rule for BankruptcyLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("bankruptcy_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "破产法深度规则",
            &[
                ("破产申请规则", &self.application_deep()),
                ("破产清算规则", &self.liquidation_deep()),
                ("破产重整规则", &self.reorganization_deep()),
                ("破产和解规则", &self.reconciliation_deep()),
                ("破产债权规则", &self.claims_deep()),
                ("破产费用规则", &self.expenses_deep()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bankruptcy_law_deep_rules() {
        let rules = BankruptcyLawDeepRules::new();
        assert_eq!(rules.metadata().name, "破产法深度规则");
        assert!(!rules.application_deep().is_empty());
        assert!(!rules.liquidation_deep().is_empty());
        assert!(!rules.reorganization_deep().is_empty());
        assert!(!rules.reconciliation_deep().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_application_count() {
        let rules = BankruptcyLawDeepRules::new();
        assert_eq!(rules.application_deep().len(), 8);
    }

    #[test]
    fn test_liquidation_count() {
        let rules = BankruptcyLawDeepRules::new();
        assert_eq!(rules.liquidation_deep().len(), 9);
    }

    #[test]
    fn test_category() {
        let rules = BankruptcyLawDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("bankruptcy_law_deep"));
    }
}