//! 粒子物理详细规则
//!
//! 粒子物理学研究物质的基本组成和基本相互作用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 粒子物理详细规则集合
pub struct ParticlePhysicsDetailedRules {
    metadata: RuleMetadata,
}

impl ParticlePhysicsDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("粒子物理详细规则", "粒子物理学基本概念和定律")
                .with_origin("物理学")
                .with_tags(vec![
                    "科学".into(),
                    "物理".into(),
                    "粒子物理".into(),
                    "量子场论".into(),
                ]),
        }
    }

    /// 标准模型基本粒子
    pub fn standard_model_particles(&self) -> Vec<&'static str> {
        vec![
            "费米子: 构成物质的基本粒子，服从费米-狄拉克统计",
            "夸克家族: 上夸克(u)、下夸克(d)、粲夸克(c)、奇夸克(s)、顶夸克(t)、底夸克(b)",
            "轻子家族: 电子(e)、μ子、τ子及对应中微子",
            "玻色子: 传递相互作用的基本粒子，服从玻色-爱因斯坦统计",
            "规范玻色子: 光子(电磁力)、W/Z玻色子(弱力)、胶子(强力)",
            "希格斯玻色子: 赋予其他粒子质量的粒子",
            "三代粒子: 标准模型包含三代费米子，每代包含两个夸克和两个轻子",
        ]
    }

    /// 基本相互作用
    pub fn fundamental_interactions(&self) -> Vec<&'static str> {
        vec![
            "强相互作用: 夸克之间的相互作用，由胶子传递，最强",
            "弱相互作用: 费米子之间的相互作用，由W/Z玻色子传递",
            "电磁相互作用: 带电粒子之间的相互作用，由光子传递",
            "引力相互作用: 所有物质之间的相互作用，由引力子传递(假设)",
            "相互作用强度: 强力≈强电磁≈弱引力≈最弱",
            "作用范围: 强力短程、电磁长程、弱力短程、引力长程",
            "夸克禁闭: 夸克不能单独存在，只能组成强子",
        ]
    }

    /// 夸克规则
    pub fn quark_rules(&self) -> Vec<&'static str> {
        vec![
            "夸克六种: 上(u)下(d)粲(c)奇(s)顶(t)底(b)六种夸克",
            "夸克电荷: 上粲顶夸克电荷+2/3e，下奇底夸克电荷-1/3e",
            "夸克色荷: 夸克有红绿蓝三种颜色，色荷是强相互作用源",
            "夸克组合: 夸克组成介子(夸克-反夸克)和重子(三夸克)",
            "质子结构: 质子由两个上夸克和一个下夸克组成(uud)",
            "中子结构: 中子由一个上夸克和两个下夸克组成(udd)",
            "夸克味变: β衰变中下夸克变为上夸克，伴随W玻色子发射",
        ]
    }

    /// 轻子规则
    pub fn lepton_rules(&self) -> Vec<&'static str> {
        vec![
            "轻子六种: 电子(e)、μ子、τ子及对应中微子",
            "轻子电荷: 电子μ子τ子电荷-e，中微子电荷为零",
            "轻子质量: 电子0.511MeV、μ子105.7MeV、τ子1.78GeV",
            "中微子: 几乎无质量，只参与弱相互作用",
            "轻子数守恒: 每代轻子有独立的轻子数守恒",
            "μ子衰变: μ子衰变为电子、μ中微子和反电子中微子",
            "τ子衰变: τ子可衰变为μ子或电子及对应中微子",
        ]
    }

    /// 守恒定律
    pub fn conservation_laws(&self) -> Vec<&'static str> {
        vec![
            "能量守恒: 粒子反应总能量守恒",
            "动量守恒: 粒子反应总动量守恒",
            "角动量守恒: 粒子反应总角动量守恒",
            "电荷守恒: 粒子反应总电荷守恒",
            "重子数守恒: 重子数在所有反应中守恒",
            "轻子数守恒: 每代轻子数独立守恒",
            "CPT对称: 电荷共轭、空间反演、时间反演联合对称",
        ]
    }

    /// 粒子衰变规则
    pub fn particle_decay_rules(&self) -> Vec<&'static str> {
        vec![
            "衰变定律: 不稳定粒子自发转化为其他粒子",
            "半衰期: 粒子数量减半所需时间",
            "衰变宽度: Γ=1/τ，τ为平均寿命",
            "衰变模式: 同一粒子可有多种衰变方式",
            "分支比: 各衰变模式概率之比",
            "衰变链: 衰变产物可继续衰变形成链",
            "弱衰变: 通过弱相互作用衰变",
            "电磁衰变: 通过电磁相互作用衰变",
        ]
    }

    /// 粒子碰撞规则
    pub fn particle_collision_rules(&self) -> Vec<&'static str> {
        vec![
            "弹性碰撞: 碰撞后粒子种类不变",
            "非弹性碰撞: 碰撞产生新粒子",
            "截面: 粒子碰撞发生概率的度量",
            "卢瑟福散射: 带电粒子散射公式",
            "衍射散射: 高能碰撞中的衍射效应",
            "深度非弹性散射: 探测质子内部结构",
            "对撞机: 加速粒子对撞研究基本物理",
            "同步辐射: 加速粒子产生辐射",
        ]
    }

    /// 量子场论基础
    pub fn quantum_field_theory(&self) -> Vec<&'static str> {
        vec![
            "场量子化: 将场视为粒子的集合",
            "费曼图: 可视化粒子相互作用过程",
            "传播子: 粒子从一点传播到另一点的几率振幅",
            "虚粒子: 不满足能量守恒的中间态粒子",
            "正规化: 处理量子场论中发散的方法",
            "重整化: 消除理论中无限大的技术",
            "路径积分: 量子力学的另一种表述",
        ]
    }

    /// 应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "粒子加速器",
            "粒子探测器",
            "医学成像",
            "放射治疗",
            "材料分析",
            "核能发电",
            "宇宙射线研究",
            "暗物质探测",
        ]
    }
}

impl Default for ParticlePhysicsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParticlePhysicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("particle_physics_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "粒子物理详细规则",
            &[
                ("标准模型", &self.standard_model_particles()),
                ("基本相互作用", &self.fundamental_interactions()),
                ("夸克规则", &self.quark_rules()),
                ("轻子规则", &self.lepton_rules()),
                ("守恒定律", &self.conservation_laws()),
                ("粒子衰变", &self.particle_decay_rules()),
                ("粒子碰撞", &self.particle_collision_rules()),
                ("量子场论", &self.quantum_field_theory()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_physics_detailed_rules() {
        let rules = ParticlePhysicsDetailedRules::new();
        assert_eq!(rules.metadata().name, "粒子物理详细规则");
        assert!(!rules.standard_model_particles().is_empty());
        assert!(!rules.fundamental_interactions().is_empty());
        assert!(!rules.quark_rules().is_empty());
        assert!(!rules.lepton_rules().is_empty());
        assert!(!rules.conservation_laws().is_empty());
        assert!(!rules.explain().is_empty());
    }
}