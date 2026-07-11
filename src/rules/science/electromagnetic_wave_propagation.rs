//! 电磁波传播规则
//!
//! 电磁波传播研究电磁波的产生、传播、反射和折射规律。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ElectromagneticWavePropagationRules,
    name: "电磁波传播规则",
    desc: "电磁波产生、传播与应用方法",
    origin: "电磁学",
    tags: ["科学", "物理", "电磁", "电磁波"]
}

impl ElectromagneticWavePropagationRules {
    /// 电磁波产生
    pub fn wave_generation(&self) -> Vec<&'static str> {
        vec![
            "振荡电荷: 振荡电荷产生电磁波",
            "加速电荷: 加速电荷辐射电磁波",
            "天线辐射: 电流在天线中振荡产生电磁波",
            "偶极辐射: 电偶极子振荡辐射",
            "辐射功率: P = μ₀p₀²ω⁴/(12πc)",
            "辐射方向: 垂直于偶极子方向最强",
            "辐射频率: 与振荡频率相同",
            "相干辐射: 同相位叠加增强",
        ]
    }

    /// 传播特性
    pub fn propagation_properties(&self) -> Vec<&'static str> {
        vec![
            "传播速度: v = c/n，介质中速度",
            "折射率: n = √(εᵣμᵣ)",
            "波长关系: λ = v/f",
            "频率不变: 电磁波频率在不同介质中不变",
            "波长变化: 介质中波长 λ' = λ/n",
            "相位速度: vₚ = ω/k",
            "群速度: v₉ = dω/dk",
            "色散: 不同频率波速度不同",
        ]
    }

    /// 反射与折射
    pub fn reflection_refraction(&self) -> Vec<&'static str> {
        vec![
            "反射定律: θ₁ = θ₂，入射角等于反射角",
            "折射定律: n₁sinθ₁ = n₂sinθ₂（斯涅尔定律）",
            "临界角: θc = arcsin(n₂/n₁)，n₁ > n₂",
            "全反射: θ₁ > θc时发生全反射",
            "菲涅尔公式: 反射和透射振幅",
            "反射系数: R = (n₁-n₂)²/(n₁+n₂)²",
            "透射系数: T = 1 - R",
            "布儒斯特角: θB = arctan(n₂/n₁)，全透射",
        ]
    }

    /// 偏振
    pub fn polarization(&self) -> Vec<&'static str> {
        vec![
            "线偏振: E矢量振动方向固定",
            "圆偏振: E矢量端点轨迹为圆",
            "椭圆偏振: E矢量端点轨迹为椭圆",
            "自然光: 各方向偏振均匀分布",
            "偏振片: 只允许特定方向偏振通过",
            "偏振方向: 垂直于传播方向",
            "双折射: 不同偏振方向折射率不同",
            "偏振应用: 偏振显微镜、3D电影",
        ]
    }

    /// 介质中的传播
    pub fn propagation_in_media(&self) -> Vec<&'static str> {
        vec![
            "介电常数: ε = ε₀εᵣ",
            "磁导率: μ = μ₀μᵣ",
            "折射率: n = √(εᵣμᵣ)",
            "吸收: 介质吸收电磁波能量",
            "吸收系数: α = exp(-αx)",
            "色散: 不同频率折射率不同",
            "正常色散: n随频率增加而增加",
            "反常色散: 吸收区附近的色散",
        ]
    }

    /// 导体中的传播
    pub fn propagation_in_conductors(&self) -> Vec<&'static str> {
        vec![
            "衰减: 电磁波在导体中衰减",
            "趋肤深度: δ = √(2/ωμσ)",
            "趋肤效应: 电流集中在导体表面",
            "导体反射: 导体表面反射电磁波",
            "屏蔽: 导体屏蔽电磁波",
            "高频效应: 高频时趋肤深度减小",
            "表面电阻: Rₛ = 1/(σδ)",
            "电磁屏蔽应用: 电子设备防护",
        ]
    }

    /// 电磁波谱
    pub fn electromagnetic_spectrum(&self) -> Vec<&'static str> {
        vec![
            "无线电波: 频率 < 3×10⁹ Hz",
            "微波: 频率 3×10⁹ - 3×10¹² Hz",
            "红外线: 频率 3×10¹² - 4×10¹⁴ Hz",
            "可见光: 频率 4×10¹⁴ - 8×10¹⁴ Hz",
            "紫外线: 频率 8×10¹⁴ - 3×10¹⁶ Hz",
            "X射线: 频率 3×10¹⁶ - 3×10¹⁹ Hz",
            "γ射线: 频率 > 3×10¹⁹ Hz",
            "应用: 不同波段有不同应用",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "无线通信: 手机、WiFi",
            "雷达: 测距、探测",
            "卫星通信: 全球通信",
            "光纤通信: 高速数据传输",
            "微波加热: 微波炉",
            "医学成像: X射线、MRI",
            "遥感: 地球观测",
            "光通信: 激光通信",
        ]
    }
}

impl Rule for ElectromagneticWavePropagationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("electromagnetic_wave_propagation")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电磁波传播规则",
            &[
                ("电磁波产生", &self.wave_generation()),
                ("传播特性", &self.propagation_properties()),
                ("反射与折射", &self.reflection_refraction()),
                ("偏振", &self.polarization()),
                ("介质中的传播", &self.propagation_in_media()),
                ("导体中的传播", &self.propagation_in_conductors()),
                ("电磁波谱", &self.electromagnetic_spectrum()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electromagnetic_wave_propagation_rules() {
        let rules = ElectromagneticWavePropagationRules::new();
        assert_eq!(rules.metadata().name, "电磁波传播规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.wave_generation().is_empty());
        assert!(!rules.propagation_properties().is_empty());
    }
}
