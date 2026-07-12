//! 肿瘤生物学定律
//!
//! 肿瘤生物学研究肿瘤的发生、发展、转移和治疗机制，
//! 包括肿瘤遗传学、肿瘤免疫学和肿瘤治疗方法。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 肿瘤生物学定律集合
pub struct CancerBiologyLaws {
    metadata: RuleMetadata,
}

impl CancerBiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("肿瘤生物学定律", "肿瘤生物学基本定律和肿瘤机制")
                .with_origin("生物学")
                .with_tags(vec!["科学".into(), "生物".into(), "肿瘤".into()]),
        }
    }

    /// 肿瘤发生定律
    pub fn tumorigenesis_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("基因突变定律", "驱动突变", "致癌基因驱动突变"),
            ("癌基因定律", "基因激活", "癌基因异常激活"),
            ("抑癌基因定律", "基因失活", "抑癌基因功能丧失"),
            ("DNA损伤定律", "损伤积累", "DNA损伤积累致癌"),
            ("表观遗传定律", "表观改变", "表观遗传异常致癌"),
            ("基因组不稳定律", "基因组不稳定", "基因组稳定性丧失"),
            ("克隆进化定律", "克隆选择", "肿瘤克隆进化"),
            ("多步骤定律", "多步骤发生", "肿瘤多步骤发生"),
        ]
    }

    /// 肿瘤特征定律
    pub fn hallmarks_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("持续增殖定律", "增殖失控", "肿瘤持续增殖"),
            ("逃避生长抑制定律", "抑制逃避", "逃避生长抑制"),
            ("抵抗细胞死亡定律", "死亡抵抗", "抵抗细胞死亡"),
            ("无限复制定律", "复制永生", "无限复制潜能"),
            ("诱导血管生成定律", "血管新生", "诱导血管生成"),
            ("激活侵袭转移定律", "侵袭转移", "激活侵袭转移"),
            ("能量代谢重编程定律", "代谢改变", "代谢重编程"),
            ("免疫逃逸定律", "免疫逃避", "逃避免疫破坏"),
        ]
    }

    /// 肿瘤转移定律
    pub fn metastasis_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("局部侵袭定律", "局部突破", "肿瘤突破局部屏障"),
            ("血管侵入定律", "血管进入", "肿瘤进入血管"),
            ("循环传播定律", "循环运输", "肿瘤细胞循环传播"),
            ("血管外渗定律", "血管离开", "肿瘤离开血管"),
            ("远处定植定律", "远处生长", "肿瘤远处定植"),
            ("转移前环境定律", "转移环境", "转移前环境形成"),
            ("器官选择性定律", "器官选择", "转移器官选择性"),
            ("休眠定律", "转移休眠", "肿瘤休眠机制"),
        ]
    }

    /// 肿瘤治疗定律
    pub fn treatment_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("手术定律", "外科切除", "肿瘤外科手术切除"),
            ("化疗定律", "化学治疗", "化学药物杀死肿瘤"),
            ("放疗定律", "放射治疗", "放射线杀死肿瘤"),
            ("免疫治疗定律", "免疫激活", "激活免疫系统"),
            ("靶向治疗定律", "靶向药物", "靶向特定分子"),
            ("内分泌治疗定律", "激素调节", "激素相关肿瘤治疗"),
            ("联合治疗定律", "多种方法", "多种治疗方法联合"),
            ("个体化治疗定律", "精准治疗", "个体化治疗方案"),
        ]
    }

    /// 肿瘤类型
    pub fn cancer_types(&self) -> Vec<&'static str> {
        vec![
            "肺癌: 肺部恶性肿瘤",
            "乳腺癌: 乳腺恶性肿瘤",
            "肝癌: 肝脏恶性肿瘤",
            "胃癌: 胃部恶性肿瘤",
            "结直肠癌: 结肠直肠恶性肿瘤",
            "白血病: 血液系统恶性肿瘤",
            "淋巴瘤: 淋巴系统恶性肿瘤",
            "黑色素瘤: 皮肤恶性肿瘤",
        ]
    }

    /// 肿瘤标志物
    pub fn biomarkers(&self) -> Vec<&'static str> {
        vec![
            "AFP: 甲胎蛋白肝癌标志物",
            "CEA: 癌胚抗原广谱标志物",
            "PSA: 前列腺特异性抗原",
            "CA125: 卵巢癌标志物",
            "CA19-9: 胰腺癌标志物",
            "HER2: 乳腺癌标志物",
            "EGFR: 多种癌症标志物",
            "BRCA: 乳腺癌卵巢癌基因",
        ]
    }

    /// 肿瘤预防
    pub fn prevention_methods(&self) -> Vec<&'static str> {
        vec![
            "健康饮食: 均衡营养减少风险",
            "适度运动: 规律运动降低风险",
            "戒烟限酒: 避免致癌因素",
            "避免致癌物: 减少致癌物质接触",
            "疫苗接种: 预防病毒相关癌症",
            "定期筛查: 早期发现早期治疗",
            "遗传咨询: 高风险人群管理",
            "健康教育: 提高防癌意识",
        ]
    }
}

impl Default for CancerBiologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CancerBiologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("cancer_biology")
    }

    fn explain(&self) -> String {
        format!(
            "【肿瘤生物学定律】\n\n\
            肿瘤发生定律:\n{}\n\n\
            肿瘤特征定律:\n{}\n\n\
            肿瘤转移定律:\n{}\n\n\
            肿瘤治疗定律:\n{}\n\n\
            肿瘤类型:\n{}\n\n\
            肿瘤标志物:\n{}\n",
            self.tumorigenesis_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hallmarks_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.metastasis_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.treatment_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cancer_types()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.biomarkers()
                .iter()
                .map(|b| format!("  • {}", b))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancer_biology_laws() {
        let rules = CancerBiologyLaws::new();
        assert_eq!(rules.tumorigenesis_laws().len(), 8);
        assert_eq!(rules.hallmarks_laws().len(), 8);
        assert_eq!(rules.metastasis_laws().len(), 8);
        assert_eq!(rules.treatment_laws().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_cancer_types() {
        let rules = CancerBiologyLaws::new();
        assert_eq!(rules.cancer_types().len(), 8);
        assert!(rules.cancer_types().iter().any(|t| t.contains("肺癌")));
    }

    #[test]
    fn test_biomarkers() {
        let rules = CancerBiologyLaws::new();
        assert_eq!(rules.biomarkers().len(), 8);
    }
}
