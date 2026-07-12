//! 表观遗传学定律
//!
//! 表观遗传学研究不改变DNA序列的遗传变化，
//! 包括DNA甲基化、组蛋白修饰、非编码RNA和染色质重塑。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 表观遗传学定律集合
pub struct EpigeneticsLaws {
    metadata: RuleMetadata,
}

impl EpigeneticsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("表观遗传学定律", "表观遗传学基本定律和调控机制")
                .with_origin("生物学")
                .with_tags(vec!["科学".into(), "生物".into(), "表观遗传".into()]),
        }
    }

    /// DNA甲基化定律
    pub fn dna_methylation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("甲基化定律", "甲基添加", "DNA胞嘧啶甲基化"),
            ("去甲基化定律", "甲基去除", "DNA甲基去除"),
            ("CpG岛定律", "CpG区域", "CpG岛甲基化调控"),
            ("基因沉默定律", "表达抑制", "甲基化抑制表达"),
            ("印记定律", "印记基因", "基因组印记现象"),
            ("启动子甲基化定律", "启动子调控", "启动子甲基化调控"),
            ("异染色质定律", "染色质沉默", "甲基化形成异染色质"),
            ("甲基转移酶定律", "酶催化", "甲基转移酶作用"),
        ]
    }

    /// 组蛋白修饰定律
    pub fn histone_modification_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("乙酰化定律", "乙酰添加", "组蛋白乙酰化"),
            ("去乙酰化定律", "乙酰去除", "组蛋白去乙酰化"),
            ("甲基化定律", "甲基修饰", "组蛋白甲基化"),
            ("磷酸化定律", "磷酸修饰", "组蛋白磷酸化"),
            ("泛素化定律", "泛素修饰", "组蛋白泛素化"),
            ("修饰密码定律", "密码系统", "组蛋白密码"),
            ("激活标记定律", "激活修饰", "激活基因表达修饰"),
            ("抑制标记定律", "抑制修饰", "抑制基因表达修饰"),
        ]
    }

    /// 非编码RNA定律
    pub fn noncoding_rna_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("miRNA定律", "微小RNA", "miRNA调控基因"),
            ("lncRNA定律", "长链非编码", "lncRNA多种功能"),
            ("siRNA定律", "小干扰RNA", "siRNA沉默基因"),
            ("piRNA定律", "Piwi互作", "piRNA生殖调控"),
            ("RNA干扰定律", "RNA沉默", "RNA干扰机制"),
            ("转录调控定律", "转录影响", "非编码RNA调控转录"),
            ("翻译调控定律", "翻译影响", "非编码RNA调控翻译"),
            ("染色质调控定律", "染色质影响", "非编码RNA调控染色质"),
        ]
    }

    /// 染色质重塑定律
    pub fn chromatin_remodeling_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("染色质结构定律", "结构变化", "染色质结构动态"),
            ("重塑复合物定律", "重塑酶", "染色质重塑复合物"),
            ("核小体定位定律", "核小体位置", "核小体定位调控"),
            ("开放染色质定律", "开放状态", "染色质开放区域"),
            ("异染色质定律", "沉默区域", "异染色质形成"),
            ("常染色质定律", "活跃区域", "常染色质特性"),
            ("染色质边界定律", "边界确立", "染色质边界"),
            ("染色质记忆定律", "状态记忆", "染色质状态遗传"),
        ]
    }

    /// 表观遗传现象
    pub fn epigenetic_phenomena(&self) -> Vec<&'static str> {
        vec![
            "基因组印记: 父母来源特异性表达",
            "X染色体失活: 女性X染色体沉默",
            "位置效应: 基因位置影响表达",
            "副突变: 一个基因影响另一个",
            "转代遗传: 表观遗传跨代传递",
            "表观遗传重编程: 发育中表观遗传重置",
            "表观遗传变异: 表观遗传状态变化",
            "表观遗传继承: 表观遗传状态遗传",
        ]
    }

    /// 表观遗传与疾病
    pub fn epigenetic_diseases(&self) -> Vec<&'static str> {
        vec![
            "癌症表观遗传: 癌症表观遗传异常",
            "神经疾病表观遗传: 神经疾病表观变化",
            "代谢疾病表观遗传: 代谢病表观因素",
            "发育异常表观遗传: 发育障碍表观原因",
            "印记疾病: 印记基因异常疾病",
            "自身免疫表观遗传: 自身免疫表观因素",
            "心血管表观遗传: 心血管病表观变化",
            "精神疾病表观遗传: 精神疾病表观因素",
        ]
    }

    /// 表观遗传技术
    pub fn epigenetic_techniques(&self) -> Vec<&'static str> {
        vec![
            "甲基化测序: DNA甲基化检测",
            "ChIP-seq: 组蛋白修饰检测",
            "ATAC-seq: 染色质可及性检测",
            "RNA-seq: 非编码RNA检测",
            "Hi-C: 染色质结构分析",
            "单细胞表观遗传: 单细胞表观检测",
            "表观基因组编辑: 表观遗传编辑",
            "表观遗传药物: 表观遗传治疗",
        ]
    }
}

impl Default for EpigeneticsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EpigeneticsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("epigenetics")
    }

    fn explain(&self) -> String {
        format!(
            "【表观遗传学定律】\n\n\
            DNA甲基化定律:\n{}\n\n\
            组蛋白修饰定律:\n{}\n\n\
            非编码RNA定律:\n{}\n\n\
            染色质重塑定律:\n{}\n\n\
            表观遗传现象:\n{}\n\n\
            表观遗传与疾病:\n{}\n",
            self.dna_methylation_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.histone_modification_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.noncoding_rna_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.chromatin_remodeling_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.epigenetic_phenomena()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            self.epigenetic_diseases()
                .iter()
                .map(|d| format!("  • {}", d))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epigenetics_laws() {
        let rules = EpigeneticsLaws::new();
        assert_eq!(rules.dna_methylation_laws().len(), 8);
        assert_eq!(rules.histone_modification_laws().len(), 8);
        assert_eq!(rules.noncoding_rna_laws().len(), 8);
        assert_eq!(rules.chromatin_remodeling_laws().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_methylation_laws() {
        let rules = EpigeneticsLaws::new();
        let laws = rules.dna_methylation_laws();
        assert!(laws.iter().any(|(n, _, _)| n.contains("甲基化")));
    }

    #[test]
    fn test_epigenetic_phenomena() {
        let rules = EpigeneticsLaws::new();
        assert_eq!(rules.epigenetic_phenomena().len(), 8);
        assert!(rules
            .epigenetic_phenomena()
            .iter()
            .any(|p| p.contains("印记")));
    }
}
