//! 等离子体物理详细规则
//!
//! 等离子体是物质的第四态，由自由电子和离子组成。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 等离子体物理详细规则集合
pub struct PlasmaPhysicsDetailedRules {
    metadata: RuleMetadata,
}

impl PlasmaPhysicsDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("等离子体物理详细规则", "等离子体物理学基本概念和定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "等离子体".into()]),
        }
    }

    /// 等离子体基本特性
    pub fn plasma_properties(&self) -> Vec<&'static str> {
        vec![
            "等离子体定义: 由自由电子和离子组成的电离气体",
            "物质第四态: 固态→液态→气态→等离子态",
            "电中性: 等离子体宏观上保持电中性",
            "集体行为: 等离子体粒子通过电磁场相互作用",
            "德拜屏蔽: 电子屏蔽离子电荷的效应",
            "等离子体频率: 电子振荡的固有频率",
            "碰撞频率: 粒子间碰撞的概率",
            "磁化程度: 等离子体受磁场影响的程度",
        ]
    }

    /// 德拜长度和屏蔽
    pub fn debye_length(&self) -> Vec<&'static str> {
        vec![
            "德拜长度λD: 电荷被屏蔽的特征距离",
            "德拜长度公式: λD = √(ε₀kT/(ne²))",
            "屏蔽效应: 超过λD电场被显著衰减",
            "准中性条件: 系统尺度远大于λD",
            "德拜球: 德拜长度范围内电子数量",
            "库仑耦合参数: 粒子间相互作用强度",
            "弱耦合等离子体: Γ<1，粒子间弱相互作用",
            "强耦合等离子体: Γ>1，粒子间强相互作用",
        ]
    }

    /// 等离子体振荡
    pub fn plasma_oscillations(&self) -> Vec<&'static str> {
        vec![
            "等离子体频率ωp: 电子集体振荡频率",
            "等离子体频率公式: ωp = √(ne²/(ε₀m))",
            "朗缪尔振荡: 电子相对于离子的振荡",
            "振荡周期: T = 2π/ωp",
            "振荡不传播: 局部振荡不传播能量",
            "色散关系: ω² = ωp² + 3k²v²",
            "等离子体波: 振荡模式可以传播形成波",
            "离子声波: 离子和电子共同参与的波动",
        ]
    }

    /// 等离子体中的波
    pub fn plasma_waves(&self) -> Vec<&'static str> {
        vec![
            "朗缪尔波: 电子等离子体波",
            "离子声波: 离子和电子的声波模式",
            "阿尔芬波: 磁化等离子体中的磁流体波",
            "磁声波: 磁化等离子体中的压缩波",
            "哨声波: 高频电磁波在等离子体中传播",
            "等离子体振荡: 粒子集体振荡模式",
            "波色散关系: 频率与波矢的关系",
            "波衰减: 波能量转化为粒子能量",
        ]
    }

    /// 磁约束规则
    pub fn magnetic_confinement(&self) -> Vec<&'static str> {
        vec![
            "磁约束原理: 利用磁场约束等离子体",
            "磁镜效应: 粒子在磁场增强区域被反射",
            "螺旋运动: 粒子沿磁力线螺旋运动",
            "回旋频率: ωc = eB/m，粒子绕磁力线旋转频率",
            "拉莫尔半径: rL = mv/(eB)，粒子回旋半径",
            "磁通量守恒: 粒子磁矩保持不变",
            "托卡马克: 环形磁场装置约束等离子体",
            "仿星器: 扭曲磁场结构约束等离子体",
        ]
    }

    /// 惯性约束规则
    pub fn inertial_confinement(&self) -> Vec<&'static str> {
        vec![
            "惯性约束原理: 利用激光压缩燃料靶丸",
            "激光驱动: 多束激光同时照射靶丸",
            "靶丸压缩: 燃料被压缩到高密度",
            "点火条件: 温度和密度达到聚变阈值",
            "能量增益: 输出能量大于激光输入能量",
            "直接驱动: 激光直接照射靶丸表面",
            "间接驱动: 激光先照射黑腔产生X射线",
            "快点火: 先压缩后用另一束激光点火",
        ]
    }

    /// 聚变反应规则
    pub fn fusion_rules(&self) -> Vec<&'static str> {
        vec![
            "D-T聚变: 氘氚聚变反应最容易实现",
            "聚变能量: D+T→⁴He+n+17.6MeV",
            "点火条件: 温度>1亿度、密度足够、约束时间足够",
            "劳森判据: nTτE>10²⁰ keV·s/m³",
            "等离子体温度: 聚变等离子体需达1亿度以上",
            "约束时间: 等离子体保持高温的时间",
            "能量增益因子: Q=E输出/E输入",
            "自持燃烧: 聚变能量维持等离子体温度",
        ]
    }

    /// 等离子体不稳定性
    pub fn plasma_instabilities(&self) -> Vec<&'static str> {
        vec![
            "不稳定性定义: 等离子体偏离平衡态",
            "宏观不稳定性: 等离子体整体形状变化",
            "微观不稳定性: 粒子分布函数变化",
            "kink不稳定性: 等离子体柱扭曲",
            "锯齿振荡: 托卡马克中心温度周期性振荡",
            "漂移波不稳定性: 横越磁场漂移驱动",
            "湍流: 多种不稳定性的非线性耦合",
            "破裂: 等离子体突然失去约束",
        ]
    }

    /// 应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "核聚变反应堆",
            "等离子体刻蚀",
            "等离子体沉积",
            "等离子体显示技术",
            "等离子体焊接",
            "等离子体喷涂",
            "等离子体医疗",
            "空间等离子体研究",
        ]
    }
}

impl Default for PlasmaPhysicsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PlasmaPhysicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("plasma_physics_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "等离子体物理详细规则",
            &[
                ("等离子体特性", &self.plasma_properties()),
                ("德拜长度", &self.debye_length()),
                ("等离子体振荡", &self.plasma_oscillations()),
                ("等离子体波", &self.plasma_waves()),
                ("磁约束", &self.magnetic_confinement()),
                ("惯性约束", &self.inertial_confinement()),
                ("聚变反应", &self.fusion_rules()),
                ("不稳定性", &self.plasma_instabilities()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plasma_physics_detailed_rules() {
        let rules = PlasmaPhysicsDetailedRules::new();
        assert_eq!(rules.metadata().name, "等离子体物理详细规则");
        assert!(!rules.plasma_properties().is_empty());
        assert!(!rules.debye_length().is_empty());
        assert!(!rules.plasma_oscillations().is_empty());
        assert!(!rules.magnetic_confinement().is_empty());
        assert!(!rules.explain().is_empty());
    }
}
