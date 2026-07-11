//! 凝聚态物理详细规则
//!
//! 凝聚态物理学研究固体和液体等凝聚态物质的性质。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 凝聚态物理详细规则集合
pub struct CondensedMatterDetailedRules {
    metadata: RuleMetadata,
}

impl CondensedMatterDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("凝聚态物理详细规则", "凝聚态物理学基本概念和定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "凝聚态".into()]),
        }
    }

    /// 晶体结构规则
    pub fn crystal_structure(&self) -> Vec<&'static str> {
        vec![
            "晶体定义: 原子规则排列的固体",
            "晶格: 原子位置的空间排列",
            "晶胞: 晶格的最小重复单元",
            "布拉维格子: 14种基本晶格类型",
            "晶系: 7种基本晶体对称类型",
            "晶面: 晶体表面的特定平面",
            "米勒指数: 晶面的数字表示",
            "缺陷: 晶格中的不规则性",
        ]
    }

    /// 能带理论
    pub fn band_theory(&self) -> Vec<&'static str> {
        vec![
            "能带形成: 原子轨道重叠形成能带",
            "价带: 价电子占据的能带",
            "导带: 电子可自由运动的能带",
            "禁带: 价带和导带之间的能量间隙",
            "费米能级: 电子填充的最高能量",
            "金属: 导带和价带重叠或部分填充",
            "半导体: 禁带较小(约1-2eV)",
            "绝缘体: 禁带较大(>5eV)",
        ]
    }

    /// 电子输运规则
    pub fn electronic_transport(&self) -> Vec<&'static str> {
        vec![
            "电导率: σ=ne²τ/m",
            "电阻率: ρ=1/σ",
            "迁移率: μ=v/E，电子在电场中的漂移速度",
            "散射机制: 电子被杂质、晶格振动散射",
            "费米面: 费米能级处的等能面",
            "有效质量: 电子在晶体中的等效质量",
            "霍尔效应: 磁场中横向电压产生",
            "量子霍尔效应: 低维系统的量子化霍尔电阻",
        ]
    }

    /// 超导规则
    pub fn superconductivity(&self) -> Vec<&'static str> {
        vec![
            "超导定义: 零电阻和完全抗磁性",
            "临界温度Tc: 进入超导态的温度",
            "迈斯纳效应: 超导体排斥磁场",
            "临界磁场Hc: 破坏超导的磁场强度",
            "临界电流Jc: 破坏超导的电流密度",
            "BCS理论: 电子形成库珀对的理论",
            "库珀对: 两电子通过声子耦合",
            "能隙: 超导体中激发态与基态的能量差",
        ]
    }

    /// 磁性规则
    pub fn magnetism_rules(&self) -> Vec<&'static str> {
        vec![
            "抗磁性: 材料被磁场排斥",
            "顺磁性: 材料被磁场吸引但无永久磁矩",
            "铁磁性: 材料有永久磁矩且可自发磁化",
            "反铁磁性: 相邻磁矩反向排列",
            "亚铁磁性: 两种磁矩反向排列但大小不等",
            "居里温度: 铁磁性消失的温度",
            "磁畴: 铁磁材料中磁矩一致的区域",
            "磁滞回线: 磁化强度与外磁场的关系曲线",
        ]
    }

    /// 半导体规则
    pub fn semiconductor_rules(&self) -> Vec<&'static str> {
        vec![
            "本征半导体: 净半导体材料",
            "掺杂半导体: 掺入杂质的半导体",
            "N型半导体: 掺入施主杂质，电子导电",
            "P型半导体: 掺入受主杂质，空穴导电",
            "载流子浓度: 导电电子或空穴的数目",
            "pn结: P型和N型半导体的界面",
            "能带弯曲: pn结附近的能带变化",
            "耗尽层: pn结附近载流子耗尽区域",
        ]
    }

    /// 半导体器件规则
    pub fn semiconductor_devices(&self) -> Vec<&'static str> {
        vec![
            "二极管: 单向导电的pn结器件",
            "晶体管: 三端半导体放大器件",
            "MOSFET: 金属氧化物半导体场效应管",
            "集成电路: 多个晶体管集成在芯片上",
            "太阳能电池: 光生伏特效应器件",
            "LED: 发光二极管",
            "激光器: 半导体激光器",
            "存储器件: 半导体存储单元",
        ]
    }

    /// 低维系统规则
    pub fn low_dimensional_systems(&self) -> Vec<&'static str> {
        vec![
            "二维材料: 原子层厚度的材料如石墨烯",
            "量子阱: 电子在一维受限的势阱",
            "量子线: 电子在二维受限的线",
            "量子点: 电子在三维受限的点",
            "石墨烯: 单层碳原子二维材料",
            "拓扑绝缘体: 表面导电内部绝缘",
            "量子限域效应: 低维系统能级离散",
            "量子霍尔效应: 二维系统量子化霍尔电阻",
        ]
    }

    /// 应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "半导体芯片",
            "电子器件",
            "超导应用",
            "磁性材料",
            "纳米材料",
            "量子计算",
            "传感器技术",
            "显示技术",
        ]
    }
}

impl Default for CondensedMatterDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CondensedMatterDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("condensed_matter_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "凝聚态物理详细规则",
            &[
                ("晶体结构", &self.crystal_structure()),
                ("能带理论", &self.band_theory()),
                ("电子输运", &self.electronic_transport()),
                ("超导规则", &self.superconductivity()),
                ("磁性规则", &self.magnetism_rules()),
                ("半导体", &self.semiconductor_rules()),
                ("半导体器件", &self.semiconductor_devices()),
                ("低维系统", &self.low_dimensional_systems()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condensed_matter_detailed_rules() {
        let rules = CondensedMatterDetailedRules::new();
        assert_eq!(rules.metadata().name, "凝聚态物理详细规则");
        assert!(!rules.crystal_structure().is_empty());
        assert!(!rules.band_theory().is_empty());
        assert!(!rules.superconductivity().is_empty());
        assert!(!rules.semiconductor_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }
}
