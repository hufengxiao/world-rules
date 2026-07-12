//! 药理学定律 - 研究药物与机体相互作用及作用规律
//!
//! 药理学连接药学与医学，研究药物的作用机制和临床应用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PharmacologyRules,
    name: "药理学定律",
    desc: "药物与机体相互作用的基本规律",
    origin: "医学",
    tags: ["科学", "医学", "药理学"]
}

impl PharmacologyRules {
    /// 药物代谢动力学
    pub fn pharmacokinetics(&self) -> Vec<&'static str> {
        vec![
            "吸收: 药物从给药部位进入血液循环的过程",
            "分布: 药物从血液向组织器官转运的过程",
            "代谢: 药物在体内发生化学结构变化的过程",
            "排泄: 药物及其代谢物从体内排出的过程",
            "首过效应: 口服药物经肝代谢导致生物利用度降低",
            "半衰期: 血药浓度下降一半所需时间",
            "清除率: 单位时间内机体清除药物的血浆容积",
            "表观分布容积: 药物在体内分布的理论容积",
            "生物利用度: 药物进入血液循环的相对量",
            "稳态血药浓度: 连续给药后血药浓度达稳定水平",
        ]
    }

    /// 药物效应动力学
    pub fn pharmacodynamics(&self) -> Vec<&'static str> {
        vec![
            "受体: 能识别和结合特定配体的生物大分子",
            "激动剂: 与受体结合产生生物效应的药物",
            "拮抗剂: 与受体结合不产生效应但阻断激动剂作用",
            "部分激动剂: 与受体结合产生较弱效应的药物",
            "反向激动剂: 与受体结合产生与激动剂相反效应",
            "效能: 药物产生最大效应的能力",
            "效价: 药物达到一定效应所需的剂量",
            "量效关系: 药物剂量与效应之间的关系",
            "治疗指数: LD50/ED50，药物安全性的指标",
            "治疗窗: 药物有效且安全的血药浓度范围",
        ]
    }

    /// 药物相互作用
    pub fn drug_interactions(&self) -> Vec<&'static str> {
        vec![
            "协同作用: 两药合用效应大于单用之和",
            "相加作用: 两药合用效应等于单用之和",
            "拮抗作用: 一药减弱另一药的效应",
            "药动学相互作用: 影响吸收、分布、代谢、排泄",
            "药效学相互作用: 影响药物与受体结合或效应",
            "酶诱导: 药物诱导肝药酶活性加速其他药物代谢",
            "酶抑制: 药物抑制肝药酶活性减慢其他药物代谢",
            "血浆蛋白置换: 药物竞争血浆蛋白结合位点",
            "影响吸收: 改变胃肠道pH或蠕动影响药物吸收",
            "影响排泄: 改变尿液pH影响药物排泄",
        ]
    }

    /// 药物不良反应
    pub fn adverse_reactions(&self) -> Vec<&'static str> {
        vec![
            "副作用: 治疗剂量下出现的与治疗目的无关的作用",
            "毒性反应: 药物剂量过大或蓄积引起的严重反应",
            "后遗效应: 停药后血药浓度已低但仍残存的效应",
            "变态反应: 免疫反应引起的过敏反应",
            "特异质反应: 遗传异常导致的不良反应",
            "继发反应: 药物治疗作用引起的不良后果",
            "停药反应: 长期用药突然停药引起的反应",
            "依赖性: 长期用药后产生的心理或生理依赖",
            "三致作用: 致畸、致癌、致突变作用",
            "药物耐受性: 连续用药后机体对药物反应降低",
        ]
    }

    /// 影响药物作用的因素
    pub fn influencing_factors(&self) -> Vec<&'static str> {
        vec![
            "年龄: 儿童和老人对药物敏感性不同",
            "性别: 性别差异影响药物代谢和效应",
            "体重: 体重影响药物分布容积和剂量",
            "遗传: 遗传因素影响药物代谢酶活性",
            "病理状态: 肝肾功能影响药物代谢排泄",
            "心理因素: 精神状态影响药物疗效",
            "给药途径: 不同途径影响药物吸收和效应",
            "给药时间: 生物节律影响药物敏感性",
            "饮食: 食物影响药物吸收和代谢",
            "吸烟饮酒: 影响肝药酶活性改变药物代谢",
        ]
    }

    /// 抗感染药物
    pub fn antiinfective_drugs(&self) -> Vec<&'static str> {
        vec![
            "青霉素类: 抑制细菌细胞壁合成，β-内酰胺类",
            "头孢菌素类: 广谱抗菌，分代发展",
            "氨基糖苷类: 抑制蛋白质合成，有耳肾毒性",
            "大环内酯类: 抑制蛋白质合成，大环内酯结构",
            "喹诺酮类: 抑制DNA旋转酶，广谱抗菌",
            "四环素类: 抑制蛋白质合成，影响骨骼牙齿",
            "磺胺类: 抑制叶酸合成，首用磺胺药",
            "抗真菌药: 干扰真菌细胞膜或细胞壁合成",
            "抗病毒药: 抑制病毒复制各环节",
            "抗结核药: 抗结核分枝杆菌，需联合用药",
        ]
    }

    /// 心血管系统药物
    pub fn cardiovascular_drugs(&self) -> Vec<&'static str> {
        vec![
            "抗高血压药: 降低血压，ACEI、ARB、钙拮抗剂等",
            "抗心绞痛药: 扩张冠脉、降低心肌耗氧",
            "抗心律失常药: 纠正心律失常，分类作用",
            "强心苷: 增强心肌收缩力，治疗心衰",
            "抗心衰药: 改善心功能，减轻症状",
            "调血脂药: 降低血脂，他汀类为主",
            "抗血栓药: 抗凝血、抗血小板、溶栓",
            "血管活性药: 收缩或扩张血管调节血压",
            "利尿药: 促进钠水排出，减少血容量",
        ]
    }

    /// 中枢神经系统药物
    pub fn cns_drugs(&self) -> Vec<&'static str> {
        vec![
            "镇静催眠药: 抑制中枢神经，诱导睡眠",
            "抗癫痫药: 控制癫痫发作，调节神经元兴奋性",
            "抗帕金森药: 补充多巴胺或抗胆碱",
            "抗精神病药: 阻断多巴胺受体，治疗精神分裂",
            "抗抑郁药: 增强中枢神经递质功能",
            "抗焦虑药: 减轻焦虑，苯二氮䓬类常用",
            "镇痛药: 作用于中枢缓解疼痛，阿片类",
            "解热镇痛药: 降低体温缓解疼痛，NSAIDs",
            "麻醉药: 引起意识消失和痛觉缺失",
            "中枢兴奋药: 提高中枢神经兴奋性",
        ]
    }

    /// 激素类药物
    pub fn hormonal_drugs(&self) -> Vec<&'static str> {
        vec![
            "糖皮质激素: 抗炎、抗过敏、免疫抑制",
            "盐皮质激素: 调节水盐代谢，醛固酮为主",
            "甲状腺激素: 促进代谢，维持生长发育",
            "胰岛素: 降低血糖，促进糖原合成",
            "口服降糖药: 促进胰岛素分泌或增敏",
            "性激素: 雌激素、孕激素、雄激素",
            "避孕药: 抑制排卵或改变宫颈黏液",
            "同化激素: 促进蛋白质合成，雄激素样作用",
            "抗甲状腺药: 抑制甲状腺激素合成",
        ]
    }
}

impl Rule for PharmacologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("pharmacology")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "药理学定律",
            &[
                ("药物代谢动力学", &self.pharmacokinetics()),
                ("药物效应动力学", &self.pharmacodynamics()),
                ("药物相互作用", &self.drug_interactions()),
                ("药物不良反应", &self.adverse_reactions()),
                ("影响药物作用的因素", &self.influencing_factors()),
                ("抗感染药物", &self.antiinfective_drugs()),
                ("心血管系统药物", &self.cardiovascular_drugs()),
                ("中枢神经系统药物", &self.cns_drugs()),
                ("激素类药物", &self.hormonal_drugs()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pharmacology_rules() {
        let rules = PharmacologyRules::new();
        assert!(!rules.pharmacokinetics().is_empty());
        assert!(!rules.pharmacodynamics().is_empty());
        assert!(!rules.drug_interactions().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_pharmacology_metadata() {
        let rules = PharmacologyRules::new();
        assert_eq!(rules.metadata().name, "药理学定律");
    }
}
