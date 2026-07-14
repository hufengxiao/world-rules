//! 社会法深度规则
//!
//! 涵盖社会法核心领域的详细内容，包括：
//! - 劳动法深度规则
//! - 社会保险深度规则
//! - 特殊群体保护深度规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: SocialLawDeepRules,
    name: "社会法深度规则",
    desc: "社会法核心领域的详细规则解析",
    origin: "中国",
    tags: ["法律", "社会法", "劳动法", "社会保险"]
}

impl SocialLawDeepRules {
    /// 劳动法深度规则
    pub fn labor_law_detailed(&self) -> Vec<&'static str> {
        vec![
            "劳动合同订立: 建立劳动关系应当订立书面劳动合同，已建立劳动关系未同时订立书面劳动合同的，应当自用工之日起一个月内订立",
            "劳动合同期限: 劳动合同分为固定期限劳动合同、无固定期限劳动合同和以完成一定工作任务为期限的劳动合同",
            "试用期规定: 劳动合同期限三个月以上不满一年的，试用期不得超过一个月；一年以上不满三年的，试用期不得超过二个月",
            "劳动报酬: 用人单位应当按照劳动合同约定和国家规定，向劳动者及时足额支付劳动报酬",
            "工作时间: 国家实行劳动者每日工作时间不超过八小时、平均每周工作时间不超过四十四小时的工时制度",
            "休息休假: 用人单位应当保证劳动者每周至少休息一日，劳动者连续工作一年以上享受带薪年休假",
            "加班规定: 用人单位由于生产经营需要，经与工会和劳动者协商后可以延长工作时间，一般每日不得超过一小时",
            "劳动保护: 用人单位必须建立、健全劳动安全卫生制度，严格执行国家劳动安全卫生规程和标准",
            "社会保险: 用人单位应当依法为劳动者缴纳社会保险费，劳动者应当缴纳的社会保险费由用人单位代扣代缴",
            "劳动合同解除: 用人单位与劳动者协商一致可以解除劳动合同，符合法定情形的用人单位可以单方解除劳动合同",
        ]
    }

    /// 社会保险深度规则
    pub fn social_insurance_detailed(&self) -> Vec<&'static str> {
        vec![
            "基本养老保险: 职工应当参加基本养老保险，由用人单位和职工共同缴纳基本养老保险费",
            "基本医疗保险: 职工应当参加职工基本医疗保险，由用人单位和职工按照国家规定共同缴纳基本医疗保险费",
            "工伤保险: 职工应当参加工伤保险，由用人单位缴纳工伤保险费，职工不缴纳工伤保险费",
            "失业保险: 职工应当参加失业保险，由用人单位和职工按照国家规定共同缴纳失业保险费",
            "生育保险: 职工应当参加生育保险，由用人单位按照国家规定缴纳生育保险费，职工不缴纳生育保险费",
            "社会保险登记: 用人单位应当自成立之日起三十日内凭营业执照、登记证书或单位印章向当地社会保险经办机构申请办理社会保险登记",
            "缴费基数: 用人单位应当按照国家规定的本单位职工工资总额的比例缴纳基本养老保险费",
            "养老金领取: 参加基本养老保险的个人，达到法定退休年龄时累计缴费满十五年的，按月领取基本养老金",
            "医疗待遇: 参加职工基本医疗保险的个人，符合基本医疗保险药品目录、诊疗项目、医疗服务设施标准的医疗费用，按照国家规定从基本医疗保险基金中支付",
            "工伤认定: 职工在工作时间和工作场所内，因工作原因受到事故伤害的，应当认定为工伤",
        ]
    }

    /// 特殊群体保护深度规则
    pub fn special_group_protection_detailed(&self) -> Vec<&'static str> {
        vec![
            "女职工保护: 禁止安排女职工从事矿山井下、国家规定的第四级体力劳动强度的劳动和其他禁忌从事的劳动",
            "孕期保护: 用人单位不得安排女职工在怀孕期间从事国家规定的第三级体力劳动强度的劳动和孕期禁忌从事的劳动",
            "产假规定: 女职工生育享受九十八天产假，其中产前可以休假十五天；难产的，增加产假十五天",
            "未成年工保护: 禁止用人单位招用未满十六周岁的未成年人，文艺、体育和特种工艺单位招用未满十六周岁的未成年人需经批准",
            "未成年工禁忌劳动: 禁止安排未成年工从事矿山井下、有毒有害、国家规定的第四级体力劳动强度的劳动和其他禁忌从事的劳动",
            "残疾人保障: 国家保障残疾人劳动的权利，用人单位应当按照一定比例安排残疾人就业",
            "残疾人就业保障金: 用人单位安排残疾人就业达不到其所在地省级人民政府规定比例的，应当缴纳残疾人就业保障金",
            "老年人权益: 国家保障老年人依法享有的权益，禁止歧视、侮辱、虐待或遗弃老年人",
            "老年人赡养: 赡养人应当履行对老年人经济上供养、生活上照料和精神上慰藉的义务",
            "儿童保护: 国家保护儿童的合法权益，禁止虐待、遗弃未成年人，禁止对未成年人实施家庭暴力",
        ]
    }
}

impl Rule for SocialLawDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("social_law_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "社会法深度规则",
            &[
                ("劳动法深度规则", &self.labor_law_detailed()),
                ("社会保险深度规则", &self.social_insurance_detailed()),
                (
                    "特殊群体保护深度规则",
                    &self.special_group_protection_detailed(),
                ),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_law_deep_rules() {
        let rules = SocialLawDeepRules::new();
        assert_eq!(rules.metadata().name, "社会法深度规则");
        assert!(!rules.labor_law_detailed().is_empty());
        assert!(!rules.social_insurance_detailed().is_empty());
        assert!(!rules.special_group_protection_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_labor_law_count() {
        let rules = SocialLawDeepRules::new();
        assert_eq!(rules.labor_law_detailed().len(), 10);
    }

    #[test]
    fn test_social_insurance_count() {
        let rules = SocialLawDeepRules::new();
        assert_eq!(rules.social_insurance_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = SocialLawDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("social_law_deep"));
    }
}
