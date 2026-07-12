//! 刑法分则深度规则
//!
//! 涵盖刑法分则各类犯罪的详细内容，包括：
//! - 危害国家安全罪详解
//! - 危害公共安全罪详解
//! - 破坏社会主义市场经济秩序罪详解
//! - 侵犯公民人身权利民主权利罪详解
//! - 侵犯财产罪详解
//! - 妨害社会管理秩序罪详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CriminalLawSpecificDeepRules,
    name: "刑法分则深度规则",
    desc: "刑法分则各类犯罪的详细规则解析",
    origin: "中国",
    tags: ["法律", "刑法", "分则"]
}

impl CriminalLawSpecificDeepRules {
    /// 危害国家安全罪详解
    pub fn national_security_crimes_detailed(&self) -> Vec<&'static str> {
        vec![
            "背叛国家罪: 勾结外国或与境外机构、组织、个人相勾结，危害中华人民共和国的国家主权、领土完整和安全的行为",
            "分裂国家罪: 组织、策划、实施分裂国家、破坏国家统一的行为",
            "煽动分裂国家罪: 煽动分裂国家、破坏国家统一的行为",
            "武装叛乱暴乱罪: 组织、策划、实施武装叛乱或武装暴乱的行为",
            "颠覆国家政权罪: 组织、策划、实施颠覆国家政权、推翻社会主义制度的行为",
            "煽动颠覆国家政权罪: 以造谣、诽谤或其他方式煽动颠覆国家政权、推翻社会主义制度的行为",
            "资助危害国家安全犯罪活动罪: 境内外机构、组织或个人资助实施背叛国家罪、分裂国家罪等的行为",
            "投敌叛变罪: 中国公民投奔敌方或敌对阵营，或在敌方或敌对阵营进行危害国家安全活动的行为",
            "叛逃罪: 国家机关工作人员在履行公务期间擅离岗位叛逃境外或在境外叛逃的行为",
            "间谍罪: 参加间谍组织或接受间谍组织及其代理人的任务，或为敌人指示轰击目标的行为",
        ]
    }

    /// 危害公共安全罪详解
    pub fn public_security_crimes_detailed(&self) -> Vec<&'static str> {
        vec![
            "放火罪: 故意放火焚烧公私财物，危害公共安全的行为",
            "决水罪: 故意破坏水利设施，制造水患，危害公共安全的行为",
            "爆炸罪: 故意引发爆炸物或其他爆炸性物质，危害公共安全的行为",
            "投放危险物质罪: 故意投放毒害性、放射性、传染病病原体等物质，危害公共安全的行为",
            "以危险方法危害公共安全罪: 使用除放火、决水、爆炸、投放危险物质以外的其他危险方法危害公共安全的行为",
            "破坏交通工具罪: 破坏火车、汽车、电车、船只、航空器，足以使火车、汽车、电车、船只、航空器发生倾覆、毁坏危险的行为",
            "破坏交通设施罪: 破坏轨道、桥梁、隧道、公路、机场、航道、灯塔、标志或进行其他破坏活动，足以使火车、汽车、电车、船只、航空器发生倾覆、毁坏危险的行为",
            "组织领导参加恐怖组织罪: 组织、领导或积极参加恐怖活动组织的行为",
            "劫持航空器罪: 以暴力、胁迫或其他方法劫持航空器的行为",
            "非法制造买卖运输邮寄储存枪支弹药爆炸物罪: 违反法律规定，擅自制造、买卖、运输、邮寄、储存枪支、弹药、爆炸物的行为",
        ]
    }

    /// 侵犯人身权利罪详解
    pub fn personal_rights_crimes_detailed(&self) -> Vec<&'static str> {
        vec![
            "故意杀人罪: 故意非法剥夺他人生命的行为，是最严重的侵犯公民人身权利的犯罪",
            "过失致人死亡罪: 因过失致使他人死亡的行为",
            "故意伤害罪: 故意非法损害他人身体健康的行为，分为轻伤、重伤、伤害致死三个量刑档次",
            "强奸罪: 以暴力、胁迫或其他手段强奸妇女的行为",
            "强制猥亵侮辱罪: 以暴力、胁迫或其他手段强制猥亵他人或侮辱妇女的行为",
            "非法拘禁罪: 以拘押、禁闭或其他强制方法非法剥夺他人人身自由的行为",
            "绑架罪: 以勒索财物为目的绑架他人或绑架他人作为人质的行为",
            "拐卖妇女儿童罪: 以出卖为目的拐骗、绑架、收买、贩卖、接送、中转妇女、儿童的行为",
            "收买被拐卖的妇女儿童罪: 收买被拐卖的妇女、儿童的行为",
            "聚众阻碍解救被收买的妇女儿童罪: 聚众阻碍国家机关工作人员解救被收买的妇女、儿童的行为",
        ]
    }

    /// 侵犯财产罪详解
    pub fn property_crimes_detailed(&self) -> Vec<&'static str> {
        vec![
            "抢劫罪: 以暴力、胁迫或其他方法抢劫公私财物的行为",
            "盗窃罪: 盗窃公私财物数额较大的，或多次盗窃、入户盗窃、携带凶器盗窃、扒窃的行为",
            "诈骗罪: 诈骗公私财物数额较大的行为",
            "抢夺罪: 抢夺公私财物数额较大的行为",
            "聚众哄抢罪: 聚众哄抢公私财物数额较大或情节严重的行为",
            "侵占罪: 将代为保管的他人财物非法占为己有数额较大拒不退还，或将他人的遗忘物、埋藏物非法占为己有数额较大拒不交出的行为",
            "职务侵占罪: 公司、企业或其他单位的人员利用职务上的便利将本单位财物非法占为己有数额较大的行为",
            "挪用资金罪: 公司、企业或其他单位的人员利用职务上的便利挪用本单位资金归个人使用或借贷给他人数额较大超过三个月未还的行为",
            "敲诈勒索罪: 以威胁或要挟方法强索公私财物的行为",
            "故意毁坏财物罪: 故意毁坏公私财物数额较大或有其他严重情节的行为",
        ]
    }

    /// 妨害社会管理秩序罪详解
    pub fn social_management_crimes_detailed(&self) -> Vec<&'static str> {
        vec![
            "妨害公务罪: 以暴力、威胁方法阻碍国家机关工作人员依法执行职务的行为",
            "煽动暴力抗拒法律实施罪: 煽动群众暴力抗拒国家法律、行政法规实施的行为",
            "招摇撞骗罪: 冒充国家机关工作人员进行招摇撞骗的行为",
            "伪造变造买卖国家机关公文证件印章罪: 伪造、变造、买卖国家机关公文、证件、印章的行为",
            "盗窃抢夺毁灭国家机关公文证件印章罪: 盗窃、抢夺、毁灭国家机关公文、证件、印章的行为",
            "聚众扰乱社会秩序罪: 聚众扰乱社会秩序情节严重致使工作、生产、营业和教学、科研、医疗无法进行的行为",
            "聚众冲击国家机关罪: 财务冲击国家机关致使国家机关工作无法进行造成严重损失的行为",
            "聚众扰乱公共场所秩序交通秩序罪: 财务扰乱车站、码头、民用航空站、商场、公园、影剧院、展览会、运动场或其他公共场所秩序的行为",
            "投放虚假危险物质罪: 投放虚假的爆炸性、毒害性、放射性、传染病病原体等物质严重扰乱社会秩序的行为",
            "编造故意传播虚假恐怖信息罪: 编造爆炸威胁、生化威胁、放射威胁等恐怖信息或明知是编造的恐怖信息而故意传播严重扰乱社会秩序的行为",
        ]
    }

    /// 贪污贿赂罪详解
    pub fn corruption_crimes_detailed(&self) -> Vec<&'static str> {
        vec![
            "贪污罪: 国家工作人员利用职务上的便利侵吞、窃取、骗取或以其他手段非法占有公共财物的行为",
            "挪用公款罪: 国家工作人员利用职务上的便利挪用公款归个人使用进行非法活动或数额较大进行营利活动或数额较大超过三个月未还的行为",
            "受贿罪: 国家工作人员利用职务上的便利索取他人财物或非法收受他人财物为他人谋取利益的行为",
            "单位受贿罪: 国家机关、国有公司、企业、事业单位、人民团体索取、非法收受他人财物为他人谋取利益情节严重的行为",
            "利用影响力受贿罪: 国家工作人员的近亲属或其他与该国家工作人员关系密切的人通过该国家工作人员职务上的行为或利用该国家工作人员职权或地位形成的便利条件受贿的行为",
            "行贿罪: 为谋取不正当利益给予国家工作人员以财物的行为",
            "对单位行贿罪: 为谋取不正当利益给予国家机关、国有公司、企业、事业单位、人民团体以财物或在经济往来中违反国家规定给予各种名义的回扣、手续费的行为",
            "介绍贿赂罪: 向国家工作人员介绍贿赂情节严重的行为",
            "单位行贿罪: 单位为谋取不正当利益而行贿或违反国家规定给予国家工作人员以回扣、手续费情节严重的行为",
            "巨额财产来源不明罪: 国家工作人员的财产、支出明显超过合法收入差额巨大不能说明来源的行为",
        ]
    }
}

impl Rule for CriminalLawSpecificDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_law_specific_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑法分则深度规则",
            &[
                (
                    "危害国家安全罪详解",
                    &self.national_security_crimes_detailed(),
                ),
                (
                    "危害公共安全罪详解",
                    &self.public_security_crimes_detailed(),
                ),
                (
                    "侵犯人身权利罪详解",
                    &self.personal_rights_crimes_detailed(),
                ),
                ("侵犯财产罪详解", &self.property_crimes_detailed()),
                (
                    "妨害社会管理秩序罪详解",
                    &self.social_management_crimes_detailed(),
                ),
                ("贪污贿赂罪详解", &self.corruption_crimes_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminal_law_specific_deep_rules() {
        let rules = CriminalLawSpecificDeepRules::new();
        assert_eq!(rules.metadata().name, "刑法分则深度规则");
        assert!(!rules.national_security_crimes_detailed().is_empty());
        assert!(!rules.public_security_crimes_detailed().is_empty());
        assert!(!rules.personal_rights_crimes_detailed().is_empty());
        assert!(!rules.property_crimes_detailed().is_empty());
        assert!(!rules.social_management_crimes_detailed().is_empty());
        assert!(!rules.corruption_crimes_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_crime_sections_count() {
        let rules = CriminalLawSpecificDeepRules::new();
        assert_eq!(rules.national_security_crimes_detailed().len(), 10);
        assert_eq!(rules.public_security_crimes_detailed().len(), 10);
        assert_eq!(rules.personal_rights_crimes_detailed().len(), 10);
        assert_eq!(rules.property_crimes_detailed().len(), 10);
        assert_eq!(rules.social_management_crimes_detailed().len(), 10);
        assert_eq!(rules.corruption_crimes_detailed().len(), 10);
    }
}
