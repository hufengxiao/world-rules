//! 发育生物学定律
//!
//! 发育生物学研究生物体从受精卵到成熟个体的发育过程，
//! 包括胚胎发育、细胞分化、形态发生和器官形成等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 发育生物学定律集合
pub struct DevelopmentalBiologyLaws {
    metadata: RuleMetadata,
}

impl DevelopmentalBiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("发育生物学定律", "发育生物学基本定律和发育机制")
                .with_origin("生物学")
                .with_tags(vec!["科学".into(), "生物".into(), "发育".into()]),
        }
    }

    /// 胚胎发育定律
    pub fn embryonic_development_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("卵裂定律", "细胞分裂", "受精卵快速分裂形成囊胚"),
            ("囊胚形成定律", "囊胚结构", "细胞排列形成囊胚腔"),
            ("原肠形成定律", "三胚层", "囊胚发育成原肠胚"),
            ("神经形成定律", "神经管", "神经板形成神经管"),
            ("器官形成定律", "器官发育", "三胚层分化形成器官"),
            ("胚胎诱导定律", "组织诱导", "组织相互作用影响发育"),
            ("胚胎极性定律", "轴确立", "胚胎体轴的确立"),
            ("体节形成定律", "节段分化", "中胚层分节形成体节"),
        ]
    }

    /// 细胞分化定律
    pub fn cell_differentiation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("分化决定定律", "命运决定", "细胞分化方向的决定"),
            ("基因表达定律", "选择性表达", "分化时基因选择性表达"),
            ("转录因子定律", "调控分化", "转录因子调控分化程序"),
            ("信号通路定律", "分化信号", "信号通路引导分化"),
            ("表观遗传定律", "表观调控", "表观遗传影响分化"),
            ("全能性定律", "全能细胞", "受精卵的全能性"),
            ("多能性定律", "多能细胞", "干细胞的多能性"),
            ("终末分化定律", "不可逆转", "终末分化不可逆转"),
        ]
    }

    /// 形态发生定律
    pub fn morphogenesis_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("形态发生素定律", "浓度梯度", "形态发生素浓度决定命运"),
            ("位置信息定律", "位置决定", "细胞位置决定分化方向"),
            ("图式形成定律", "模式建立", "体轴和节段模式形成"),
            ("细胞迁移定律", "定向移动", "细胞定向迁移形成结构"),
            ("细胞粘附定律", "粘附组装", "细胞粘附形成组织"),
            ("细胞凋亡定律", "程序死亡", "凋亡塑造形态"),
            ("分支形态定律", "分支形成", "分支结构的形成"),
            ("管腔形成定律", "管状结构", "管状器官的形成"),
        ]
    }

    /// 器官发生定律
    pub fn organogenesis_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("心脏发育定律", "心脏形成", "心脏原基发育成心脏"),
            ("神经系统定律", "神经发育", "神经管发育成神经系统"),
            ("肢体发育定律", "肢体形成", "肢芽发育成肢体"),
            ("眼睛发育定律", "视觉系统", "眼原基发育成眼睛"),
            ("肾脏发育定律", "泌尿系统", "肾脏原基发育"),
            ("肝脏发育定律", "消化器官", "肝原基发育"),
            ("肺发育定律", "呼吸系统", "肺原基发育"),
            ("生殖系统定律", "性别分化", "生殖腺发育分化"),
        ]
    }

    /// 发育调控机制
    pub fn regulatory_mechanisms(&self) -> Vec<&'static str> {
        vec![
            "Hox基因: 同源框基因控制体轴模式",
            "Pax基因: 配对框基因控制器官发育",
            "Sox基因: SRY相关基因控制性别决定",
            "信号分子: BMP、Wnt、Notch、FGF等信号通路",
            "转录因子: 调控发育相关基因表达",
            "表观遗传: DNA甲基化和组蛋白修饰调控",
            "miRNA: 小RNA调控发育时序",
            "母体效应: 母体基因产物影响早期发育",
        ]
    }

    /// 发育阶段
    pub fn developmental_stages(&self) -> Vec<&'static str> {
        vec![
            "受精期: 精卵结合形成受精卵",
            "卵裂期: 受精卵快速分裂",
            "囊胚期: 形成囊胚结构",
            "原肠胚期: 三胚层形成",
            "神经胚期: 神经系统原基形成",
            "器官发生期: 各器官系统形成",
            "胎儿期: 器官成熟和生长",
            "出生期: 新个体诞生",
        ]
    }

    /// 发育异常类型
    pub fn developmental_abnormalities(&self) -> Vec<&'static str> {
        vec![
            "先天畸形: 发育过程中的结构异常",
            "发育迟缓: 发育速度低于正常",
            "染色体异常: 染色体数目或结构异常",
            "基因突变: 发育关键基因突变",
            "环境因素: 畸原物质影响发育",
            "母体因素: 母体疾病影响胚胎",
            "多因素畸形: 基因与环境共同作用",
            "发育停滞: 发育过程中止",
        ]
    }
}

impl Default for DevelopmentalBiologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DevelopmentalBiologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("developmental_biology")
    }

    fn explain(&self) -> String {
        format!(
            "【发育生物学定律】\n\n\
            胚胎发育定律:\n{}\n\n\
            细胞分化定律:\n{}\n\n\
            形态发生定律:\n{}\n\n\
            器官发生定律:\n{}\n\n\
            发育调控机制:\n{}\n\n\
            发育阶段:\n{}\n",
            self.embryonic_development_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cell_differentiation_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.morphogenesis_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.organogenesis_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.regulatory_mechanisms()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n"),
            self.developmental_stages()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_developmental_biology_laws() {
        let rules = DevelopmentalBiologyLaws::new();
        assert_eq!(rules.embryonic_development_laws().len(), 8);
        assert_eq!(rules.cell_differentiation_laws().len(), 8);
        assert_eq!(rules.morphogenesis_laws().len(), 8);
        assert_eq!(rules.organogenesis_laws().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_regulatory_mechanisms() {
        let rules = DevelopmentalBiologyLaws::new();
        assert_eq!(rules.regulatory_mechanisms().len(), 8);
        assert!(rules
            .regulatory_mechanisms()
            .iter()
            .any(|m| m.contains("Hox")));
    }

    #[test]
    fn test_developmental_stages() {
        let rules = DevelopmentalBiologyLaws::new();
        assert_eq!(rules.developmental_stages().len(), 8);
    }
}
