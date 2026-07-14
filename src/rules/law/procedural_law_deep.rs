//! 程序法深度规则
//!
//! 涵盖程序法核心领域的详细内容，包括：
//! - 民事诉讼深度规则
//! - 行政诉讼深度规则
//! - 仲裁深度规则
//! - 调解深度规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ProceduralLawDeepRules,
    name: "程序法深度规则",
    desc: "程序法核心领域的详细规则解析",
    origin: "中国",
    tags: ["法律", "程序法", "民事诉讼", "行政诉讼"]
}

impl ProceduralLawDeepRules {
    /// 民事诉讼深度规则
    pub fn civil_procedure_detailed(&self) -> Vec<&'static str> {
        vec![
            "管辖原则: 基层人民法院管辖第一审民事案件，中级人民法院管辖重大涉外案件、本辖区有重大影响的案件等",
            "起诉条件: 原告是与本案有直接利害关系的公民、法人和其他组织；有明确的被告；有具体的诉讼请求和事实、理由",
            "立案登记: 人民法院应当保障当事人依照法律规定享有的起诉权利，符合起诉条件的应当在七日内立案",
            "一审程序: 人民法院审理第一审民事案件，由审判员、陪审员共同组成合议庭或由审判员组成合议庭",
            "举证责任: 当事人对自己提出的主张有责任提供证据，人民法院应当按照法定程序全面、客观地审查核实证据",
            "财产保全: 利害关系人因情况紧急，不立即申请保全将会使其合法权益受到难以弥补的损害的，可以申请诉前财产保全",
            "二审程序: 当事人不服地方人民法院第一审判决的，有权在判决书送达之日起十五日内向上一级人民法院提起上诉",
            "再审程序: 当事人对已经发生法律效力的判决、裁定，认为有错误的，可以向上一级人民法院申请再审",
            "执行程序: 发生法律效力的民事判决、裁定，当事人必须履行，一方拒绝履行的，对方当事人可以向人民法院申请执行",
            "公益诉讼: 对污染环境、侵害众多消费者合法权益等损害社会公共利益的行为，法律规定的机关和有关组织可以向人民法院提起诉讼",
        ]
    }

    /// 行政诉讼深度规则
    pub fn admin_procedure_detailed(&self) -> Vec<&'static str> {
        vec![
            "受案范围: 对行政拘留、暂扣或吊销许可证和执照、责令停产停业、没收违法所得、没收非法财物、罚款、警告等行政处罚不服的",
            "管辖规定: 基层人民法院管辖第一审行政案件，中级人民法院管辖海关处理的案件、本辖区内重大、复杂的案件等",
            "原告资格: 公民、法人或其他组织认为行政机关和行政机关工作人员的行政行为侵犯其合法权益，有权提起行政诉讼",
            "被告确定: 公民、法人或其他组织直接向人民法院提起诉讼的，作出行政行为的行政机关是被告",
            "起诉期限: 公民、法人或其他组织直接向人民法院提起诉讼的，应当自知道或应当知道作出行政行为之日起六个月内提出",
            "证据规则: 被告对作出的行政行为负有举证责任，应当提供作出该行政行为的证据和所依据的规范性文件",
            "合法性审查: 人民法院审理行政案件，对行政行为是否合法进行审查",
            "判决类型: 人民法院判决驳回原告的诉讼请求、撤销或部分撤销行政行为、责令被告重新作出行政行为等",
            "行政机关负责人出庭: 人民法院对行政案件宣告判决或裁定前，原告申请撤诉的，人民法院裁定准许或不准许",
            "执行效力: 公民、法人或其他组织拒绝履行判决、裁定、调解书的，行政机关或第三人可以向第一审人民法院申请强制执行",
        ]
    }

    /// 仲裁深度规则
    pub fn arbitration_detailed(&self) -> Vec<&'static str> {
        vec![
            "仲裁范围: 平等主体的公民、法人和其他组织之间发生的合同纠纷和其他财产权益纠纷，可以仲裁",
            "仲裁协议: 当事人申请仲裁应当有仲裁协议，仲裁协议包括合同中订立的仲裁条款和纠纷发生后达成的仲裁协议书",
            "仲裁委员会: 仲裁委员会可以在直辖市和省、自治区人民政府所在地的市设立，也可以根据需要在其他设区的市设立",
            "仲裁程序: 当事人申请仲裁应当向仲裁委员会递交仲裁协议、仲裁申请书及副本，仲裁委员会收到仲裁申请书之日起五日内决定是否受理",
            "仲裁庭组成: 仲裁庭可以由三名仲裁员或一名仲裁员组成，由三名仲裁员组成的设首席仲裁员",
            "开庭审理: 仲裁应当开庭进行，当事人协议不开庭的，仲裁庭可以根据仲裁申请书、答辩书及其他材料作出裁决",
            "证据规则: 当事人应当对自己的主张提供证据，仲裁庭认为有必要收集的证据可以自行收集",
            "仲裁裁决: 裁决应当按照多数仲裁员的意见作出，少数仲裁员的不同意见可以记入笔录，仲裁庭不能形成多数意见时裁决应当按照首席仲裁员的意见作出",
            "裁决效力: 仲裁裁决是终局的，裁决书自作出之日起发生法律效力",
            "裁决撤销: 当事人提出证据证明裁决有法定情形的，可以向仲裁委员会所在地的中级人民法院申请撤销裁决",
        ]
    }

    /// 调解深度规则
    pub fn mediation_detailed(&self) -> Vec<&'static str> {
        vec![
            "人民调解: 人民调解委员会是依法设立的调解民间纠纷的群众性组织，调解民间纠纷应当遵循自愿、平等的原则",
            "调解范围: 人民调解委员会调解公民之间、公民与法人或其他社会组织之间涉及民事权利义务争议的各种纠纷",
            "调解协议: 经人民调解委员会调解达成的调解协议，具有法律约束力，当事人应当按照约定履行",
            "司法确认: 经人民调解委员会调解达成调解协议后，双方当事人认为有必要的，可以自调解协议生效之日起三十日内共同向人民法院申请司法确认",
            "法院调解: 人民法院进行调解，可以由审判员一人主持，也可以由合议庭主持，并尽可能就地进行",
            "调解自愿原则: 人民法院审理民事案件，应当根据自愿和合法的原则进行调解，调解不成的，应当及时判决",
            "调解书效力: 调解达成协议，人民法院应当制作调解书，调解书经双方当事人签收后，即具有法律效力",
            "商事调解: 商事调解组织可以调解法人、非法人组织之间、法人、非法人组织与自然人之间发生的商事纠纷",
            "行政调解: 行政机关依据法律、法规、规章规定，对公民、法人和其他组织之间与行政管理相关的民事纠纷进行调解",
            "行业调解: 行业协会、商会等社会组织可以设立调解组织，调解会员之间以及会员与其他当事人之间的纠纷",
        ]
    }
}

impl Rule for ProceduralLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("procedural_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "程序法深度规则",
            &[
                ("民事诉讼深度规则", &self.civil_procedure_detailed()),
                ("行政诉讼深度规则", &self.admin_procedure_detailed()),
                ("仲裁深度规则", &self.arbitration_detailed()),
                ("调解深度规则", &self.mediation_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procedural_law_deep_rules() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(rules.metadata().name, "程序法深度规则");
        assert!(!rules.civil_procedure_detailed().is_empty());
        assert!(!rules.admin_procedure_detailed().is_empty());
        assert!(!rules.arbitration_detailed().is_empty());
        assert!(!rules.mediation_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_civil_procedure_count() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(rules.civil_procedure_detailed().len(), 10);
    }

    #[test]
    fn test_arbitration_count() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(rules.arbitration_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("procedural_law_deep")
        );
    }
}