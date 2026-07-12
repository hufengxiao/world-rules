//! 再生生物学规则
//!
//! 生物体组织和器官再生的生物学原理，包括再生机制、
//! 干细胞调控、再生医学等核心概念。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 再生生物学规则集合
pub struct RegenerativeBiologyRules {
    metadata: RuleMetadata,
}

impl RegenerativeBiologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("再生生物学规则", "生物体组织器官再生的生物学原理")
                .with_origin("再生生物学")
                .with_tags(vec!["科学".into(), "生命科学".into(), "再生医学".into()]),
        }
    }

    /// 再生类型定律
    pub fn regeneration_types(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("生理性再生", "正常更新", "组织细胞的正常更替"),
            ("修复性再生", "损伤修复", "损伤后组织修复再生"),
            ("变形再生", "形态重建", "身体部分重建形态"),
            ("新建再生", "从头形成", "从残余部分形成新个体"),
            ("补偿性再生", "功能补偿", "器官部分切除后增生"),
            ("再生性医学", "治疗应用", "利用再生机制治疗疾病"),
        ]
    }

    /// 再生能力定律
    pub fn regeneration_capacity(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("肝脏再生定律", "强大再生", "肝脏可再生至原大小"),
            ("皮肤再生定律", "持续再生", "表皮细胞持续更新"),
            ("血液再生定律", "干细胞驱动", "造血干细胞持续再生血细胞"),
            ("骨再生定律", "骨折愈合", "骨骼损伤后可完全再生"),
            ("肌肉再生定律", "卫星细胞", "肌肉卫星细胞介导再生"),
            ("神经再生定律", "有限再生", "外周神经可有限再生"),
            ("心脏再生定律", "再生受限", "心肌细胞再生能力有限"),
        ]
    }

    /// 干细胞再生定律
    pub fn stem_cell_regeneration(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("干细胞定律", "自我更新分化", "干细胞可自我更新和分化"),
            ("不对称分裂定律", "一个干细胞一个子细胞", "干细胞不对称分裂"),
            ("对称分裂定律", "两个相同细胞", "干细胞对称分裂产生相同细胞"),
            ("干细胞龛定律", "微环境调控", "干细胞龛调控干细胞行为"),
            ("干细胞动员定律", "迁移分化", "干细胞可迁移至损伤部位"),
            ("分化潜能定律", "分化能力", "干细胞分化能力不同"),
            ("干性维持定律", "转录因子", "转录因子网络维持干性"),
        ]
    }

    /// 再生信号通路定律
    pub fn regeneration_signaling(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Wnt通路定律", "增殖激活", "Wnt信号促进细胞增殖"),
            ("BMP通路定律", "骨形成", "BMP信号促进骨再生"),
            ("FGF通路定律", "多能性", "FGF维持干细胞多能性"),
            ("Notch通路定律", "细胞命运", "Notch调控细胞命运决定"),
            ("Hedgehog通路定律", "形态发生", "Hedgehog调控形态发生"),
            ("TGF-β通路定律", "纤维化调控", "TGF-β调控纤维化和再生"),
            ("VEGF通路定律", "血管生成", "VEGF促进血管新生"),
        ]
    }

    /// 再生因子定律
    pub fn regeneration_factors(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("生长因子定律", "促进再生", "生长因子刺激细胞增殖"),
            ("细胞因子定律", "调控再生", "细胞因子调控再生过程"),
            ("细胞外基质定律", "支架支持", "ECM提供再生支架"),
            ("细胞黏附分子定律", "细胞迁移", "黏附分子介导细胞迁移"),
            ("转录因子定律", "基因调控", "转录因子调控再生基因"),
            ("microRNA定律", "转录后调控", "miRNA调控再生相关基因"),
            ("代谢因子定律", "能量供应", "代谢因子提供再生能量"),
        ]
    }

    /// 再生抑制定律
    pub fn regeneration_inhibition(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("瘢痕形成定律", "阻碍再生", "瘢痕组织阻碍再生"),
            ("纤维化定律", "功能丧失", "纤维化导致功能丧失"),
            ("炎症抑制定律", "过度炎症", "过度炎症抑制再生"),
            ("年龄定律", "再生能力下降", "年龄增加再生能力下降"),
            ("免疫排斥定律", "移植排斥", "免疫系统排斥外来组织"),
            ("肿瘤风险定律", "癌变风险", "过度增殖可能致癌"),
            ("基因突变定律", "再生障碍", "基因突变影响再生能力"),
        ]
    }

    /// 器官再生定律
    pub fn organ_regeneration(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("肝再生定律", "部分切除再生", "肝脏部分切除后可再生"),
            ("肾再生定律", "代偿性肥大", "肾脏切除后代偿性肥大"),
            ("心脏再生定律", "有限再生", "心肌再生能力极其有限"),
            ("肺再生定律", "肺泡再生", "肺泡上皮可有限再生"),
            ("肠再生定律", "隐窝干细胞", "肠隐窝干细胞驱动再生"),
            ("皮肤再生定律", "表皮干细胞", "表皮干细胞维持皮肤更新"),
            ("骨再生定律", "骨祖细胞", "骨祖细胞介导骨再生"),
        ]
    }

    /// 肢体再生定律
    pub fn limb_regeneration(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("两栖类再生定律", "完全再生", "两栖类可完全再生肢体"),
            ("蜥蜴尾再生定律", "部分再生", "蜥蜴可再生尾部"),
            ("哺乳动物限制定律", "再生受限", "哺乳动物肢体再生受限"),
            ("芽基定律", "再生芽基", "芽基是再生关键结构"),
            ("去分化定律", "细胞去分化", "成熟细胞去分化参与再生"),
            ("形态发生定律", "形态重建", "再生组织重建原始形态"),
            ("位置记忆定律", "位置信息", "细胞记忆位置信息"),
        ]
    }

    /// 再生医学应用定律
    pub fn regenerative_medicine(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("组织工程定律", "构建组织", "利用工程方法构建组织"),
            ("干细胞疗法定律", "细胞替代", "干细胞替代受损细胞"),
            ("器官移植定律", "器官替换", "移植器官替代功能"),
            ("3D生物打印定律", "打印器官", "3D打印构建组织器官"),
            ("基因编辑定律", "基因修正", "基因编辑修复遗传缺陷"),
            ("生物材料定律", "支架材料", "生物材料提供再生支架"),
            ("细胞因子疗法定律", "因子治疗", "细胞因子促进再生"),
        ]
    }

    /// 再生研究方法定律
    pub fn regeneration_methods(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("谱系追踪定律", "细胞命运追踪", "追踪细胞命运和来源"),
            ("单细胞测序定律", "单细胞分析", "分析单个细胞基因表达"),
            ("基因编辑定律", "基因功能研究", "编辑基因研究功能"),
            ("类器官培养定律", "体外模型", "体外培养类器官模型"),
            ("异种移植定律", "体内验证", "移植到动物体内验证"),
            ("成像技术定律", "动态观察", "实时观察再生过程"),
            ("生物信息学定律", "数据分析", "分析再生相关数据"),
        ]
    }
}

impl Default for RegenerativeBiologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RegenerativeBiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("regenerative_biology")
    }

    fn explain(&self) -> String {
        format!(
            "【再生生物学规则】\n\n\
            再生生物学研究生物体组织器官再生的生物学原理，是再生医学的基础。\n\n\
            再生类型:\n{}\n\n\
            再生能力:\n{}\n\n\
            干细胞再生:\n{}\n\n\
            再生信号通路:\n{}\n\n\
            再生因子:\n{}\n\n\
            再生抑制:\n{}\n\n\
            器官再生:\n{}\n\n\
            肢体再生:\n{}\n\n\
            再生医学:\n{}\n\n\
            再生研究方法:\n{}",
            self.regeneration_types()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.regeneration_capacity()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stem_cell_regeneration()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.regeneration_signaling()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.regeneration_factors()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.regeneration_inhibition()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.organ_regeneration()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.limb_regeneration()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.regenerative_medicine()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.regeneration_methods()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regenerative_biology_rules() {
        let rules = RegenerativeBiologyRules::new();
        assert_eq!(rules.regeneration_types().len(), 6);
        assert_eq!(rules.regeneration_capacity().len(), 7);
        assert_eq!(rules.stem_cell_regeneration().len(), 7);
        assert_eq!(rules.regeneration_signaling().len(), 7);
        assert_eq!(rules.regeneration_factors().len(), 7);
        assert_eq!(rules.regeneration_inhibition().len(), 7);
        assert_eq!(rules.organ_regeneration().len(), 7);
        assert_eq!(rules.limb_regeneration().len(), 7);
        assert_eq!(rules.regenerative_medicine().len(), 7);
        assert_eq!(rules.regeneration_methods().len(), 7);
    }

    #[test]
    fn test_regenerative_biology_metadata() {
        let rules = RegenerativeBiologyRules::new();
        assert_eq!(rules.metadata().name, "再生生物学规则");
    }
}
