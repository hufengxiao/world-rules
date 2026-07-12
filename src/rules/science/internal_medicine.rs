//! 内科学定律 - 研究成人疾病的诊断、治疗和预防
//!
//! 内科学是临床医学的基础学科，涵盖各系统疾病的诊治原则。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: InternalMedicineRules,
    name: "内科学定律",
    desc: "成人疾病诊断、治疗和预防的基本原则",
    origin: "医学",
    tags: ["科学", "医学", "内科"]
}

impl InternalMedicineRules {
    /// 呼吸系统疾病
    pub fn respiratory_diseases(&self) -> Vec<&'static str> {
        vec![
            "肺炎: 细菌、病毒等引起的肺部感染性疾病",
            "慢性阻塞性肺病: 气流受限为特征的慢性呼吸系统疾病",
            "支气管哮喘: 气道慢性炎症和气道高反应性",
            "肺结核: 结核分枝杆菌引起的慢性传染病",
            "肺癌: 支气管黏膜或腺体的恶性肿瘤",
            "支气管扩张: 支气管持久性扩张伴慢性炎症",
            "肺栓塞: 血栓阻塞肺动脉或其分支",
            "胸腔积液: 胸膜腔内液体积聚",
            "气胸: 气体进入胸膜腔",
            "间质性肺病: 肺间质炎症和纤维化",
        ]
    }

    /// 循环系统疾病
    pub fn cardiovascular_diseases(&self) -> Vec<&'static str> {
        vec![
            "冠心病: 冠状动脉粥样硬化导致心肌缺血",
            "高血压病: 体循环动脉血压持续升高",
            "心力衰竭: 心输出量不能满足机体代谢需要",
            "心律失常: 心脏冲动起源或传导异常",
            "心肌病: 心肌结构和功能异常",
            "心瓣膜病: 心瓣膜结构或功能异常",
            "感染性心内膜炎: 心内膜微生物感染",
            "心包疾病: 心包炎症或积液",
            "主动脉疾病: 主动脉瘤、主动脉夹层",
            "周围血管病: 动脉硬化闭塞症、静脉血栓",
        ]
    }

    /// 消化系统疾病
    pub fn digestive_diseases(&self) -> Vec<&'static str> {
        vec![
            "胃炎: 胃黏膜的炎症性疾病",
            "消化性溃疡: 胃或十二指肠溃疡",
            "胃癌: 胃黏膜的恶性肿瘤",
            "炎症性肠病: 克罗恩病、溃疡性结肠炎",
            "肠结核: 肠道结核分枝杆菌感染",
            "肝硬化: 肝脏弥漫性纤维化和假小叶形成",
            "肝癌: 肝细胞或胆管细胞的恶性肿瘤",
            "急性胰腺炎: 胰腺急性炎症反应",
            "慢性胰腺炎: 胰腺慢性进行性炎症",
            "上消化道出血: 食管、胃、十二指肠出血",
        ]
    }

    /// 泌尿系统疾病
    pub fn urinary_diseases(&self) -> Vec<&'static str> {
        vec![
            "肾小球肾炎: 肾小球免疫介导性炎症",
            "肾病综合征: 大量蛋白尿、低蛋白血症、水肿",
            "尿路感染: 细菌侵入尿路引起的感染",
            "急性肾损伤: 肾功能急剧下降",
            "慢性肾脏病: 肾功能进行性减退",
            "肾结石: 尿路结石形成于肾脏",
            "肾囊肿: 肾脏囊性病变",
            "肾癌: 肾脏恶性肿瘤",
            "肾小球疾病: 原发或继发性肾小球病变",
            "肾小管疾病: 肾小管功能异常",
        ]
    }

    /// 血液系统疾病
    pub fn hematologic_diseases(&self) -> Vec<&'static str> {
        vec![
            "贫血: 红细胞或血红蛋白减少",
            "缺铁性贫血: 铁缺乏导致血红蛋白合成不足",
            "巨幼细胞贫血: 叶酸或维生素B12缺乏",
            "再生障碍性贫血: 骨髓造血功能衰竭",
            "溶血性贫血: 红细胞破坏加速超过骨髓代偿",
            "白血病: 造血干细胞恶性克隆性疾病",
            "淋巴瘤: 淋巴细胞恶性增殖性疾病",
            "多发性骨髓瘤: 浆细胞恶性增殖",
            "血小板减少症: 血小板数量减少",
            "凝血功能障碍: 凝血因子缺乏或功能异常",
        ]
    }

    /// 内分泌系统疾病
    pub fn endocrine_diseases(&self) -> Vec<&'static str> {
        vec![
            "糖尿病: 胰岛素分泌或作用缺陷导致高血糖",
            "甲状腺功能亢进: 甲状腺激素分泌过多",
            "甲状腺功能减退: 甲状腺激素分泌不足",
            "甲状腺炎: 甲状腺炎症性疾病",
            "甲状腺结节: 甲状腺内局灶性病变",
            "肾上腺疾病: 肾上腺功能亢进或减退",
            "垂体疾病: 垂体功能异常",
            "骨质疏松症: 骨量减少、骨组织微结构破坏",
            "肥胖症: 能量摄入过多导致脂肪堆积",
            "痛风: 尿酸代谢紊乱导致关节炎",
        ]
    }

    /// 风湿免疫系统疾病
    pub fn rheumatic_diseases(&self) -> Vec<&'static str> {
        vec![
            "类风湿关节炎: 慢性对称性多关节炎",
            "系统性红斑狼疮: 多系统自身免疫病",
            "干燥综合征: 外分泌腺体慢性炎症",
            "强直性脊柱炎: 中轴关节慢性炎症",
            "系统性硬化症: 皮肤和内脏纤维化",
            "皮肌炎/多发性肌炎: 皮肤和肌肉炎症",
            "血管炎: 血管壁炎症坏死",
            "骨关节炎: 关节软骨退行性变",
            "痛风性关节炎: 尿酸盐结晶沉积",
            "白塞病: 全身性血管炎性疾病",
        ]
    }

    /// 神经系统疾病
    pub fn neurological_diseases(&self) -> Vec<&'static str> {
        vec![
            "脑卒中: 脑血管突然破裂或阻塞",
            "癫痫: 脑神经元异常放电导致发作",
            "帕金森病: 黑质多巴胺能神经元变性",
            "阿尔茨海默病: 中枢神经系统退行性疾病",
            "偏头痛: 反复发作的单侧或双侧头痛",
            "重症肌无力: 神经肌肉接头传递障碍",
            "多发性硬化: 中枢神经系统脱髓鞘疾病",
            "脑炎: 脑实质炎症",
            "脑膜炎: 软脑膜炎症",
            "周围神经病: 周围神经损伤或病变",
        ]
    }

    /// 感染性疾病
    pub fn infectious_diseases(&self) -> Vec<&'static str> {
        vec![
            "败血症: 病原体侵入血液循环并繁殖",
            "感染性休克: 严重感染导致的休克",
            "病毒性肝炎: 肝炎病毒引起的肝脏炎症",
            "艾滋病: HIV感染导致的获得性免疫缺陷",
            "伤寒: 伤寒沙门菌引起的急性肠道传染病",
            "细菌性痢疾: 志贺菌引起的肠道传染病",
            "流行性脑脊髓膜炎: 脑膜炎奈瑟菌引起",
            "流行性乙型脑炎: 乙脑病毒引起的中枢神经系统感染",
            "登革热: 登革病毒引起的急性传染病",
            "布鲁菌病: 布鲁菌引起的人畜共患病",
        ]
    }

    /// 中毒与物理因素疾病
    pub fn toxicology_diseases(&self) -> Vec<&'static str> {
        vec![
            "急性中毒: 短时间内接触大量毒物",
            "有机磷中毒: 有机磷农药抑制胆碱酯酶",
            "一氧化碳中毒: 一氧化碳与血红蛋白结合",
            "镇静催眠药中毒: 苯二氮䓬类等药物过量",
            "酒精中毒: 急性或慢性酒精中毒",
            "食物中毒: 摄入有毒食物引起",
            "药物中毒: 药物过量或误服引起",
            "中暑: 高温环境下体温调节功能障碍",
            "电击伤: 电流通过人体造成的损伤",
            "淹溺: 水分吸入呼吸道引起窒息",
        ]
    }
}

impl Rule for InternalMedicineRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("internal_medicine")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "内科学定律",
            &[
                ("呼吸系统疾病", &self.respiratory_diseases()),
                ("循环系统疾病", &self.cardiovascular_diseases()),
                ("消化系统疾病", &self.digestive_diseases()),
                ("泌尿系统疾病", &self.urinary_diseases()),
                ("血液系统疾病", &self.hematologic_diseases()),
                ("内分泌系统疾病", &self.endocrine_diseases()),
                ("风湿免疫系统疾病", &self.rheumatic_diseases()),
                ("神经系统疾病", &self.neurological_diseases()),
                ("感染性疾病", &self.infectious_diseases()),
                ("中毒与物理因素疾病", &self.toxicology_diseases()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_medicine_rules() {
        let rules = InternalMedicineRules::new();
        assert!(!rules.respiratory_diseases().is_empty());
        assert!(!rules.cardiovascular_diseases().is_empty());
        assert!(!rules.digestive_diseases().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_internal_medicine_metadata() {
        let rules = InternalMedicineRules::new();
        assert_eq!(rules.metadata().name, "内科学定律");
    }
}
