//! 材料力学规则
//!
//! 材料力学研究材料在外力作用下的应力、应变和强度问题。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: MaterialsMechanicsRules,
    name: "材料力学规则",
    desc: "材料力学基本概念与强度分析方法",
    origin: "力学",
    tags: ["科学", "物理", "力学", "材料力学"]
}

impl MaterialsMechanicsRules {
    /// 应力和应变
    pub fn stress_strain(&self) -> Vec<&'static str> {
        vec![
            "应力定义: σ = F/A，单位面积上的力",
            "正应力: 垂直于截面的应力",
            "剪应力: 平行于截面的应力 τ = F/A",
            "应变定义: ε = ΔL/L₀，相对变形",
            "线应变: 长度方向的正应变",
            "剪应变: 角度变化的应变 γ = tanφ",
            "泊松比: ν = -ε横向/ε轴向",
            "体积应变: εv = ΔV/V₀",
        ]
    }

    /// 弹性变形
    pub fn elastic_deformation(&self) -> Vec<&'static str> {
        vec![
            "胡克定律: σ = Eε（应力与应变成正比）",
            "弹性模量 E: 材料的刚度指标",
            "剪切模量 G: τ = Gγ",
            "模量关系: G = E/[2(1+ν)]",
            "弹性极限: 材料保持弹性的最大应力",
            "比例极限: 应力应变保持线性关系的极限",
            "弹性变形特点: 外力撤除后变形恢复",
            "弹性变形范围: σ ≤ σe（弹性极限）",
        ]
    }

    /// 拉伸和压缩
    pub fn tension_compression(&self) -> Vec<&'static str> {
        vec![
            "轴向拉力: 沿轴线方向的拉力",
            "轴向压力: 沿轴线方向的压力",
            "拉应力: σ = F/A（正值）",
            "压应力: σ = F/A（负值）",
            "变形公式: ΔL = FL/(EA)",
            "强度条件: σ ≤ [σ]（许用应力）",
            "安全系数: n = σs/[σ] 或 n = σb/[σ]",
            "屈服强度 σs: 材料开始塑性变形的应力",
        ]
    }

    /// 剪切和扭转
    pub fn shear_torsion(&self) -> Vec<&'static str> {
        vec![
            "剪切应力: τ = F/A",
            "剪切强度: τ ≤ [τ]",
            "扭转应力: τ = Tr/Iₚ",
            "极惯性矩: Iₚ = πd⁴/32（圆轴）",
            "扭转角: φ = TL/(GIp)",
            "扭转刚度: GIp",
            "功率与扭矩: T = 9550P/n（kW, rpm）",
            "扭转强度条件: τmax ≤ [τ]",
        ]
    }

    /// 弯曲
    pub fn bending(&self) -> Vec<&'static str> {
        vec![
            "弯矩: M = Fd，使梁弯曲的力矩",
            "剪力: V = ΣF，截面上的横向力",
            "弯曲正应力: σ = My/I",
            "惯性矩: I = bh³/12（矩形）",
            "抗弯截面模量: W = I/ymax",
            "弯曲强度条件: σmax = M/W ≤ [σ]",
            "挠度: y = f(x)，梁的变形",
            "转角: θ = dy/dx，梁截面的转角",
        ]
    }

    /// 组合变形
    pub fn combined_deformation(&self) -> Vec<&'static str> {
        vec![
            "拉弯组合: σ = σ拉 + σ弯",
            "压弯组合: σ = σ压 + σ弯",
            "弯扭组合: σ = σ弯，τ = τ扭",
            "强度理论: 第四强度理论 σr₄ = √(σ² + 3τ²)",
            "第一强度理论: σr₁ = σmax",
            "第二强度理论: σr₂ = σ₁ - ν(σ₂ + σ₃)",
            "第三强度理论: σr₃ = σ₁ - σ₃",
            "弯扭强度校核: σr ≤ [σ]",
        ]
    }

    /// 疲劳和断裂
    pub fn fatigue_fracture(&self) -> Vec<&'static str> {
        vec![
            "疲劳极限: σ₋₁，无限次循环不破坏的最大应力",
            "循环特征: r = σmin/σmax",
            "应力幅: σa = (σmax - σmin)/2",
            "平均应力: σm = (σmax + σmin)/2",
            "S-N 曲线: 应力与疲劳寿命关系",
            "疲劳强度系数: Kf",
            "断裂韧性: KIC，材料抵抗裂纹扩展的能力",
            "裂纹扩展: da/dN = C(ΔK)ⁿ",
        ]
    }

    /// 材料性能
    pub fn material_properties(&self) -> Vec<&'static str> {
        vec![
            "强度: 材料抵抗破坏的能力",
            "刚度: 材料抵抗变形的能力",
            "塑性: 材料产生永久变形而不破坏的能力",
            "韧性: 材料吸收能量而不断裂的能力",
            "硬度: 材料抵抗局部压入的能力",
            "延展性: 材料被拉伸或压缩的能力",
            "脆性: 材料无明显塑性变形即断裂",
            "疲劳: 循环载荷下的破坏",
        ]
    }
}

impl Rule for MaterialsMechanicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("materials_mechanics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "材料力学规则",
            &[
                ("应力和应变", &self.stress_strain()),
                ("弹性变形", &self.elastic_deformation()),
                ("拉伸和压缩", &self.tension_compression()),
                ("剪切和扭转", &self.shear_torsion()),
                ("弯曲", &self.bending()),
                ("组合变形", &self.combined_deformation()),
                ("疲劳和断裂", &self.fatigue_fracture()),
                ("材料性能", &self.material_properties()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_materials_mechanics_rules() {
        let rules = MaterialsMechanicsRules::new();
        assert_eq!(rules.metadata().name, "材料力学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.stress_strain().is_empty());
        assert!(!rules.elastic_deformation().is_empty());
        assert!(!rules.bending().is_empty());
    }
}
