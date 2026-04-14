//! 核物理定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 核物理定律集合
pub struct NuclearPhysicsLaws {
    metadata: RuleMetadata,
}

impl NuclearPhysicsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "核物理定律",
                "核物理基本定律"
            )
            .with_origin("物理学")
            .with_tags(vec!["科学".into(), "物理".into(), "核物理".into()]),
        }
    }

    /// 核衰变定律
    pub fn decay_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("放射性衰变定律", "N = N₀e^(-λt)", "放射性物质衰变规律"),
            ("半衰期定律", "T½ = ln2/λ", "放射性物质半衰期"),
            ("α衰变定律", "发射α粒子", "原子核发射氦核"),
            ("β衰变定律", "发射β粒子", "原子核发射电子或正电子"),
            ("γ衰变定律", "发射γ射线", "原子核发射光子"),
            ("衰变链定律", "级联衰变", "一系列衰变过程"),
            ("分支衰变定律", "多种衰变", "同一核素多种衰变方式"),
        ]
    }

    /// 核反应定律
    pub fn reaction_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("核裂变定律", "重核分裂", "重核分裂成轻核"),
            ("核聚变定律", "轻核聚合", "轻核聚合成重核"),
            ("质能守恒定律", "E = Δmc²", "核反应能量释放"),
            ("核反应截面定律", "反应概率", "核反应发生概率"),
            ("反应能定律", "Q值", "核反应释放能量"),
            ("核连锁反应定律", "链式反应", "核裂变连锁反应"),
            ("临界质量定律", "最小质量", "维持链式反应的最小质量"),
            ("中子俘获定律", "中子吸收", "原子核吸收中子"),
        ]
    }

    /// 核结构定律
    pub fn structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("核子组成定律", "质子+中子", "原子核由质子和中子组成"),
            ("核力定律", "强相互作用", "核子间的强相互作用"),
            ("壳层模型定律", "核壳层", "核子壳层结构"),
            ("液滴模型定律", "核形状", "原子核类似液滴"),
            ("核结合能定律", "质量亏损", "核子结合释放能量"),
            ("核自旋定律", "角动量", "原子核总角动量"),
            ("核磁矩定律", "磁性质", "原子核磁矩"),
            ("核形状定律", "椭球形", "原子核形状变化"),
        ]
    }

    /// 粒子加速定律
    pub fn acceleration_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("回旋加速定律", "磁场约束", "粒子在磁场中旋转加速"),
            ("同步加速定律", "相稳定", "粒子与加速电场同步"),
            ("直线加速定律", "直线加速", "粒子直线加速"),
            ("对撞机定律", "相对碰撞", "两束粒子对撞"),
            ("能量限制定律", "极限能量", "加速器能量极限"),
        ]
    }

    /// 核物理应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "核电站",
            "核武器",
            "核医学",
            "放射性同位素",
            "核分析技术",
            "核辐射防护",
            "核废料处理",
            "核安全",
            "核聚变反应堆",
            "粒子物理研究",
        ]
    }

    /// 核常数
    pub fn constants(&self) -> Vec<(&'static str, f64, &'static str)> {
        vec![
            ("原子质量单位 u", 1.661e-27, "kg"),
            ("电子质量 me", 9.109e-31, "kg"),
            ("质子质量 mp", 1.673e-27, "kg"),
            ("中子质量 mn", 1.675e-27, "kg"),
            ("核力强度", 1.0e2, "相对电磁力"),
        ]
    }
}

impl Default for NuclearPhysicsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NuclearPhysicsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("nuclear_physics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【核物理定律】\n\n衰变定律:\n{}\n\n核反应定律:\n{}\n\n核结构定律:\n{}\n",
            self.decay_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.reaction_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.structure_laws().iter()
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
    fn test_nuclear_physics_laws() {
        let laws = NuclearPhysicsLaws::new();
        assert!(!laws.decay_laws().is_empty());
        assert!(!laws.reaction_laws().is_empty());
    }
}