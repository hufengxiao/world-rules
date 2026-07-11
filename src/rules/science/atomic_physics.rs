//! 原子物理规则
//!
//! 原子物理学研究原子的结构和性质。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 原子物理规则集合
pub struct AtomicPhysicsRules {
    metadata: RuleMetadata,
}

impl AtomicPhysicsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("原子物理规则", "原子物理学基本概念和定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "原子物理".into()]),
        }
    }

    /// 原子结构规则
    pub fn atomic_structure(&self) -> Vec<&'static str> {
        vec![
            "原子定义: 由原子核和电子组成的最小化学单位",
            "原子核: 由质子和中子组成位于原子中心",
            "电子: 原子核周围带负电的粒子",
            "质子: 原子核中带正电的粒子",
            "中子: 原子核中不带电的粒子",
            "电子数: 等于质子数原子整体电中性",
            "原子半径: 约0.1-0.3nm",
            "原子核半径: 约原子半径的万分之一",
        ]
    }

    /// 电子壳层规则
    pub fn electron_shell_rules(&self) -> Vec<&'static str> {
        vec![
            "电子壳层: 电子在原子中按能量分层分布",
            "主量子数n: 壳层数从1开始",
            "子壳层: 每个主壳层包含多个子壳层",
            "角量子数l: 子壳层类型(s p d f)",
            "磁量子数m: 电子轨道在空间的取向",
            "自旋量子数s: 电子自旋方向",
            "泡利原理: 每个态最多一个电子",
            "电子排布: 电子按能量从低到高填充",
        ]
    }

    /// 原子光谱规则
    pub fn atomic_spectrum(&self) -> Vec<&'static str> {
        vec![
            "原子光谱: 原子发射或吸收光的特征谱线",
            "发射光谱: 原子激发后发射光",
            "吸收光谱: 原子吸收特定波长光",
            "连续谱: 所有波长都有光的谱",
            "线状谱: 只有特定波长有光的谱",
            "氢原子光谱: 最简单的原子光谱",
            "巴尔默系: 氢原子可见光区谱线",
            "莱曼系: 氢原子紫外区谱线",
        ]
    }

    /// 原子能级规则
    pub fn atomic_energy_levels(&self) -> Vec<&'static str> {
        vec![
            "能级定义: 原子中电子的能量状态",
            "基态: 原子最低能量的稳定状态",
            "激发态: 原子高于基态的能量状态",
            "能级跃迁: 电子从一能级跃迁到另一能级",
            "能量吸收: 电子跃迁到高能级吸收能量",
            "能量发射: 电子跃迁到低能级释放能量",
            "跃迁选择: 不是所有跃迁都允许",
            "能级图: 原子能级的可视化表示",
        ]
    }

    /// 原子辐射规则
    pub fn atomic_radiation(&self) -> Vec<&'static str> {
        vec![
            "自发辐射: 原子自发从激发态跃迁到低能态",
            "受激辐射: 外部光子诱发原子辐射",
            "激光原理: 受激辐射放大产生激光",
            "荧光: 短寿命激发态的辐射",
            "磷光: 长寿命激发态的辐射",
            "共振辐射: 原子吸收后立即发射相同光",
            "谱线宽度: 谱线有一定宽度而非无限细",
            "自然宽度: 由原子寿命决定的谱线宽度",
        ]
    }

    /// 原子模型规则
    pub fn atomic_models(&self) -> Vec<&'static str> {
        vec![
            "汤姆逊模型: 电子嵌入正电球体",
            "卢瑟福模型: 原子核加轨道电子",
            "玻尔模型: 电子在特定轨道运动",
            "玻尔假设: 轨道量子化和跃迁规则",
            "玻尔半径: 氢原子基态电子轨道半径",
            "量子力学模型: 电子用波函数描述",
            "电子云: 电子在原子中的概率分布",
            "轨道: 电子在原子中的量子态",
        ]
    }

    /// 原子磁性质规则
    pub fn atomic_magnetic_properties(&self) -> Vec<&'static str> {
        vec![
            "原子磁矩: 原子整体的磁矩",
            "电子轨道磁矩: 电子轨道运动产生的磁矩",
            "电子自旋磁矩: 电子自旋产生的磁矩",
            "核磁矩: 原子核的磁矩",
            "塞曼效应: 磁场分裂原子能级",
            "正常塞曼效应: 磁场分裂成三条谱线",
            "反常塞曼效应: 磁场分裂更多谱线",
            "磁共振: 原子磁矩与磁场共振",
        ]
    }

    /// 原子相互作用规则
    pub fn atomic_interactions(&self) -> Vec<&'static str> {
        vec![
            "原子间力: 原子之间的相互作用力",
            "范德华力: 原子间的弱相互作用",
            "化学键: 原子结合成分子的力",
            "离子键: 阴阳离子间的静电作用",
            "共价键: 电子共享的原子结合",
            "金属键: 金属原子间的电子公有",
            "氢键: 氢原子与电负性原子的作用",
            "原子散射: 原子对粒子的散射",
        ]
    }

    /// 应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "光谱分析",
            "激光技术",
            "原子钟",
            "核磁共振",
            "原子力显微镜",
            "原子操控",
            "量子计算",
            "原子物理教育",
        ]
    }
}

impl Default for AtomicPhysicsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AtomicPhysicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("atomic_physics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "原子物理规则",
            &[
                ("原子结构", &self.atomic_structure()),
                ("电子壳层", &self.electron_shell_rules()),
                ("原子光谱", &self.atomic_spectrum()),
                ("原子能级", &self.atomic_energy_levels()),
                ("原子辐射", &self.atomic_radiation()),
                ("原子模型", &self.atomic_models()),
                ("原子磁性质", &self.atomic_magnetic_properties()),
                ("原子相互作用", &self.atomic_interactions()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_physics_rules() {
        let rules = AtomicPhysicsRules::new();
        assert_eq!(rules.metadata().name, "原子物理规则");
        assert!(!rules.atomic_structure().is_empty());
        assert!(!rules.electron_shell_rules().is_empty());
        assert!(!rules.atomic_spectrum().is_empty());
        assert!(!rules.explain().is_empty());
    }
}
