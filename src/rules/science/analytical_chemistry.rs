//! 分析化学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 分析化学定律集合
pub struct AnalyticalChemistryLaws {
    metadata: RuleMetadata,
}

impl AnalyticalChemistryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "分析化学定律",
                "分析化学基本定律"
            )
            .with_origin("化学")
            .with_tags(vec!["科学".into(), "化学".into(), "分析".into()]),
        }
    }

    /// 定量分析定律
    pub fn quantitative_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("定量定律", "准确测量", "定量分析方法"),
            ("摩尔定律", "摩尔计算", "摩尔定量关系"),
            ("浓度定律", "浓度计算", "溶液浓度计算"),
            ("滴定律", "滴定分析", "滴定定量方法"),
            ("重量定律", "重量分析", "重量分析方法"),
            ("容量定律", "容量分析", "容量分析方法"),
            ("标准定律", "标准物质", "标准物质使用"),
            ("误差定律", "误差分析", "分析误差控制"),
        ]
    }

    /// 定性分析定律
    pub fn qualitative_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("定性定律", "物质鉴定", "定性分析方法"),
            ("鉴别定律", "特征鉴别", "物质特征鉴别"),
            ("分离定律", "分离鉴定", "分离后鉴定"),
            ("检测定律", "检测反应", "检测反应特征"),
            ("验证定律", "验证确认", "定性验证"),
            ("鉴定定律", "鉴定方法", "物质鉴定方法"),
        ]
    }

    /// 仪器分析定律
    pub fn instrumental_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("光谱定律", "光谱分析", "光谱分析方法"),
            ("色谱定律", "色谱分离", "色谱分析方法"),
            ("质谱定律", "质量分析", "质谱分析方法"),
            ("电化学定律", "电分析方法", "电化学分析"),
            ("热分析定律", "热分析法", "热分析方法"),
            ("核磁定律", "NMR分析", "核磁共振分析"),
            ("色谱联用定律", "联用技术", "色谱联用分析"),
            ("光学定律", "光学分析", "光学分析方法"),
        ]
    }

    /// 分离定律
    pub fn separation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("萃取定律", "溶剂萃取", "溶剂萃取分离"),
            ("蒸馏定律", "蒸馏分离", "蒸馏分离方法"),
            ("结晶定律", "结晶分离", "结晶分离方法"),
            ("吸附定律", "吸附分离", "吸附分离方法"),
            ("离子交换定律", "交换分离", "离子交换分离"),
            ("膜分离定律", "膜分离", "膜分离技术"),
            ("色谱分离定律", "色谱法", "色谱分离技术"),
        ]
    }

    /// 分析方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "滴定分析",
            "重量分析",
            "光谱分析",
            "色谱分析",
            "质谱分析",
            "电化学分析",
            "热分析",
            "核磁共振",
        ]
    }

    /// 分析仪器
    pub fn instruments(&self) -> Vec<&'static str> {
        vec![
            "分光光度计",
            "色谱仪",
            "质谱仪",
            "电化学分析仪",
            "热分析仪",
            "核磁共振仪",
            "X射线衍射仪",
            "红外光谱仪",
        ]
    }
}

impl Default for AnalyticalChemistryLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AnalyticalChemistryLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("analytical_chemistry")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【分析化学定律】\n\n定量定律:\n{}\n\n定性定律:\n{}\n\n仪器定律:\n{}\n",
            self.quantitative_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.qualitative_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.instrumental_laws().iter()
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
    fn test_analytical_chemistry_laws() {
        let laws = AnalyticalChemistryLaws::new();
        assert!(!laws.quantitative_laws().is_empty());
        assert!(!laws.instrumental_laws().is_empty());
    }
}