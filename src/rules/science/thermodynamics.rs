//! 热力学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 热力学定律集合
pub struct ThermodynamicsLaws {
    metadata: RuleMetadata,
}

impl ThermodynamicsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "热力学定律",
                "热力学基本定律"
            )
            .with_origin("物理学")
            .with_tags(vec!["科学".into(), "物理".into(), "热力学".into()]),
        }
    }

    /// 热力学定律
    pub fn all_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热力学第零定律", "热平衡传递", "若A与B热平衡，B与C热平衡，则A与C热平衡"),
            ("热力学第一定律", "ΔU = Q - W", "能量守恒定律在热力学中的应用"),
            ("热力学第二定律", "ΔS ≥ 0", "孤立系统熵永不减少"),
            ("热力学第三定律", "T→0时 S→常数", "绝对零度不可达到"),
            ("卡诺定理", "η ≤ 1-Tc/Th", "热机效率上限"),
            ("克劳修斯不等式", "循环过程dS≥dQ/T", "任意循环过程的熵变"),
            ("吉布斯自由能", "G = H - TS", "恒温恒压系统的自发判据"),
            ("亥姆霍兹自由能", "A = U - TS", "恒温恒容系统的自发判据"),
            ("理想气体定律", "PV = nRT", "理想气体状态方程"),
            ("范德瓦尔斯方程", "(P+a/V²)(V-b)=RT", "实际气体状态方程"),
            ("热传导定律", "Q = -kA(dT/dx)", "傅里叶热传导定律"),
            ("热对流定律", "Q = hA(Ts-Tf)", "牛顿冷却定律"),
        ]
    }

    /// 热力学过程
    pub fn processes(&self) -> Vec<&'static str> {
        vec![
            "等温过程",
            "等压过程",
            "等容过程",
            "绝热过程",
            "循环过程",
            "卡诺循环",
            "奥托循环",
            "狄塞尔循环",
            "朗肯循环",
            "可逆过程",
            "不可逆过程",
        ]
    }

    /// 热力学函数
    pub fn functions(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("内能 U", "系统内部总能量"),
            ("焓 H", "H = U + PV"),
            ("熵 S", "系统无序度度量"),
            ("自由能 G", "Gibbs自由能"),
            ("自由能 A", "Helmholtz自由能"),
            ("热容 C", "C = dQ/dT"),
            ("比热容 c", "单位质量热容"),
            ("潜热 L", "相变所需热量"),
        ]
    }

    /// 热力学常数
    pub fn constants(&self) -> Vec<(&'static str, f64, &'static str)> {
        vec![
            ("理想气体常数 R", 8.314, "J/(mol·K)"),
            ("玻尔兹曼常数 k", 1.381e-23, "J/K"),
            ("阿伏伽德罗常数 NA", 6.022e23, "mol⁻¹"),
            ("标准大气压", 101325.0, "Pa"),
            ("标准温度", 273.15, "K"),
        ]
    }
}

impl Default for ThermodynamicsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ThermodynamicsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("thermodynamics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【热力学定律】\n\n{}\n",
            self.all_laws().iter()
                .map(|(name, formula, desc)| format!(
                    "▶ {}\n   公式/原理: {}\n   说明: {}\n",
                    name, formula, desc
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermodynamics_laws() {
        let laws = ThermodynamicsLaws::new();
        assert!(!laws.all_laws().is_empty());
    }
}