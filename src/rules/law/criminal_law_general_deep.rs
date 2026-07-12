//! 刑法总则深度规则
//!
//! 涵盖刑法总则的详细内容，包括：
//! - 犯罪构成要件详解
//! - 刑罚制度详解
//! - 刑事责任详解
//! - 刑罚裁量详解
//! - 刑罚执行详解
//! - 时效制度详解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CriminalLawGeneralDeepRules,
    name: "刑法总则深度规则",
    desc: "刑法总则的详细规则解析",
    origin: "中国",
    tags: ["法律", "刑法", "总则"]
}

impl CriminalLawGeneralDeepRules {
    /// 犯罪构成要件详解
    pub fn crime_elements_detailed(&self) -> Vec<&'static str> {
        vec![
            "犯罪客体: 犯罪行为所侵害的为刑法所保护的社会关系，包括国家安全、公共安全、人身权利、财产权利等",
            "犯罪客观方面: 危害行为（作为或不作为）、危害结果、因果关系、犯罪的时间地点方法等",
            "犯罪主体: 实施犯罪行为的自然人或单位，自然人须达到刑事责任年龄、具有刑事责任能力",
            "犯罪主观方面: 故意（直接故意、间接故意）或过失（疏忽大意过失、过于自信过失）",
            "作为: 行为人积极实施刑法禁止的行为，如杀人、抢劫等",
            "不作为: 行为人负有实施某种行为的特定义务，能够履行而不履行，如遗弃罪",
            "因果关系: 危害行为与危害结果之间存在引起与被引起的客观联系",
            "犯罪故意: 明知自己的行为会发生危害社会的结果，并且希望或放任这种结果发生",
            "犯罪过失: 应当预见自己的行为可能发生危害社会的结果，因为疏忽大意而没有预见，或已经预见而轻信能够避免",
            "意外事件: 行为在客观上虽然造成了损害结果，但不是出于故意或过失，而是由于不能抗拒或不能预见的原因所引起，不构成犯罪",
        ]
    }

    /// 刑罚制度详解
    pub fn punishment_system_detailed(&self) -> Vec<&'static str> {
        vec![
            "管制: 对罪犯不予关押，但限制其一定自由，由社区矫正机构执行，期限为3个月至2年，数罪并罚最高3年",
            "拘役: 短期剥夺罪犯自由，就近强制劳动改造，期限为1个月至6个月，数罪并罚最高1年",
            "有期徒刑: 剥夺罪犯一定期限的自由，强制劳动改造，期限为6个月至15年，数罪并罚最高25年",
            "无期徒刑: 终身剥夺罪犯自由，强制劳动改造，适用于罪行极其严重的犯罪分子",
            "死刑: 剥夺罪犯生命，只适用于罪行极其严重的犯罪分子，犯罪时不满18周岁的人和审判时怀孕的妇女不适用死刑",
            "罚金: 强制罪犯向国家缴纳一定数额的金钱，可单处或附加适用，数额根据犯罪情节确定",
            "剥夺政治权利: 剥夺罪犯参加国家管理和政治活动的权利，包括选举权、被选举权、言论出版自由等",
            "没收财产: 将罪犯个人所有财产的一部分或全部强制无偿收归国有的刑罚",
            "驱逐出境: 强迫犯罪的外国人离开中国国境，可独立适用或附加适用",
            "剥夺政治权利期限: 独立适用为1年至5年；附加适用为剥夺政治权利的期限与管制、拘役、有期徒刑的期限相同",
        ]
    }

    /// 刑事责任详解
    pub fn criminal_responsibility_detailed(&self) -> Vec<&'static str> {
        vec![
            "完全不负刑事责任年龄: 不满12周岁的人实施的任何行为都不负刑事责任",
            "相对负刑事责任年龄（12-14周岁）: 犯故意杀人、故意伤害致人死亡或以特别残忍手段致人重伤造成严重残疾，经最高人民检察院核准追诉的，应负刑事责任",
            "相对负刑事责任年龄（14-16周岁）: 犯故意杀人、故意伤害致人重伤或死亡、强奸、抢劫、贩卖毒品、放火、爆炸、投放危险物质罪的，应负刑事责任",
            "完全负刑事责任年龄: 已满16周岁的人犯罪，应当负刑事责任",
            "未成年人犯罪处罚原则: 已满14周岁不满18周岁的人犯罪，应当从轻或减轻处罚",
            "醉酒的人犯罪: 醉酒的人犯罪应当负刑事责任，生理性醉酒不免责",
            "精神病人的刑事责任: 精神病人在不能辨认或不能控制自己行为时造成危害结果，经法定程序鉴定确认的，不负刑事责任",
            "间歇性精神病: 间歇性的精神病人在精神正常的时候犯罪，应当负刑事责任",
            "又聋又哑的人或盲人犯罪: 可以从轻、减轻或免除处罚",
            "单位犯罪: 公司、企业、事业单位、机关、团体实施的危害社会的行为，法律规定为单位犯罪的，应当负刑事责任",
        ]
    }

    /// 刑罚裁量详解
    pub fn punishment_measurement_detailed(&self) -> Vec<&'static str> {
        vec![
            "量刑原则: 以犯罪事实为根据，以刑法为准绳，综合考虑犯罪的事实、性质、情节和社会危害程度",
            "从重处罚: 在法定刑幅度内选择较重的刑种或较长的刑期，适用于累犯、教唆未成年人犯罪等情形",
            "从轻处罚: 在法定刑幅度内选择较轻的刑种或较短的刑期，适用于未成年人犯罪、自首等情形",
            "减轻处罚: 在法定刑以下判处刑罚，适用于有重大立功表现、犯罪情节较轻等情形",
            "免除处罚: 对犯罪分子作有罪宣告但免除其刑罚处罚，适用于犯罪情节轻微不需要判处刑罚的情形",
            "累犯: 被判处有期徒刑以上刑罚的犯罪分子，刑罚执行完毕或赦免以后，在5年内再犯应当判处有期徒刑以上刑罚之罪的",
            "特别累犯: 危害国家安全犯罪、恐怖活动犯罪、黑社会性质的组织犯罪的犯罪分子，在任何时候再犯上述任一类罪的",
            "自首: 犯罪以后自动投案，如实供述自己的罪行的，可以从轻或减轻处罚",
            "立功: 犯罪分子有揭发他人犯罪行为，查证属实的，或提供重要线索，从而得以侦破其他案件等立功表现的，可以从轻或减轻处罚",
            "数罪并罚: 判决宣告以前一人犯数罪的，除判处死刑和无期徒刑的以外，应当在总和刑期以下、数刑中最高刑期以上酌情决定执行的刑期",
        ]
    }

    /// 刑罚执行详解
    pub fn punishment_execution_detailed(&self) -> Vec<&'static str> {
        vec![
            "缓刑适用条件: 被判处拘役、3年以下有期徒刑的犯罪分子，同时符合犯罪情节较轻、有悔罪表现、没有再犯罪的危险、宣告缓刑对所居住社区没有重大不良影响的",
            "缓刑考验期限: 拘役的缓刑考验期限为原判刑期以上1年以下，但不能少于2个月；有期徒刑的缓刑考验期限为原判刑期以上5年以下，但不能少于1年",
            "缓刑撤销: 被宣告缓刑的犯罪分子在缓刑考验期限内犯新罪或发现判决宣告以前还有其他罪没有判决的，应当撤销缓刑",
            "减刑条件: 被判处管制、拘役、有期徒刑、无期徒刑的犯罪分子，在执行期间确有悔改表现或立功表现的，可以减刑",
            "减刑限度: 减刑以后实际执行的刑期，判处管制、拘役、有期徒刑的，不能少于原判刑期的二分之一；判处无期徒刑的，不能少于13年",
            "假释条件: 被判处有期徒刑的犯罪分子执行原判刑期二分之一以上，被判处无期徒刑的犯罪分子实际执行13年以上，确有悔改表现，没有再犯罪的危险的",
            "假释撤销: 被假释的犯罪分子在假释考验期限内犯新罪或发现漏罪，或违反假释监督管理规定的，应当撤销假释",
            "减刑假释程序: 减刑、假释由执行机关向中级以上人民法院提出减刑、假释建议书，人民法院应当组成合议庭进行审理",
            "社区矫正: 对判处管制、宣告缓刑、假释的犯罪分子依法实行社区矫正，由社区矫正机构负责执行",
            "剥夺政治权利执行: 剥夺政治权利的刑期从徒刑、拘役执行完毕之日或从假释之日起计算，剥夺政治权利的效力当然施用于主刑执行期间",
        ]
    }

    /// 时效制度详解
    pub fn limitation_period_detailed(&self) -> Vec<&'static str> {
        vec![
            "追诉时效期限: 法定最高刑为不满5年有期徒刑的，经过5年；法定最高刑为5年以上不满10年有期徒刑的，经过10年",
            "追诉时效期限（续）: 法定最高刑为10年以上有期徒刑的，经过15年；法定最高刑为无期徒刑、死刑的，经过20年",
            "追诉时效延长: 在人民检察院、公安机关、国家安全机关立案侦查或人民法院受理案件以后，逃避侦查或审判的，不受追诉期限的限制",
            "追诉时效中断: 在追诉期限以内又犯罪的，前罪追诉的期限从犯后罪之日起计算",
            "追诉期限计算: 追诉期限从犯罪之日起计算；犯罪行为有连续或继续状态的，从犯罪行为终了之日起计算",
            "最高刑不满5年: 包括判处拘役、管制、3年以下有期徒刑等刑罚的犯罪",
            "最高刑5年以上不满10年: 包括判处5年至9年有期徒刑刑罚的犯罪",
            "最高刑10年以上有期徒刑: 包括判处10年至15年有期徒刑刑罚的犯罪",
            "最高刑无期徒刑死刑: 包括判处无期徒刑、死刑刑罚的犯罪，如故意杀人罪、抢劫罪等严重犯罪",
            "核准追诉: 如果20年以后认为必须追诉的，须报请最高人民检察院核准",
        ]
    }
}

impl Rule for CriminalLawGeneralDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal_law_general_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刑法总则深度规则",
            &[
                ("犯罪构成要件详解", &self.crime_elements_detailed()),
                ("刑罚制度详解", &self.punishment_system_detailed()),
                ("刑事责任详解", &self.criminal_responsibility_detailed()),
                ("刑罚裁量详解", &self.punishment_measurement_detailed()),
                ("刑罚执行详解", &self.punishment_execution_detailed()),
                ("时效制度详解", &self.limitation_period_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminal_law_general_deep_rules() {
        let rules = CriminalLawGeneralDeepRules::new();
        assert_eq!(rules.metadata().name, "刑法总则深度规则");
        assert!(!rules.crime_elements_detailed().is_empty());
        assert!(!rules.punishment_system_detailed().is_empty());
        assert!(!rules.criminal_responsibility_detailed().is_empty());
        assert!(!rules.punishment_measurement_detailed().is_empty());
        assert!(!rules.punishment_execution_detailed().is_empty());
        assert!(!rules.limitation_period_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_crime_elements_count() {
        let rules = CriminalLawGeneralDeepRules::new();
        assert_eq!(rules.crime_elements_detailed().len(), 10);
    }

    #[test]
    fn test_punishment_system_count() {
        let rules = CriminalLawGeneralDeepRules::new();
        assert_eq!(rules.punishment_system_detailed().len(), 10);
    }

    #[test]
    fn test_criminal_responsibility_count() {
        let rules = CriminalLawGeneralDeepRules::new();
        assert_eq!(rules.criminal_responsibility_detailed().len(), 10);
    }
}