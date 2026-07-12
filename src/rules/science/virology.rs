//! 病毒学定律
//!
//! 病毒学研究病毒的结构、分类、复制机制和致病机理，
//! 包括病毒遗传学、病毒免疫学和病毒防治策略。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 病毒学定律集合
pub struct VirologyLaws {
    metadata: RuleMetadata,
}

impl VirologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("病毒学定律", "病毒学基本定律和病毒机制")
                .with_origin("生物学")
                .with_tags(vec!["科学".into(), "生物".into(), "病毒".into()]),
        }
    }

    /// 病毒结构定律
    pub fn virus_structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("病毒颗粒定律", "基本单位", "病毒颗粒是基本单位"),
            ("衣壳定律", "蛋白外壳", "病毒蛋白衣壳结构"),
            ("包膜定律", "膜结构", "病毒包膜结构"),
            ("核衣壳定律", "核酸蛋白", "核酸与蛋白质复合"),
            ("刺突蛋白定律", "表面蛋白", "病毒表面刺突蛋白"),
            ("基因组定律", "遗传物质", "病毒基因组类型"),
            ("对称定律", "结构对称", "病毒衣壳对称类型"),
            ("组装定律", "组装方式", "病毒颗粒组装机制"),
        ]
    }

    /// 病毒分类定律
    pub fn virus_classification_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("DNA病毒定律", "DNA基因组", "DNA病毒类型"),
            ("RNA病毒定律", "RNA基因组", "RNA病毒类型"),
            ("逆转录病毒定律", "逆转录机制", "逆转录病毒特性"),
            ("包膜病毒定律", "膜包裹", "包膜病毒特征"),
            ("无包膜病毒定律", "裸露颗粒", "无包膜病毒特征"),
            ("动物病毒定律", "动物宿主", "感染动物的病毒"),
            ("植物病毒定律", "植物宿主", "感染植物的病毒"),
            ("噬菌体定律", "细菌宿主", "感染细菌的病毒"),
        ]
    }

    /// 病毒复制定律
    pub fn virus_replication_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("吸附定律", "病毒吸附", "病毒吸附宿主细胞"),
            ("侵入定律", "病毒进入", "病毒进入宿主细胞"),
            ("脱壳定律", "病毒脱壳", "病毒释放基因组"),
            ("复制定律", "病毒复制", "病毒基因组复制"),
            ("转录定律", "病毒转录", "病毒基因转录"),
            ("翻译定律", "病毒翻译", "病毒蛋白合成"),
            ("组装定律", "病毒组装", "病毒颗粒组装"),
            ("释放定律", "病毒释放", "病毒颗粒释放"),
        ]
    }

    /// 病毒致病定律
    pub fn virus_pathogenesis_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("感染定律", "感染类型", "病毒感染类型"),
            ("潜伏定律", "潜伏感染", "病毒潜伏感染"),
            ("慢性定律", "慢性感染", "病毒慢性感染"),
            ("急性定律", "急性感染", "病毒急性感染"),
            ("致癌定律", "病毒致癌", "病毒致癌机制"),
            ("免疫逃逸定律", "逃逸机制", "病毒免疫逃逸"),
            ("细胞病变定律", "细胞损伤", "病毒细胞病变"),
            ("传播定律", "病毒传播", "病毒传播途径"),
        ]
    }

    /// 病毒类型
    pub fn virus_types(&self) -> Vec<&'static str> {
        vec![
            "冠状病毒: COVID-19、SARS、MARS",
            "流感病毒: 甲乙丙型流感病毒",
            "肝炎病毒: 甲乙丙丁戊型肝炎病毒",
            "疱疹病毒: HSV、VZV、EBV",
            "艾滋病毒: HIV逆转录病毒",
            "乳头瘤病毒: HPV病毒家族",
            "肠道病毒: 轮状病毒、诺如病毒",
            "呼吸道病毒: RSV、腺病毒",
        ]
    }

    /// 病毒防治
    pub fn virus_prevention_control(&self) -> Vec<&'static str> {
        vec![
            "疫苗接种: 预防病毒感染",
            "抗病毒药物: 抑制病毒复制",
            "免疫治疗: 增强免疫应答",
            "阻断传播: 切断传播途径",
            "消毒措施: 病毒消杀处理",
            "隔离措施: 感染者隔离管理",
            "监测预警: 病毒监测系统",
            "公共卫生: 公共卫生干预",
        ]
    }

    /// 病毒检测方法
    pub fn detection_methods(&self) -> Vec<&'static str> {
        vec![
            "PCR检测:核酸检测病毒基因组",
            "抗原检测: 检测病毒抗原",
            "抗体检测: 检测病毒抗体",
            "病毒培养: 培养分离病毒",
            "电镜观察: 电镜观察病毒形态",
            "血清学检测: 血清学方法检测",
            "基因测序: 病毒基因测序分析",
            "快速检测: 快速筛查方法",
        ]
    }
}

impl Default for VirologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for VirologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("virology")
    }

    fn explain(&self) -> String {
        format!(
            "【病毒学定律】\n\n\
            病毒结构定律:\n{}\n\n\
            病毒分类定律:\n{}\n\n\
            病毒复制定律:\n{}\n\n\
            病毒致病定律:\n{}\n\n\
            病毒类型:\n{}\n\n\
            病毒防治:\n{}\n",
            self.virus_structure_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.virus_classification_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.virus_replication_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.virus_pathogenesis_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.virus_types()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.virus_prevention_control()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virology_laws() {
        let rules = VirologyLaws::new();
        assert_eq!(rules.virus_structure_laws().len(), 8);
        assert_eq!(rules.virus_classification_laws().len(), 8);
        assert_eq!(rules.virus_replication_laws().len(), 8);
        assert_eq!(rules.virus_pathogenesis_laws().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_virus_types() {
        let rules = VirologyLaws::new();
        assert_eq!(rules.virus_types().len(), 8);
        assert!(rules.virus_types().iter().any(|t| t.contains("冠状病毒")));
    }

    #[test]
    fn test_detection_methods() {
        let rules = VirologyLaws::new();
        assert_eq!(rules.detection_methods().len(), 8);
    }
}
