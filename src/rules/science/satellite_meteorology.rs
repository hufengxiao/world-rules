//! 卫星气象学规则
//!
//! 卫星气象学研究气象卫星探测大气的方法，
//! 包括卫星遥感原理、卫星产品和卫星资料应用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 卫星气象学规则集合
pub struct SatelliteMeteorologyRules {
    metadata: RuleMetadata,
}

impl SatelliteMeteorologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("卫星气象学规则", "气象卫星探测和资料应用方法")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "气象".into(), "卫星".into()]),
        }
    }

    /// 卫星遥感原理规则
    pub fn satellite_remote_sensing_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("卫星轨道定律", "轨道特性", "气象卫星轨道类型分析"),
            ("卫星观测定律", "观测方式", "卫星观测方式分类"),
            ("遥感辐射定律", "辐射探测", "卫星遥感辐射测量原理"),
            ("遥感光谱定律", "光谱波段", "卫星遥感光谱波段设置"),
            ("遥感分辨率定律", "分辨率", "卫星遥感空间时间分辨率"),
            ("遥感覆盖定律", "覆盖范围", "卫星观测覆盖范围分析"),
            ("遥感传输定律", "数据传输", "卫星数据传输方式"),
            ("遥感定位定律", "定位方法", "卫星遥感定位技术"),
        ]
    }

    /// 卫星通道规则
    pub fn satellite_channel_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("可见光通道定律", "VIS通道", "可见光通道探测原理"),
            ("红外通道定律", "IR通道", "红外通道温度探测"),
            ("水汽通道定律", "WV通道", "水汽通道水汽探测"),
            ("短波红外定律", "SWIR", "短波红外通道应用"),
            ("长波红外定律", "LWIR", "长波红外通道分析"),
            ("多光谱定律", "多通道", "多光谱通道组合应用"),
            ("高光谱定律", "高光谱", "高光谱卫星探测"),
            ("通道选择定律", "通道优化", "卫星通道选择原则"),
        ]
    }

    /// 卫星云图规则
    pub fn satellite_cloud_image_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("可见光云图定律", "VIS云图", "可见光云图特征分析"),
            ("红外云图定律", "IR云图", "红外云图温度分析"),
            ("水汽云图定律", "WV云图", "水汽云图分析应用"),
            ("增强云图定律", "增强处理", "云图增强显示方法"),
            ("彩色云图定律", "彩色合成", "彩色云图合成技术"),
            ("动画云图定律", "动画显示", "云图动画连续显示"),
            ("云图识别定律", "云型识别", "卫星云图云型识别"),
            ("云图判读定律", "云图判读", "卫星云图判读方法"),
        ]
    }

    /// 卫星云导风规则
    pub fn satellite_wind_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("云导风原理定律", "云迹追踪", "云导风追踪原理方法"),
            ("云导风算法定律", "追踪算法", "云导风追踪算法类型"),
            ("云导风高度定律", "高度分配", "云导风高度确定方法"),
            ("云导风质量控制定律", "质量控制", "云导风质量控制方法"),
            ("云导风产品定律", "风场产品", "云导风产品类型分析"),
            ("云导风密度定律", "密度分布", "云导风密度分布特征"),
            ("云导风误差定律", "误差分析", "云导风误差来源分析"),
            ("云导风应用定律", "风场应用", "云导风资料应用领域"),
        ]
    }

    /// 卫星反演规则
    pub fn satellite_retrieval_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("温度反演定律", "温度剖面", "卫星反演温度垂直剖面"),
            ("湿度反演定律", "湿度剖面", "卫星反演湿度垂直分布"),
            ("臭氧反演定律", "臭氧浓度", "卫星反演臭氧总量"),
            ("气溶胶反演定律", "气溶胶浓度", "卫星反演气溶胶参数"),
            ("云参数反演定律", "云特性", "卫星反演云物理参数"),
            ("降水反演定律", "降水强度", "卫星反演降水强度"),
            ("海温反演定律", "SST反演", "卫星反演海表温度"),
            ("辐射反演定律", "辐射收支", "卫星反演辐射收支"),
        ]
    }

    /// 卫星监测规则
    pub fn satellite_monitoring_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("台风监测定律", "台风追踪", "卫星监测台风路径"),
            ("暴雨监测定律", "暴雨监测", "卫星监测暴雨云团"),
            ("云系监测定律", "云系追踪", "卫星追踪云系演变"),
            ("灾害监测定律", "灾害天气", "卫星监测灾害天气"),
            ("环境监测定律", "环境监测", "卫星监测大气环境"),
            ("沙尘监测定律", "沙尘天气", "卫星监测沙尘天气"),
            ("火灾监测定律", "火点监测", "卫星监测火灾火点"),
            ("洪水监测定律", "洪水范围", "卫星监测洪水范围"),
        ]
    }

    /// 卫星产品规则
    pub fn satellite_products_rules(&self) -> Vec<&'static str> {
        vec![
            "云图产品: 卫星云图产品类型和格式",
            "云导风产品: 云导风资料产品类型",
            "温度湿度产品: 反演温度湿度垂直剖面",
            "降水估计产品: 卫星降水估计产品",
            "云参数产品: 云物理参数产品类型",
            "臭氧产品: 卫星臭氧监测产品",
            "气溶胶产品: 卫星气溶胶监测产品",
            "辐射收支产品: 卫星辐射收支产品",
        ]
    }

    /// 卫星应用领域
    pub fn application_areas(&self) -> Vec<&'static str> {
        vec![
            "天气预报: 卫星资料应用于天气预报",
            "气候研究: 卫星资料在气候研究中应用",
            "灾害预警: 卫星资料灾害天气预警",
            "环境监测: 卫星资料环境监测应用",
            "数值模式: 卫星资料同化到数值模式",
            "海洋监测: 卫星资料海洋监测应用",
            "农业气象: 卫星资料农业气象应用",
            "科研应用: 卫星资料科研应用领域",
        ]
    }

    /// 卫星资料同化规则
    pub fn satellite_assimilation_rules(&self) -> Vec<&'static str> {
        vec![
            "辐射率同化: 卫星辐射率资料直接同化",
            "反演产品同化: 卫星反演产品同化方法",
            "云导风同化: 云导风资料同化技术",
            "质量控制: 卫星资料同化质量控制",
            "偏差订正: 卫星资料偏差订正方法",
            "同化算法: 卫星资料同化算法类型",
            "同化效果: 卫星资料同化效果评估",
            "同化系统: 卫星资料同化系统集成",
        ]
    }
}

impl Default for SatelliteMeteorologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SatelliteMeteorologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("satellite_meteorology")
    }

    fn explain(&self) -> String {
        format!(
            "【卫星气象学规则】\n\n\
            卫星遥感原理规则:\n{}\n\n\
            卫星通道规则:\n{}\n\n\
            卫星云图规则:\n{}\n\n\
            卫星云导风规则:\n{}\n\n\
            卫星反演规则:\n{}\n\n\
            卫星监测规则:\n{}\n\n\
            卫星产品规则:\n{}\n\n\
            卫星应用领域:\n{}\n\n\
            卫星资料同化规则:\n{}",
            self.satellite_remote_sensing_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.satellite_channel_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.satellite_cloud_image_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.satellite_wind_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.satellite_retrieval_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.satellite_monitoring_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.satellite_products_rules()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            self.application_areas()
                .iter()
                .map(|a| format!("  • {}", a))
                .collect::<Vec<_>>()
                .join("\n"),
            self.satellite_assimilation_rules()
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
    fn test_satellite_meteorology_rules() {
        let rules = SatelliteMeteorologyRules::new();
        assert_eq!(rules.satellite_remote_sensing_rules().len(), 8);
        assert_eq!(rules.satellite_channel_rules().len(), 8);
        assert_eq!(rules.satellite_cloud_image_rules().len(), 8);
        assert_eq!(rules.satellite_wind_rules().len(), 8);
        assert_eq!(rules.satellite_retrieval_rules().len(), 8);
        assert_eq!(rules.satellite_monitoring_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_channel_rules() {
        let rules = SatelliteMeteorologyRules::new();
        let laws = rules.satellite_channel_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("通道")));
    }

    #[test]
    fn test_cloud_image_rules() {
        let rules = SatelliteMeteorologyRules::new();
        assert_eq!(rules.satellite_cloud_image_rules().len(), 8);
    }

    #[test]
    fn test_products_rules() {
        let rules = SatelliteMeteorologyRules::new();
        assert_eq!(rules.satellite_products_rules().len(), 8);
    }
}