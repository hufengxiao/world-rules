//! 物理气象学规则
//!
//! 物理气象学研究大气中的物理过程和现象，
//! 包括大气辐射、云物理学、大气光学和大气电学。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 物理气象学规则集合
pub struct PhysicalMeteorologyRules {
    metadata: RuleMetadata,
}

impl PhysicalMeteorologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("物理气象学规则", "大气物理过程和现象")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "气象".into(), "物理".into()]),
        }
    }

    /// 大气辐射规则
    pub fn atmospheric_radiation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("太阳辐射定律", "太阳能量", "太阳辐射在大气中传输"),
            ("地球辐射定律", "地面发射", "地面发射的长波辐射"),
            ("辐射平衡定律", "收支平衡", "辐射收支平衡方程"),
            ("辐射传输定律", "传输方程", "辐射传输方程求解"),
            ("散射定律", "散射过程", "大气散射机制分析"),
            ("吸收定律", "吸收过程", "大气吸收光谱特征"),
            ("反射定律", "反射特性", "大气和地面反射特性"),
            ("温室效应定律", "辐射强迫", "温室气体辐射强迫"),
        ]
    }

    /// 云物理学规则
    pub fn cloud_physics_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("云凝结核定律", "CCN作用", "云凝结核活化过程"),
            ("云滴形成定律", "凝结增长", "云滴凝结增长过程"),
            ("冰核形成定律", "冰晶核化", "冰核核化形成机制"),
            ("冰晶增长定律", "凝华增长", "冰晶凝华增长过程"),
            ("云滴碰并定律", "碰并过程", "云滴碰并增长机制"),
            ("降水形成定律", "降水生成", "暖云和冷云降水机制"),
            ("云微结构定律", "微物理特征", "云微物理结构分析"),
            ("云液水含量定律", "LWC分布", "云液水含量分布"),
        ]
    }

    /// 大气光学规则
    pub fn atmospheric_optics_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("折射定律", "光线弯曲", "大气折射现象分析"),
            ("散射定律", "光线散射", "大气散射产生蓝天"),
            ("衍射定律", "光波衍射", "大气衍射现象特征"),
            ("虹定律", "彩虹形成", "彩虹的形成机制分析"),
            ("晕定律", "冰晶晕", "晕的形成和类型分析"),
            ("幻日定律", "太阳幻影", "幻日现象形成机制"),
            ("晨昏蒙影定律", "曙暮光", "晨昏蒙影时间计算"),
            ("闪烁定律", "星光闪烁", "大气闪烁现象分析"),
        ]
    }

    /// 大气电学规则
    pub fn atmospheric_electrical_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("大气电场定律", "电场分布", "大气电场分布特征"),
            ("雷暴起电定律", "电荷分离", "雷暴云内电荷分离"),
            ("闪电形成定律", "放电过程", "闪电放电形成机制"),
            ("闪电类型定律", "闪电分类", "地闪和云闪分类"),
            ("雷声定律", "声波传播", "雷声产生和传播"),
            ("大气离子定律", "离子浓度", "大气离子浓度分布"),
            ("大气导电定律", "导电特性", "大气导电率分析"),
            ("闪电定位定律", "定位技术", "闪电定位方法技术"),
        ]
    }

    /// 大气声学规则
    pub fn atmospheric_acoustics_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("声波传播定律", "声波传输", "声波在大气中传播"),
            ("声速定律", "传播速度", "声速随温度变化"),
            ("声折射定律", "声波弯曲", "声波折射现象分析"),
            ("声吸收定律", "能量衰减", "声波在大气中衰减"),
            ("声散射定律", "散射效应", "声波散射现象"),
            ("声反射定律", "反射特性", "声波在大气中反射"),
            ("次声波定律", "低频声波", "次声波传播特征"),
            ("声定位定律", "声源定位", "声源定位方法技术"),
        ]
    }

    /// 大气化学规则
    pub fn atmospheric_chemistry_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("大气成分定律", "气体组成", "大气主要成分分析"),
            ("光化学反应定律", "光解过程", "大气光化学反应"),
            ("臭氧形成定律", "臭氧生成", "臭氧形成和消耗机制"),
            ("污染物扩散定律", "扩散过程", "大气污染物扩散"),
            ("酸雨形成定律", "酸性降水", "酸雨形成化学反应"),
            ("气溶胶形成定律", "颗粒生成", "气溶胶形成机制"),
            ("大气氧化定律", "氧化过程", "大气氧化能力分析"),
            ("大气还原定律", "还原过程", "大气还原反应分析"),
        ]
    }

    /// 大气热力学规则
    pub fn atmospheric_thermodynamics_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热力学第一定律", "能量守恒", "大气能量守恒原理"),
            ("热力学第二定律", "熵增原理", "大气熵变化过程"),
            ("状态方程定律", "气体状态", "理想气体状态方程"),
            ("绝热过程定律", "绝热变化", "大气绝热过程分析"),
            ("位温定律", "位温概念", "大气位温计算分析"),
            ("湿空气定律", "湿空气特性", "湿空气热力学特性"),
            ("凝结潜热定律", "潜热释放", "凝结潜热释放过程"),
            ("蒸发定律", "蒸发过程", "大气蒸发能量过程"),
        ]
    }

    /// 气溶胶物理学规则
    pub fn aerosol_physics_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("气溶胶形成定律", "生成机制", "气溶胶形成来源"),
            ("气溶胶尺度定律", "粒径分布", "气溶胶粒径分布特征"),
            ("气溶胶光学定律", "光学效应", "气溶胶光学特性分析"),
            ("气溶胶辐射定律", "辐射强迫", "气溶胶辐射强迫效应"),
            ("气溶胶云作用定律", "云微物理", "气溶胶对云的影响"),
            ("气溶胶输送定律", "传输过程", "气溶胶输送扩散"),
            ("气溶胶清除定律", "清除机制", "气溶胶清除过程"),
            ("气溶胶气候定律", "气候效应", "气溶胶气候影响"),
        ]
    }

    /// 大气扩散规则
    pub fn atmospheric_diffusion_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("扩散方程定律", "扩散求解", "大气扩散方程求解"),
            ("湍流扩散定律", "湍流输送", "湍流扩散系数计算"),
            ("扩散稳定度定律", "稳定度分类", "大气稳定度分类"),
            ("扩散风速定律", "风速影响", "风速对扩散的影响"),
            ("扩散地形定律", "地形效应", "地形对扩散的影响"),
            ("扩散源强定律", "源强计算", "污染源强计算方法"),
            ("扩散模式定律", "模式应用", "大气扩散模式类型"),
            ("扩散预测定律", "浓度预报", "污染物浓度预测"),
        ]
    }

    /// 物理研究方法
    pub fn research_methods(&self) -> Vec<&'static str> {
        vec![
            "观测分析: 大气物理要素观测和分析",
            "数值模拟: 大气物理过程数值模拟",
            "实验研究: 实验室大气物理实验",
            "理论推导: 大气物理理论推导分析",
            "参数化: 大气物理过程参数化方法",
            "遥感探测: 遥感方法探测大气物理量",
            "统计分析: 统计方法分析大气物理",
            "能量分析: 大气能量收支分析方法",
        ]
    }

    /// 物理应用领域
    pub fn application_areas(&self) -> Vec<&'static str> {
        vec![
            "天气预报: 大气物理应用于天气预报",
            "气候研究: 大气物理在气候研究中应用",
            "环境监测: 大气物理在环境监测中应用",
            "航空安全: 大气物理保障航空安全",
            "辐射防护: 大气辐射研究应用防护",
            "污染防治: 大气物理应用于污染防治",
            "气候变化: 大气物理研究气候变化",
            "云降水研究: 云物理学应用于降水预报",
        ]
    }
}

impl Default for PhysicalMeteorologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PhysicalMeteorologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("physical_meteorology")
    }

    fn explain(&self) -> String {
        format!(
            "【物理气象学规则】\n\n\
            大气辐射规则:\n{}\n\n\
            云物理学规则:\n{}\n\n\
            大气光学规则:\n{}\n\n\
            大气电学规则:\n{}\n\n\
            大气声学规则:\n{}\n\n\
            大气化学规则:\n{}\n\n\
            大气热力学规则:\n{}\n\n\
            气溶胶物理学规则:\n{}\n\n\
            大气扩散规则:\n{}\n\n\
            物理研究方法:\n{}\n\n\
            物理应用领域:\n{}",
            self.atmospheric_radiation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cloud_physics_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_optics_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_electrical_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_acoustics_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_chemistry_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_thermodynamics_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aerosol_physics_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_diffusion_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.research_methods()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n"),
            self.application_areas()
                .iter()
                .map(|a| format!("  • {}", a))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_meteorology_rules() {
        let rules = PhysicalMeteorologyRules::new();
        assert_eq!(rules.atmospheric_radiation_rules().len(), 8);
        assert_eq!(rules.cloud_physics_rules().len(), 8);
        assert_eq!(rules.atmospheric_optics_rules().len(), 8);
        assert_eq!(rules.atmospheric_electrical_rules().len(), 8);
        assert_eq!(rules.atmospheric_acoustics_rules().len(), 8);
        assert_eq!(rules.atmospheric_chemistry_rules().len(), 8);
        assert_eq!(rules.atmospheric_thermodynamics_rules().len(), 8);
        assert_eq!(rules.aerosol_physics_rules().len(), 8);
        assert_eq!(rules.atmospheric_diffusion_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_radiation_rules() {
        let rules = PhysicalMeteorologyRules::new();
        let laws = rules.atmospheric_radiation_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("辐射")));
    }

    #[test]
    fn test_cloud_physics_rules() {
        let rules = PhysicalMeteorologyRules::new();
        assert_eq!(rules.cloud_physics_rules().len(), 8);
    }

    #[test]
    fn test_research_methods() {
        let rules = PhysicalMeteorologyRules::new();
        assert_eq!(rules.research_methods().len(), 8);
    }
}