//! 民法典物权编深度规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CivilCodePropertyDeepRules,
    name: "民法典物权编深度规则",
    desc: "民法典物权编的详细规则解析",
    origin: "中国",
    tags: ["法律", "民法", "民法典", "物权"]
}

impl CivilCodePropertyDeepRules {
    pub fn property_principles_detailed(&self) -> Vec<&'static str> {
        vec![
            "物权平等保护: 国家、集体、私人的物权和其他权利人的物权受法律平等保护",
            "物权法定: 物权的种类和内容由法律规定,当事人不得自由创设物权",
            "公示原则: 物权的设立、变更、转让和消灭应当依法公示",
            "不动产登记: 不动产物权的设立、变更、转让和消灭经依法登记发生效力",
            "动产交付: 动产物权的设立和转让自交付时发生效力",
            "物权优先: 同一物上既有物权又有债权时物权优先于债权",
            "一物一权: 一个物上只能设立一个所有权,但可以设立多个担保物权",
            "物尽其用: 物权人应当合理利用物,节约资源保护环境",
        ]
    }

    pub fn ownership_detailed(&self) -> Vec<&'static str> {
        vec![
            "所有权内容: 所有权人对自己的不动产或动产依法享有占有、使用、收益和处分权利",
            "国家所有权: 法律规定属于国家所有的财产属于国家所有即全民所有",
            "集体所有权: 集体所有的不动产和动产依法属于集体成员集体所有",
            "私人所有权: 私人对合法的财产享有所有权,包括收入、房屋、生活用品等",
            "所有权取得: 劳动生产、继承、接受赠与、合法买卖等方式取得所有权",
            "善意取得: 无处分权人将不动产或动产转让给受让人的受让人善意取得该物的所有权",
            "善意取得条件: 受让人受让该物时是善意;以合理的价格转让;转让的不动产已登记或动产已交付",
            "征收征用: 为了公共利益的需要依照法律规定的权限和程序可以征收征用不动产或动产",
            "征收补偿: 征收不动产应当依法给予公平合理的补偿",
            "相邻关系: 不动产的相邻权利人应当按照有利生产、方便生活、团结互助、公平合理的原则处理相邻关系",
        ]
    }

    pub fn usufruct_detailed(&self) -> Vec<&'static str> {
        vec![
            "用益物权概念: 权利人对他人所有的不动产或动产依法享有占有、使用和收益的权利",
            "土地承包经营权: 农民集体所有和国家所有由农民集体使用的土地依法承包经营",
            "土地承包期限: 耕地的承包期为三十年;草地的承包期为三十年至五十年;林地的承包期为三十年至七十年",
            "建设用地使用权: 权利人依法对国家所有的土地享有占有、使用和收益的权利",
            "宅基地使用权: 权利人依法对集体所有的土地享有占有和使用的权利",
            "宅基地限制: 宅基地使用权不得抵押,宅基地的转让需符合法律规定",
            "地役权: 地役权人有权按照合同约定利用他人的不动产以提高自己不动产的效益",
            "地役权设立: 地役权自地役权合同生效时设立,当事人要求登记的应当登记",
            "居住权: 居住权人有权按照合同约定对他人的住宅享有占有、使用的权利以满足生活居住需要",
            "居住权期限: 居住权期限届满或居住权人死亡的居住权消灭",
        ]
    }

    pub fn security_rights_detailed(&self) -> Vec<&'static str> {
        vec![
            "担保物权概念: 担保物权人在债务人不履行到期债务时依法享有就担保财产优先受偿的权利",
            "抵押权: 抵押权人对于债务人或第三人提供抵押的财产依法享有优先受偿的权利",
            "抵押财产: 建筑物和其他土地附着物,建设用地使用权,海域使用权,生产设备原材料半成品产品",
            "禁止抵押: 土地所有权,宅基地自留地自留山等集体所有土地的使用权,学校医院的教育设施医疗卫生设施",
            "抵押登记: 以建筑物和其他土地附着物建设用地使用权海域使用权抵押的应当办理登记抵押权自登记时设立",
            "抵押合同: 设立抵押权当事人应当采用书面形式订立抵押合同",
            "质权: 权分为动产质权和权利质权,质权自出质人交付质押财产时设立",
            "动产质押: 为担保债务的履行债务人或第三人将其动产移交债权人占有的债权人有优先受偿权",
            "权利质押: 可以质押的权利包括汇票支票本票债券存款单仓单提单,可以转让的基金份额股权",
            "留置权: 债权人因合同关系占有债务人的动产债务人不按照合同约定履行债务的债权人有权留置该动产",
        ]
    }

    pub fn possession_detailed(&self) -> Vec<&'static str> {
        vec![
            "占有概念: 占有人对不动产或动产的实际控制",
            "有权占有: 基于法律规定或合同约定而产生的占有",
            "无权占有: 没有法律依据或合同约定的占有",
            "善意占有: 占有人不知其无占有的权利而进行的占有",
            "恶意占有: 占有人明知其无占有的权利而进行的占有",
            "占有保护: 占有的不动产或动产被侵占的占有人有权请求返还原物",
            "占有返还请求权: 占有人返还原物的请求权自侵占发生之日起一年内未行使则消灭",
            "占有损害赔偿: 因侵占或妨害造成损害的占有人有权请求损害赔偿",
            "占有转移: 占有可以因交付而转移",
            "占有辅助: 占有辅助人基于雇主或他人的指示而进行的占有",
        ]
    }

    pub fn property_protection_detailed(&self) -> Vec<&'static str> {
        vec![
            "物权保护方式: 物权受到侵害的权利人可以通过和解调解仲裁诉讼等途径解决",
            "确认物权: 因物权的归属内容发生争议的利害关系人可以请求确认物权",
            "返还原物: 无权占有不动产或动产的物权人可以请求返还原物",
            "排除妨害: 物权受到妨害的物权人有权请求排除妨害或消除危险",
            "恢复原状: 不动产或动产毁损的物权人可以请求恢复原状",
            "损害赔偿: 侵害物权造成权利人损害的权利人可以请求损害赔偿",
            "物权请求权: 物权人行使物权请求权不受诉讼时效限制",
            "权利竞合: 物权受到侵害的同时侵害人构成违约或侵权的物权人可以选择行使物权请求权",
            "修理重作更换: 物权被侵害的物权人可以请求修理重作更换",
            "预防措施: 物权可能受到侵害的物权人可以请求采取预防措施",
        ]
    }

    pub fn registration_detailed(&self) -> Vec<&'static str> {
        vec![
            "登记效力: 不动产物权的设立变更转让和消灭经依法登记发生效力未经登记不发生效力",
            "登记机构: 不动产登记由不动产所在地的登记机构办理",
            "登记申请: 当事人申请不动产登记应当提交权属证明不动产界址面积等必要材料",
            "登记审查: 登记机构应当查验申请材料询问申请人如实及时登记",
            "登记簿: 不动产登记簿是物权归属和内容的根据不动产登记簿由登记机构管理",
            "登记错误: 登记簿记载的事项错误的权利人可以申请更正登记",
            "异议登记: 权利人对登记簿记载事项有异议的可以申请异议登记",
            "预告登记: 当事人签订买卖房屋协议为保障将来实现物权可以申请预告登记",
            "预告登记效力: 预告登记后未经预告登记的权利人同意处分该不动产的不发生物权效力",
            "登记费用: 不动产登记的费用按照国家规定收取",
        ]
    }
}

impl Rule for CivilCodePropertyDeepRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("civil_code_property_deep")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "民法典物权编深度规则",
            &[
                ("物权基本原则", &self.property_principles_detailed()),
                ("所有权规则", &self.ownership_detailed()),
                ("用益物权规则", &self.usufruct_detailed()),
                ("担保物权规则", &self.security_rights_detailed()),
                ("占有规则", &self.possession_detailed()),
                ("物权保护规则", &self.property_protection_detailed()),
                ("不动产登记规则", &self.registration_detailed()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_code_property_deep_rules() {
        let rules = CivilCodePropertyDeepRules::new();
        assert_eq!(rules.metadata().name, "民法典物权编深度规则");
        assert!(!rules.property_principles_detailed().is_empty());
        assert!(!rules.ownership_detailed().is_empty());
        assert!(!rules.usufruct_detailed().is_empty());
        assert!(!rules.security_rights_detailed().is_empty());
        assert!(!rules.possession_detailed().is_empty());
        assert!(!rules.property_protection_detailed().is_empty());
        assert!(!rules.registration_detailed().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_ownership_count() {
        let rules = CivilCodePropertyDeepRules::new();
        assert_eq!(rules.ownership_detailed().len(), 10);
    }

    #[test]
    fn test_usufruct_count() {
        let rules = CivilCodePropertyDeepRules::new();
        assert_eq!(rules.usufruct_detailed().len(), 10);
    }

    #[test]
    fn test_category() {
        let rules = CivilCodePropertyDeepRules::new();
        assert_eq!(
            rules.category(),
            RuleCategory::law("civil_code_property_deep")
        );
    }
}
