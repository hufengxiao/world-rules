//! 刑事证据规则深度规则
//!
//! 涵盖刑事证据规则的详细内容，包括：
//! - 证据收集规则详解
//! - 证据审查规则详解
//! - 证据认定规则详解
//! - 证据排除规则详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CriminalEvidenceDeepRules,
    name: "刑事证据规则深度规则",
    desc: "刑事证据规则的详细规则解析",
    origin: "中国",
    tags: ["法律", "刑法", "证据"]
}

impl CriminalEvidenceDeepRules {
    /// 证据收集规则详解
    pub fn evidence_collection_detailed(&self) -> Vec<&'static str> {
        vec![
            "物证收集规则: 收集物证应当采用合法手段，不得采用非法搜查、扣押等方式",
            "书证收集规则: 收集书证应当采用合法手段，不得采用非法搜查、扣押等方式",
            "证人证言收集规则: 收集证人证言应当依法进行，不得采用威胁、引诱、欺骗等方式",
            "被告人供述收集规则: 收集被告人供述应当依法进行，不得采用刑讯逼供等方式",
            "鉴定意见收集规则: 收集鉴定意见应当依法进行，鉴定人应当具备相应资质",
            "勘验检查笔录收集规则: 收集勘验检查笔录应当依法进行，由侦查人员制作",
            "视听资料收集规则: 收集视听资料应当采用合法手段，不得采用非法窃听、窃录等方式",
            "电子数据收集规则: 收集电子数据应当采用合法手段，不得采用非法侵入计算机系统等方式",
            "证据保全规则: 对需要保全的证据应当及时采取保全措施，防止证据灭失或毁损",
            "证据收集程序规则: 证据收集应当遵守法定程序，制作相应笔录和文书",
        ]
    }

    /// 证据审查规则详解
    pub fn evidence_review_detailed(&self) -> Vec<&'static str> {
        vec![
            "证据真实性审查: 审查证据的真实性，判断证据是否真实可靠",
            "证据合法性审查: 审查证据的合法性，判断证据是否依法收集",
            "证据关联性审查: 审查证据的关联性，判断证据是否与案件有关",
            "证据充分性审查: 审查证据的充分性，判断证据是否足以证明案件事实",
            "证据证明力审查: 审查证据的证明力，判断证据证明案件事实的力度",
            "证据矛盾性审查: 审查证据之间的矛盾，判断如何解决证据矛盾",
            "证据补强规则审查: 审查需要补强的证据，判断补强证据是否足够",
            "证据鉴定规则审查: 审查鉴定意见，判断鉴定是否科学可靠",
            "证据辨认规则审查: 审查辨认结论，判断辨认是否准确可靠",
            "证据推理规则审查: 审查证据推理，判断推理是否合理可靠",
        ]
    }

    /// 证据认定规则详解
    pub fn evidence_determination_detailed(&self) -> Vec<&'static str> {
        vec![
            "证据认定标准: 证据必须经过查证属实才能作为定案的根据",
            "证据认定程序: 证据认定应当经过法庭调查和法庭辩论程序",
            "证据认定方法: 证据认定应当采用科学方法进行审查判断",
            "证据认定责任: 证据认定由审判人员负责，审判人员应当独立认定证据",
            "证据认定原则: 证据认定应当坚持客观公正原则，不得主观臆断",
            "证据认定依据: 证据认定应当依据事实和法律进行判断",
            "证据认定结论: 证据认定应当得出明确的结论，作为定案的依据",
            "证据认定说明: 证据认定应当在判决书中说明理由和依据",
            "证据认定监督: 证据认定应当接受监督，防止认定错误",
            "证据认定纠正: 证据认定错误应当及时纠正，防止冤假错案",
        ]
    }

    /// 证据排除规则详解
    pub fn evidence_exclusion_detailed(&self) -> Vec<&'static str> {
        vec![
            "非法证据排除规则: 以刑讯逼供等非法方法收集的犯罪嫌疑人、被告人供述应当予以排除",
            "非法物证书证排除规则: 收集物证、书证不符合法定程序可能严重影响司法公正的应当予以补正或作出合理解释否则予以排除",
            "非法证人证言排除规则: 采用暴力、威胁等非法方法收集的证人证言应当予以排除",
            "非法鉴定意见排除规则: 鉴定人不具备相应资质或鉴定程序违法的鉴定意见应当予以排除",
            "非法视听资料排除规则: 采用非法窃听、窃录等方法收集的视听资料应当予以排除",
            "非法电子数据排除规则: 采用非法侵入计算机系统等方法收集的电子数据应当予以排除",
            "非法证据排除程序: 非法证据排除应当经过法定程序由人民法院审查决定",
            "非法证据排除责任: 非法证据排除由人民检察院承担举证责任",
            "非法证据排除申请: 犯罪嫌疑人、被告人及其辩护人有权申请排除非法证据",
            "非法证据排除救济: 对非法证据排除决定不服的可以申请复议或上诉",
        ]
    }

    /// 证据证明规则详解
    pub fn evidence_proof_detailed(&self) -> Vec<&'static str> {
        vec![
            "证明责任分配: 证明被告人有罪的责任由人民检察院承担",
            "证明责任倒置: 在特定情形下证明责任可以倒置给被告人",
            "证明标准: 案件事实清楚证据确实充分是刑事案件的证明标准",
            "证明程度: 证明程度应当达到排除合理怀疑的程度",
            "证明方法: 证明应当采用直接证明和间接证明相结合的方法",
            "证明顺序: 证明应当按照先证明犯罪事实后证明量刑事实的顺序进行",
            "证明对象: 证明对象包括犯罪事实、量刑事实、程序事实等",
            "证明范围: 证明范围应当覆盖案件的主要事实和关键事实",
            "证明效力: 证明效力应当达到足以认定案件事实的程度",
            "证明瑕疵: 证明瑕疵应当通过补强证据或合理解释进行补救",
        ]
    }

    /// 证据保全规则详解
    pub fn evidence_preservation_detailed(&self) -> Vec<&'static str> {
        vec![
            "证据保全申请: 当事人有权申请证据保全防止证据灭失或毁损",
            "证据保全措施: 人民法院应当及时采取证据保全措施",
            "证据保全方法: 证据保全应当采用科学方法进行保全",
            "证据保全程序: 证据保全应当遵守法定程序进行",
            "证据保全期限: 证据保全应当在法定期限内进行",
            "证据保全责任: 证据保全由人民法院负责实施",
            "证据保全监督: 证据保全应当接受监督防止滥用",
            "证据保全费用: 证据保全费用由申请人承担或由败诉方承担",
            "证据保全效力: 证据保全的证据可以作为定案的依据",
            "证据保全救济: 对证据保全决定不服的可以申请复议",
        ]
    }
}

impl Rule for CriminalEvidenceDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_evidence_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑事证据规则深度规则",
            &[
                ("证据收集规则详解", &self.evidence_collection_detailed()),
                ("证据审查规则详解", &self.evidence_review_detailed()),
                ("证据认定规则详解", &self.evidence_determination_detailed()),
                ("证据排除规则详解", &self.evidence_exclusion_detailed()),
                ("证据证明规则详解", &self.evidence_proof_detailed()),
                ("证据保全规则详解", &self.evidence_preservation_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminal_evidence_deep_rules() {
        let rules = CriminalEvidenceDeepRules::new();
        assert_eq!(rules.metadata().name, "刑事证据规则深度规则");
        assert!(!rules.evidence_collection_detailed().is_empty());
        assert!(!rules.evidence_review_detailed().is_empty());
        assert!(!rules.evidence_determination_detailed().is_empty());
        assert!(!rules.evidence_exclusion_detailed().is_empty());
        assert!(!rules.evidence_proof_detailed().is_empty());
        assert!(!rules.evidence_preservation_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_sections_count() {
        let rules = CriminalEvidenceDeepRules::new();
        assert_eq!(rules.evidence_collection_detailed().len(), 10);
        assert_eq!(rules.evidence_review_detailed().len(), 10);
        assert_eq!(rules.evidence_determination_detailed().len(), 10);
        assert_eq!(rules.evidence_exclusion_detailed().len(), 10);
        assert_eq!(rules.evidence_proof_detailed().len(), 10);
        assert_eq!(rules.evidence_preservation_detailed().len(), 10);
    }
}
