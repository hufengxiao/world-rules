//! 程序法深度规则
//!
//! 涵盖程序法核心领域的详细内容，包括：
//! - 民事诉讼深度规则
//! - 行政诉讼深度规则
//! - 仲裁程序深度规则
//! - 调解程序深度规则
//! - 执行程序深度规则
//! - 非诉讼程序规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ProceduralLawDeepRules,
    name: "程序法深度规则",
    desc: "程序法核心领域的详细规则解析",
    origin: "中国",
    tags: ["法律", "程序法", "诉讼", "仲裁", "调解"]
}

impl ProceduralLawDeepRules {
    /// 民事诉讼深度规则
    pub fn civil_litigation_detailed(&self) -> Vec<&'static str> {
        vec![
            "诉讼管辖: 基层人民法院管辖第一审民事案件，中级人民法院管辖重大涉外案件、在本辖区有重大影响的案件",
            "地域管辖: 对公民提起的民事诉讼由被告住所地人民法院管辖，被告住所地与经常居住地不一致的由经常居住地人民法院管辖",
            "级别管辖: 基层人民法院管辖第一审民事案件，中级人民法院管辖重大涉外案件、在本辖区有重大影响的案件",
            "专属管辖: 因不动产纠纷提起的诉讼由不动产所在地人民法院管辖，因港口作业纠纷提起的诉讼由港口所在地人民法院管辖",
            "协议管辖: 合同或者其他财产权益纠纷的当事人可以书面协议选择被告住所地、合同履行地、合同签订地、原告住所地、标的物所在地等与争议有实际联系的地点的人民法院管辖",
            "管辖权异议: 当事人对管辖权有异议的应当在提交答辩状期间提出，人民法院应当对当事人提出的异议进行审查",
            "当事人资格: 原告是与本案有直接利害关系的公民、法人和其他组织，有明确的被告，有具体的诉讼请求和事实、理由",
            "诉讼时效: 向人民法院请求保护民事权利的诉讼时效期间为三年，诉讼时效期间自权利人知道或者应当知道权利受到损害以及义务人之日起计算",
            "证据规则: 当事人对自己提出的主张有责任提供证据，人民法院应当按照法定程序全面地、客观地审查核实证据",
            "举证期限: 当事人应当在举证期限内提交证据材料，当事人在举证期限内提交证据材料确有困难的可以在举证期限内申请延长举证期限",
        ]
    }

    /// 行政诉讼深度规则
    pub fn administrative_litigation_detailed(&self) -> Vec<&'static str> {
        vec![
            "受案范围: 人民法院受理公民、法人或者其他组织提起的下列诉讼：对行政拘留、暂扣或者吊销许可证和执照、责令停产停业、没收违法所得、没收非法财物、罚款、警告等行政处罚不服的",
            "管辖规定: 行政案件由最初作出行政行为的行政机关所在地人民法院管辖，经复议的案件也可以由复议机关所在地人民法院管辖",
            "诉讼参加人: 公民、法人或者其他组织直接向人民法院提起诉讼的，作出行政行为的行政机关是被告",
            "起诉条件: 提起诉讼应当符合下列条件：原告是符合本法规定的公民、法人或者其他组织；有明确的被告；有具体的诉讼请求和事实根据",
            "起诉期限: 公民、法人或者其他组织直接向人民法院提起诉讼的，应当自知道或者应当知道作出行政行为之日起六个月内提出",
            "举证责任: 被告对作出的行政行为负有举证责任，应当提供作出该行政行为的证据和所依据的规范性文件",
            "证据保全: 在诉讼过程中，人民法院认为对行政行为证据可能灭失或者以后难以取得的情况下，可以根据诉讼参加人的申请或者依职权采取证据保全措施",
            "审理程序: 人民法院应当在立案之日起五日内将起诉状副本发送被告，被告应当在收到起诉状副本之日起十五日内向人民法院提交作出行政行为的证据和所依据的规范性文件",
            "判决类型: 人民法院经过审理，根据不同情况分别作出判决：行政行为证据确凿，适用法律、法规正确，符合法定程序的，判决驳回原告的诉讼请求",
            "上诉规定: 当事人不服人民法院第一审判决的，有权在判决书送达之日起十五日内向上一级人民法院提起上诉",
        ]
    }

    /// 仲裁程序深度规则
    pub fn arbitration_procedure_detailed(&self) -> Vec<&'static str> {
        vec![
            "仲裁范围: 平等主体的公民、法人和其他组织之间发生的合同纠纷和其他财产权益纠纷，可以仲裁",
            "仲裁协议: 仲裁协议包括合同中订立的仲裁条款和以其他书面方式在纠纷发生前或者纠纷发生后达成的请求仲裁的协议",
            "仲裁协议效力: 仲裁协议独立存在，合同的变更、解除、终止或者无效，不影响仲裁协议的效力",
            "仲裁管辖: 仲裁委员会应当由当事人协议选定，仲裁不实行级别管辖和地域管辖",
            "仲裁庭组成: 仲裁庭可以由三名仲裁员或者一名仲裁员组成，由三名仲裁员组成的设首席仲裁员",
            "仲裁员回避: 仲裁员有下列情形之一的必须回避，当事人也有权提出回避申请：是本案当事人或者当事人、代理人的近亲属",
            "开庭审理: 仲裁应当开庭进行，当事人协议不开庭的，仲裁庭可以根据仲裁申请书、答辩书以及其他材料作出裁决",
            "证据规则: 当事人应当对自己的主张提供证据，仲裁庭认为有必要收集的证据可以自行收集",
            "仲裁裁决: 裁决应当按照多数仲裁员的意见作出，少数仲裁员的不同意见可以记入笔录，仲裁庭不能形成多数意见时裁决应当按照首席仲裁员的意见作出",
            "裁决效力: 仲裁裁决是终局的，裁决书自作出之日起发生法律效力，当事人就同一纠纷再申请仲裁或者向人民法院起诉的仲裁委员会或者人民法院不予受理",
        ]
    }

    /// 调解程序深度规则
    pub fn mediation_procedure_detailed(&self) -> Vec<&'static str> {
        vec![
            "调解原则: 人民调解委员会调解民间纠纷应当遵循下列原则：在双方当事人自愿、平等的基础上进行调解",
            "调解组织: 人民调解委员会是调解民间纠纷的群众性组织，村民委员会、居民委员会下设人民调解委员会",
            "调解范围: 人民调解委员会调解的民间纠纷包括公民之间、公民与法人或者其他组织之间发生的涉及民事权利义务争议的纠纷",
            "调解程序: 人民调解委员会调解纠纷应当进行登记，制作笔录，根据需要或者当事人的请求制作调解协议书",
            "调解协议: 经人民调解委员会调解达成的调解协议具有法律约束力，当事人应当按照约定履行",
            "司法确认: 双方当事人认为有必要的可以自调解协议生效之日起三十日内共同向人民法院申请司法确认",
            "法院调解: 人民法院审理民事案件根据当事人自愿的原则，在事实清楚的基础上分清是非进行调解",
            "调解书效力: 调解达成协议后人民法院应当制作调解书，调解书经双方当事人签收后即具有法律效力",
            "调解期限: 人民法院对受理的案件可以在答辩期满后裁判作出前进行调解，也可以在宣判前进行调解",
            "不予调解: 当事人拒绝调解或者调解不成的应当及时判决，人民法院审理离婚案件应当进行调解",
        ]
    }

    /// 执行程序深度规则
    pub fn execution_procedure_detailed(&self) -> Vec<&'static str> {
        vec![
            "执行依据: 发生法律效力的民事判决、裁定、调解书和其他应当由人民法院执行的法律文书，当事人必须履行",
            "执行申请: 一方拒绝履行的对方当事人可以向人民法院申请执行，也可以由审判员移送执行员执行",
            "执行期限: 申请执行的期间为二年，申请执行时效的中止、中断适用法律有关诉讼时效中止、中断的规定",
            "执行管辖: 发生法律效力的民事判决、裁定由第一审人民法院或者与第一审人民法院同级的被执行的财产所在地人民法院执行",
            "执行通知: 执行员接到申请执行书或者移交执行书应当向被执行人发出执行通知，责令其在指定的期间履行",
            "执行措施: 人民法院有权向有关单位查询被执行人的存款、债券、股票、基金份额等财产情况",
            "财产查封: 人民法院查封、扣押财产时被执行人是公民的应当通知被执行人或者他的成年家属到场",
            "冻结期限: 冻结被执行人的银行存款的期限不得超过一年，查封、扣押动产的期限不得超过两年，查封不动产、冻结其他财产权的期限不得超过三年",
            "执行异议: 执行过程中案外人对执行标的提出书面异议的，人民法院应当自收到书面异议之日起十五日内审查",
            "执行中止: 有下列情形之一的人民法院应当裁定中止执行：申请人表示可以延期执行的；案外人对执行标的提出确有理由的异议的",
        ]
    }

    /// 非诉讼程序规则
    pub fn non_litigation_procedure(&self) -> Vec<&'static str> {
        vec![
            "特别程序: 人民法院审理选民资格案件、宣告失踪或者宣告死亡案件、认定公民无民事行为能力或者限制民事行为能力案件等案件适用特别程序",
            "一审终审: 依照特别程序审理的案件实行一审终审，选民资格案件或者重大疑难的案件由审判员组成合议庭审理",
            "选民资格案件: 公民不服选举委员会对选民资格的申诉所作的处理决定，可以在选举日的五日以前向选区所在地基层人民法院起诉",
            "宣告失踪: 公民下落不明满二年，利害关系人申请宣告其失踪的向下落不明人住所地基层人民法院提出",
            "宣告死亡: 公民下落不明满四年，或者因意外事故下落不明满二年，利害关系人申请宣告其死亡的向下落不明人住所地基层人民法院提出",
            "督促程序: 债权人请求债务人给付金钱、有价证券，符合条件的可以向有管辖权的基层人民法院申请支付令",
            "公示催告: 按照规定可以背书转让的票据持有人因票据被盗、遗失或者灭失，可以向票据支付地的基层人民法院申请公示催告",
            "认定无主财产: 申请认定财产无主由公民、法人或者其他组织向财产所在地基层人民法院提出",
            "确认调解协议: 申请司法确认调解协议由双方当事人自调解协议生效之日起三十日内共同向调解组织所在地基层人民法院提出",
            "实现担保物权: 申请实现担保物权由担保物权人以及其他有权请求实现担保物权的人向担保财产所在地或者担保物权登记地基层人民法院提出",
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
                ("民事诉讼深度规则", &self.civil_litigation_detailed()),
                (
                    "行政诉讼深度规则",
                    &self.administrative_litigation_detailed(),
                ),
                ("仲裁程序深度规则", &self.arbitration_procedure_detailed()),
                ("调解程序深度规则", &self.mediation_procedure_detailed()),
                ("执行程序深度规则", &self.execution_procedure_detailed()),
                ("非诉讼程序规则", &self.non_litigation_procedure()),
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
        assert!(!rules.civil_litigation_detailed().is_empty());
        assert!(!rules.administrative_litigation_detailed().is_empty());
        assert!(!rules.arbitration_procedure_detailed().is_empty());
        assert!(!rules.mediation_procedure_detailed().is_empty());
        assert!(!rules.execution_procedure_detailed().is_empty());
        assert!(!rules.non_litigation_procedure().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_civil_litigation_count() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(rules.civil_litigation_detailed().len(), 10);
    }

    #[test]
    fn test_administrative_litigation_count() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(rules.administrative_litigation_detailed().len(), 10);
    }

    #[test]
    fn test_arbitration_count() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(rules.arbitration_procedure_detailed().len(), 10);
    }

    #[test]
    fn test_mediation_count() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(rules.mediation_procedure_detailed().len(), 10);
    }

    #[test]
    fn test_execution_count() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(rules.execution_procedure_detailed().len(), 10);
    }

    #[test]
    fn test_non_litigation_count() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(rules.non_litigation_procedure().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = ProceduralLawDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("procedural_law_deep"));
    }
}
