//! 材料科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 材料科学定律集合
pub struct MaterialScienceLaws {
    metadata: RuleMetadata,
}

impl MaterialScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "材料科学定律",
                "材料科学基本定律"
            )
            .with_origin("材料科学")
            .with_tags(vec!["科学".into(), "材料".into()]),
        }
    }

    /// 固体力学定律
    pub fn mechanics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("胡克定律", "F = kx", "弹性变形范围内应力与应变成正比"),
            ("杨氏模量定律", "E = σ/ε", "材料弹性模量定义"),
            ("泊松比定律", "ν = -ε横/ε纵", "横向与纵向应变比"),
            ("应力应变定律", "σ = Eε", "线弹性关系"),
            ("剪切定律", "τ = Gγ", "剪切应力与剪切应变"),
            ("断裂定律", "临界应力", "材料断裂临界条件"),
            ("疲劳定律", "循环失效", "材料疲劳寿命"),
            ("蠕变定律", "时间变形", "材料随时间缓慢变形"),
        ]
    }

    /// 相变定律
    pub fn phase_change_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("熔化定律", "T熔化", "固体转变为液体"),
            ("凝固定律", "T凝固", "液体转变为固体"),
            ("蒸发定律", "T沸点", "液体转变为气体"),
            ("升华定律", "固→气", "固体直接转变为气体"),
            ("相图定律", "状态边界", "相图显示不同相边界"),
            ("吉布斯相律", "F = C - P + 2", "相平衡自由度"),
            ("杠杆定律", "相比例", "两相区各相比例"),
        ]
    }

    /// 晶体定律
    pub fn crystal_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("晶体结构定律", "周期排列", "原子周期性排列"),
            ("布拉格定律", "2d sinθ = nλ", "晶体衍射条件"),
            ("缺陷定律", "晶格缺陷", "晶体中存在缺陷"),
            ("位错定律", "塑性变形", "位错运动导致塑性变形"),
            ("晶粒定律", "多晶体", "多晶体晶粒结构"),
            ("生长定律", "晶体生长", "晶体生长机制"),
        ]
    }

    /// 材料分类
    pub fn material_types(&self) -> Vec<&'static str> {
        vec![
            "金属材料",
            "陶瓷材料",
            "聚合物材料",
            "复合材料",
            "半导体材料",
            "超导材料",
            "磁性材料",
            "光学材料",
            "生物材料",
            "纳米材料",
            "智能材料",
            "功能材料",
        ]
    }

    /// 材料性能
    pub fn properties(&self) -> Vec<&'static str> {
        vec![
            "强度",
            "硬度",
            "韧性",
            "延展性",
            "导电性",
            "导热性",
            "密度",
            "熔点",
            "抗腐蚀性",
            "耐磨性",
        ]
    }
}

impl Default for MaterialScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MaterialScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("material_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【材料科学定律】\n\n力学定律:\n{}\n\n相变定律:\n{}\n\n晶体定律:\n{}\n",
            self.mechanics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.phase_change_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.crystal_laws().iter()
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
    fn test_material_science_laws() {
        let laws = MaterialScienceLaws::new();
        assert!(!laws.mechanics_laws().is_empty());
    }
}