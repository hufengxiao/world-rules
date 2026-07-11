//! 宇宙学详细规则
//!
//! 宇宙学研究宇宙的整体结构、起源和演化。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 宇宙学详细规则集合
pub struct CosmologyDetailedRules {
    metadata: RuleMetadata,
}

impl CosmologyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("宇宙学详细规则", "宇宙学基本概念和定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "宇宙学".into()]),
        }
    }

    /// 宇宙膨胀规则
    pub fn cosmic_expansion(&self) -> Vec<&'static str> {
        vec![
            "哈勃定律: v = H₀d 星系退移速度与距离成正比",
            "哈勃常数: H₀ ≈ 70 km/s/Mpc",
            "宇宙膨胀: 空间本身在膨胀",
            "退移速度: 星系因宇宙膨胀远离我们",
            "宇宙红移: 光因宇宙膨胀波长变长",
            "距离测量: 用红移估算星系距离",
            "膨胀历史: 宇宙膨胀率随时间变化",
            "加速膨胀: 宇宙膨胀在加速",
        ]
    }

    /// 宇宙起源规则
    pub fn cosmic_origin(&self) -> Vec<&'static str> {
        vec![
            "大爆炸: 宇宙从一个极热极密状态演化",
            "大爆炸时间: 约138亿年前",
            "初始状态: 极高温度和密度",
            "暴胀阶段: 宇宙极早期快速膨胀",
            "重子产生: 物质和反物质不对称产生",
            "核合成: 早期宇宙合成轻元素",
            "原子形成: 电子与原子核结合",
            "结构形成: 物质凝聚形成结构",
        ]
    }

    /// 宇宙微波背景规则
    pub fn cosmic_microwave_background(&self) -> Vec<&'static str> {
        vec![
            "CMB定义: 大爆炸遗留的热辐射",
            "CMB温度: 约2.7K均匀分布",
            "黑体辐射谱: 完美黑体辐射",
            "CMB形成: 原子形成时光子自由传播",
            "红移效应: 光子波长被宇宙膨胀拉长",
            "温度涨落: 微小温度差异反映早期密度差异",
            "CMB观测: 揭示宇宙早期信息",
            "功率谱: 涨落在不同尺度上的分布",
        ]
    }

    /// 暗物质规则
    pub fn dark_matter_rules(&self) -> Vec<&'static str> {
        vec![
            "暗物质定义: 不发光但通过引力作用的物质",
            "暗物质占比: 约占宇宙物质总量的85%",
            "暗物质证据: 星系旋转曲线引力透镜",
            "星系旋转曲线: 星系外围恒星速度比预期快",
            "引力透镜: 暗物质弯曲光线",
            "暗物质候选: WIMPs轴子等",
            "暗物质探测: 直接探测间接探测",
            "暗物质晕: 星系周围暗物质分布",
        ]
    }

    /// 暗能量规则
    pub fn dark_energy_rules(&self) -> Vec<&'static str> {
        vec![
            "暗能量定义: 导致宇宙加速膨胀的能量",
            "暗能量占比: 约占宇宙总能量的70%",
            "加速膨胀证据: 远星系亮度变暗",
            "宇宙常数: Λ Einstein引入的概念",
            "真空能量: 量子真空可能产生暗能量",
            "能量密度: 暗能量密度基本不变",
            "状态方程: w ≈ -1",
            "未来演化: 暗能量决定宇宙未来",
        ]
    }

    /// 宇宙结构规则
    pub fn cosmic_structure(&self) -> Vec<&'static str> {
        vec![
            "宇宙结构: 宇宙中物质分布的结构",
            "星系: 基本的宇宙结构单元",
            "星系团: 多个星系引力束缚的集合",
            "超星系团: 更大的星系团集合",
            "宇宙纤维: 大尺度结构呈纤维状",
            "宇宙空洞: 纤维之间的物质稀疏区域",
            "结构形成: 重子物质在暗物质晕中聚集",
            "结构演化: 宇宙结构随时间演化",
        ]
    }

    /// 宇宙学常数
    pub fn cosmological_constants(&self) -> Vec<&'static str> {
        vec![
            "哈勃常数: H₀ ≈ 70 km/s/Mpc",
            "宇宙年龄: 约138亿年",
            "可观测宇宙半径: 约460亿光年",
            "临界密度: ρc = 3H₀²/(8πG)",
            "密度参数: Ω = ρ/ρc",
            "暗物质参数: Ωm ≈ 0.3",
            "暗能量参数: ΩΛ ≈ 0.7",
            "重子参数: Ωb ≈ 0.05",
        ]
    }

    /// 宇宙演化阶段
    pub fn cosmic_evolution_stages(&self) -> Vec<&'static str> {
        vec![
            "普朗克时期: t < 10⁻⁴³s 极高温",
            "暴胀时期: t ≈ 10⁻³⁶s 快速膨胀",
            "夸克时期: t < 10⁻⁶s 夸克自由",
            "核合成时期: t ≈ 3min 元素合成",
            "物质主导: t > 47000年",
            "原子形成: t ≈ 380000年",
            "结构形成: t > 100Myr",
            "暗能量主导: t > 9.8Gyr",
        ]
    }

    /// 应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "宇宙观测",
            "宇宙模型构建",
            "宇宙演化研究",
            "暗物质探测",
            "暗能量研究",
            "宇宙大尺度结构",
            "引力波天文学",
            "基础物理检验",
        ]
    }
}

impl Default for CosmologyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CosmologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("cosmology_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "宇宙学详细规则",
            &[
                ("宇宙膨胀", &self.cosmic_expansion()),
                ("宇宙起源", &self.cosmic_origin()),
                ("宇宙微波背景", &self.cosmic_microwave_background()),
                ("暗物质", &self.dark_matter_rules()),
                ("暗能量", &self.dark_energy_rules()),
                ("宇宙结构", &self.cosmic_structure()),
                ("宇宙学常数", &self.cosmological_constants()),
                ("宇宙演化阶段", &self.cosmic_evolution_stages()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosmology_detailed_rules() {
        let rules = CosmologyDetailedRules::new();
        assert_eq!(rules.metadata().name, "宇宙学详细规则");
        assert!(!rules.cosmic_expansion().is_empty());
        assert!(!rules.dark_matter_rules().is_empty());
        assert!(!rules.dark_energy_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }
}
