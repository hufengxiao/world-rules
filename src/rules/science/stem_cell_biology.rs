//! 干细胞生物学规则
//!
//! 干细胞的基础生物学原理，包括干细胞类型、分化机制、
//! 干细胞龛、干细胞应用等核心概念。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 干细胞生物学规则集合
pub struct StemCellBiologyRules {
    metadata: RuleMetadata,
}

impl StemCellBiologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("干细胞生物学规则", "干细胞的基础生物学原理")
                .with_origin("干细胞生物学")
                .with_tags(vec!["科学".into(), "生命科学".into(), "干细胞".into()]),
        }
    }

    /// 干细胞类型定律
    pub fn stem_cell_types(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("全能干细胞", "受精卵", "可发育成完整个体"),
            ("胚胎干细胞", "囊胚内细胞团", "可分化为所有细胞类型"),
            ("成体干细胞", "成体组织", "分化为特定组织细胞"),
            ("诱导多能干细胞", "重编程细胞", "体细胞重编程为多能细胞"),
            ("间充质干细胞", "骨髓脂肪", "分化为骨软骨脂肪细胞"),
            ("造血干细胞", "骨髓", "分化为所有血细胞"),
            ("神经干细胞", "脑室下区", "分化为神经元胶质细胞"),
        ]
    }

    /// 干细胞特性定律
    pub fn stem_cell_properties(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("自我更新定律", "无限增殖", "干细胞可无限自我更新"),
            ("多向分化定律", "分化潜能", "干细胞分化为多种细胞"),
            ("克隆形成定律", "单细胞克隆", "单个干细胞形成克隆"),
            ("不对称分裂定律", "分化与更新", "分裂产生干细胞和子细胞"),
            ("对称分裂定律", "两个相同细胞", "分裂产生两个相同细胞"),
            ("静息态定律", "G0期", "干细胞可长期处于静息态"),
            ("可塑性定律", "跨系分化", "干细胞可跨谱系分化"),
        ]
    }

    /// 干细胞龛定律
    pub fn stem_cell_niche(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("微环境定律", "龛定义", "干细胞龛是干细胞微环境"),
            ("细胞外基质定律", "ECM支持", "ECM提供物理和信号支持"),
            ("细胞间作用定律", "细胞通讯", "龛细胞与干细胞通讯"),
            ("信号分子定律", "调控信号", "龛分泌信号调控干细胞"),
            ("血管定律", "营养供应", "血管为龛提供营养"),
            ("神经定律", "神经调控", "神经调控干细胞龛"),
            ("低氧定律", "缺氧环境", "干细胞龛常为低氧环境"),
        ]
    }

    /// 干细胞分化定律
    pub fn stem_cell_differentiation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("谱系决定定律", "分化方向", "干细胞决定分化方向"),
            ("转录因子定律", "基因调控", "转录因子调控分化基因"),
            ("表观遗传定律", "染色质修饰", "表观遗传调控分化"),
            ("信号通路定律", "分化信号", "信号通路引导分化"),
            ("细胞命运定律", "命运决定", "细胞命运决定机制"),
            ("定向分化定律", "体外诱导", "体外诱导定向分化"),
            ("谱系转化定律", "细胞重编程", "一种细胞转为另一种"),
        ]
    }

    /// 干细胞调控定律
    pub fn stem_cell_regulation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Wnt信号定律", "自我更新", "Wnt信号维持干细胞自我更新"),
            ("Notch信号定律", "命运决定", "Notch信号调控细胞命运"),
            ("BMP信号定律", "分化抑制", "BMP信号抑制分化"),
            ("FGF信号定律", "多能性维持", "FGF信号维持多能性"),
            ("TGF-β信号定律", "多能性调控", "TGF-β调控多能性"),
            ("Hedgehog信号定律", "增殖调控", "Hedgehog信号调控增殖"),
            ("PI3K/AKT信号定律", "存活增殖", "PI3K/AKT促进存活增殖"),
        ]
    }

    /// 转录因子网络定律
    pub fn transcription_factor_network(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Oct4定律", "多能性核心", "Oct4是维持多能性的核心因子"),
            ("Sox2定律", "多能性维持", "Sox2与Oct4协同维持多能性"),
            ("Nanog定律", "自我更新", "Nanog维持干细胞自我更新"),
            ("Klf4定律", "重编程因子", "Klf4是重编程的关键因子"),
            ("c-Myc定律", "增殖调控", "c-Myc调控干细胞增殖"),
            ("Lin28定律", "代谢调控", "Lin28调控干细胞代谢"),
            ("p53定律", "分化调控", "p53调控干细胞分化"),
        ]
    }

    /// 表观遗传调控定律
    pub fn epigenetic_regulation(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("DNA甲基化定律", "基因沉默", "DNA甲基化沉默基因"),
            ("组蛋白修饰定律", "染色质状态", "组蛋白修饰调控染色质"),
            ("染色质重塑定律", "可及性调控", "染色质重塑改变可及性"),
            ("非编码RN定律A", "转录后调控", "非编码RNA调控基因表达"),
            ("印记基因定律", "亲源表达", "印记基因亲源特异性表达"),
            ("X染色体失活定律", "剂量补偿", "雌性X染色体失活"),
            ("端粒定律", "复制潜能", "端粒长度影响复制潜能"),
        ]
    }

    /// 干细胞应用定律
    pub fn stem_cell_applications(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("细胞治疗定律", "疾病治疗", "干细胞用于疾病治疗"),
            ("组织工程定律", "组织构建", "干细胞构建组织器官"),
            ("药物筛选定律", "药物测试", "干细胞用于药物筛选"),
            ("疾病模型定律", "疾病研究", "干细胞建立疾病模型"),
            ("基因治疗定律", "基因修复", "干细胞介导基因治疗"),
            ("个性化医疗定律", "个体化治疗", "iPSC实现个性化医疗"),
            ("再生医学定律", "组织修复", "干细胞促进组织修复"),
        ]
    }

    /// 干细胞技术定律
    pub fn stem_cell_techniques(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("分离培养定律", "体外扩增", "体外分离培养干细胞"),
            ("定向分化定律", "特定细胞", "诱导分化为特定细胞"),
            ("重编程定律", "iPSC产生", "体细胞重编程为iPSC"),
            ("基因编辑定律", "基因修饰", "编辑干细胞基因"),
            ("流式分选定律", "纯化干细胞", "流式细胞术分选干细胞"),
            ("克隆形成定律", "单克隆", "形成单细胞来源克隆"),
            ("类器官培养定律", "3D培养", "培养形成类器官结构"),
        ]
    }

    /// 干细胞伦理定律
    pub fn stem_cell_ethics(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("胚胎伦理定律", "胚胎来源", "胚胎干细胞来源的伦理问题"),
            ("知情同意定律", "患者知情", "干细胞治疗的知情同意"),
            ("安全性定律", "风险控制", "干细胞治疗的安全性"),
            ("临床转化定律", "规范转化", "基础到临床的规范转化"),
            ("监管定律", "监管框架", "干细胞研究的监管"),
            ("知识产权定律", "专利保护", "干细胞技术的专利"),
            ("公平获取定律", "资源公平", "干细胞资源的公平获取"),
        ]
    }
}

impl Default for StemCellBiologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for StemCellBiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("stem_cell_biology")
    }

    fn explain(&self) -> String {
        format!(
            "【干细胞生物学规则】\n\n\
            干细胞生物学研究干细胞的生物学特性，是再生医学和组织工程的基础。\n\n\
            干细胞类型:\n{}\n\n\
            干细胞特性:\n{}\n\n\
            干细胞龛:\n{}\n\n\
            干细胞分化:\n{}\n\n\
            干细胞调控:\n{}\n\n\
            转录因子网络:\n{}\n\n\
            表观遗传调控:\n{}\n\n\
            干细胞应用:\n{}\n\n\
            干细胞技术:\n{}\n\n\
            干细胞伦理:\n{}",
            self.stem_cell_types()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stem_cell_properties()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stem_cell_niche()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stem_cell_differentiation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stem_cell_regulation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.transcription_factor_network()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.epigenetic_regulation()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stem_cell_applications()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stem_cell_techniques()
                .iter()
                .map(|(name, formula, desc)| format!("  • {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stem_cell_ethics()
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
    fn test_stem_cell_biology_rules() {
        let rules = StemCellBiologyRules::new();
        assert_eq!(rules.stem_cell_types().len(), 7);
        assert_eq!(rules.stem_cell_properties().len(), 7);
        assert_eq!(rules.stem_cell_niche().len(), 7);
        assert_eq!(rules.stem_cell_differentiation().len(), 7);
        assert_eq!(rules.stem_cell_regulation().len(), 7);
        assert_eq!(rules.transcription_factor_network().len(), 7);
        assert_eq!(rules.epigenetic_regulation().len(), 7);
        assert_eq!(rules.stem_cell_applications().len(), 7);
        assert_eq!(rules.stem_cell_techniques().len(), 7);
        assert_eq!(rules.stem_cell_ethics().len(), 7);
    }

    #[test]
    fn test_stem_cell_biology_metadata() {
        let rules = StemCellBiologyRules::new();
        assert_eq!(rules.metadata().name, "干细胞生物学规则");
    }
}
