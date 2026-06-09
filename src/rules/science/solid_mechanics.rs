//! 固体力学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 固体力学定律集合
pub struct SolidMechanicsLaws {
    metadata: RuleMetadata,
}

impl SolidMechanicsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("固体力学定律", "固体力学基本定律")
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

    /// 复合材料力学定律
    pub fn composite_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("层合板定律", "层叠结构", "层合板力学行为"),
            ("纤维增强定律", "纤维承载", "纤维增强复合材料"),
            ("等效刚度定律", "等效参数", "复合材料等效刚度"),
            ("界面定律", "界面结合", "纤维基体界面性能"),
            ("损伤累积定律", "渐进损伤", "复合材料损伤累积"),
        ]
    }

    /// 结构稳定性定律
    pub fn stability_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("欧拉压杆定律", "F = π²EI/L²", "细长压杆临界载荷"),
            ("屈曲定律", "结构失稳", "结构屈曲失稳现象"),
            ("后屈曲定律", "后屈曲行为", "屈曲后结构行为"),
            ("跳跃定律", "突变失稳", "结构跳跃失稳"),
            ("蠕变屈曲定律", "时间相关", "蠕变导致屈曲"),
        ]
    }

    /// 接触力学定律
    pub fn contact_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("赫兹接触定律", "接触应力", "弹性体接触应力分布"),
            ("摩擦接触定律", "摩擦力", "接触面摩擦力"),
            ("磨损定律", "表面磨损", "接触面磨损规律"),
            ("润滑定律", "减少摩擦", "润滑减少摩擦磨损"),
            ("粘着定律", "粘着磨损", "粘着磨损机制"),
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

    /// 应力应变
    pub fn stress_strain(&self) -> Vec<&'static str> {
        vec![
            "胡克定律: 在弹性范围内应力与应变成正比",
            "弹性模量: 材料抵抗弹性变形能力的度量",
            "泊松比: 横向应变与轴向应变之比",
            "剪切模量: 材料抵抗剪切变形的能力",
            "体积模量: 材料抵抗均匀压缩的能力",
            "屈服准则: 判断材料是否开始塑性变形的条件",
        ]
    }

    /// 强度理论
    pub fn failure_theory(&self) -> Vec<&'static str> {
        vec![
            "最大拉应力理论: 第一强度理论",
            "最大拉应变理论: 第二强度理论",
            "最大剪应力理论: 第三强度理论Tresca准则",
            "形状改变比能理论: 第四强度理论von Mises准则",
            "断裂力学: 含裂纹构件的强度和寿命评估",
            "疲劳: 材料在循环载荷下的渐进损伤和破坏",
            "蠕变: 材件在恒定应力下随时间缓慢变形",
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

    fn explain(&self) -> String {
        format!(
            "【固体力学定律】\n\n弹性定律:\n{}\n\n塑性定律:\n{}\n\n断裂定律:\n{}\n\n复合材料定律:\n{}\n\n结构稳定性定律:\n{}\n\n接触力学定律:\n{}\n",
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
                .join("\n"),
            self.composite_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.stability_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.contact_laws().iter()
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
