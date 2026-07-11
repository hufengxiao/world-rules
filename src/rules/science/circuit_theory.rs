//! 电路理论规则
//!
//! 电路理论研究电路中电流、电压、功率的分析方法。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CircuitTheoryRules,
    name: "电路理论规则",
    desc: "电路分析与设计的理论基础",
    origin: "电磁学",
    tags: ["科学", "物理", "电磁", "电路"]
}

impl CircuitTheoryRules {
    /// 基本电路定律
    pub fn basic_laws(&self) -> Vec<&'static str> {
        vec![
            "欧姆定律: V = IR，电压、电流、电阻关系",
            "电阻定义: R = ρl/A，材料电阻",
            "电阻串联: R = R₁ + R₂ + ...",
            "电阻并联: 1/R = 1/R₁ + 1/R₂ + ...",
            "电功率: P = VI = I²R = V²/R",
            "电能: W = Pt = VIt，消耗的能量",
            "焦耳定律: Q = I²Rt，电流热效应",
            "电阻温度系数: R = R₀[1 + α(T - T₀)]",
        ]
    }

    /// 基尔霍夫定律
    pub fn kirchhoff_laws(&self) -> Vec<&'static str> {
        vec![
            "基尔霍夫电流定律(KCL): ΣI = 0，节点电流守恒",
            "基尔霍夫电压定律(KVL): ΣV = 0，回路电压守恒",
            "节点: 三条以上导线的连接点",
            "支路: 连接两个节点的电路路径",
            "回路: 从某点出发回到该点的闭合路径",
            "独立节点方程数: n - 1 个",
            "独立回路方程数: b - n + 1 个",
            "网孔分析: 选择网孔建立方程",
        ]
    }

    /// 电路分析方法
    pub fn analysis_methods(&self) -> Vec<&'static str> {
        vec![
            "节点分析法: 以节点电压为变量",
            "网孔分析法: 以网孔电流为变量",
            "叠加原理: 线性电路多源叠加",
            "戴维南定理: 等效为电压源串联电阻",
            "诺顿定理: 等效为电流源并联电阻",
            "最大功率传输: Rₗ = Rₛ 时负载功率最大",
            "源变换: 电压源与电流源等效变换",
            "等效电阻: 从端口看入的总电阻",
        ]
    }

    /// 电容电路
    pub fn capacitor_circuits(&self) -> Vec<&'static str> {
        vec![
            "电容充放电: V(t) = V₀(1 - e⁻ᵗ/RC)",
            "时间常数: τ = RC，充电速度",
            "电容能量: W = ½CV²",
            "RC电路稳态: 电容视为断路",
            "电容串联: 1/C = 1/C₁ + 1/C₂ + ...",
            "电容并联: C = C₁ + C₂ + ...",
            "电容电流: i = C(dv/dt)",
            "相位关系: 纯电容电路电流超前电压90°",
        ]
    }

    /// 电感电路
    pub fn inductor_circuits(&self) -> Vec<&'static str> {
        vec![
            "电感定义: L = NΦ/I",
            "电感能量: W = ½LI²",
            "电感充放电: i(t) = I₀(1 - e⁻ᵗ/RL)",
            "时间常数: τ = L/R，充放电速度",
            "RL电路稳态: 电感视为短路",
            "电感串联: L = L₁ + L₂ + ...",
            "电感并联: 1/L = 1/L₁ + 1/L₂ + ...",
            "电感电压: v = L(di/dt)",
        ]
    }

    /// RLC电路
    pub fn rlc_circuits(&self) -> Vec<&'static str> {
        vec![
            "RLC串联电路方程: L(d²q/dt²) + R(dq/dt) + q/C = 0",
            "固有频率: ω₀ = 1/√(LC)",
            "阻尼系数: α = R/(2L)",
            "过阻尼: α > ω₀，无振荡",
            "临界阻尼: α = ω₀，最快衰减",
            "欠阻尼: α < ω₀，有振荡",
            "谐振频率: ω₀ = 1/√(LC)",
            "品质因子: Q = ω₀L/R",
        ]
    }

    /// 交流电路
    pub fn ac_circuits(&self) -> Vec<&'static str> {
        vec![
            "正弦交流: v(t) = Vₘ sin(ωt + φ)",
            "角频率: ω = 2πf",
            "有效值: V = Vₘ/√2",
            "阻抗: Z = R + jX",
            "感抗: Xₗ = ωL",
            "容抗: Xc = 1/(ωC)",
            "相位角: φ = arctan(X/R)",
            "功率因数: cosφ = P/S",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "滤波器: RC、RL滤波电路",
            "振荡器: LC振荡产生正弦波",
            "调谐电路: RLC谐振选频",
            "整流电路: AC转DC",
            "放大电路: 晶体管放大",
            "开关电路: MOS管开关",
            "电源电路: 稳压、降压",
            "信号处理: 滤波、放大、整形",
        ]
    }
}

impl Rule for CircuitTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("circuit_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电路理论规则",
            &[
                ("基本电路定律", &self.basic_laws()),
                ("基尔霍夫定律", &self.kirchhoff_laws()),
                ("电路分析方法", &self.analysis_methods()),
                ("电容电路", &self.capacitor_circuits()),
                ("电感电路", &self.inductor_circuits()),
                ("RLC电路", &self.rlc_circuits()),
                ("交流电路", &self.ac_circuits()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_theory_rules() {
        let rules = CircuitTheoryRules::new();
        assert_eq!(rules.metadata().name, "电路理论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.basic_laws().is_empty());
        assert!(!rules.kirchhoff_laws().is_empty());
    }
}
