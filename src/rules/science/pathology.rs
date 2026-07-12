//! 病理学定律 - 研究疾病的病因、发病机制和病理变化
//!
//! 病理学是连接基础医学与临床医学的桥梁学科，研究疾病的发生发展规律。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PathologyRules,
    name: "病理学定律",
    desc: "疾病病因、发病机制和病理变化的规律",
    origin: "医学",
    tags: ["科学", "医学", "病理学"]
}

impl PathologyRules {
    /// 细胞损伤与适应
    pub fn cell_injury_adaptation(&self) -> Vec<&'static str> {
        vec![
            "萎缩: 发育正常的器官或组织体积缩小",
            "肥大: 细胞体积增大导致器官增大",
            "增生: 实质细胞数量增多导致器官增大",
            "化生: 一种分化成熟的细胞被另一种细胞替代",
            "变性: 细胞内或间质出现异常物质或正常物质过多",
            "坏死: 活体内局部组织细胞的死亡",
            "凋亡: 程序性细胞死亡，不引起炎症反应",
            "细胞老化: 细胞功能进行性下降直至死亡",
        ]
    }

    /// 炎症规则
    pub fn inflammation_rules(&self) -> Vec<&'static str> {
        vec![
            "急性炎症: 血管反应、液体和白细胞渗出为主",
            "慢性炎症: 单核细胞和淋巴细胞浸润、组织增生为主",
            "渗出: 血管内液体和细胞成分进入组织间隙",
            "趋化作用: 炎症细胞向炎症部位定向移动",
            "吞噬作用: 白细胞吞噬和消化病原体及异物",
            "炎症介质: 组胺、前列腺素、白三烯等参与炎症反应",
            "发热: 炎症介质作用于体温调节中枢",
            "白细胞游出: 白细胞穿过血管壁进入炎症区域",
        ]
    }

    /// 肿瘤病理
    pub fn tumor_pathology(&self) -> Vec<&'static str> {
        vec![
            "良性肿瘤: 生长缓慢、有包膜、不转移",
            "恶性肿瘤: 生长迅速、浸润性生长、可转移",
            "异型性: 肿瘤组织在形态上与正常组织的差异",
            "转移: 恶性肿瘤细胞从原发部位扩散到其他部位",
            "血道转移: 肿瘤细胞经血液循环转移",
            "淋巴道转移: 肿瘤细胞经淋巴管转移",
            "种植性转移: 肿瘤细胞脱落种植于体腔",
            "癌前病变: 具有癌变潜在可能性的良性病变",
            "原位癌: 癌细胞局限于上皮层内未突破基底膜",
        ]
    }

    /// 血液循环障碍
    pub fn circulatory_disturbance(&self) -> Vec<&'static str> {
        vec![
            "充血: 器官或组织血管内血液含量增多",
            "淤血: 静脉回流受阻导致血液淤积",
            "血栓形成: 活体心血管内血液凝固或血液成分凝集",
            "栓塞: 循环血液中出现不溶于血液的异常物质",
            "梗死: 器官或组织因血流阻断而发生的缺血性坏死",
            "出血: 血液从血管或心脏逸出",
            "水肿: 组织间隙内液体积聚过多",
            "弥散性血管内凝血: 微循环内广泛微血栓形成",
        ]
    }

    /// 免疫病理
    pub fn immunopathology(&self) -> Vec<&'static str> {
        vec![
            "变态反应: 免疫反应导致的组织损伤或功能障碍",
            "自身免疫病: 机体对自身抗原发生免疫反应",
            "免疫缺陷病: 免疫系统功能低下或缺失",
            "移植排斥: 受体免疫系统攻击移植物",
            "免疫耐受: 免疫系统对特定抗原不产生免疫应答",
            "超敏反应: 抗原刺激后引起的异常免疫反应",
            "免疫复合物病: 抗原抗体复合物沉积引起组织损伤",
        ]
    }

    /// 遗传与先天性疾病
    pub fn genetic_congenital(&self) -> Vec<&'static str> {
        vec![
            "基因突变: DNA序列发生改变",
            "染色体异常: 染色体数目或结构异常",
            "单基因遗传病: 单个基因突变导致的疾病",
            "多基因遗传病: 多个基因和环境因素共同作用",
            "先天畸形: 出生时即存在的形态结构异常",
            "代谢性遗传病: 遗传性酶缺陷导致代谢紊乱",
        ]
    }

    /// 感染性疾病病理
    pub fn infectious_pathology(&self) -> Vec<&'static str> {
        vec![
            "细菌感染: 细菌引起的炎症和组织损伤",
            "病毒感染: 病毒侵入细胞引起病变",
            "真菌感染: 真菌引起的化脓性或肉芽肿性病变",
            "寄生虫感染: 寄生虫引起的组织损伤和炎症",
            "肉芽肿性炎: 巨噬细胞增生形成境界清楚的结节",
            "脓肿: 化脓性炎形成的局限性脓腔",
            "空洞: 病变组织坏死液化经支气管排出",
        ]
    }

    /// 环境与营养病理
    pub fn environmental_pathology(&self) -> Vec<&'static str> {
        vec![
            "化学损伤: 化学物质引起的组织损伤",
            "物理损伤: 机械力、温度、辐射等引起的损伤",
            "营养不良: 营养素缺乏或过剩导致的疾病",
            "维生素缺乏症: 维生素摄入不足或吸收障碍",
            "微量元素缺乏: 铁、锌、碘等微量元素缺乏",
            "肥胖症: 能量摄入过多导致的代谢性疾病",
        ]
    }

    /// 心血管系统病理
    pub fn cardiovascular_pathology(&self) -> Vec<&'static str> {
        vec![
            "动脉粥样硬化: 动脉内膜脂质沉积形成斑块",
            "冠心病: 冠状动脉粥样硬化导致心肌缺血",
            "高血压病: 体循环动脉血压持续升高",
            "心肌病: 心肌原发性疾病",
            "心瓣膜病: 心瓣膜结构或功能异常",
            "心肌炎: 心肌的炎症性疾病",
            "心包炎: 心包膜的炎症性疾病",
            "主动脉瘤: 主动脉壁局限性扩张",
        ]
    }

    /// 呼吸系统病理
    pub fn respiratory_pathology(&self) -> Vec<&'static str> {
        vec![
            "慢性支气管炎: 支气管黏膜慢性炎症",
            "肺气肿: 肺组织弹性减退、过度膨胀",
            "肺炎: 肺组织的炎症性疾病",
            "肺结核: 结核杆菌引起的慢性肉芽肿性炎",
            "肺癌: 支气管黏膜或腺体的恶性肿瘤",
            "支气管扩张: 支气管持久性扩张",
            "尘肺: 长期吸入粉尘引起的肺部疾病",
            "间质性肺病: 肺间质的炎症和纤维化",
        ]
    }
}

impl Rule for PathologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("pathology")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "病理学定律",
            &[
                ("细胞损伤与适应", &self.cell_injury_adaptation()),
                ("炎症规则", &self.inflammation_rules()),
                ("肿瘤病理", &self.tumor_pathology()),
                ("血液循环障碍", &self.circulatory_disturbance()),
                ("免疫病理", &self.immunopathology()),
                ("遗传与先天性疾病", &self.genetic_congenital()),
                ("感染性疾病病理", &self.infectious_pathology()),
                ("环境与营养病理", &self.environmental_pathology()),
                ("心血管系统病理", &self.cardiovascular_pathology()),
                ("呼吸系统病理", &self.respiratory_pathology()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pathology_rules() {
        let rules = PathologyRules::new();
        assert!(!rules.cell_injury_adaptation().is_empty());
        assert!(!rules.inflammation_rules().is_empty());
        assert!(!rules.tumor_pathology().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_pathology_metadata() {
        let rules = PathologyRules::new();
        assert_eq!(rules.metadata().name, "病理学定律");
    }
}
