//! 行政法深度规则
//!
//! 涵盖行政法核心领域的详细内容，包括：
//! - 行政处罚深度规则
//! - 行政许可深度规则
//! - 行政强制深度规则
//! - 行政复议深度规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: AdministrativeLawDeepRules,
    name: "行政法深度规则",
    desc: "行政法核心领域的详细规则解析",
    origin: "中国",
    tags: ["法律", "行政法", "行政处罚", "行政许可"]
}

impl AdministrativeLawDeepRules {
    /// 行政处罚深度规则
    pub fn admin_penalty_detailed(&self) -> Vec<&'static str> {
        vec![
            "行政处罚种类: 警告、罚款、没收违法所得、责令停产停业、暂扣或吊销许可证、行政拘留等",
            "处罚法定原则: 行政处罚的种类和幅度由法律、行政法规、地方性法规设定",
            "处罚公正原则: 设定和实施行政处罚必须以事实为依据，与违法行为的事实、性质、情节及社会危害程度相当",
            "处罚公开原则: 对违法行为给予行政处罚的规定必须公布；未经公布的，不得作为行政处罚的依据",
            "一事不再罚原则: 对当事人的同一个违法行为，不得给予两次以上罚款的行政处罚",
            "简易程序: 违法事实确凿并有法定依据，对公民处以五十元以下、对法人处以一千元以下罚款或警告的行政处罚",
            "一般程序: 除简易程序外，行政处罚适用一般程序，包括立案、调查、审查、决定、送达等环节",
            "听证程序: 行政机关作出责令停产停业、吊销许可证或执照、较大数额罚款等决定前，应当告知当事人有要求听证的权利",
            "处罚执行: 行政处罚决定依法作出后，当事人应当在行政处罚决定的期限内予以履行",
            "救济途径: 公民、法人或其他组织对行政处罚不服的，可以依法申请行政复议或提起行政诉讼",
        ]
    }

    /// 行政许可深度规则
    pub fn admin_license_detailed(&self) -> Vec<&'static str> {
        vec![
            "行政许可设定: 行政许可由法律、行政法规、国务院决定、地方性法规设定",
            "许可法定原则: 设定和实施行政许可，应当依照法定的权限、范围、条件和程序",
            "许可公开原则: 有关行政许可的规定应当公布；未经公布的，不得作为实施行政许可的依据",
            "许可便民原则: 实施行政许可，应当遵循便民的原则，提高办事效率，提供优质服务",
            "申请与受理: 行政机关对申请人提出的行政许可申请，应当根据情况作出受理、补正、不予受理等决定",
            "审查与决定: 行政机关应当对申请人提交的申请材料进行审查，并在法定期限内作出决定",
            "听证程序: 法律、法规、规章规定实施行政许可应当听证的事项，行政机关应当向社会公告并举行听证",
            "变更与延续: 被许可人要求变更行政许可事项的，应当向作出行政许可决定的行政机关提出申请",
            "监督检查: 行政机关应当建立健全监督制度，通过核查反映被许可人从事行政许可事项活动情况",
            "许可撤销: 行政机关可以依法变更或撤销行政许可，被许可人以欺骗、贿赂等不正当手段取得行政许可的应当撤销",
        ]
    }

    /// 行政强制深度规则
    pub fn admin_coercion_detailed(&self) -> Vec<&'static str> {
        vec![
            "行政强制种类: 行政强制措施包括限制公民人身自由、查封场所设施或财物、扣押财物、冻结存款汇款等",
            "强制措施设定: 行政强制措施由法律设定，法律以外的规范性文件不得设定行政强制措施",
            "强制执行方式: 行政强制执行包括加处罚款、代履行、申请人民法院强制执行等",
            "强制法定原则: 行政强制的设定和实施，应当依照法定的权限、范围、条件和程序",
            "强制适当原则: 行政强制的设定和实施应当适当，采用非强制手段可以达到行政管理目的的不得实施行政强制",
            "实施程序: 行政机关实施行政强制措施应当遵守事先报批、当场告知、制作笔录等程序",
            "查封扣押: 行政机关决定实施查封、扣押的，应当制作并当场交付查封、扣押决定书和清单",
            "强制执行催告: 行政机关作出强制执行决定前，应当事先催告当事人履行义务",
            "中止执行: 有下列情形之一的，中止执行：当事人履行行政决定确有困难或暂无履行能力的；第三人对执行标的主张权利的",
            "执行和解: 实施行政强制执行，行政机关可以在不损害公共利益和他人合法权益的情况下，与当事人达成执行协议",
        ]
    }

    /// 行政复议深度规则
    pub fn admin_reconsideration_detailed(&self) -> Vec<&'static str> {
        vec![
            "复议范围: 对行政机关作出的行政处罚、行政许可、行政强制、行政征收等具体行政行为不服的可以申请行政复议",
            "复议机关: 对县级以上地方各级人民政府工作部门的具体行政行为不服的，由申请人选择向该部门的本级人民政府或上一级主管部门申请行政复议",
            "申请期限: 可以自知道该具体行政行为之日起六十日内提出行政复议申请",
            "申请形式: 申请行政复议，可以书面申请，也可以口头申请",
            "受理审查: 行政复议机关收到行政复议申请后，应当在五日内进行审查，决定是否受理",
            "审理方式: 行政复议原则上采取书面审查的办法，必要时可以向有关组织和人员调查情况",
            "决定类型: 行政复议机关作出维持、责令履行、撤销、变更、确认违法等行政复议决定",
            "决定期限: 行政复议机关应当自受理申请之日起六十日内作出行政复议决定",
            "复议中止: 有申请人死亡、作为申请人的法人终止等情形的，中止行政复议",
            "复议终止: 有申请人撤回行政复议申请、作为申请人的法人终止且其权利义务承受人放弃权利等情形的，终止行政复议",
        ]
    }
}

impl Rule for AdministrativeLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("administrative_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "行政法深度规则",
            &[
                ("行政处罚深度规则", &self.admin_penalty_detailed()),
                ("行政许可深度规则", &self.admin_license_detailed()),
                ("行政强制深度规则", &self.admin_coercion_detailed()),
                ("行政复议深度规则", &self.admin_reconsideration_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_administrative_law_deep_rules() {
        let rules = AdministrativeLawDeepRules::new();
        assert_eq!(rules.metadata().name, "行政法深度规则");
        assert!(!rules.admin_penalty_detailed().is_empty());
        assert!(!rules.admin_license_detailed().is_empty());
        assert!(!rules.admin_coercion_detailed().is_empty());
        assert!(!rules.admin_reconsideration_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_admin_penalty_count() {
        let rules = AdministrativeLawDeepRules::new();
        assert_eq!(rules.admin_penalty_detailed().len(), 10);
    }

    #[test]
    fn test_admin_license_count() {
        let rules = AdministrativeLawDeepRules::new();
        assert_eq!(rules.admin_license_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = AdministrativeLawDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("administrative_law_deep")
        );
    }
}
