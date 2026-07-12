//! 民法典人格权编深度规则
//!
//! 涵盖人格权编的详细内容，包括：
//! - 人格权一般规则
//! - 生命权身体权健康权
//! - 姓名权名称权
//! - 名誉权荣誉权
//! - 隐私权个人信息保护
//! - 肖像权
//! - 人格权保护规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CivilCodePersonalityDeepRules,
    name: "民法典人格权编深度规则",
    desc: "民法典人格权编的详细规则解析",
    origin: "中国",
    tags: ["法律", "民法", "民法典", "人格权"]
}

impl CivilCodePersonalityDeepRules {
    /// 人格权一般规则
    pub fn personality_general_detailed(&self) -> Vec<&'static str> {
        vec![
            "人格权概念:人格权是民事主体享有的生命权、身体权、健康权、姓名权、名称权、肖像权、名誉权、荣誉权、隐私权等权利",
            "人格权保护:人格权不得放弃、转让或继承，人格权受到侵害的权利人有权请求行为人承担民事责任",
            "人格权内容:民事主体的人格权受法律保护，任何组织或个人不得侵害",
            "人格利益许可使用:民事主体可以许可他人使用其姓名、名称、肖像等人格利益，但不得违反公序良俗",
            "人格权请求权:人格权受到侵害或有被侵害之虞的权利人有权请求行为人停止侵害、排除妨害、消除危险",
            "精神损害赔偿:侵害自然人人身权益造成严重精神损害的受害人有权请求精神损害赔偿",
            "死者人格利益保护:死者的姓名、肖像、名誉、荣誉、隐私、遗体等受到侵害的其配偶、子女、父母有权请求保护",
            "人格权禁令:民事主体有证据证明行为人正在实施或即将实施侵害其人格权的行为有权依法申请责令行为人停止有关行为",
        ]
    }

    /// 生命权身体权健康权
    pub fn life_body_health_detailed(&self) -> Vec<&'static str> {
        vec![
            "生命权:自然人享有生命权，有权维护生命安全，生命权不得非法剥夺",
            "生命权内容:自然人的生命安全和生命尊严受法律保护，任何组织或个人不得侵害他人的生命权",
            "身体权:自然人享有身体权，有权维护身体完整",
            "身体权内容:自然人的身体完整和行动自由受法律保护，任何组织或个人不得侵害他人的身体权",
            "身体处置限制:自然人的身体不得买卖、出租或非法处置",
            "捐献人体组织:自然人可以捐献人体细胞、人体组织、人体器官、遗体，捐献应当自愿无偿",
            "健康权:自然人享有健康权，有权维护身心健康",
            "健康权内容:自然人的身心健康受法律保护，任何组织或个人不得侵害他人的健康权",
            "医疗知情同意:医务人员应当向患者说明医疗风险、替代医疗方案等情况并取得其明确同意",
            "紧急医疗:不能取得患者或近亲属意见的经医疗机构负责人批准可以立即实施相应医疗措施",
        ]
    }

    /// 姓名权名称权
    pub fn name_rights_detailed(&self) -> Vec<&'static str> {
        vec![
            "姓名权:自然人享有姓名权有权依法决定使用变更或许可他人使用自己的姓名",
            "姓名权内容:自然人可以随父姓或母姓也可以选取其他直系长辈血亲的姓氏或其他正当姓氏",
            "姓名变更:自然人变更姓名应当向有关机关办理登记手续",
            "姓名保护:任何组织或个人不得以干涉盗用假冒等方式侵害他人的姓名权",
            "名称权:法人非法人组织享有名称权有权依法决定使用变更转让或许可他人使用自己的名称",
            "名称登记:法人非法人组织的名称应当依法登记",
            "名称变更:法人非法人组织变更名称应当依法办理变更登记",
            "名称转让:法人非法人组织可以转让其名称",
            "名称保护:任何组织或个人不得以干涉盗用假冒等方式侵害他人的名称权",
            "名称使用:使用名称的组织应当规范使用登记名称不得损害他人名称权",
        ]
    }

    /// 名誉权荣誉权
    pub fn reputation_honor_detailed(&self) -> Vec<&'static str> {
        vec![
            "名誉权:自然人法人非法人组织享有名誉权有权维护自己的名誉",
            "名誉权内容:任何组织或个人不得以侮辱诽谤等方式侵害他人的名誉权",
            "名誉侵权认定:行为人为公共利益实施新闻报道舆论监督等行为影响他人名誉的应当承担民事责任",
            "名誉侵权抗辩:行为人为公共利益实施新闻报道舆论监督等行为有下列情形之一的不承担民事责任",
            "名誉侵权例外:捏造歪曲事实；对他人提供的严重失实内容未尽合理核实义务；使用侮辱性言辞贬损他人名誉",
            "核实义务认定:考虑内容来源的可信度；对明显可能引发争议的内容是否进行调查；内容的时限性",
            "荣誉权:自然人法人非法人组织享有荣誉权有权维护自己的荣誉",
            "荣誉权内容:任何组织或个人不得非法剥夺他人的荣誉称号不得诋毁贬损他人的荣誉",
            "荣誉侵害救济:荣誉权受到侵害的权利人有权请求停止侵害恢复名誉消除影响赔礼道歉赔偿损失",
            "荣誉权请求:荣誉权受到侵害的权利人可以依法请求责令侵害人停止侵害恢复荣誉",
        ]
    }

    /// 隐私权个人信息保护
    pub fn privacy_personal_info_detailed(&self) -> Vec<&'static str> {
        vec![
            "隐私权:自然人享有隐私权有权维护自己的隐私",
            "隐私范围:隐私是自然人的私人生活安宁和不愿为他人知晓的私密空间私密活动私密信息",
            "隐私权内容:任何组织或个人不得以刺探侵扰泄露公开等方式侵害他人的隐私权",
            "隐私侵害方式:进入窥视拍摄他人的住宅宾馆房间等私密空间；拍摄录制公开他人的私密活动",
            "个人信息保护:自然人的个人信息受法律保护",
            "个人信息范围:姓名出生日期身份证号码生物识别信息住址电话号码电子邮箱健康信息行踪信息等",
            "个人信息处理原则:处理个人信息应当遵循合法正当必要原则不得过度处理",
            "个人信息处理条件:征得该自然人或其监护人同意；公开处理信息的规则；明示处理信息的目的方式和范围",
            "个人信息安全:信息处理者应当采取技术措施和其他必要措施确保其收集存储的个人信息安全",
            "个人信息泄露救济:个人信息泄露篡改丢失的应当及时采取补救措施告知自然人并向有关主管部门报告",
        ]
    }

    /// 肖像权
    pub fn portrait_rights_detailed(&self) -> Vec<&'static str> {
        vec![
            "肖像权:自然人享有肖像权有权依法制作使用公开或许可他人使用自己的肖像",
            "肖像概念:肖像是通过影像雕塑绘画等方式在一定载体上所反映的特定自然人可以被识别的外部形象",
            "肖像权内容:任何组织或个人不得以丑化污损或者利用信息技术手段伪造等方式侵害他人的肖像权",
            "肖像使用:未经肖像权人同意不得制作使用公开肖像权人的肖像",
            "肖像使用例外:为个人学习艺术欣赏课堂教学或科学研究在必要范围内使用肖像权人已经公开的肖像",
            "新闻报道例外:为实施新闻报道不可避免地制作使用公开肖像权人的肖像",
            "公共利益例外:国家机关为依法履行职责在必要范围内制作使用公开肖像权人的肖像",
            "肖像许可使用:肖像权人可以许可他人使用其肖像并可以就肖像使用的范围方式等作出约定",
            "肖像许可合同:当事人对肖像许可使用期限没有约定或约定不明确的任何一方可以随时解除肖像许可使用合同",
            "肖像合理使用:合理使用肖像的行为肖像权人不得禁止",
        ]
    }

    /// 人格权保护规则
    pub fn personality_protection_detailed(&self) -> Vec<&'static str> {
        vec![
            "人格权禁令:民事主体有证据证明行为人正在实施或即将实施侵害其人格权的行为有权依法申请责令行为人停止有关行为",
            "人格权请求权:人格权受到侵害或有被侵害之虞的权利人有权请求行为人停止侵害排除妨害消除危险消除影响恢复名誉赔礼道歉",
            "精神损害赔偿:侵害自然人人身权益造成严重精神损害的受害人有权请求精神损害赔偿",
            "损害赔偿计算:侵害他人人身权益造成财产损失的按照被侵权人因此受到的损失或侵权人因此获得的利益赔偿",
            "人格权侵害认定:认定行为人侵害人格权应当考虑行为人和受害人的职业影响范围过错程度行为的目的方式后果等因素",
            "消除影响恢复名誉:侵害人格权需要消除影响恢复名誉的应当根据行为方式和造成的影响范围合理确定",
            "赔礼道歉:侵害人格权的应当赔礼道歉赔礼道歉应当公开进行",
            "人格权诉讼时效:人格权受到侵害的权利人请求行为人承担民事责任不受诉讼时效限制",
            "人格权救济方式:人格权受到侵害的权利人可以通过和解调解仲裁诉讼等方式寻求救济",
            "人格权证据保全:权利人为制止侵权行为在证据可能灭失或以后难以取得的情况下可以申请证据保全",
        ]
    }
}

impl Rule for CivilCodePersonalityDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_personality_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典人格权编深度规则",
            &[
                ("人格权一般规则", &self.personality_general_detailed()),
                ("生命权身体权健康权", &self.life_body_health_detailed()),
                ("姓名权名称权", &self.name_rights_detailed()),
                ("名誉权荣誉权", &self.reputation_honor_detailed()),
                ("隐私权个人信息保护", &self.privacy_personal_info_detailed()),
                ("肖像权", &self.portrait_rights_detailed()),
                ("人格权保护规则", &self.personality_protection_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_code_personality_deep_rules() {
        let rules = CivilCodePersonalityDeepRules::new();
        assert_eq!(rules.metadata().name, "民法典人格权编深度规则");
        assert!(!rules.personality_general_detailed().is_empty());
        assert!(!rules.life_body_health_detailed().is_empty());
        assert!(!rules.name_rights_detailed().is_empty());
        assert!(!rules.reputation_honor_detailed().is_empty());
        assert!(!rules.privacy_personal_info_detailed().is_empty());
        assert!(!rules.portrait_rights_detailed().is_empty());
        assert!(!rules.personality_protection_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_life_body_health_count() {
        let rules = CivilCodePersonalityDeepRules::new();
        assert_eq!(rules.life_body_health_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = CivilCodePersonalityDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("civil_code_personality_deep")
        );
    }
}
