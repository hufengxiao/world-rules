//! 社会法深度规则
//!
//! 涵盖社会法核心领域的详细内容，包括：
//! - 劳动法深度规则
//! - 社会保险深度规则
//! - 特殊群体保护深度规则
//! - 劳动争议处理规则
//! - 劳动监察规则
//! - 职业安全健康规则

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

    /// 劳动争议处理规则
    pub fn labor_dispute_resolution(&self) -> Vec<&'static str> {
        vec![
            "协商解决: 发生劳动争议，劳动者可以与用人单位协商，也可以请工会或者第三方共同与用人单位协商，达成和解协议",
            "调解程序: 当事人可以向劳动争议调解组织申请调解，调解组织应当充分听取双方当事人对事实和理由的陈述，耐心疏导，帮助其达成协议",
            "调解期限: 调解组织收到调解申请后应当在十五日内提出调解方案，当事人对调解方案无异议的，应当在七日内签订调解协议",
            "仲裁申请: 发生劳动争议，当事人不愿协商、协商不成或者达成和解协议后不履行的，可以向劳动争议仲裁委员会申请仲裁",
            "仲裁时效: 劳动争议申请仲裁的时效期间为一年，仲裁时效期间从当事人知道或者应当知道其权利被侵害之日起计算",
            "仲裁期限: 劳动争议仲裁委员会收到仲裁申请之日起五日内决定是否受理，仲裁庭应当自劳动争议仲裁委员会受理仲裁申请之日起四十五日内结案",
            "诉讼程序: 当事人对仲裁裁决不服的，可以自收到仲裁裁决书之日起十五日内向人民法院提起诉讼",
            "举证责任: 在劳动争议纠纷案件中，因用人单位作出的开除、除名、辞退、解除劳动合同、减少劳动报酬、计算劳动者工作年限等决定而发生的劳动争议，用人单位负举证责任",
            "先予执行: 仲裁庭对追索劳动报酬、工伤医疗费、经济补偿或者赔偿金的案件，根据当事人的申请，可以裁决先予执行，移送人民法院执行",
            "终局裁决: 追索劳动报酬、工伤医疗费、经济补偿或者赔偿金不超过当地月最低工资标准十二个月金额的争议，仲裁裁决为终局裁决",
        ]
    }

    /// 劳动监察规则
    pub fn labor_inspection_rules(&self) -> Vec<&'static str> {
        vec![
            "监察主体: 县级以上劳动行政部门负责对用人单位遵守劳动法律、法规的情况进行监督检查",
            "监察权力: 劳动行政部门履行监督检查职责，有权采取实地检查、书面检查、举报投诉等方式",
            "监察内容: 用人单位制定内部劳动保障规章制度的情况；用人单位与劳动者订立劳动合同的情况；用人单位遵守禁止使用童工规定的情况",
            "监察程序: 劳动保障监察员进行调查、检查，不得少于两人，并应当出示执法证件",
            "举报受理: 任何组织或者个人对违反劳动保障法律、法规或者规章的行为，有权向劳动行政部门举报",
            "调查期限: 劳动行政部门对违反劳动保障法律、法规或者规章的行为的调查，应当自立案之日起六十个工作日内完成",
            "处理决定: 劳动行政部门对事实清楚、证据确凿的违法行为，可以当场予以纠正，或者责令限期改正",
            "处罚权限: 劳动行政部门有权对违反劳动法律、法规的用人单位给予警告、罚款、吊销许可证等行政处罚",
            "整改期限: 用人单位违反劳动法律、法规，由劳动行政部门责令限期改正，逾期不改正的，给予行政处罚",
            "权益保护: 劳动者认为劳动行政部门未依法履行劳动保障监察职责的，有权向上级行政机关或者监察机关举报",
        ]
    }

    /// 职业安全健康规则
    pub fn occupational_safety_health(&self) -> Vec<&'static str> {
        vec![
            "安全制度: 用人单位必须建立、健全劳动安全卫生制度，严格执行国家劳动安全卫生规程和标准",
            "安全设施: 劳动安全卫生设施必须符合国家规定的标准，新建、改建、扩建工程的劳动安全卫生设施必须与主体工程同时设计、同时施工、同时投入生产和使用",
            "安全培训: 用人单位必须为劳动者提供符合国家规定的劳动安全卫生条件和必要的劳动防护用品，对从事有职业危害作业的劳动者应当定期进行健康检查",
            "特种作业: 从事特种作业的劳动者必须经过专门培训并取得特种作业资格",
            "事故报告: 发生重大伤亡事故时，用人单位必须立即采取抢救措施并按规定向有关部门报告",
            "职业防护: 用人单位必须建立职业病危害申报制度，对从事接触职业病危害作业的劳动者，应当按照规定组织上岗前、在岗期间和离岗时的职业健康检查",
            "危险告知: 用人单位与劳动者订立劳动合同时，应当将工作过程中可能产生的职业病危害及其后果、职业病防护措施和待遇等如实告知劳动者",
            "女工保护: 禁止安排女职工从事矿山井下、国家规定的第四级体力劳动强度的劳动和其他禁忌从事的劳动",
            "未成年保护: 禁止安排未成年工从事矿山井下、有毒有害、国家规定的第四级体力劳动强度的劳动和其他禁忌从事的劳动",
            "工伤保险: 职工因工作遭受事故伤害或者患职业病进行治疗，享受工伤医疗待遇",
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
                ("劳动争议处理规则", &self.labor_dispute_resolution()),
                ("劳动监察规则", &self.labor_inspection_rules()),
                ("职业安全健康规则", &self.occupational_safety_health()),
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
        assert!(!rules.labor_dispute_resolution().is_empty());
        assert!(!rules.labor_inspection_rules().is_empty());
        assert!(!rules.occupational_safety_health().is_empty());
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
    fn test_labor_dispute_count() {
        let rules = SocialLawDeepRules::new();
        assert_eq!(rules.labor_dispute_resolution().len(), 10);
    }

    #[test]
    fn test_labor_inspection_count() {
        let rules = SocialLawDeepRules::new();
        assert_eq!(rules.labor_inspection_rules().len(), 10);
    }

    #[test]
    fn test_occupational_safety_count() {
        let rules = SocialLawDeepRules::new();
        assert_eq!(rules.occupational_safety_health().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = SocialLawDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("social_law_deep"));
    }
}