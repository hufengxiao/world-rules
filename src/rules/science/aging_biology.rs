//! 衰老生物学定律
//!
//! 衰老生物学研究生物体衰老的机制和过程，
//! 包括细胞衰老、遗传程序、端粒理论和抗衰老策略。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 衰老生物学定律集合
pub struct AgingBiologyLaws {
    metadata: RuleMetadata,
}

impl AgingBiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("衰老生物学定律", "衰老生物学基本定律和衰老机制")
                .with_origin("生物学")
                .with_tags(vec!["科学".into(), "生物".into(), "衰老".into()]),
        }
    }

    /// 衰老理论定律
    pub fn aging_theory_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("端粒定律", "端粒缩短", "端粒缩短导致衰老"),
            ("氧化损伤定律", "自由基损伤", "氧化损伤积累衰老"),
            ("遗传程序定律", "基因程序", "遗传程序控制衰老"),
            ("DNA损伤定律", "损伤积累", "DNA损伤积累衰老"),
            ("代谢定律", "代谢速率", "代谢速率影响寿命"),
            ("炎症定律", "慢性炎症", "炎症衰老理论"),
            ("熵定律", "熵增衰老", "熵增导致系统衰退"),
            ("激素定律", "激素变化", "激素变化影响衰老"),
        ]
    }

    /// 细胞衰老定律
    pub fn cellular_senescence_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("复制衰老定律", "增殖极限", "细胞复制极限"),
            ("Hayflick极限定律", "分裂次数", "细胞分裂次数限制"),
            ("衰老标志定律", "衰老特征", "衰老细胞特征"),
            ("SASP定律", "分泌因子", "衰老相关分泌表型"),
            ("细胞周期停滞定律", "周期停止", "衰老细胞周期停滞"),
            ("DNA损伤响应定律", "损伤信号", "DNA损伤引发衰老"),
            ("氧化应激定律", "氧化引发", "氧化应激引发衰老"),
            ("线粒体功能障碍定律", "线粒体损伤", "线粒体功能下降"),
        ]
    }

    /// 衰老器官定律
    pub fn organ_aging_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("皮肤衰老定律", "皮肤老化", "皮肤结构和功能改变"),
            ("心血管衰老定律", "血管老化", "心血管系统功能下降"),
            ("神经系统衰老定律", "神经老化", "神经系统功能减退"),
            ("免疫系统衰老定律", "免疫老化", "免疫功能下降"),
            ("内分泌衰老定律", "激素变化", "内分泌系统改变"),
            ("肌肉骨骼衰老定律", "肌肉骨骼老化", "肌肉骨骼系统衰退"),
            ("感官衰老定律", "感官减退", "感官功能下降"),
            ("消化系统衰老定律", "消化老化", "消化系统功能减退"),
        ]
    }

    /// 抗衰老定律
    pub fn anti_aging_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热量限制定律", "饮食控制", "热量限制延长寿命"),
            ("运动定律", "规律运动", "运动延缓衰老"),
            ("抗氧化定律", "抗氧化剂", "抗氧化延缓衰老"),
            ("激素替代定律", "激素补充", "激素替代抗衰老"),
            ("端粒延长定律", "端粒保护", "保护端粒延缓衰老"),
            ("干细胞定律", "细胞更新", "干细胞补充抗衰老"),
            ("清除衰老细胞定律", "细胞清除", "清除衰老细胞"),
            ("基因治疗定律", "基因干预", "基因干预抗衰老"),
        ]
    }

    /// 衰老标志物
    pub fn aging_markers(&self) -> Vec<&'static str> {
        vec![
            "端粒长度: 反映细胞复制潜能",
            "SASP因子: 衰老细胞分泌标志",
            "p16INK4a: 衰老相关基因表达",
            "p21: 细胞周期调控因子",
            "β-半乳糖苷酶: 衰老细胞活性",
            "线粒体功能: 线粒体活性指标",
            "氧化损伤: 氧化损伤程度",
            "炎症因子: 慢性炎症水平",
        ]
    }

    /// 衰老相关疾病
    pub fn age_related_diseases(&self) -> Vec<&'static str> {
        vec![
            "阿尔茨海默病: 年龄相关神经退行病",
            "帕金森病: 年龄相关运动障碍",
            "心血管疾病: 年龄相关心血管病",
            "骨质疏松: 年龄相关骨骼疾病",
            "糖尿病: 年龄相关代谢疾病",
            "癌症: 年龄增加癌症风险",
            "关节炎: 年龄相关关节疾病",
            "白内障: 年龄相关眼部疾病",
        ]
    }

    /// 衰老干预方法
    pub fn intervention_methods(&self) -> Vec<&'static str> {
        vec![
            "热量限制: 减少能量摄入延长寿命",
            "规律运动: 有氧运动延缓衰老",
            "健康饮食: 均衡营养抗衰老",
            "充足睡眠: 睡眠修复抗衰老",
            "压力管理: 减少压力延缓衰老",
            "社交活动: 社交联系有益长寿",
            "认知训练: 大脑训练延缓认知衰老",
            "预防保健: 疾病预防健康长寿",
        ]
    }
}

impl Default for AgingBiologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AgingBiologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("aging_biology")
    }

    fn explain(&self) -> String {
        format!(
            "【衰老生物学定律】\n\n\
            衰老理论定律:\n{}\n\n\
            细胞衰老定律:\n{}\n\n\
            衰老器官定律:\n{}\n\n\
            抗衰老定律:\n{}\n\n\
            衰老标志物:\n{}\n\n\
            衰老相关疾病:\n{}\n",
            self.aging_theory_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cellular_senescence_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.organ_aging_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.anti_aging_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aging_markers()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n"),
            self.age_related_diseases()
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
    fn test_aging_biology_laws() {
        let rules = AgingBiologyLaws::new();
        assert_eq!(rules.aging_theory_laws().len(), 8);
        assert_eq!(rules.cellular_senescence_laws().len(), 8);
        assert_eq!(rules.organ_aging_laws().len(), 8);
        assert_eq!(rules.anti_aging_laws().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_telomere_theory() {
        let rules = AgingBiologyLaws::new();
        let laws = rules.aging_theory_laws();
        assert!(laws.iter().any(|(n, _, _)| n.contains("端粒")));
    }

    #[test]
    fn test_aging_markers() {
        let rules = AgingBiologyLaws::new();
        assert_eq!(rules.aging_markers().len(), 8);
    }
}
