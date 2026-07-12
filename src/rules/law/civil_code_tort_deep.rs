//! 民法典侵权责任编深度规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CivilCodeTortDeepRules,
    name: "民法典侵权责任编深度规则",
    desc: "民法典侵权责任编的详细规则解析",
    origin: "中国",
    tags: ["法律", "民法", "民法典", "侵权责任"]
}

impl CivilCodeTortDeepRules {
    pub fn tort_general_detailed(&self) -> Vec<&'static str> {
        vec![
            "侵权责任: 行为人因过错侵害他人民事权益造成损害的应当承担侵权责任",
            "过错责任: 行为人因过错侵害他人民事权益造成损害的应当承担侵权责任",
            "无过错责任: 法律规定无过错责任的无论行为人有无过错都应当承担侵权责任",
            "过错推定: 根据法律规定推定行为人有过错行为人不能证明自己没有过错的应当承担侵权责任",
            "损害后果: 侵权行为造成损害的应当承担侵权责任",
            "因果关系: 损害与行为之间存在因果关系的应当承担侵权责任",
            "共同侵权: 二人以上共同实施侵权行为造成他人损害的应当承担连带责任",
            "分别侵权: 二人以上分别实施侵权行为造成同一损害能够确定责任大小的各自承担相应的责任",
            "教唆帮助侵权: 教唆帮助他人实施侵权行为的应当与行为人承担连带责任",
            "数人侵权责任: 二人以上实施危及他人人身财产安全的行为造成损害的应当承担侵权责任",
        ]
    }

    pub fn special_tort_rules_detailed(&self) -> Vec<&'static str> {
        vec![
            "监护人责任: 无民事行为能力人限制民事行为能力人造成他人损害的由监护人承担侵权责任",
            "用人单位责任: 用人单位的工作人员因执行工作任务造成他人损害的由用人单位承担侵权责任",
            "劳务派遣责任: 劳务派遣期间被派遣的工作人员因执行工作任务造成他人损害的由劳务派遣单位承担侵权责任",
            "个人劳务责任: 个人之间形成劳务关系提供劳务一方因劳务造成他人损害的由接受劳务一方承担侵权责任",
            "网络侵权责任: 网络用户网络服务提供者利用网络侵害他人民事权益的应当承担侵权责任",
            "网络服务避风港: 网络服务提供者接到通知后未及时采取必要措施的应当承担侵权责任",
            "产品责任: 因产品存在缺陷造成他人损害的被侵权人可以向产品的生产者请求赔偿也可以向产品的销售者请求赔偿",
            "机动车事故责任: 机动车发生交通事故造成损害的依照道路交通安全法的规定承担赔偿责任",
            "医疗损害责任: 医务人员在诊疗活动中未尽到与当时的医疗水平相应的诊疗义务造成患者损害的医疗机构应当承担赔偿责任",
            "环境污染责任: 因污染环境破坏生态造成他人损害的侵权人应当承担侵权责任",
        ]
    }

    pub fn liability_methods_detailed(&self) -> Vec<&'static str> {
        vec![
            "停止侵害: 侵权行为危及他人人身财产安全的被侵权人有权请求侵权人承担停止侵害等侵权责任",
            "排除妨碍: 侵权行为妨碍他人行使权利的被侵权人有权请求排除妨碍",
            "消除危险: 侵权行为造成危险的被侵权人有权请求消除危险",
            "返还财产: 侵占他人财产的被侵权人有权请求返还财产",
            "恢复原状: 损坏他人财产的被侵权人有权请求恢复原状",
            "修理重作更换: 损坏他人财产不能恢复原状的可以请求修理重作更换",
            "赔偿损失: 侵害他人财产造成损失的应当赔偿损失",
            "精神损害赔偿: 侵害自然人人身权益造成严重精神损害的受害人有权请求精神损害赔偿",
            "消除影响恢复名誉: 侵害他人名誉权的应当消除影响恢复名誉",
            "赔礼道歉: 侵害他人名誉权荣誉权等的应当赔礼道歉",
        ]
    }

    pub fn damage_calculation_detailed(&self) -> Vec<&'static str> {
        vec![
            "财产损失计算: 侵害他人财产的财产损失按照损失发生时的市场价格或者其他合理方式计算",
            "人身损害赔偿: 侵害他人造成人身损害的应当赔偿医疗费护理费交通费营养费等为治疗和康复支出的合理费用",
            "误工损失: 因误工减少的收入应当予以赔偿",
            "残疾赔偿: 造成残疾的应当赔偿辅助器具费和残疾赔偿金",
            "死亡赔偿: 造成死亡的应当赔偿丧葬费和死亡赔偿金",
            "精神损害标准: 精神损害赔偿的数额根据侵权人的过错程度侵害的手段场合行为方式等具体情节确定",
            "同命同价: 因同一侵权行为造成多人死亡的可以以相同数额确定死亡赔偿金",
            "损益相抵: 侵权行为发生后被侵权人获得利益的在计算损害赔偿额时应当扣除获得的利益",
            "过失相抵: 被侵权人对同一损害的发生或者扩大有过错的可以减轻侵权人的责任",
            "惩罚性赔偿: 故意侵害他人知识产权情节严重的被侵权人有权请求惩罚性赔偿",
        ]
    }

    pub fn product_liability_detailed(&self) -> Vec<&'static str> {
        vec![
            "产品缺陷: 产品存在缺陷是指产品存在危及人身财产安全的不合理危险",
            "缺陷认定标准: 产品有保障人体健康和人身财产安全的国家标准行业标准的不符合标准即为缺陷",
            "生产者责任: 因产品存在缺陷造成他人损害的生产者应当承担侵权责任",
            "销售者责任: 因销售者的过错使产品存在缺陷造成他人损害的销售者应当承担侵权责任",
            "销售者追偿: 销售者不能指明缺陷产品的生产者也不能指明缺陷产品的供货人的销售者应当承担侵权责任",
            "第三人过错: 因运输者仓储者等第三人的过错使产品存在缺陷造成他人损害的生产者销售者赔偿后有权向第三人追偿",
            "产品召回: 产品投入流通后发现存在缺陷的应当及时采取停止销售警示召回等补救措施",
            "召回费用: 生产者销售者依照规定采取召回措施的应当承担被侵权人因此支出的必要费用",
            "请求权时效: 因产品缺陷造成损害要求赔偿的诉讼时效期间为三年",
            "最长保护期: 因产品存在缺陷造成损害要求赔偿的请求权在造成损害的缺陷产品交付最初消费者十年丧失",
        ]
    }

    pub fn medical_liability_detailed(&self) -> Vec<&'static str> {
        vec![
            "医疗过错认定: 医务人员在诊疗活动中未尽到与当时的医疗水平相应的诊疗义务造成患者损害的应当承担赔偿责任",
            "说明义务: 医务人员在诊疗活动中应当向患者说明病情和医疗措施",
            "知情同意: 需要实施手术特殊检查特殊治疗的医务人员应当及时向患者具体说明医疗风险替代医疗方案等情况",
            "紧急情况: 因抢救生命垂危的患者等紧急情况不能取得患者或者其近亲属意见的经医疗机构负责人批准可以立即实施",
            "过错推定情形: 违反法律行政法规规章以及其他有关诊疗规范的规定;隐匿或者拒绝提供与纠纷有关的病历资料",
            "病历资料: 医疗机构及其医务人员应当按照规定填写并妥善保管住院志医嘱单检验报告等病历资料",
            "病历查阅: 患者要求查阅复制病历资料的医疗机构应当及时提供",
            "药品器械缺陷: 因药品消毒产品医疗器械的缺陷造成患者损害的患者可以向药品上市许可持持人生产者请求赔偿",
            "医疗机构追偿: 医疗机构赔偿后有权向负有责任的药品上市许可持有人生产者追偿",
            "医疗损害鉴定: 医疗损害责任纠纷需要进行医疗损害鉴定的由双方共同委托鉴定机构进行鉴定",
        ]
    }

    pub fn environmental_tort_detailed(&self) -> Vec<&'static str> {
        vec![
            "无过错责任: 因污染环境破坏生态造成他人损害的侵权人应当承担侵权责任",
            "举证责任倒置: 因污染环境发生纠纷污染者应当就法律规定的不承担责任或者减轻责任的情形承担举证责任",
            "因果关系推定: 污染者应当就其行为与损害之间不存在因果关系承担举证责任",
            "第三方责任: 因第三人的过错污染环境造成损害的被侵权人可以向侵权人请求赔偿也可以向第三人请求赔偿",
            "生态损害修复: 违反国家规定造成生态环境损害能够修复的应当承担修复责任",
            "修复费用: 生态环境损害无法修复的应当承担赔偿生态环境损害修复费用",
            "惩罚性赔偿: 侵权人违反法律规定故意污染环境破坏生态造成严重后果的被侵权人有权请求惩罚性赔偿",
            "公益诉讼: 对污染环境破坏生态损害社会公共利益的行为检察机关可以提起公益诉讼",
            "生态环境损害赔偿: 省级人民政府可以提起生态环境损害赔偿诉讼",
            "损害赔偿范围: 生态环境损害赔偿的范围包括清除污染费用生态环境修复费用生态环境服务功能损失等",
        ]
    }
}

impl Rule for CivilCodeTortDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_tort_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典侵权责任编深度规则",
            &[
                ("侵权责任一般规则", &self.tort_general_detailed()),
                ("特殊侵权规则", &self.special_tort_rules_detailed()),
                ("责任承担方式", &self.liability_methods_detailed()),
                ("损害赔偿计算", &self.damage_calculation_detailed()),
                ("产品责任规则", &self.product_liability_detailed()),
                ("医疗损害责任", &self.medical_liability_detailed()),
                ("环境侵权责任", &self.environmental_tort_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_code_tort_deep_rules() {
        let rules = CivilCodeTortDeepRules::new();
        assert_eq!(rules.metadata().name, "民法典侵权责任编深度规则");
        assert!(!rules.tort_general_detailed().is_empty());
        assert!(!rules.special_tort_rules_detailed().is_empty());
        assert!(!rules.liability_methods_detailed().is_empty());
        assert!(!rules.damage_calculation_detailed().is_empty());
        assert!(!rules.product_liability_detailed().is_empty());
        assert!(!rules.medical_liability_detailed().is_empty());
        assert!(!rules.environmental_tort_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_tort_general_count() {
        let rules = CivilCodeTortDeepRules::new();
        assert_eq!(rules.tort_general_detailed().len(), 10);
    }

    #[test]
    fn test_special_tort_count() {
        let rules = CivilCodeTortDeepRules::new();
        assert_eq!(rules.special_tort_rules_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = CivilCodeTortDeepRules::new();
        assert_eq!(rules.category(), RuleCategory::law("civil_code_tort_deep"));
    }
}
