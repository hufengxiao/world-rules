//! 民法典总则编深度规则
//!
//! 涵盖民法典总则编的详细内容，包括：
//! - 基本原则详解
//! - 自然人制度详解
//! - 法人制度详解
//! - 民事法律行为详解
//! - 代理制度详解
//! - 民事责任详解
//! - 诉讼时效详解
//! - 期间计算详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CivilCodeGeneralDeepRules,
    name: "民法典总则编深度规则",
    desc: "民法典总则编的详细规则解析",
    origin: "中国",
    tags: ["法律", "民法", "民法典", "总则"]
}

impl CivilCodeGeneralDeepRules {
    /// 基本原则详解
    pub fn basic_principles_detailed(&self) -> Vec<&'static str> {
        vec![
            "平等原则: 民事主体在民事活动中法律地位一律平等，不允许任何一方享有特权",
            "自愿原则: 民事主体按照自己的意愿设立、变更、终止民事法律关系",
            "公平原则: 民事主体应当合理确定各方的权利和义务，承担相应的民事责任",
            "诚信原则: 民事主体从事民事活动应当秉持诚实、恪守承诺",
            "守法与公序良俗原则: 民事活动不得违反法律，不得违背公序良俗",
            "绿色原则: 民事活动应当有利于节约资源、保护生态环境",
            "权益保护原则: 民事主体的人身权利、财产权利以及其他合法权益受法律保护",
            "法律补充原则: 法律没有规定的，可以适用习惯，但不得违背公序良俗",
        ]
    }

    /// 自然人制度详解
    pub fn natural_person_detailed(&self) -> Vec<&'static str> {
        vec![
            "民事权利能力: 自然人从出生时起到死亡时止，具有民事权利能力",
            "出生时间认定: 出生证明记载的时间为准；无出生证明的，户籍登记为准",
            "胎儿利益保护: 胎儿视为具有民事权利能力，但娩出为死体的除外",
            "完全民事行为能力: 年满18周岁的自然人；16周岁以上以劳动收入为主要生活来源的视为完全民事行为能力人",
            "限制民事行为能力: 8周岁以上的未成年人；不能完全辨认自己行为的成年人",
            "无民事行为能力: 不满8周岁的未成年人；不能辨认自己行为的成年人",
            "监护制度: 父母是未成年人的法定监护人；无父母或父母不能履职时按顺序确定监护人",
            "成年人意定监护: 成年人可预先确定监护人，在自己丧失行为能力时由该监护人履行监护职责",
            "宣告失踪: 自然人下落不明满2年的，利害关系人可申请宣告失踪",
            "宣告死亡: 自然人下落不明满4年的，或意外事件下落不明满2年的，可申请宣告死亡",
        ]
    }

    /// 法人制度详解
    pub fn legal_person_detailed(&self) -> Vec<&'static str> {
        vec![
            "营利法人: 以取得利润并分配给股东等出资人为目的成立的法人，包括有限责任公司、股份有限公司等",
            "非营利法人: 为公益目的或其他非营利目的成立，不向出资人分配利润的法人",
            "特别法人: 机关法人、农村集体经济组织法人、城镇农村的合作经济组织法人、基层群众性自治组织法人",
            "法人设立: 依法成立，有自己的名称、组织机构、住所、财产或经费",
            "法人能力: 法人享有民事权利能力和民事行为能力，依法独立享有民事权利和承担民事义务",
            "法定代表人: 依照法律或法人章程的规定，代表法人从事民事活动的负责人",
            "法人机关: 权力机关、执行机关、监督机关等，依法行使相应职权",
            "法人分支机构: 法人可以设立分支机构，分支机构以法人名义从事活动，责任由法人承担",
            "法人变更: 法人合并、分立需依法进行，权利义务由变更后的法人享有和承担",
            "法人终止: 法人因解散、破产等终止，需依法进行清算",
        ]
    }

    /// 民事法律行为详解
    pub fn civil_act_detailed(&self) -> Vec<&'static str> {
        vec![
            "行为有效条件: 行为人具有相应民事行为能力；意思表示真实；不违反法律、行政法规的强制性规定，不违背公序良俗",
            "无效民事行为: 无民事行为能力人实施的行为；虚假意思表示实施的行为；违反强制性规定的行为；违背公序良俗的行为",
            "可撤销民事行为: 基于重大误解实施的行为；一方以欺诈手段使对方实施的行为；一方以胁迫手段使对方实施的行为",
            "显失公平: 一方利用对方处于危困状态、缺乏判断能力等情形，致使行为成立时显失公平的",
            "撤销权行使: 撤销权自权利人知道或应当知道撤销事由之日起1年内行使",
            "效力待定: 限制民事行为能力人实施的超出其行为能力范围的行为，需法定代理人追认",
            "无权代理: 行为人没有代理权、超越代理权或代理权终止后实施的代理行为",
            "表见代理: 行为人虽无代理权，但相对人有理由相信其有代理权的，代理行为有效",
            "附条件行为: 民事法律行为可以附条件，附生效条件的自条件成就时生效",
            "附期限行为: 民事法律行为可以附期限，附生效期限的自期限届至时生效",
        ]
    }

    /// 代理制度详解
    pub fn agency_detailed(&self) -> Vec<&'static str> {
        vec![
            "委托代理: 代理人按照被代理人的委托行使代理权",
            "法定代理: 代理人依照法律规定行使代理权，如父母代理未成年子女",
            "指定代理: 代理人按照人民法院或有权机关的指定行使代理权",
            "代理权限: 代理人应当在代理权限内行使代理权，不得超越代理权限",
            "转代理: 代理人需要转委托第三人代理的，应当取得被代理人的同意或追认",
            "共同代理: 数人为同一代理事项的代理人的，应当共同行使代理权",
            "代理终止: 代理期间届满或代理事务完成；被代理人取消委托或代理人辞去委托",
            "代理人死亡: 代理人丧失民事行为能力；作为代理人或被代理人的法人终止",
            "无权代理责任: 行为人没有代理权实施代理行为，未经追认的，由行为人承担责任",
            "代理违法: 代理人知道或应当知道代理事项违法仍然实施代理行为的，与被代理人承担连带责任",
        ]
    }

    /// 民事责任详解
    pub fn civil_liability_detailed(&self) -> Vec<&'static str> {
        vec![
            "民事责任: 民事主体因实施侵权行为或违约行为等而应承担的民事法律后果",
            "违约责任: 当事人一方不履行合同义务或履行不符合约定的，应承担继续履行、采取补救措施或赔偿损失等责任",
            "侵权责任: 行为人因过错侵害他人民事权益造成损害的，应承担侵权责任",
            "无过错责任: 法律规定无过错责任的，行为人无论有无过错都应承担责任",
            "连带责任: 二人以上依法承担连带责任的，权利人有权请求部分或全部责任人承担责任",
            "按份责任: 二人以上依法承担按份责任的，各自按照份额承担责任",
            "责任竞合: 因当事人一方的违约行为损害对方人身权益、财产权益的，受损害方有权选择请求其承担违约责任或侵权责任",
            "免责情形: 因不可抗力不能履行民事义务的，不承担民事责任，法律另有规定的除外",
            "正当防卫: 因正当防卫造成损害的，不承担民事责任",
            "紧急避险: 因紧急避险造成损害的，由引起险情发生的人承担民事责任",
        ]
    }

    /// 诉讼时效详解
    pub fn limitation_period_detailed(&self) -> Vec<&'static str> {
        vec![
            "普通诉讼时效: 向人民法院请求保护民事权利的诉讼时效期间为3年",
            "时效起算: 自权利人知道或应当知道权利受到损害以及义务人之日起计算",
            "最长保护期: 自权利受到损害之日起超过20年的，人民法院不予保护",
            "时效中止: 在诉讼时效期间的最后6个月内，因不可抗力等障碍不能行使请求权的，时效中止",
            "时效中断: 权利人向义务人提出履行请求；义务人同意履行；权利人提起诉讼或申请仲裁",
            "时效延长: 有特殊情况的，人民法院可以根据权利人的申请决定延长",
            "不适用时效: 下列请求权不适用诉讼时效：支付存款本金及利息请求权；兑付国债、金融债券请求权等",
            "时效利益放弃: 诉讼时效期间届满后，义务人同意履行的，不得以时效届满为由抗辩",
            "分期履行: 当事人约定同一债务分期履行的，诉讼时效期间自最后一期履行期限届满之日起计算",
            "未成年人受侵: 无民事行为能力人或限制民事行为能力人对其法定代理人的请求权，自法定代理终止之日起计算",
        ]
    }

    /// 期间计算详解
    pub fn period_calculation_detailed(&self) -> Vec<&'static str> {
        vec![
            "期间单位: 民事法律规定的期间按照公历年、月、日、小时计算",
            "期间开始: 按照小时计算的，自规定时开始计算；按照日、月、年计算的，开始的当日不计入",
            "期间届满: 按照日计算的，自开始日的次日计算至届满日的当日；按照月、年计算的，至最后月的对应日",
            "最后月份无对应日: 最后一个月没有对应日的，以该月的最后一日为期间的最后一日",
            "期间末日: 期间的最后一日是法定休假日的，以休假日结束的次日为期间的最后一日",
            "期间结束时间: 期间的最后一日的截止时间为二十四时；有业务时间的，为停止业务活动的时间",
            "期间计算方法: 民事法律规定的期间的计算方法，本法没有规定的，适用本法第四章的规定",
            "期限约定: 当事人可以约定期间的起算点和计算方法，但不得违反法律规定",
        ]
    }
}

impl Rule for CivilCodeGeneralDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_general_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典总则编深度规则",
            &[
                ("基本原则详解", &self.basic_principles_detailed()),
                ("自然人制度详解", &self.natural_person_detailed()),
                ("法人制度详解", &self.legal_person_detailed()),
                ("民事法律行为详解", &self.civil_act_detailed()),
                ("代理制度详解", &self.agency_detailed()),
                ("民事责任详解", &self.civil_liability_detailed()),
                ("诉讼时效详解", &self.limitation_period_detailed()),
                ("期间计算详解", &self.period_calculation_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_code_general_deep_rules() {
        let rules = CivilCodeGeneralDeepRules::new();
        assert_eq!(rules.metadata().name, "民法典总则编深度规则");
        assert!(!rules.basic_principles_detailed().is_empty());
        assert!(!rules.natural_person_detailed().is_empty());
        assert!(!rules.legal_person_detailed().is_empty());
        assert!(!rules.civil_act_detailed().is_empty());
        assert!(!rules.agency_detailed().is_empty());
        assert!(!rules.civil_liability_detailed().is_empty());
        assert!(!rules.limitation_period_detailed().is_empty());
        assert!(!rules.period_calculation_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_basic_principles_count() {
        let rules = CivilCodeGeneralDeepRules::new();
        assert_eq!(rules.basic_principles_detailed().len(), 8);
    }

    #[test]
    fn test_natural_person_count() {
        let rules = CivilCodeGeneralDeepRules::new();
        assert_eq!(rules.natural_person_detailed().len(), 10);
    }

    #[test]
    fn test_legal_person_count() {
        let rules = CivilCodeGeneralDeepRules::new();
        assert_eq!(rules.legal_person_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = CivilCodeGeneralDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("civil_code_general_deep")
        );
    }
}
