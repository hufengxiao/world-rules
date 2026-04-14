//! 物理化学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 物理化学定律集合
pub struct PhysicalChemistryLaws {
    metadata: RuleMetadata,
}

impl PhysicalChemistryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "物理化学定律",
                "物理化学基本定律"
            )
            .with_origin("化学")
            .with_tags(vec!["科学".into(), "化学".into(), "物理".into()]),
        }
    }

    /// 化学热力学定律
    pub fn thermodynamics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("化学平衡定律", "平衡常数", "反应平衡条件"),
            ("吉布斯自由能定律", "ΔG = ΔH - TΔS", "反应自发判据"),
            ("反应熵定律", "熵变计算", "反应熵变"),
            ("标准自由能定律", "标准态", "标准态自由能"),
            ("温度依赖定律", "温度影响", "温度对平衡影响"),
            ("压力依赖定律", "压力影响", "压力对平衡影响"),
            ("化学势定律", "偏摩尔量", "化学势概念"),
        ]
    }

    /// 化学动力学定律
    pub fn kinetics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("反应速率定律", "速率方程", "反应速率表达式"),
            ("零级反应定律", "速率恒定", "速率与浓度无关"),
            ("一级反应定律", "ln[C] = -kt", "速率与浓度成正比"),
            ("二级反应定律", "速率与浓度平方", "二级反应规律"),
            ("阿伦尼乌斯定律", "k = Ae^(-Ea/RT)", "温度与速率关系"),
            ("活化能定律", "反应门槛", "反应能量门槛"),
            ("催化定律", "降低活化能", "催化剂作用"),
            ("反应机理定律", "步骤顺序", "反应发生过程"),
        ]
    }

    /// 电化学定律
    pub fn electrochemistry_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("电势定律", "电极电势", "电极电势概念"),
            ("能斯特定律", "E = E° - RTlnQ/nF", "电势与浓度关系"),
            ("电解定律", "电流反应", "电解过程规律"),
            ("法拉第定律", "电量与物质", "电量与产物关系"),
            ("电池定律", "电池电势", "电池电势计算"),
            ("腐蚀定律", "金属腐蚀", "电化学腐蚀"),
            ("电解质定律", "离子溶液", "电解质溶液性质"),
        ]
    }

    /// 相平衡定律
    pub fn phase_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("相律定律", "F = C - P + 2", "相平衡自由度"),
            ("气液平衡定律", "拉乌尔定律", "气液平衡规律"),
            ("固液平衡定律", "熔点定律", "固液平衡"),
            ("溶解定律", "溶解度", "溶解度规律"),
            ("分配定律", "两相分配", "物质两相分配"),
            ("气固平衡定律", "吸附平衡", "气体吸附平衡"),
        ]
    }

    /// 表面化学定律
    pub fn surface_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("表面张力定律", "张力大小", "液体表面张力"),
            ("吸附定律", "吸附量", "吸附规律"),
            ("润湿定律", "接触角", "润湿程度"),
            ("毛细现象定律", "毛细上升", "毛细现象"),
            ("表面活性定律", "活性剂作用", "表面活性剂"),
            ("乳化定律", "乳化稳定", "乳化规律"),
        ]
    }

    /// 研究方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "热分析法",
            "光谱法",
            "电化学方法",
            "动力学测量",
            "量子化学计算",
            "分子模拟",
            "表面分析",
            "结构测定",
        ]
    }
}

impl Default for PhysicalChemistryLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PhysicalChemistryLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("physical_chemistry")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【物理化学定律】\n\n热力学定律:\n{}\n\n动力学定律:\n{}\n\n电化学定律:\n{}\n",
            self.thermodynamics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.kinetics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.electrochemistry_laws().iter()
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
    fn test_physical_chemistry_laws() {
        let laws = PhysicalChemistryLaws::new();
        assert!(!laws.thermodynamics_laws().is_empty());
        assert!(!laws.kinetics_laws().is_empty());
    }
}