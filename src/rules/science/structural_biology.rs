//! 结构生物学定律
//!
//! 结构生物学研究生物大分子的三维结构及其功能关系，
//! 包括蛋白质结构、核酸结构、结构测定方法和结构功能关系。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 结构生物学定律集合
pub struct StructuralBiologyLaws {
    metadata: RuleMetadata,
}

impl StructuralBiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("结构生物学定律", "结构生物学基本定律和结构分析方法")
                .with_origin("生物学")
                .with_tags(vec!["科学".into(), "生物".into(), "结构".into()]),
        }
    }

    /// 蛋白质结构定律
    pub fn protein_structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("一级结构定律", "氨基酸序列", "氨基酸线性排列"),
            ("二级结构定律", "局部结构", "α螺旋β折叠转角"),
            ("三级结构定律", "三维折叠", "蛋白质三维形状"),
            ("四级结构定律", "亚基组装", "多亚基组装方式"),
            ("结构域定律", "功能单元", "蛋白质结构域"),
            ("折叠定律", "折叠规则", "蛋白质折叠规律"),
            ("稳定性定律", "结构稳定", "蛋白质结构稳定性"),
            ("构象定律", "构象变化", "蛋白质构象动态"),
        ]
    }

    /// 核酸结构定律
    pub fn nucleic_acid_structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("DNA双螺旋定律", "双螺旋结构", "DNA双螺旋结构"),
            ("碱基配对定律", "配对规则", "A-T G-C配对"),
            ("RNA结构定律", "单链折叠", "RNA单链折叠结构"),
            ("核糖体结构定律", "RNA-蛋白质", "核糖体复合结构"),
            ("染色质结构定律", "DNA包装", "染色质层次结构"),
            ("染色体结构定律", "高度有序", "染色体结构组织"),
            ("DNA超螺旋定律", "拓扑结构", "DNA超螺旋拓扑"),
            ("核酸-蛋白质定律", "复合结构", "核酸蛋白质复合"),
        ]
    }

    /// 结构测定定律
    pub fn structure_determination_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("X射线晶体学定律", "晶体衍射", "X射线衍射测定结构"),
            ("冷冻电镜定律", "冷冻成像", "冷冻电镜成像技术"),
            ("NMR定律", "核磁共振", "核磁共振测定结构"),
            ("光谱定律", "光谱分析", "光谱技术分析结构"),
            ("质谱定律", "质量分析", "质谱分析结构"),
            ("分子建模定律", "计算预测", "分子建模预测"),
            ("同源建模定律", "模板建模", "同源蛋白建模"),
            ("从头预测定律", "理论预测", "从头结构预测"),
        ]
    }

    /// 结构功能定律
    pub fn structure_function_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("结构决定功能定律", "功能基础", "结构决定功能"),
            ("活性位点定律", "催化位点", "酶活性位点结构"),
            ("结合位点定律", "分子结合", "分子结合位点"),
            ("通道孔洞定律", "运输通道", "分子运输通道"),
            ("表面性质定律", "表面特征", "分子表面特征"),
            ("动力学定律", "结构动态", "结构动态变化"),
            ("变构定律", "变构调节", "变构效应机制"),
            ("组装定律", "结构组装", "分子组装机制"),
        ]
    }

    /// 结构分析方法
    pub fn analysis_methods(&self) -> Vec<&'static str> {
        vec![
            "X射线晶体学: 原子分辨率结构测定",
            "冷冻电镜: 大分子复合物成像",
            "核磁共振: 小分子结构测定",
            "圆二色谱: 二级结构分析",
            "荧光光谱: 结构和动力学分析",
            "分子动力学: 结构动态模拟",
            "同源建模: 模板结构预测",
            "AlphaFold: 深度学习结构预测",
        ]
    }

    /// 结构类型
    pub fn structure_types(&self) -> Vec<&'static str> {
        vec![
            "蛋白质结构: 各级蛋白质结构",
            "核酸结构: DNA和RNA结构",
            "膜蛋白结构: 膜蛋白特殊结构",
            "复合物结构: 多分子复合结构",
            "病毒结构: 病毒颗粒结构",
            "细胞器结构: 细胞器三维结构",
            "超分子结构: 大分子组装结构",
            "纤维结构: 蛋白纤维结构",
        ]
    }

    /// 结构数据库
    pub fn databases(&self) -> Vec<&'static str> {
        vec![
            "PDB: 蛋白质数据库",
            "UniProt: 蛋白质序列数据库",
            "CATH: 结构分类数据库",
            "SCOP: 结构分类数据库",
            "Nucleic Acid DB: 核酸结构数据库",
            "EMDB: 电镜数据库",
            "BMRB: NMR数据库",
            "Protein Data Bank: 全球蛋白质数据库",
        ]
    }
}

impl Default for StructuralBiologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for StructuralBiologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("structural_biology")
    }

    fn explain(&self) -> String {
        format!(
            "【结构生物学定律】\n\n\
            蛋白质结构定律:\n{}\n\n\
            核酸结构定律:\n{}\n\n\
            结构测定定律:\n{}\n\n\
            结构功能定律:\n{}\n\n\
            结构分析方法:\n{}\n\n\
            结构类型:\n{}\n",
            self.protein_structure_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.nucleic_acid_structure_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.structure_determination_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.structure_function_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.analysis_methods()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n"),
            self.structure_types()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structural_biology_laws() {
        let rules = StructuralBiologyLaws::new();
        assert_eq!(rules.protein_structure_laws().len(), 8);
        assert_eq!(rules.nucleic_acid_structure_laws().len(), 8);
        assert_eq!(rules.structure_determination_laws().len(), 8);
        assert_eq!(rules.structure_function_laws().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_protein_structure_levels() {
        let rules = StructuralBiologyLaws::new();
        let laws = rules.protein_structure_laws();
        assert!(laws.iter().any(|(n, _, _)| n.contains("一级")));
        assert!(laws.iter().any(|(n, _, _)| n.contains("四级")));
    }

    #[test]
    fn test_analysis_methods() {
        let rules = StructuralBiologyLaws::new();
        assert_eq!(rules.analysis_methods().len(), 8);
        assert!(rules.analysis_methods().iter().any(|m| m.contains("X射线")));
    }
}
