//! 航空气象学规则
//!
//! 航空气象学研究对航空活动有影响的大气现象，
//! 包括航空天气预报、飞行安全和机场气象保障。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 航空气象学规则集合
pub struct AviationMeteorologyRules {
    metadata: RuleMetadata,
}

impl AviationMeteorologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("航空气象学规则", "航空天气预报和飞行安全保障")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "气象".into(), "航空".into()]),
        }
    }

    /// 飞行气象保障规则
    pub fn flight_meteorology_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("起飞气象定律", "起飞天气", "影响飞机起飞的气象条件"),
            ("降落气象定律", "降落天气", "影响飞机降落的气象条件"),
            ("巡航气象定律", "巡航天气", "巡航高度气象条件分析"),
            ("飞行安全定律", "安全保障", "飞行气象安全保障方法"),
            ("航线气象定律", "航线天气", "航线气象预报分析"),
            ("备降机场定律", "备降选择", "备降机场气象条件评估"),
            ("飞行延误定律", "延误原因", "气象原因飞行延误分析"),
            ("飞行取消定律", "取消条件", "恶劣天气飞行取消判断"),
        ]
    }

    /// 机场气象规则
    pub fn airport_meteorology_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("机场风定律", "风场分析", "机场风对飞行的影响"),
            ("机场云定律", "云层分析", "机场云层高度分析"),
            ("机场能见度定律", "能见度", "机场能见度变化规律"),
            ("机场温度定律", "温度影响", "温度对机场运行的影响"),
            ("机场降水定律", "降水分析", "机场降水天气影响"),
            ("机场雷暴定律", "雷暴影响", "机场雷暴天气防护"),
            ("机场雾定律", "雾影响", "机场雾天运行规则"),
            ("机场综合气象定律", "综合分析", "机场综合气象评估"),
        ]
    }

    /// 航空危险天气规则
    pub fn aviation_hazard_weather_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("低能见度定律", "飞行危险", "低能见度飞行危险评估"),
            ("强风定律", "风切变", "强风和风切变危害分析"),
            ("雷暴定律", "雷暴影响", "雷暴天气飞行危险"),
            ("冰雹定律", "冰雹危害", "冰雹对飞行的影响"),
            ("湍流定律", "颠簸分析", "湍流引起飞机颠簸"),
            ("积冰定律", "机身积冰", "机身积冰危害分析"),
            ("闪电定律", "雷电危害", "雷电对飞行的危害"),
            ("沙尘暴定律", "沙尘影响", "沙尘暴飞行安全"),
        ]
    }

    /// 飞行颠簸规则
    pub fn turbulence_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("轻度颠簸定律", "轻微颠簸", "轻度颠簸强度分类"),
            ("中度颠簸定律", "中等颠簸", "中度颠簸影响评估"),
            ("严重颠簸定律", "剧烈颠簸", "严重颠簸危害分析"),
            ("对流颠簸定律", "对流颠簸", "对流引起的颠簸分析"),
            ("山地颠簸定律", "山地波", "山地波动颠簸分析"),
            ("晴空颠簸定律", "CAT", "晴空湍流颠簸分析"),
            ("低空颠簸定律", "边界层颠簸", "低空颠簸分布特征"),
            ("颠簸预报定律", "颠簸预测", "颠簸预报方法技术"),
        ]
    }

    /// 飞机积冰规则
    pub fn aircraft_icing_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("积冰形成定律", "积冰条件", "飞机积冰形成条件"),
            ("积冰类型定律", "积冰分类", "明冰毛冰混合冰分类"),
            ("积冰强度定律", "强度分级", "积冰强度分级标准"),
            ("积冰探测定律", "积冰探测", "飞机积冰探测方法"),
            ("积冰防护定律", "防护措施", "积冰防护技术方法"),
            ("积冰影响定律", "性能影响", "积冰对飞行性能影响"),
            ("积冰预报定律", "积冰预测", "积冰预报方法技术"),
            ("积冰应急定律", "应急处理", "积冰应急处理措施"),
        ]
    }

    /// 风切变规则
    pub fn wind_shear_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("风切变形成定律", "切变条件", "风切变形成机制分析"),
            ("低空风切变定律", "LLWS", "低空风切变危害分析"),
            ("垂直风切变定律", "垂直切变", "垂直风切变特征"),
            ("水平风切变定律", "水平切变", "水平风切变分布"),
            ("风切变探测定律", "探测技术", "风切变探测方法"),
            ("风切变预警定律", "预警方法", "风切变预警技术"),
            ("风切变应对定律", "应对措施", "风切变飞行应对"),
            ("风切变灾害定律", "灾害评估", "风切变灾害影响"),
        ]
    }

    /// 航空气象预报规则
    pub fn aviation_forecast_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("机场预报定律", "TAF预报", "机场终端预报TAF格式"),
            ("航路预报定律", "航路天气", "航路天气预报方法"),
            ("区域预报定律", "区域分析", "区域航空气象预报"),
            ("高空风预报定律", "高空风场", "高空风预报方法"),
            ("高空温度预报定律", "高空温度", "高空温度预报"),
            ("颠簸预报定律", "颠簸预报", "颠簸预报技术方法"),
            ("积冰预报定律", "积冰预报", "积冰预报方法技术"),
            ("危险天气预报定律", "危险预报", "危险天气预报方法"),
        ]
    }

    /// 航空气象观测规则
    pub fn aviation_observation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("机场观测定律", "METAR", "机场例行气象报告METAR"),
            ("特殊观测定律", "SPECI", "机场特殊气象报告SPECI"),
            ("风观测定律", "风测量", "机场风向风速观测方法"),
            ("云观测定律", "云观测", "云量云高观测方法"),
            ("能见度观测定律", "RVR", "跑道视程RVR观测"),
            ("温度观测定律", "温度测量", "温度湿度观测方法"),
            ("气压观测定律", "气压测量", "机场气压观测方法"),
            ("降水观测定律", "降水测量", "降水强度观测方法"),
        ]
    }

    /// 航空气象产品规则
    pub fn aviation_products_rules(&self) -> Vec<&'static str> {
        vec![
            "METAR报文: 机场例行气象观测报文格式",
            "SPECI报文: 机场特殊气象观测报文",
            "TAF报文: 机场终端天气预报报文格式",
            "SIGMET报文: 重要气象情报报文",
            "AIRMET报文: 航空气象情报报文",
            "GAMET报文: 区域航空气象报文",
            "TAF趋势预报: TAF趋势预报分析",
            "火山灰报文: 火山灰情报报文",
        ]
    }

    /// 航空气象服务规则
    pub fn aviation_services_rules(&self) -> Vec<&'static str> {
        vec![
            "飞行情报服务: 飞行气象情报服务提供",
            "机场气象服务: 机场气象观测预报服务",
            "航路气象服务: 航路气象预报服务",
            "危险天气服务: 危险天气预警服务",
            "飞行计划服务: 飞行气象计划服务",
            "气象咨询: 飞行气象咨询服务",
            "气象 briefing: 飞行前气象 briefing",
            "气象情报交换: 航空气象情报交换",
        ]
    }
}

impl Default for AviationMeteorologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AviationMeteorologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("aviation_meteorology")
    }

    fn explain(&self) -> String {
        format!(
            "【航空气象学规则】\n\n\
            飞行气象保障规则:\n{}\n\n\
            机场气象规则:\n{}\n\n\
            航空危险天气规则:\n{}\n\n\
            飞行颠簸规则:\n{}\n\n\
            飞机积冰规则:\n{}\n\n\
            风切变规则:\n{}\n\n\
            航空气象预报规则:\n{}\n\n\
            航空气象观测规则:\n{}\n\n\
            航空气象产品规则:\n{}\n\n\
            航空气象服务规则:\n{}",
            self.flight_meteorology_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.airport_meteorology_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aviation_hazard_weather_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.turbulence_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aircraft_icing_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.wind_shear_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aviation_forecast_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aviation_observation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aviation_products_rules()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aviation_services_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aviation_meteorology_rules() {
        let rules = AviationMeteorologyRules::new();
        assert_eq!(rules.flight_meteorology_rules().len(), 8);
        assert_eq!(rules.airport_meteorology_rules().len(), 8);
        assert_eq!(rules.aviation_hazard_weather_rules().len(), 8);
        assert_eq!(rules.turbulence_rules().len(), 8);
        assert_eq!(rules.aircraft_icing_rules().len(), 8);
        assert_eq!(rules.wind_shear_rules().len(), 8);
        assert_eq!(rules.aviation_forecast_rules().len(), 8);
        assert_eq!(rules.aviation_observation_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_turbulence_rules() {
        let rules = AviationMeteorologyRules::new();
        let laws = rules.turbulence_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("颠簸")));
    }

    #[test]
    fn test_icing_rules() {
        let rules = AviationMeteorologyRules::new();
        assert_eq!(rules.aircraft_icing_rules().len(), 8);
    }

    #[test]
    fn test_products_rules() {
        let rules = AviationMeteorologyRules::new();
        assert_eq!(rules.aviation_products_rules().len(), 8);
    }
}