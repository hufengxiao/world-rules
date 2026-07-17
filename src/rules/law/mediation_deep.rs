//! 调解法深度规则
//!
//! 涵盖调解法的详细内容，包括：
//! - 调解类型详解
//! - 调解程序详解
//! - 调解效力详解
//! - 人民调解详解
//! - 法院调解详解
//! - 行政调解详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: MediationDeepRules,
    name: "调解法深度规则",
    desc: "调解法的详细规则解析",
    origin: "中国",
    tags: ["法律", "调解", "程序"]
}

impl MediationDeepRules {
    /// 调解类型详解
    pub fn mediation_types_detailed(&self) -> Vec<&'static str> {
        vec![
            "人民调解: 人民调解委员会通过说服、疏导等方法促使当事人在平等协商基础上自愿达成调解协议解决民间纠纷的活动",
            "法院调解: 人民法院在审理民事案件过程中对能够调解的案件在查明事实、分清是非的基础上依法通过调解方式解决纠纷",
            "行政调解: 行政机关依照法律、法规、规章的规定对特定的民事纠纷或者行政争议通过调解方式解决的活动",
            "仲裁调解: 仲裁庭在作出裁决前可以先行调解当事人自愿调解的仲裁庭应当调解调解不成的应当及时作出裁决",
            "行业调解: 行业组织依据法律、法规、行业规范对涉及行业领域的纠纷通过调解方式解决的活动",
            "商事调解: 商事调解组织对商事纠纷通过调解方式解决的活动商事调解协议可以向人民法院申请司法确认",
            "律师调解: 律师事务所、律师接受当事人委托在律师调解工作室对纠纷进行调解的活动",
            "公证调解: 公证机构对经公证的事项在发生争议时通过调解方式解决的活动",
            "基层调解: 村民委员会、居民委员会等基层群众性自治组织对民间纠纷进行调解的活动",
            "特邀调解: 人民法院特邀调解组织和特邀调解员对适宜调解的纠纷进行调解的活动",
        ]
    }

    /// 调解程序详解
    pub fn mediation_procedure_detailed(&self) -> Vec<&'static str> {
        vec![
            "调解申请: 当事人可以向人民调解委员会申请调解人民调解委员会也可以主动调解当事人一方明确拒绝调解的不得调解",
            "调解受理: 人民调解委员会收到调解申请后应当及时审查决定是否受理并告知当事人",
            "调解员确定: 人民调解委员会调解纠纷可以由一名调解员调解也可以由多名调解员调解由多名调解员调解的应当有一名主调解员",
            "调解期限: 人民调解委员会调解纠纷应当自受理之日起三十日内调结；情况复杂需要延长的经双方当事人同意可以延长但不得超过六十日",
            "调解方式: 调解员可以采取多种方式调解纠纷包括但不限于: 说服、疏导、协商、斡旋等",
            "调解记录: 调解员应当记录调解情况包括调解时间、地点、参加人员、调解过程和结果等",
            "调解协议: 经人民调解委员会调解达成调解协议的需要制作调解协议书的应当制作调解协议书",
            "协议履行: 调解协议对当事人具有约束力当事人应当履行调解协议",
            "司法确认: 经人民调解委员会调解达成调解协议后当事人可以自调解协议生效之日起三十日内共同向人民法院申请司法确认",
            "调解终止: 当事人明确表示不愿继续调解的；调解期限届满未达成调解协议的；当事人拒绝在调解协议上签名的应当终止调解",
        ]
    }

    /// 调解效力详解
    pub fn mediation_effect_detailed(&self) -> Vec<&'static str> {
        vec![
            "调解协议效力: 经人民调解委员会调解达成的调解协议具有法律约束力当事人应当按照约定履行",
            "司法确认效力: 人民法院对调解协议进行司法确认后调解协议具有强制执行力一方当事人拒绝履行或者未全部履行的对方当事人可以向人民法院申请执行",
            "法院调解书效力: 人民法院制作的调解书经双方当事人签收后即具有法律效力",
            "仲裁调解书效力: 仲裁调解书经双方当事人签收后即发生法律效力当事人应当履行",
            "调解协议可诉性: 当事人之间就调解协议的履行或者调解协议的内容发生争议的可以向人民法院提起诉讼",
            "调解协议变更: 当事人认为调解协议内容有错误或者显失公平的可以请求变更或者撤销",
            "调解协议无效: 一方以欺诈、胁迫的手段订立调解协议损害国家利益的；恶意串通损害国家、集体或者第三人利益的",
            "调解协议撤销: 因重大误解订立的调解协议；在订立调解协议时显失公平的当事人可以请求人民法院或者仲裁机构变更或者撤销",
            "违约责任: 当事人不履行调解协议的应当承担违约责任",
            "证据效力: 调解协议可以作为证据使用证明当事人对纠纷的处置情况",
        ]
    }

    /// 人民调解详解
    pub fn people_mediation_detailed(&self) -> Vec<&'static str> {
        vec![
            "人民调解委员会: 村民委员会、居民委员会设立人民调解委员会企业事业单位根据需要设立人民调解委员会",
            "调解员资格: 人民调解员应当由公道正派、热心人民调解工作并具有一定文化水平、政策水平和法律知识的成年公民担任",
            "调解员职责: 人民调解员的职责包括调解民间纠纷促进当事人平等协商达成调解协议；主动调解民间纠纷防止矛盾激化",
            "调解原则: 人民调解应当遵循自愿平等原则；不违背法律、法规、国家政策和公序良俗原则；尊重当事人权利原则",
            "调解范围: 人民调解委员会调解的纠纷包括: 婚姻家庭纠纷、邻里纠纷、房屋宅基地纠纷、合同纠纷、损害赔偿纠纷等",
            "不收费原则: 人民调解委员会调解民间纠纷不收取任何费用",
            "保密原则: 人民调解员应当尊重当事人的隐私权未经当事人同意不得公开调解过程和调解内容",
            "回避制度: 人民调解员有下列情形之一的应当回避: 是本案当事人或者当事人、代理人的近亲属；与本案有利害关系",
            "调解协议书: 调解协议书应当载明当事人的基本情况纠纷的主要事实和争议事项当事人的权利和义务等内容",
            "口头协议: 当事人认为无需制作调解协议书的可以采取口头协议方式调解员应当记录协议内容",
        ]
    }

    /// 法院调解详解
    pub fn court_mediation_detailed(&self) -> Vec<&'static str> {
        vec![
            "调解原则: 人民法院审理民事案件应当根据自愿和合法的原则进行调解调解不成的应当及时判决",
            "调解程序: 人民法院进行调解可以由审判员一人主持也可以由合议庭主持并尽可能就地进行",
            "调解参加人: 人民法院进行调解可以用简便方式通知当事人、证人到庭当事人不能到庭的可以委托代理人参加调解",
            "调解书制作: 调解达成协议人民法院应当制作调解书调解书应当写明诉讼请求、案件的事实和调解结果",
            "调解书签收: 调解书经双方当事人签收后即具有法律效力一方拒绝签收的调解书不发生法律效力",
            "不需制作调解书的情形: 调解和好的离婚案件；调解维持收养关系的案件；能够即时履行的案件等",
            "调解书效力: 调解书与判决书具有同等法律效力当事人必须履行",
            "调解不成: 调解未达成协议或者调解书送达前一方反悔的人民法院应当及时判决",
            "二审调解: 第二审人民法院审理上诉案件可以进行调解调解达成协议的应当制作调解书",
            "再审调解: 人民法院按照审判监督程序再审的案件可以进行调解调解达成协议的应当制作调解书",
        ]
    }

    /// 行政调解详解
    pub fn administrative_mediation_detailed(&self) -> Vec<&'static str> {
        vec![
            "行政调解范围: 行政调解的范围包括: 公民、法人或者其他组织之间产生的与行政管理有关的民事纠纷；行政机关与公民、法人或者其他组织之间产生的行政争议",
            "行政调解原则: 行政调解应当遵循自愿原则；合法原则；公平公正原则；效率原则",
            "行政调解程序: 行政机关对适宜调解的纠纷可以在征得当事人同意后进行调解",
            "行政调解期限: 行政调解期限一般不得超过三十日情况复杂需要延长的经行政机关负责人批准可以适当延长",
            "行政调解协议: 经行政调解达成协议的行政机关应当制作行政调解协议书",
            "协议内容: 行政调解协议书应当载明当事人的基本情况、纠纷的主要事实、争议事项、当事人的权利和义务等内容",
            "协议履行: 当事人应当履行行政调解协议行政机关应当督促当事人履行",
            "司法确认: 经行政调解达成的协议当事人可以依法向人民法院申请司法确认",
            "调解费用: 行政调解不得收取任何费用",
            "与其他调解衔接: 行政机关调解纠纷可以邀请人民法院、人民检察院、人民调解组织等有关单位和人员参与",
        ]
    }
}

impl Rule for MediationDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("mediation_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "调解法深度规则",
            &[
                ("调解类型", &self.mediation_types_detailed()),
                ("调解程序", &self.mediation_procedure_detailed()),
                ("调解效力", &self.mediation_effect_detailed()),
                ("人民调解", &self.people_mediation_detailed()),
                ("法院调解", &self.court_mediation_detailed()),
                ("行政调解", &self.administrative_mediation_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mediation_deep_rules() {
        let rules = MediationDeepRules::new();
        assert_eq!(rules.metadata().name, "调解法深度规则");
        assert!(!rules.mediation_types_detailed().is_empty());
        assert!(!rules.mediation_procedure_detailed().is_empty());
        assert!(!rules.mediation_effect_detailed().is_empty());
        assert!(!rules.people_mediation_detailed().is_empty());
        assert!(!rules.court_mediation_detailed().is_empty());
        assert!(!rules.administrative_mediation_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_mediation_types_count() {
        let rules = MediationDeepRules::new();
        assert_eq!(rules.mediation_types_detailed().len(), 10);
    }

    #[test]
    fn test_mediation_procedure_count() {
        let rules = MediationDeepRules::new();
        assert_eq!(rules.mediation_procedure_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = MediationDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("mediation_deep"));
    }
}
