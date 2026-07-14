//! 商法深度规则
//!
//! 涵盖商法核心领域的详细内容，包括：
//! - 公司法深度规则
//! - 证券法深度规则
//! - 保险法深度规则
//! - 破产法深度规则
//! - 票据法深度规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CommercialLawDeepRules,
    name: "商法深度规则",
    desc: "商法核心领域的详细规则解析",
    origin: "中国",
    tags: ["法律", "商法", "公司法", "证券法"]
}

impl CommercialLawDeepRules {
    /// 公司法深度规则
    pub fn company_law_detailed(&self) -> Vec<&'static str> {
        vec![
            "有限责任公司设立: 股东人数1-50人，注册资本为在公司登记机关登记的全体股东认缴的出资额",
            "股份有限公司设立: 发起人2-200人，半数以上发起人在中国境内有住所",
            "公司章程: 公司章程对公司、股东、董事、监事、高级管理人员具有约束力",
            "股东权利: 股东享有资产收益、参与重大决策、选择管理者等权利",
            "股东会职权: 决定公司经营方针和投资计划，选举和更换非职工代表董事、监事",
            "董事会职权: 召集股东会会议，执行股东会决议，决定公司经营计划和投资方案",
            "监事会职权: 检查公司财务，监督董事、高级管理人员履职，提议召开临时股东会",
            "股权转让: 有限责任公司股东之间可以相互转让股权，向股东以外的人转让需过半数同意",
            "股份发行: 股份有限公司的资本划分为股份，每一股金额相等",
            "公司债券: 公司债券发行需符合净资产、累计债券余额等条件",
        ]
    }

    /// 证券法深度规则
    pub fn securities_law_detailed(&self) -> Vec<&'static str> {
        vec![
            "证券发行注册制: 公开发行证券需向国务院证券监督管理机构注册",
            "信息披露义务: 发行人及法律规定的披露义务人应当真实、准确、完整披露信息",
            "内幕交易禁止: 禁止证券交易内幕信息的知情人和非法获取内幕信息的人利用内幕信息从事证券交易",
            "操纵市场禁止: 禁止任何人操纵证券市场，影响证券交易价格或证券交易量",
            "虚假陈述禁止: 禁止任何单位和个人编造、传播虚假信息或误导性信息",
            "投资者保护: 投资者保护机构可以依法提起证券诉讼，支持投资者维权",
            "上市公司收购: 通过证券交易所交易持有一上市公司股份达5%时需公告",
            "要约收购: 通过证券交易所交易持有一上市公司股份达30%继续收购的，应当发出要约",
            "证券交易所规则: 证券交易所应当创造公开、公平、公正的市场环境",
            "证券违法行为责任: 证券违法行为承担民事责任、行政责任，构成犯罪的追究刑事责任",
        ]
    }

    /// 保险法深度规则
    pub fn insurance_law_detailed(&self) -> Vec<&'static str> {
        vec![
            "保险合同成立: 投保人提出保险要求，经保险人同意承保，保险合同成立",
            "投保人义务: 如实告知义务、支付保险费义务、危险增加通知义务",
            "保险人义务: 及时签发保险单义务、说明义务、保密义务",
            "保险利益: 投保人或被保险人对保险标的应当具有法律上承认的利益",
            "保险金额: 保险金额不得超过保险价值，超过部分无效",
            "保险责任: 保险事故发生后，保险人应当承担赔偿或给付保险金责任",
            "责任免除: 保险人对责任免除条款应当向投保人明确说明",
            "索赔时效: 人寿保险索赔时效5年，其他保险索赔时效2年",
            "保险资金运用: 保险资金运用应当稳健、安全，符合规定比例",
            "保险监管: 国务院保险监督管理机构依法对保险业实施监督管理",
        ]
    }

    /// 破产法深度规则
    pub fn bankruptcy_law_detailed(&self) -> Vec<&'static str> {
        vec![
            "破产原因: 企业法人不能清偿到期债务，且资产不足以清偿全部债务或明显缺乏清偿能力",
            "破产申请: 债务人、债权人可以向人民法院提出破产申请",
            "破产受理: 人民法院裁定受理破产申请的，应当同时指定管理人",
            "管理人职责: 接管债务人财产，调查债务人财产状况，管理处分债务人财产",
            "债权人会议: 依法申报债权的债权人为债权人会议成员，有权参加债权人会议",
            "债权人委员会: 债权人会议可以决定设立债权人委员会，监督债务人财产管理",
            "重整程序: 债务人或管理人应当自人民法院裁定重整之日起6个月内提交重整计划草案",
            "和解程序: 债务人可以直接向人民法院申请和解，提出和解协议草案",
            "破产清算: 破产财产优先清偿破产费用和共益债务后，按法定顺序清偿",
            "破产终结: 破产人无财产可供分配的，管理人请求人民法院裁定终结破产程序",
        ]
    }

    /// 票据法深度规则
    pub fn negotiable_instruments_law_detailed(&self) -> Vec<&'static str> {
        vec![
            "票据特征: 票据是设权证券、债权证券、无因证券、流通证券",
            "汇票定义: 出票人签发的，委托付款人在见票时或在指定日期无条件支付确定的金额给收款人或持票人的票据",
            "本票定义: 出票人签发的，承诺自己在见票时无条件支付确定的金额给收款人或持票人的票据",
            "支票定义: 出票人签发的，委托办理支票存款业务的银行或其他金融机构在见票时无条件支付确定金额给收款人或持票人的票据",
            "背书规则: 背书应当连续，持票人以背书连续证明其汇票权利",
            "承兑规则: 汇票付款人承诺在汇票到期日支付汇票金额的票据行为",
            "保证规则: 保证人对合法取得票据持票人所享有的票据权利承担保证责任",
            "追索权: 汇票到期被拒绝付款的，持票人可以对背书人、出票人及汇票其他债务人行使追索权",
            "票据时效: 持票人对票据出票人和承兑人的权利自票据到期日起2年",
            "票据丧失: 票据丧失后，失票人可以及时通知票据付款人挂失止付",
        ]
    }
}

impl Rule for CommercialLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("commercial_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "商法深度规则",
            &[
                ("公司法深度规则", &self.company_law_detailed()),
                ("证券法深度规则", &self.securities_law_detailed()),
                ("保险法深度规则", &self.insurance_law_detailed()),
                ("破产法深度规则", &self.bankruptcy_law_detailed()),
                (
                    "票据法深度规则",
                    &self.negotiable_instruments_law_detailed(),
                ),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commercial_law_deep_rules() {
        let rules = CommercialLawDeepRules::new();
        assert_eq!(rules.metadata().name, "商法深度规则");
        assert!(!rules.company_law_detailed().is_empty());
        assert!(!rules.securities_law_detailed().is_empty());
        assert!(!rules.insurance_law_detailed().is_empty());
        assert!(!rules.bankruptcy_law_detailed().is_empty());
        assert!(!rules.negotiable_instruments_law_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_company_law_count() {
        let rules = CommercialLawDeepRules::new();
        assert_eq!(rules.company_law_detailed().len(), 10);
    }

    #[test]
    fn test_securities_law_count() {
        let rules = CommercialLawDeepRules::new();
        assert_eq!(rules.securities_law_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = CommercialLawDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("commercial_law_deep"));
    }
}
