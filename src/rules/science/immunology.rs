//! 免疫学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 免疫学定律集合
pub struct ImmunologyLaws {
    metadata: RuleMetadata,
}

impl ImmunologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "免疫学定律",
                "免疫学基本定律"
            )
            .with_origin("医学")
            .with_tags(vec!["科学".into(), "医学".into(), "免疫".into()]),
        }
    }

    /// 免疫识别定律
    pub fn recognition_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("抗原识别定律", "特异性识别", "免疫系统识别抗原"),
            ("自身非自身定律", "区分识别", "区分自身与非自身"),
            ("受体定律", "受体识别", "免疫受体识别抗原"),
            ("MHC定律", "分子识别", "MHC分子呈递抗原"),
            ("表位定律", "抗原表位", "抗原特定识别部位"),
            ("克隆选择定律", "克隆扩增", "识别后克隆扩增"),
        ]
    }

    /// 免疫反应定律
    pub fn response_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("固有免疫定律", "即时反应", "固有免疫即时反应"),
            ("适应性免疫定律", "特异性反应", "适应性免疫特异性"),
            ("初次反应定律", "首次反应", "首次接触抗原反应"),
            ("再次反应定律", "增强反应", "再次接触增强反应"),
            ("免疫记忆定律", "记忆细胞", "免疫记忆细胞"),
            ("体液免疫定律", "抗体介导", "抗体介导免疫"),
            ("细胞免疫定律", "细胞介导", "细胞介导免疫"),
        ]
    }

    /// 免疫调节定律
    pub fn regulation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("免疫调节定律", "精细调控", "免疫系统精细调控"),
            ("免疫耐受定律", "不反应", "特定抗原不反应"),
            ("自身耐受定律", "自身不攻击", "不攻击自身组织"),
            ("免疫抑制定律", "反应抑制", "免疫反应抑制"),
            ("免疫增强定律", "反应增强", "免疫反应增强"),
            ("反馈调节定律", "正负反馈", "免疫反馈调节"),
            ("免疫网络定律", "网络调节", "免疫网络理论"),
        ]
    }

    /// 免疫异常定律
    pub fn abnormality_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("超敏反应定律", "过度反应", "免疫过度反应"),
            ("自身免疫定律", "攻击自身", "攻击自身组织"),
            ("免疫缺陷定律", "功能缺失", "免疫功能缺失"),
            ("过敏定律", "过敏反应", "过敏反应机制"),
            ("免疫肿瘤定律", "肿瘤免疫", "肿瘤免疫逃逸"),
            ("移植排斥定律", "排斥反应", "移植排斥机制"),
        ]
    }

    /// 免疫细胞
    pub fn immune_cells(&self) -> Vec<&'static str> {
        vec![
            "T细胞",
            "B细胞",
            "NK细胞",
            "巨噬细胞",
            "树突状细胞",
            "中性粒细胞",
            "淋巴细胞",
            "浆细胞",
        ]
    }

    /// 免疫分子
    pub fn immune_molecules(&self) -> Vec<&'static str> {
        vec![
            "抗体",
            "抗原",
            "补体",
            "细胞因子",
            "干扰素",
            "白细胞介素",
            "趋化因子",
            "MHC分子",
        ]
    }
}

impl Default for ImmunologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ImmunologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("immunology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【免疫学定律】\n\n识别定律:\n{}\n\n反应定律:\n{}\n\n调节定律:\n{}\n",
            self.recognition_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.response_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.regulation_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immunology_laws() {
        let laws = ImmunologyLaws::new();
        assert!(!laws.recognition_laws().is_empty());
        assert!(!laws.response_laws().is_empty());
    }
}