//! 民法典婚姻家庭编深度规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CivilCodeMarriageDeepRules,
    name: "民法典婚姻家庭编深度规则",
    desc: "民法典婚姻家庭编的详细规则解析",
    origin: "中国",
    tags: ["法律", "民法", "民法典", "婚姻家庭"]
}

impl CivilCodeMarriageDeepRules {
    pub fn marriage_general_detailed(&self) -> Vec<&'static str> {
        vec![
            "婚姻自由: 结婚应当男女双方完全自愿,禁止任何一方对另一方加以强迫或干涉",
            "结婚年龄: 男不得早于二十二周岁,女不得早于二十周岁",
            "禁止结婚情形: 直系血亲或者三代以内的旁系血亲禁止结婚",
            "无效婚姻: 重婚的;有禁止结婚的亲属关系的;未到法定婚龄的",
            "无效婚姻效力: 无效的婚姻自始没有法律约束力",
            "可撤销婚姻: 因胁迫结婚的,受胁迫的一方可以向人民法院请求撤销婚姻",
            "撤销期限: 受胁迫的一方撤销婚姻的请求应当自结婚登记之日起一年内提出",
            "隐瞒疾病: 一方患有重大疾病的应当在结婚登记前如实告知另一方",
            "隐瞒疾病撤销: 一方隐瞒重大疾病的另一方可以向人民法院请求撤销婚姻",
            "撤销期限疾病: 撤销婚姻的请求应当自知道或者应当知道撤销事由之日起一年内提出",
        ]
    }

    pub fn husband_wife_rights_detailed(&self) -> Vec<&'static str> {
        vec![
            "夫妻地位平等: 夫妻在婚姻家庭中地位平等",
            "姓名权: 夫妻双方都有各使用自己姓名的权利",
            "生产经营: 夫妻双方都有参加生产、工作、学习和社会活动的自由",
            "子女姓氏: 子女可以随父姓也可以随母姓",
            "夫妻扶养: 夫妻有互相扶养的义务",
            "扶养请求: 需要扶养的一方在另一方不履行扶养义务时有权要求其给付扶养费",
            "夫妻财产: 夫妻在婚姻关系存续期间所得的财产为夫妻共同财产",
            "共同财产范围: 工资奖金劳务报酬;生产经营投资的收益;知识产权的收益",
            "夫妻个人财产: 一方的婚前财产;一方因受到人身损害获得的赔偿或者补偿",
            "夫妻财产约定: 夫妻可以约定婚姻关系存续期间所得的财产归各自所有",
        ]
    }

    pub fn divorce_rules_detailed(&self) -> Vec<&'static str> {
        vec![
            "离婚自由: 夫妻双方自愿离婚的应当签订书面离婚协议并亲自到婚姻登记机关申请离婚登记",
            "离婚协议内容: 离婚协议应当载明双方自愿离婚的意思表示和对子女抚养财产债务处理等事项协商一致的意见",
            "离婚冷静期: 自婚姻登记机关收到离婚登记申请之日起三十日内任何一方不愿意离婚的可以撤回离婚登记申请",
            "离婚申请期限: 冷静期届满后三十日内双方未亲自到婚姻登记机关申请发给离婚证的视为撤回离婚登记申请",
            "诉讼离婚: 夫妻一方要求离婚的可以由有关组织进行调解或者直接向人民法院提起离婚诉讼",
            "离婚调解: 人民法院审理离婚案件应当进行调解;如果感情确已破裂调解无效的应当准予离婚",
            "离婚情形: 重婚或者与他人同居;实施家庭暴力或者虐待遗弃家庭成员;有赌博吸毒等恶习屡教不改",
            "分居离婚: 因感情不和分居满二年调解无效的应当准予离婚",
            "离婚判决生效: 离婚判决或者调解书生效即解除婚姻关系",
            "离婚损害赔偿: 因重婚与他人同居实施家庭暴力虐待遗弃家庭成员等原因离婚的无过错方有权请求损害赔偿",
        ]
    }

    pub fn child_parent_rules_detailed(&self) -> Vec<&'static str> {
        vec![
            "父母抚养义务: 父母不履行抚养义务的未成年子女或者不能独立生活的成年子女有要求父母给付抚养费的权利",
            "子女赡养义务: 成年子女不履行赡养义务的缺乏劳动能力或者生活困难的父母有要求成年子女给付赡养费的权利",
            "教育权利: 父母有教育保护未成年子女的权利和义务",
            "子女保护: 未成年子女造成他人损害的父母应当依法承担民事责任",
            "父母婚姻自由: 子女应当尊重父母的婚姻权利不得干涉父母离婚再婚以及婚后的生活",
            "亲子关系确认: 对亲子关系有异议且有正当理由的父或者母可以向人民法院提起诉讼请求确认亲子关系",
            "亲子关系否认: 对亲子关系有异议且有正当理由的成年子女可以向人民法院提起诉讼请求否认亲子关系",
            "离婚后子女抚养: 离婚后不满两周岁的子女以由母亲直接抚养为原则",
            "抚养协议: 离婚后子女由一方直接抚养的另一方应当负担部分或者全部抚养费",
            "抚养费标准: 抄养费的数额根据子女的实际需要父母双方的负担能力和当地的实际生活水平确定",
        ]
    }

    pub fn adoption_rules_detailed(&self) -> Vec<&'static str> {
        vec![
            "收养条件: 收养人应当同时具备下列条件:无子女或者只有一名子女;有抚养教育被收养人的能力",
            "收养年龄: 收养人年满三十周岁",
            "收养限制: 有配偶者收养子女应当夫妻共同收养",
            "收养人数: 收养人可以收养一名子女,收养孤儿残疾未成年人或者查找不到生父母的未成年人的可以不受收养人数限制",
            "被收养人年龄: 被收养人应当是不满十八周岁的未成年人",
            "被收养人条件: 丧失父母的孤儿;查找不到生父母的未成年人;生父母有特殊困难无力抚养的子女",
            "送养条件: 生父母送养子女应当双方共同送养",
            "送养限制: 生父母一方不明或者查找不到的可以单方送养",
            "监护人送养: 监护人送养未成年孤儿或者残疾未成年人的应当征得有抚养义务的人同意",
            "收养登记: 收养应当向县级以上人民政府民政部门登记",
        ]
    }

    pub fn inheritance_general_detailed(&self) -> Vec<&'static str> {
        vec![
            "继承开始: 继承从被继承人死亡时开始",
            "遗产范围: 遗产是自然人死亡时遗留的个人合法财产",
            "继承方式: 继承开始后按照法定继承办理;有遗嘱的按照遗嘱继承或者遗赠办理",
            "遗赠扶养协议: 有遗赠扶养协议的按照协议办理",
            "继承权丧失: 故意杀害被继承人;为争夺遗产而杀害其他继承人",
            "继承权恢复: 继承人有继承权丧失的情形确有悔改表现被继承人表示宽恕或者事后在遗嘱中将其列为继承人的",
            "继承权诉讼: 继承权纠纷提起诉讼的期限为三年,自继承人知道或者应当知道其权利被侵犯之日起计算",
            "遗产保管: 继承开始后继承人应当妥善保管遗产",
            "遗产管理人: 继承开始后继承人应当及时推选遗产管理人",
            "遗产管理人职责: 清理遗产并制作遗产清单;向继承人报告遗产情况;采取必要措施防止遗产毁损灭失",
        ]
    }

    pub fn intestate_succession_detailed(&self) -> Vec<&'static str> {
        vec![
            "法定继承顺序: 第一顺序配偶子女父母;第二顺序兄弟姐妹祖父母外祖父母",
            "第一顺序继承: 继承开始后由第一顺序继承人继承,第二顺序继承人不继承",
            "无第一顺序继承: 没有第一顺序继承人继承的由第二顺序继承人继承",
            "子女继承权: 子女包括婚生子女非婚生子女养子女和有扶养关系的继子女",
            "父母继承权: 父母包括生父母养父母和有扶养关系的继父母",
            "兄弟姐妹继承权: 兄弟姐妹包括同父母的兄弟姐妹同父异母或者同母异父的兄弟姐妹养兄弟姐妹",
            "代位继承: 被继承人的子女先于被继承人死亡的由被继承人的子女的直系晚辈血亲代位继承",
            "代位继承份额: 代位继承人一般只能继承被代位继承人有权继承的遗产份额",
            "丧偶儿媳女婿: 丧偶儿媳对公婆丧偶女婿对岳父母尽了主要赡养义务的作为第一顺序继承人",
            "遗产分配: 同一顺序继承人继承遗产的份额一般应当均等",
        ]
    }
}

impl Rule for CivilCodeMarriageDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_marriage_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典婚姻家庭编深度规则",
            &[
                ("婚姻一般规则", &self.marriage_general_detailed()),
                ("夫妻权利义务", &self.husband_wife_rights_detailed()),
                ("离婚规则", &self.divorce_rules_detailed()),
                ("亲子关系规则", &self.child_parent_rules_detailed()),
                ("收养规则", &self.adoption_rules_detailed()),
                ("继承一般规则", &self.inheritance_general_detailed()),
                ("法定继承规则", &self.intestate_succession_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_code_marriage_deep_rules() {
        let rules = CivilCodeMarriageDeepRules::new();
        assert_eq!(rules.metadata().name, "民法典婚姻家庭编深度规则");
        assert!(!rules.marriage_general_detailed().is_empty());
        assert!(!rules.husband_wife_rights_detailed().is_empty());
        assert!(!rules.divorce_rules_detailed().is_empty());
        assert!(!rules.child_parent_rules_detailed().is_empty());
        assert!(!rules.adoption_rules_detailed().is_empty());
        assert!(!rules.inheritance_general_detailed().is_empty());
        assert!(!rules.intestate_succession_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_marriage_general_count() {
        let rules = CivilCodeMarriageDeepRules::new();
        assert_eq!(rules.marriage_general_detailed().len(), 10);
    }

    #[test]
    fn test_divorce_count() {
        let rules = CivilCodeMarriageDeepRules::new();
        assert_eq!(rules.divorce_rules_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = CivilCodeMarriageDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("civil_code_marriage_deep")
        );
    }
}
