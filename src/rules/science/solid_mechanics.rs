//! 固体力学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 固体力学定律集合
pub struct SolidMechanicsLaws {
    metadata: RuleMetadata,
}

impl SolidMechanicsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "固体力学定律",
                "固体力学基本定律"
            )
            .with_origin("物理学")
            .with_tags(vec!["科学".into(), "物理".into(), "力学".into()]),
        }
    }

    /// 弹性力学定律
    pub fn elasticity_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("胡克定律", "σ = Eε", "应力应变线性关系"),
            ("广义胡克定律", "多维应力", "三维应力应变关系"),
            ("杨氏模量定律", "E = σ/ε", "材料刚度度量"),
            ("剪切模量定律", "G = τ/γ", "剪切刚度度量"),
            ("泊松比定律", "ν = -ε横/ε纵", "横向应变比"),
            ("体积模量定律", "K = -P/(ΔV/V)", "压缩刚度度量"),
            ("弹性常数关系", "G = E/(2(1+ν))", "各弹性常数关系"),
            ("弹性极限定律", "屈服点", "弹性变形极限"),
        ]
    }

    /// 塑性力学定律
    pub fn plasticity_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("屈服定律", "屈服条件", "材料开始塑性变形"),
            ("塑性流动定律", "应变增量", "塑性应变方向"),
            ("硬化定律", "屈服面扩大", "材料硬化效应"),
            ("塑性应变定律", "不可逆变形", "塑性变形不可恢复"),
            ("塑性功定律", "耗散能量", "塑性变形耗散功"),
            ("鲍辛格效应", "反向屈服", "反向加载屈服降低"),
        ]
    }

    /// 断裂力学定律
    pub fn fracture_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("格里菲斯断裂定律", "裂纹扩展", "脆性材料断裂条件"),
            ("应力强度因子定律", "K = σ√πa", "裂纹尖端应力强度"),
            ("断裂韧性定律", "Kc", "材料断裂临界值"),
            ("疲劳裂纹定律", "裂纹扩展速率", "疲劳裂纹扩展规律"),
            ("断裂判据定律", "K ≥ Kc", "断裂发生判据"),
            ("能量释放率定律", "G = dW/da", "裂纹扩展能量释放"),
        ]
    }

    /// 疲劳定律
    pub fn fatigue_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("疲劳寿命定律", "N次循环", "疲劳失效循环次数"),
            ("S-N曲线定律", "应力寿命曲线", "应力与寿命关系"),
            ("疲劳极限定律", "无限寿命", "疲劳极限应力"),
            ("疲劳累积定律", "损伤累积", "多次载荷损伤累积"),
            ("疲劳裂纹定律", "裂纹萌生扩展", "疲劳裂纹发展过程"),
            ("疲劳门槛值定律", "最低应力", "裂纹不扩展门槛"),
        ]
    }

    /// 蠕变定律
    pub fn creep_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("蠕变定律", "时间变形", "恒载荷下缓慢变形"),
            ("蠕变阶段定律", "三阶段", "蠕变三个阶段"),
            ("蠕变速率定律", "稳态蠕变", "蠕变速率规律"),
            ("蠕变温度定律", "高温效应", "高温蠕变显著"),
            ("应力蠕变定律", "应力影响", "应力对蠕变影响"),
            ("松弛定律", "应力松弛", "恒应变下应力下降"),
        ]
    }

    /// 应力状态
    pub fn stress_states(&self) -> Vec<&'static str> {
        vec![
            "单向应力",
            "双向应力",
            "三向应力",
            "纯剪切",
            "平面应力",
            "平面应变",
            "主应力",
            "应力莫尔圆",
        ]
    }

    /// 应变状态
    pub fn strain_states(&self) -> Vec<&'static str> {
        vec![
            "线应变",
            "剪切应变",
            "体积应变",
            "主应变",
            "应变莫尔圆",
            "应变率",
            "塑性应变",
            "弹性应变",
        ]
    }
}

impl Default for SolidMechanicsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SolidMechanicsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("solid_mechanics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【固体力学定律】\n\n弹性定律:\n{}\n\n塑性定律:\n{}\n\n断裂定律:\n{}\n",
            self.elasticity_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.plasticity_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.fracture_laws().iter()
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
    fn test_solid_mechanics_laws() {
        let laws = SolidMechanicsLaws::new();
        assert!(!laws.elasticity_laws().is_empty());
        assert!(!laws.plasticity_laws().is_empty());
    }
}