//! 刑事司法改革深度规则
//!
//! 涵盖刑事司法改革的详细内容，包括：
//! - 司法体制改革详解
//! - 量刑制度改革详解
//! - 执行制度改革详解
//! - 辩护制度改革详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CriminalJusticeReformDeepRules,
    name: "刑事司法改革深度规则",
    desc: "刑事司法改革的详细规则解析",
    origin: "中国",
    tags: ["法律", "刑法", "司法"]
}

impl CriminalJusticeReformDeepRules {
    /// 司法体制改革详解
    pub fn judicial_system_reform_detailed(&self) -> Vec<&'static str> {
        vec![
            "司法独立原则改革: 强化人民法院依法独立行使审判权，保障司法公正",
            "司法责任制改革: 建立司法责任制，落实审判人员办案责任，提高司法质量",
            "司法人员分类管理改革: 实行法官、检察官、司法辅助人员分类管理，优化司法队伍",
            "司法人员职业保障改革: 建立司法人员职业保障制度，提高司法人员待遇和地位",
            "司法人员选拔改革: 改进司法人员选拔机制，实行司法人员统一招录和遴选",
            "司法人员培训改革: 加强司法人员培训，提高司法人员专业素质和能力",
            "司法信息化改革: 推进司法信息化建设，提高司法效率和质量",
            "司法公开改革: 推进司法公开，提高司法透明度，保障司法公正",
            "司法监督改革: 建立司法监督机制，加强司法监督，防止司法腐败",
            "司法考核改革: 建立司法考核制度，科学考核司法工作，激励司法人员",
        ]
    }

    /// 量刑制度改革详解
    pub fn sentencing_reform_detailed(&self) -> Vec<&'static str> {
        vec![
            "量刑规范化改革: 制定量刑指导意见，规范量刑程序，提高量刑质量",
            "量刑公开改革: 推进量刑公开，提高量刑透明度，保障量刑公正",
            "量刑建议改革: 推进量刑建议制度，人民检察院提出量刑建议供人民法院参考",
            "量刑辩论改革: 推进量刑辩论程序，控辩双方就量刑问题进行辩论",
            "量刑说理改革: 推进量刑说理制度，人民法院在判决书中说明量刑理由",
            "量刑均衡改革: 建立量刑均衡机制，防止量刑畸轻畸重",
            "量刑监督改革: 建立量刑监督机制，加强量刑监督，防止量刑不公",
            "量刑信息化改革: 推进量刑信息化建设，提高量刑效率和质量",
            "量刑评估改革: 建立量刑评估制度，评估量刑效果，改进量刑工作",
            "量刑调整改革: 根据社会发展和犯罪变化适时调整量刑标准",
        ]
    }

    /// 执行制度改革详解
    pub fn execution_reform_detailed(&self) -> Vec<&'static str> {
        vec![
            "监狱体制改革: 推进监狱体制改革，提高监狱管理水平，保障罪犯权益",
            "社区矫正改革: 推进社区矫正改革，扩大社区矫正适用范围，提高社区矫正效果",
            "减刑假释改革: 改进减刑假释制度，规范减刑假释程序，提高减刑假释质量",
            "刑罚执行监督改革: 加强刑罚执行监督，防止刑罚执行中的违法行为",
            "刑罚执行评估改革: 建立刑罚执行评估制度，评估刑罚执行效果",
            "刑罚执行信息化改革: 推进刑罚执行信息化建设，提高刑罚执行效率和质量",
            "刑罚执行人权保障改革: 加强刑罚执行中的人权保障，防止侵犯罪犯合法权益",
            "刑罚执行人道主义改革: 推进刑罚执行人道主义改革，保障罪犯基本权利",
            "刑罚执行社会化改革: 推进刑罚执行社会化改革，加强社会力量参与刑罚执行",
            "刑罚执行国际化改革: 推进刑罚执行国际化改革，学习借鉴国际先进经验",
        ]
    }

    /// 辩护制度改革详解
    pub fn defense_reform_detailed(&self) -> Vec<&'static str> {
        vec![
            "辩护权保障改革: 加强辩护权保障，保障犯罪嫌疑人、被告人的辩护权",
            "辩护律师制度改革: 改进辩护律师制度，提高辩护律师素质和能力",
            "法律援助制度改革: 改进法律援助制度，扩大法律援助范围，提高法律援助质量",
            "辩护律师权利保障改革: 加强辩护律师权利保障，保障辩护律师执业权利",
            "辩护律师执业监督改革: 加强辩护律师执业监督，规范辩护律师执业行为",
            "辩护律师收费改革: 改进辩护律师收费制度，规范辩护律师收费标准",
            "辩护律师培训改革: 加强辩护律师培训，提高辩护律师专业素质和能力",
            "辩护律师信息化改革: 推进辩护律师信息化建设，提高辩护律师工作效率",
            "辩护律师国际化改革: 推进辩护律师国际化改革，学习借鉴国际先进经验",
            "辩护律师社会责任改革: 强化辩护律师社会责任，提高辩护律师社会责任意识",
        ]
    }

    /// 刑事诉讼程序改革详解
    pub fn criminal_procedure_reform_detailed(&self) -> Vec<&'static str> {
        vec![
            "立案制度改革: 改进立案制度，规范立案程序，提高立案质量",
            "侦查制度改革: 改进侦查制度，规范侦查行为，提高侦查质量",
            "起诉制度改革: 改进起诉制度，规范起诉程序，提高起诉质量",
            "审判制度改革: 改进审判制度，规范审判程序，提高审判质量",
            "证据制度改革: 改进证据制度，规范证据规则，提高证据质量",
            "辩护制度改革: 改进辩护制度，保障辩护权利，提高辩护质量",
            "上诉制度改革: 改进上诉制度，规范上诉程序，提高上诉质量",
            "执行制度改革: 改进执行制度，规范执行程序，提高执行质量",
            "监督制度改革: 改进监督制度，加强监督机制，提高监督质量",
            "信息化制度改革: 推进信息化建设，提高刑事诉讼效率和质量",
        ]
    }

    /// 刑事司法人权保障改革详解
    pub fn human_rights_reform_detailed(&self) -> Vec<&'static str> {
        vec![
            "辩护权保障改革: 加强辩护权保障，保障犯罪嫌疑人、被告人的辩护权",
            "人身自由权保障改革: 加强人身自由权保障，防止非法拘禁和超期羁押",
            "隐私权保障改革: 加强隐私权保障，防止非法搜查和非法取证",
            "公平审判权保障改革: 加强公平审判权保障，保障犯罪嫌疑人、被告人的公平审判权",
            "申诉权保障改革: 加强申诉权保障，保障犯罪嫌疑人、被告人的申诉权",
            "赔偿权保障改革: 加强赔偿权保障，保障犯罪被害人的赔偿权",
            "被害人权利保障改革: 加强被害人权利保障，保障被害人的合法权益",
            "未成年人权利保障改革: 加强未成年人权利保障，保障未成年人的合法权益",
            "弱势群体权利保障改革: 加强弱势群体权利保障，保障弱势群体的合法权益",
            "死刑制度改革: 推进死刑制度改革，减少死刑适用，保障死刑案件质量",
        ]
    }
}

impl Rule for CriminalJusticeReformDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_justice_reform_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑事司法改革深度规则",
            &[
                ("司法体制改革详解", &self.judicial_system_reform_detailed()),
                ("量刑制度改革详解", &self.sentencing_reform_detailed()),
                ("执行制度改革详解", &self.execution_reform_detailed()),
                ("辩护制度改革详解", &self.defense_reform_detailed()),
                (
                    "刑事诉讼程序改革详解",
                    &self.criminal_procedure_reform_detailed(),
                ),
                (
                    "刑事司法人权保障改革详解",
                    &self.human_rights_reform_detailed(),
                ),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminal_justice_reform_deep_rules() {
        let rules = CriminalJusticeReformDeepRules::new();
        assert_eq!(rules.metadata().name, "刑事司法改革深度规则");
        assert!(!rules.judicial_system_reform_detailed().is_empty());
        assert!(!rules.sentencing_reform_detailed().is_empty());
        assert!(!rules.execution_reform_detailed().is_empty());
        assert!(!rules.defense_reform_detailed().is_empty());
        assert!(!rules.criminal_procedure_reform_detailed().is_empty());
        assert!(!rules.human_rights_reform_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_sections_count() {
        let rules = CriminalJusticeReformDeepRules::new();
        assert_eq!(rules.judicial_system_reform_detailed().len(), 10);
        assert_eq!(rules.sentencing_reform_detailed().len(), 10);
        assert_eq!(rules.execution_reform_detailed().len(), 10);
        assert_eq!(rules.defense_reform_detailed().len(), 10);
        assert_eq!(rules.criminal_procedure_reform_detailed().len(), 10);
        assert_eq!(rules.human_rights_reform_detailed().len(), 10);
    }
}
