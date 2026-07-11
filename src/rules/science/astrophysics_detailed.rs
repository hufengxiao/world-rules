//! 天体物理详细规则
//!
//! 天体物理学研究天体的物理性质和演化过程。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 天体物理详细规则集合
pub struct AstrophysicsDetailedRules {
    metadata: RuleMetadata,
}

impl AstrophysicsDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("天体物理详细规则", "天体物理学基本概念和定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "天体物理".into()]),
        }
    }

    /// 恒星结构规则
    pub fn stellar_structure(&self) -> Vec<&'static str> {
        vec![
            "恒星定义: 通过核聚变产生能量的天体",
            "恒星质量: 决定恒星演化路径的关键参数",
            "恒星半径: 从中心到表面的距离",
            "恒星温度: 核心可达数百万度表面几千度",
            "恒星密度: 核心密度极高表面密度低",
            "压力平衡: 重力和内部压力平衡",
            "辐射传输: 能量从核心传递到表面",
            "恒星大气: 恒星最外层的气体层",
        ]
    }

    /// 恒星演化规则
    pub fn stellar_evolution(&self) -> Vec<&'static str> {
        vec![
            "恒星形成: 气体云坍缩形成原恒星",
            "主序阶段: 恒星在赫罗图上的主序带",
            "氢燃烧: 主序恒星核聚变氢为氦",
            "氦燃烧: 恒星核心氢耗尽后燃烧氦",
            "红巨星: 氦燃烧阶段的恒星膨胀",
            "超巨星: 更晚演化阶段的巨大恒星",
            "恒星死亡: 最终形成白矮星中子星或黑洞",
            "质量决定: 恒星质量决定最终命运",
        ]
    }

    /// 恒星残骸规则
    pub fn stellar_remnants(&self) -> Vec<&'static str> {
        vec![
            "白矮星: 小质量恒星残骸电子简态支撑",
            "白矮星质量: 最大1.44太阳质量(钱德拉塞卡极限)",
            "白矮星半径: 约地球大小密度极高",
            "中子星: 更大质量恒星残骸中子简态支撑",
            "中子星质量: 1.4-3太阳质量",
            "脉冲星: 旋转中子星发出周期性脉冲",
            "黑洞: 超大质量恒星残骸引力极强",
            "黑洞特性: 事件视界内光也无法逃逸",
        ]
    }

    /// 恒星系统规则
    pub fn stellar_systems(&self) -> Vec<&'static str> {
        vec![
            "双星系统: 两颗恒星引力束缚",
            "双星轨道: 两颗恒星绕公共中心运动",
            "食变星: 双星相互遮挡光度变化",
            "星团: 多颗恒星引力束缚的集合",
            "球状星团: 密集球形分布的星团",
            "疏散星团: 松散分布的年轻星团",
            "星系: 大量恒星组成的系统",
            "星系类型: 椭圆星系螺旋星系不规则星系",
        ]
    }

    /// 赫罗图规则
    pub fn hertzsprung_russell_diagram(&self) -> Vec<&'static str> {
        vec![
            "赫罗图定义: 恒星光度与温度的关系图",
            "主序带: 大多数恒星在主序带上",
            "光度: 恒星辐射功率的度量",
            "光谱型: O B A F G K M温度递减",
            "红巨星支: 红巨星在图上的分布",
            "白矮星区: 白矮星在图上的分布",
            "演化轨迹: 恒星演化时在图上移动",
            "应用: 确定恒星距离和年龄",
        ]
    }

    /// 核聚变规则
    pub fn nuclear_fusion_rules(&self) -> Vec<&'static str> {
        vec![
            "聚变条件: 高温高密度才能发生聚变",
            "质子-质子链: 太阳等小质量恒星主反应",
            "PP链步骤: 4H → ⁴He + 能量 + 正电子 + 中微子",
            "CNO循环: 大质量恒星中碳氮氧催化聚变",
            "氦燃烧: 三个氦原子合成一个碳原子",
            "后续燃烧: 碳氧硅等元素逐级聚变",
            "能量产生: 聚变释放巨大能量",
            "元素合成: 聚变合成比铁轻的元素",
        ]
    }

    /// 星际介质规则
    pub fn interstellar_medium(&self) -> Vec<&'static str> {
        vec![
            "星际介质: 星系中恒星之间的物质",
            "星际气体: 主要为氢气约占90%",
            "星际尘埃: 微小固体颗粒",
            "分子云: 高密度区域可形成恒星",
            "星际磁场: 星际介质中的磁场",
            "星际辐射: 宇宙射线和光子",
            "星际吸收: 气体和尘埃吸收星光",
            "星际红化: 尘埃散射蓝光使星光偏红",
        ]
    }

    /// 应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "恒星观测",
            "星系研究",
            "宇宙演化",
            "天文导航",
            "时间标准",
            "引力波探测",
            "宇宙射线研究",
            "暗物质探测",
        ]
    }
}

impl Default for AstrophysicsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AstrophysicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("astrophysics_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "天体物理详细规则",
            &[
                ("恒星结构", &self.stellar_structure()),
                ("恒星演化", &self.stellar_evolution()),
                ("恒星残骸", &self.stellar_remnants()),
                ("恒星系统", &self.stellar_systems()),
                ("赫罗图", &self.hertzsprung_russell_diagram()),
                ("核聚变", &self.nuclear_fusion_rules()),
                ("星际介质", &self.interstellar_medium()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astrophysics_detailed_rules() {
        let rules = AstrophysicsDetailedRules::new();
        assert_eq!(rules.metadata().name, "天体物理详细规则");
        assert!(!rules.stellar_structure().is_empty());
        assert!(!rules.stellar_evolution().is_empty());
        assert!(!rules.stellar_remnants().is_empty());
        assert!(!rules.explain().is_empty());
    }
}
