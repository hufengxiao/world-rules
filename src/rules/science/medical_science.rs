//! 医学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 医学定律集合
pub struct MedicalScienceLaws {
    metadata: RuleMetadata,
}

impl MedicalScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "医学定律",
                "医学科学基本定律"
            )
            .with_origin("医学")
            .with_tags(vec!["科学".into(), "医学".into()]),
        }
    }

    /// 生理学定律
    pub fn physiology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("心脏泵血定律", "心输出量 = HR × SV", "心率与每搏输出量决定心输出量"),
            ("血液流动定律", "血流 = ΔP/R", "血压差与血流阻力决定血流"),
            ("肺通气定律", "通气量 = TV × RR", "潮气量与呼吸频率决定通气量"),
            ("氧扩散定律", "扩散 = ΔP × D", "氧气通过肺泡扩散"),
            ("能量代谢定律", "能量摄入 = 能量消耗 + 存储", "能量平衡方程"),
            ("体温调节定律", "产热 = 散热", "体温恒定条件"),
            ("渗透定律", "水分子跨膜移动", "渗透压决定水分移动"),
            ("神经传导定律", "动作电位传播", "神经信号传导机制"),
        ]
    }

    /// 药理学定律
    pub fn pharmacology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("剂量效应定律", "效应与剂量关系", "药物剂量决定效应"),
            ("半数致死量LD50", "毒性指标", "导致50%实验对象死亡的剂量"),
            ("半数有效量ED50", "效力指标", "产生50%最大效应的剂量"),
            ("药物代谢定律", "代谢动力学", "药物在体内的代谢规律"),
            ("首过效应定律", "肝脏代谢", "口服药物肝脏代谢"),
            ("药物相互作用", "协同与拮抗", "药物组合效应"),
            ("生物利用度", "吸收效率", "药物实际吸收比例"),
            ("稳态浓度定律", "给药平衡", "多次给药后药物浓度稳定"),
        ]
    }

    /// 微生物学定律
    pub fn microbiology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("细菌生长定律", "指数生长", "细菌呈指数级增长"),
            ("病毒复制定律", "宿主依赖", "病毒需要宿主细胞复制"),
            ("免疫定律", "免疫识别", "免疫系统识别病原体"),
            ("抗生素定律", "杀菌机制", "抗生素抑制细菌生长"),
            ("突变定律", "随机变异", "微生物随机突变"),
            ("进化定律", "适应环境", "微生物适应环境"),
        ]
    }

    /// 医学定律
    pub fn medical_principles(&self) -> Vec<&'static str> {
        vec![
            "希波克拉底誓言: 医生伦理准则",
            "循证医学: 证据指导决策",
            "预防医学: 预防胜于治疗",
            "整体医学: 身心一体",
            "个体化治疗: 因人而异",
            "多学科协作: 综合诊疗",
            "医学伦理: 患者权益",
            "知情同意: 患者知情权",
        ]
    }
}

impl Default for MedicalScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MedicalScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("medical_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【医学定律】\n\n生理学定律:\n{}\n\n药理学定律:\n{}\n\n微生物学定律:\n{}\n\n医学原则:\n{}\n",
            self.physiology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pharmacology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.microbiology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.medical_principles().iter()
                .map(|principle| format!("▶ {}", principle))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medical_science_laws() {
        let laws = MedicalScienceLaws::new();
        assert!(!laws.physiology_laws().is_empty());
    }
}