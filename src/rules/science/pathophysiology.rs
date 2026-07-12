//! 病理生理学定律 - 研究疾病时机体功能代谢变化的规律
//!
//! 病理生理学探讨疾病发生发展的共同规律和机制。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PathophysiologyRules,
    name: "病理生理学定律",
    desc: "疾病时机体功能代谢变化的基本规律",
    origin: "医学",
    tags: ["科学", "医学", "病理生理学"]
}

impl PathophysiologyRules {
    /// 疾病概论
    pub fn disease_general(&self) -> Vec<&'static str> {
        vec![
            "健康: 机体与环境协调状态，不仅是无疾病",
            "疾病: 机体在病因作用下自稳调节紊乱",
            "病理过程: 许多疾病共有的功能代谢变化",
            "病理状态: 发展极慢或相对稳定的局部形态变化",
            "病因学: 研究疾病发生原因和条件的科学",
            "发病学: 研究疾病发生发展规律和机制",
            "因果交替: 原始病因与继发变化互为因果",
            "损伤与抗损伤: 致病因素与机体反应的矛盾",
        ]
    }

    /// 水电解质代谢紊乱
    pub fn water_electrolyte_disorder(&self) -> Vec<&'static str> {
        vec![
            "脱水: 体液容量减少，分为高渗、低渗、等渗性",
            "水中毒: 细胞内液增多为主的低钠血症",
            "水肿: 过多液体在组织间隙或体腔积聚",
            "低钠血症: 血清钠浓度低于135mmol/L",
            "高钠血症: 血清钠浓度高于145mmol/L",
            "低钾血症: 血清钾浓度低于3.5mmol/L",
            "高钾血症: 血清钾浓度高于5.5mmol/L",
            "镁代谢紊乱: 低镁血症或高镁血症",
            "钙磷代谢紊乱: 低钙血症、高钙血症等",
        ]
    }

    /// 酸碱平衡紊乱
    pub fn acid_base_disorder(&self) -> Vec<&'static str> {
        vec![
            "代谢性酸中毒: 原发性HCO3-减少导致pH下降",
            "代谢性碱中毒: 原发性HCO3-增多导致pH升高",
            "呼吸性酸中毒: 原发性PCO2升高导致pH下降",
            "呼吸性碱中毒: 原发性PCO2降低导致pH升高",
            "混合性酸碱中毒: 同时存在两种以上酸碱失衡",
            "代偿机制: 机体调节酸碱平衡的各种方式",
            "缓冲系统: 血液中的化学缓冲对调节pH",
            "肺调节: 通过排出CO2调节血液pH",
            "肾调节: 通过排酸保碱调节血液pH",
        ]
    }

    /// 缺氧
    pub fn hypoxia(&self) -> Vec<&'static str> {
        vec![
            "低张性缺氧: 动脉血氧分压降低",
            "血液性缺氧: 血氧容量降低",
            "循环性缺氧: 组织血流量减少",
            "组织性缺氧: 组织利用氧障碍",
            "发绀: 血中还原血红蛋白增多致皮肤青紫",
            "氧离曲线: 血红蛋白氧饱和度与氧分压的关系曲线",
            "组织缺氧: 组织得不到充足氧供或利用障碍",
            "缺氧性细胞损伤: 缺氧导致细胞代谢紊乱",
        ]
    }

    /// 发热
    pub fn fever(&self) -> Vec<&'static str> {
        vec![
            "发热: 体温调节中枢调定点上移导致体温升高",
            "致热原: 引起发热的物质，分内源性和外源性",
            "发热时相: 体温上升期、高温持续期、体温下降期",
            "热型: 体温曲线的不同形态，如稽留热、弛张热",
            "发热时机体变化: 代谢增强、器官功能改变",
            "发热的意义: 增强免疫，过度则有害",
            "退热药机制: 抑制前列腺素合成降低调定点",
            "超高热: 体温超过41℃危及生命",
        ]
    }

    /// 应激
    pub fn stress(&self) -> Vec<&'static str> {
        vec![
            "应激: 机体对各种强烈刺激的非特异性反应",
            "应激原: 引起应激反应的各种刺激因素",
            "急性期反应: 应激时血浆蛋白成分的变化",
            "热休克蛋白: 应激时细胞产生的保护性蛋白",
            "交感-肾上腺髓质系统: 应激时的主要神经内分泌反应",
            "下丘脑-垂体-肾上腺皮质轴: 应激时的重要调节通路",
            "应激性溃疡: 严重应激导致的急性胃黏膜病变",
            "创伤后应激障碍: 心理性应激的长期影响",
        ]
    }

    /// 休克
    pub fn shock(&self) -> Vec<&'static str> {
        vec![
            "休克: 全身微循环障碍导致组织灌注不足",
            "低血容量性休克: 血容量急剧减少引起",
            "心源性休克: 心泵功能障碍引起",
            "感染性休克: 细菌感染引起，革兰阴性菌多见",
            "过敏性休克: I型变态反应引起",
            "神经源性休克: 血管运动中枢抑制引起",
            "休克分期: 代偿期、失代偿期、难治期",
            "休克时微循环变化: 缺血期、淤血期、衰竭期",
            "多器官功能障碍综合征: 休克严重并发症",
        ]
    }

    /// 凝血与抗凝血平衡紊乱
    pub fn coagulation_disorder(&self) -> Vec<&'static str> {
        vec![
            "出血性疾病: 止血功能障碍引起的疾病",
            "血栓形成: 血液在心血管内凝固形成血栓",
            "弥散性血管内凝血: 微血管内广泛血栓形成",
            "抗凝系统: 防止血液凝固的抗凝机制",
            "纤溶系统: 溶解血栓的纤维蛋白溶解系统",
            "血小板减少: 血小板数量或功能异常",
            "凝血因子缺乏: 先天或后天性凝血因子不足",
            "高凝状态: 血液凝固性增高易形成血栓",
        ]
    }

    /// 缺血-再灌注损伤
    pub fn ischemia_reperfusion(&self) -> Vec<&'static str> {
        vec![
            "缺血-再灌注损伤: 恢复血流后组织损伤加重",
            "自由基损伤: 再灌注时产生大量自由基",
            "钙超载: 细胞内钙离子异常增多",
            "白细胞激活: 白细胞聚集释放炎性介质",
            "无复流现象: 微血管阻塞导致血流不能恢复",
            "心肌顿抑: 再灌注后心肌功能暂时性障碍",
            "缺血预适应: 预先缺血可减轻后续损伤",
            "再灌注心律失常: 再灌注时发生的心律失常",
        ]
    }

    /// 心功能不全
    pub fn cardiac_dysfunction(&self) -> Vec<&'static str> {
        vec![
            "心力衰竭: 心输出量不能满足机体代谢需要",
            "心肌收缩力减弱: 心肌收缩功能下降",
            "心室重构: 心室结构形态的改变",
            "心室顺应性降低: 心室舒张功能障碍",
            "心输出量减少: 泵血功能降低",
            "静脉淤血: 血液回流受阻淤积于静脉系统",
            "呼吸困难: 左心衰竭的典型症状",
            "水肿: 右心衰竭的重要表现",
            "心功能分级: NYHA心功能分级标准",
        ]
    }
}

impl Rule for PathophysiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("pathophysiology")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "病理生理学定律",
            &[
                ("疾病概论", &self.disease_general()),
                ("水电解质代谢紊乱", &self.water_electrolyte_disorder()),
                ("酸碱平衡紊乱", &self.acid_base_disorder()),
                ("缺氧", &self.hypoxia()),
                ("发热", &self.fever()),
                ("应激", &self.stress()),
                ("休克", &self.shock()),
                ("凝血与抗凝血平衡紊乱", &self.coagulation_disorder()),
                ("缺血-再灌注损伤", &self.ischemia_reperfusion()),
                ("心功能不全", &self.cardiac_dysfunction()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pathophysiology_rules() {
        let rules = PathophysiologyRules::new();
        assert!(!rules.disease_general().is_empty());
        assert!(!rules.water_electrolyte_disorder().is_empty());
        assert!(!rules.acid_base_disorder().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_pathophysiology_metadata() {
        let rules = PathophysiologyRules::new();
        assert_eq!(rules.metadata().name, "病理生理学定律");
    }
}
