//! 刑事被害人保护深度规则
//!
//! 涵盖刑事被害人保护的详细内容，包括：
//! - 被害人权利保障详解
//! - 被害人救助制度详解
//! - 被害人参与诉讼详解
//! - 被害人赔偿制度详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: VictimProtectionDeepRules,
    name: "刑事被害人保护深度规则",
    desc: "刑事被害人保护的详细规则解析",
    origin: "中国",
    tags: ["法律", "刑法", "被害人"]
}

impl VictimProtectionDeepRules {
    /// 被害人权利保障详解
    pub fn victim_rights_detailed(&self) -> Vec<&'static str> {
        vec![
            "被害人知情权: 被害人有权了解案件进展情况和案件处理结果",
            "被害人陈述权: 被害人有权在刑事诉讼中陈述案件情况和表达诉求",
            "被害人参与权: 被害人有权参与刑事诉讼活动如出席庭审、提出意见等",
            "被害人申请权: 被害人有权申请诉讼权利如申请回避、申请证据保全等",
            "被害人上诉权: 被害人有权对判决不服申请人民检察院抗诉",
            "被害人申诉权: 被害人有权对判决不服提出申诉",
            "被害人赔偿权: 被害人有权获得犯罪人的赔偿",
            "被害人救助权: 被害人有权获得国家和社会的救助",
            "被害人隐私权: 被害人的隐私应当受到保护防止二次伤害",
            "被害人尊严权: 被害人的人格尊严应当受到尊重和保护",
        ]
    }

    /// 被害人救助制度详解
    pub fn victim_assistance_detailed(&self) -> Vec<&'static str> {
        vec![
            "被害人救助对象: 对遭受犯罪侵害导致人身伤害、财产损失的被害人提供救助",
            "被害人救助条件: 被害人无法及时获得有效赔偿且生活困难的可以申请救助",
            "被害人救助程序: 被害人救助应当经过申请、审查、决定程序",
            "被害人救助金额: 被害人救助金额应当根据被害人的损失和困难程度确定",
            "被害人救助来源: 被害人救助资金来源于国家财政拨款和社会捐赠",
            "被害人救助时效: 被害人救助应当在法定期限内申请和发放",
            "被害人救助监督: 被害人救助应当接受监督防止滥用和挪用",
            "被害人救助评估: 被害人救助应当进行效果评估改进救助工作",
            "被害人救助协调: 被害人救助应当与社会保障制度协调衔接",
            "被害人救助完善: 被害人救助制度应当不断完善提高救助效果",
        ]
    }

    /// 被害人参与诉讼详解
    pub fn victim_participation_detailed(&self) -> Vec<&'static str> {
        vec![
            "被害人参与立案: 被害人有权报案、控告参与立案程序",
            "被害人参与侦查: 害人有权向侦查机关提供案件信息和证据",
            "被害人参与起诉: 被害人有权向人民检察院提出起诉意见",
            "被害人参与审判: 被害人有权出席庭审参与法庭调查和辩论",
            "被害人参与判决: 被害人有权了解判决内容和判决理由",
            "被害人参与执行: 被害人有权了解刑罚执行情况和罪犯表现",
            "被害人参与调解: 被害人有权参与刑事调解程序",
            "被害人参与和解: 被害人有权参与刑事和解程序",
            "被害人参与抗诉: 被害人有权申请人民检察院抗诉",
            "被害人参与申诉: 被害人有权提出申诉",
        ]
    }

    /// 被害人赔偿制度详解
    pub fn victim_compensation_detailed(&self) -> Vec<&'static str> {
        vec![
            "被害人赔偿原则: 犯罪人应当对被害人进行赔偿弥补被害人的损失",
            "被害人赔偿范围: 被害人赔偿包括人身伤害赔偿、财产损失赔偿、精神损害赔偿",
            "被害人赔偿标准: 被害人赔偿应当根据被害人的实际损失确定",
            "被害人赔偿程序: 被害人赔偿应当经过诉讼程序或调解程序",
            "被害人赔偿执行: 被害人赔偿应当及时执行保障被害人权益",
            "被害人赔偿不足补救: 被害人赔偿不足时可以通过国家救助补救",
            "被害人赔偿保障: 被害人赔偿应当得到保障防止赔偿落空",
            "被害人赔偿监督: 被害人赔偿应当接受监督防止不赔偿或少赔偿",
            "被害人赔偿评估: 被害人赔偿应当进行效果评估改进赔偿工作",
            "被害人赔偿协调: 被害人赔偿应当与社会保障制度协调衔接",
        ]
    }

    /// 被害人隐私保护详解
    pub fn victim_privacy_detailed(&self) -> Vec<&'static str> {
        vec![
            "被害人隐私保护原则: 被害人的隐私应当受到严格保护防止二次伤害",
            "被害人隐私保护范围: 被害人隐私包括个人身份信息、案件细节、精神创伤等",
            "被害人隐私保护措施: 应当采取保密措施保护被害人隐私",
            "被害人隐私保护程序: 被害人隐私保护应当贯穿刑事诉讼全过程",
            "被害人隐私保护责任: 司法机关应当承担被害人隐私保护责任",
            "被害人隐私保护监督: 被害人隐私保护应当接受监督防止泄露",
            "被害人隐私保护救济: 被害人隐私泄露应当及时救济防止进一步伤害",
            "被害人隐私保护教育: 应当加强被害人隐私保护教育提高保护意识",
            "被害人隐私保护技术: 应当采用技术手段保护被害人隐私",
            "被害人隐私保护协调: 被害人隐私保护应当与其他保护措施协调",
        ]
    }

    /// 被害人心理辅导详解
    pub fn victim_psychological_detailed(&self) -> Vec<&'static str> {
        vec![
            "被害人心理辅导意义: 心理辅导可以帮助被害人恢复心理健康",
            "被害人心理辅导对象: 需要心理辅导的被害人包括遭受严重犯罪的被害人",
            "被害人心理辅导时机: 心理辅导应当在犯罪发生后及时进行",
            "被害人心理辅导方法: 心理辅导应当采用科学方法进行",
            "被害人心理辅导人员: 心理辅导应当由专业人员进行",
            "被害人心理辅导程序: 心理辅导应当遵循专业程序进行",
            "被害人心理辅导效果: 心理辅导应当达到预期效果帮助被害人恢复",
            "被害人心理辅导评估: 心理辅导应当进行效果评估改进辅导工作",
            "被害人心理辅导费用: 心理辅导费用应当由国家或社会承担",
            "被害人心理辅导协调: 心理辅导应当与其他保护措施协调",
        ]
    }
}

impl Rule for VictimProtectionDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("victim_protection_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑事被害人保护深度规则",
            &[
                ("被害人权利保障详解", &self.victim_rights_detailed()),
                ("被害人救助制度详解", &self.victim_assistance_detailed()),
                ("被害人参与诉讼详解", &self.victim_participation_detailed()),
                ("被害人赔偿制度详解", &self.victim_compensation_detailed()),
                ("被害人隐私保护详解", &self.victim_privacy_detailed()),
                ("被害人心理辅导详解", &self.victim_psychological_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_victim_protection_deep_rules() {
        let rules = VictimProtectionDeepRules::new();
        assert_eq!(rules.metadata().name, "刑事被害人保护深度规则");
        assert!(!rules.victim_rights_detailed().is_empty());
        assert!(!rules.victim_assistance_detailed().is_empty());
        assert!(!rules.victim_participation_detailed().is_empty());
        assert!(!rules.victim_compensation_detailed().is_empty());
        assert!(!rules.victim_privacy_detailed().is_empty());
        assert!(!rules.victim_psychological_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_sections_count() {
        let rules = VictimProtectionDeepRules::new();
        assert_eq!(rules.victim_rights_detailed().len(), 10);
        assert_eq!(rules.victim_assistance_detailed().len(), 10);
        assert_eq!(rules.victim_participation_detailed().len(), 10);
        assert_eq!(rules.victim_compensation_detailed().len(), 10);
        assert_eq!(rules.victim_privacy_detailed().len(), 10);
        assert_eq!(rules.victim_psychological_detailed().len(), 10);
    }
}
