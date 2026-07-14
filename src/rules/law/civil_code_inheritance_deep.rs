//! 民法典继承编深度规则
//!
//! 涵盖民法典继承编的详细内容，包括：
//! - 法定继承详解
//! - 遗嘱继承详解
//! - 遗赠和遗赠扶养协议
//! - 继承程序详解
//! - 遗产处理详解
//! - 继承权保护

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CivilCodeInheritanceDeepRules,
    name: "民法典继承编深度规则",
    desc: "民法典继承编的详细规则解析",
    origin: "中国",
    tags: ["法律", "民法", "继承"]
}

impl CivilCodeInheritanceDeepRules {
    /// 法定继承详解
    pub fn statutory_inheritance_detailed(&self) -> Vec<&'static str> {
        vec![
            "第一顺序继承人: 配偶、子女、父母",
            "第二顺序继承人: 兄弟姐妹、祖父母、外祖父母",
            "继承顺序: 有第一顺序继承人时，第二顺序继承人不继承",
            "子女范围: 婚生子女、非婚生子女、养子女、有扶养关系的继子女",
            "父母范围: 生父母、养父母、有扶养关系的继父母",
            "兄弟姐妹范围: 同父母、同父异母、同母异父、养兄弟姐妹、有扶养关系的继兄弟姐妹",
            "代位继承: 被继承人的子女先于被继承人死亡的，由该子女的直系晚辈血亲代位继承",
            "代位继承限制: 代位继承人一般只能继承被代位继承人有权继承的遗产份额",
            "丧偶儿媳女婿: 丧偶儿媳对公婆、丧偶女婿对岳父母尽了主要赡养义务的，作为第一顺序继承人",
            "继承权男女平等: 继承权男女平等，不因性别不同而有差异",
        ]
    }

    /// 遗嘱继承详解
    pub fn testamentary_inheritance_detailed(&self) -> Vec<&'static str> {
        vec![
            "自书遗嘱: 遗嘱人亲笔书写，签名，注明年、月、日",
            "代书遗嘱: 应当有两个以上见证人在场见证，由其中一人代书，注明年、月、日，并由代书人、其他见证人和遗嘱人签名",
            "打印遗嘱: 应当有两个以上见证人在场见证，遗嘱人和见证人应当在遗嘱每一页签名，注明年、月、日",
            "录音录像遗嘱: 应当有两个以上见证人在场见证，遗嘱人和见证人应当在录音录像中记录其姓名或肖像，以及年、月、日",
            "口头遗嘱: 遗嘱人在危急情况下可以立口头遗嘱，应当有两个以上见证人在场见证",
            "公证遗嘱: 遗嘱人经公证机构办理的遗嘱，具有较强的证明力",
            "遗嘱见证人限制: 无民事行为能力人、限制民事行为能力人、继承人、受遗赠人等不得作为见证人",
            "遗嘱撤回: 遗嘱人可以撤回自己所立的遗嘱，立有数份遗嘱的，以最后的遗嘱为准",
            "遗嘱效力: 遗嘱应当为缺乏劳动能力又没有生活来源的继承人保留必要的遗产份额",
            "遗嘱无效情形: 伪造的遗嘱无效；遗嘱被篡改的，篡改的内容无效",
        ]
    }

    /// 遗赠和遗赠扶养协议详解
    pub fn legacy_detailed(&self) -> Vec<&'static str> {
        vec![
            "遗赠定义: 自然人可以立遗嘱将个人财产赠与国家、集体或法定继承人以外的组织、个人",
            "遗赠效力: 受遗赠人应当在知道受遗赠后60日内作出接受或放弃的表示，到期没有表示的，视为放弃",
            "遗赠扶养协议: 自然人与继承人以外的组织或个人签订遗赠扶养协议",
            "协议内容: 扶养人承担该自然人生养死葬的义务，享有受遗赠的权利",
            "协议效力优先: 遗赠扶养协议的效力优先于遗嘱继承和法定继承",
            "遗赠扶养协议解除: 扶养人无正当理由不履行义务的，不能再享有受遗赠的权利",
            "集体组织扶养: 集体所有制组织可以与无人继承又无人受遗赠的人签订遗赠扶养协议",
            "遗赠财产限制: 遗赠的财产应当是遗嘱人个人的合法财产",
        ]
    }

    /// 继承程序详解
    pub fn inheritance_procedure_detailed(&self) -> Vec<&'static str> {
        vec![
            "继承开始时间: 继承从被继承人死亡时开始",
            "继承地点: 继承开始后，知道被继承人死亡的继承人应当及时通知其他继承人和遗嘱执行人",
            "遗产保管: 继承开始后，遗产保管人应当妥善保管遗产，任何组织或个人不得侵吞或争抢",
            "继承接受: 继承开始后，继承人放弃继承的，应当在遗产处理前以书面形式作出放弃继承的表示",
            "默认接受: 继承人没有表示放弃继承的，视为接受继承",
            "继承权放弃效力: 继承人放弃继承的，对被继承人依法应当缴纳的税款和债务可以不负清偿责任",
            "继承权丧失: 故意杀害被继承人、为争夺遗产而杀害其他继承人等情形丧失继承权",
            "继承权恢复: 继承人丧失继承权后，确有悔改表现且被继承人表示宽恕的，可以恢复继承权",
            "遗产分割: 遗产分割应当有利于生产和生活需要，不损害遗产的效用",
            "遗产分割方法: 不宜分割的遗产可以采取折价、适当补偿或共有等方法处理",
        ]
    }

    /// 遗产处理详解
    pub fn estate_handling_detailed(&self) -> Vec<&'static str> {
        vec![
            "遗产范围: 遗产是自然人死亡时遗留的个人合法财产",
            "遗产认定: 夫妻共同财产中一半为配偶所有，另一半为被继承人的遗产",
            "遗产债务清偿: 继承人应当清偿被继承人依法应当缴纳的税款和债务",
            "清偿限度: 缴纳税款和清偿债务以遗产实际价值为限",
            "债务清偿顺序: 优先清偿破产费用和共益债务后，按照法定顺序清偿其他债务",
            "遗产不足清偿: 遗产不足以清偿债务的，各继承人按比例清偿",
            "放弃继承与债务: 放弃继承的继承人，对被继承人的债务不负清偿责任",
            "无人继承遗产: 无人继承又无人受遗赠的遗产，归国家所有，用于公益事业",
            "遗产评估: 遗产分割时应当对遗产进行评估，确定遗产价值",
            "遗产过户: 继承人继承房产、车辆等需要办理过户登记的财产，应当办理过户手续",
        ]
    }

    /// 继承权保护详解
    pub fn inheritance_protection_detailed(&self) -> Vec<&'static str> {
        vec![
            "继承权保护期限: 继承权纠纷提起诉讼的期限为3年，自继承人知道或应当知道其权利受到侵害之日起计算",
            "最长保护期限: 自继承开始之日起超过20年的，不得再提起诉讼",
            "继承权确认: 继承人可以请求人民法院确认其继承权",
            "遗产分割请求: 继承人可以请求分割遗产，其他继承人不得拒绝",
            "遗产损害赔偿: 侵占遗产的继承人应当返还遗产，造成损失的应当赔偿",
            "遗嘱执行人权利: 酒店执行人有权依照遗嘱的指定执行遗嘱",
            "遗嘱执行人义务: 遗嘱执行人应当忠实地执行遗嘱人的意愿，保护遗产",
            "遗产债权人权利: 遗产债权人有权请求继承人在遗产范围内清偿债务",
            "胎儿继承权保护: 遗产分割时应当保留胎儿的继承份额，胎儿娩出时是死体的保留份额按法定继承处理",
            "未成年人继承权: 未成年人享有的继承份额，由其监护人代为管理，不得损害未成年人的利益",
        ]
    }
}

impl Rule for CivilCodeInheritanceDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_inheritance_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典继承编深度规则",
            &[
                ("法定继承详解", &self.statutory_inheritance_detailed()),
                ("遗嘱继承详解", &self.testamentary_inheritance_detailed()),
                ("遗赠和遗赠扶养协议详解", &self.legacy_detailed()),
                ("继承程序详解", &self.inheritance_procedure_detailed()),
                ("遗产处理详解", &self.estate_handling_detailed()),
                ("继承权保护详解", &self.inheritance_protection_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_code_inheritance_deep_rules() {
        let rules = CivilCodeInheritanceDeepRules::new();
        assert_eq!(rules.metadata().name, "民法典继承编深度规则");
        assert!(!rules.statutory_inheritance_detailed().is_empty());
        assert!(!rules.testamentary_inheritance_detailed().is_empty());
        assert!(!rules.legacy_detailed().is_empty());
        assert!(!rules.inheritance_procedure_detailed().is_empty());
        assert!(!rules.estate_handling_detailed().is_empty());
        assert!(!rules.inheritance_protection_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_statutory_inheritance_count() {
        let rules = CivilCodeInheritanceDeepRules::new();
        assert_eq!(rules.statutory_inheritance_detailed().len(), 10);
    }

    #[test]
    fn test_testamentary_inheritance_count() {
        let rules = CivilCodeInheritanceDeepRules::new();
        assert_eq!(rules.testamentary_inheritance_detailed().len(), 10);
    }

    #[test]
    fn test_legacy_count() {
        let rules = CivilCodeInheritanceDeepRules::new();
        assert_eq!(rules.legacy_detailed().len(), 8);
    }

    #[test]
    fn test_category() {
        let rules = CivilCodeInheritanceDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("civil_code_inheritance_deep")
        );
    }
}
