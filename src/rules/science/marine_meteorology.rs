//! 海洋气象学规则
//!
//! 海洋气象学研究海洋与大气的相互作用，
//! 包括海气交换、海洋天气系统和海上气象预报。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 海洋气象学规则集合
pub struct MarineMeteorologyRules {
    metadata: RuleMetadata,
}

impl MarineMeteorologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("海洋气象学规则", "海洋大气相互作用和海上天气预报")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "气象".into(), "海洋".into()]),
        }
    }

    /// 海气相互作用规则
    pub fn air_sea_interaction_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热量交换定律", "能量交换", "海洋与大气热量交换"),
            ("水汽交换定律", "蒸发凝结", "海洋蒸发和大气凝结"),
            ("动量交换定律", "风应力", "大气对海洋的风应力"),
            ("气体交换定律", "CO₂交换", "海洋大气CO₂交换"),
            ("海气反馈定律", "相互作用", "海洋大气反馈机制"),
            ("海表温度定律", "SST影响", "海表温度对大气的影响"),
            ("海洋热容量定律", "热储存", "海洋热容量调节气候"),
            ("海气耦合定律", "耦合过程", "海气耦合模式分析"),
        ]
    }

    /// 海洋天气系统规则
    pub fn marine_weather_systems_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("海上气旋定律", "海洋气旋", "海上气旋形成发展"),
            ("海上锋面定律", "海洋锋", "海上锋面系统分析"),
            ("海上对流定律", "海洋对流", "海上对流活动特征"),
            ("海洋风暴定律", "风暴系统", "海洋风暴系统演变"),
            ("海上雾定律", "海雾", "海雾形成和维持机制"),
            ("海上大风定律", "强风系统", "海上大风形成条件"),
            ("海上降水定律", "海洋降水", "海上降水分布特征"),
            ("海上波浪定律", "风浪", "风浪形成和演变"),
        ]
    }

    /// 海洋气象观测规则
    pub fn marine_observation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("船舶观测定律", "船测资料", "船舶气象观测方法"),
            ("浮标观测定律", "浮标站", "海洋浮标气象观测"),
            ("岛屿观测定律", "岛站资料", "岛屿气象站观测"),
            ("海洋平台观测定律", "平台观测", "海上平台气象观测"),
            ("卫星海面观测定律", "卫星遥感", "卫星遥感海面观测"),
            ("海洋雷达观测定律", "雷达探测", "海洋气象雷达探测"),
            ("海洋探空定律", "海洋探空", "海洋上空探空观测"),
            ("海洋观测质量控制定律", "质量控制", "海洋观测质量控制"),
        ]
    }

    /// 海洋气象预报规则
    pub fn marine_forecast_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("海上风预报定律", "风速预报", "海上风速预报方法"),
            ("海浪预报定律", "波浪预报", "海浪高度预报技术"),
            ("海雾预报定律", "雾预报", "海雾形成预报方法"),
            ("海洋温度预报定律", "SST预报", "海表温度预报技术"),
            ("海洋风暴预报定律", "风暴预报", "海洋风暴路径预报"),
            ("海洋对流预报定律", "对流预报", "海上对流活动预报"),
            ("海洋航线预报定律", "航线预报", "航运气象航线预报"),
            ("海洋渔业预报定律", "渔业气象", "渔业气象预报服务"),
        ]
    }

    /// 海洋气象灾害规则
    pub fn marine_disaster_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("海上台风定律", "台风灾害", "台风对海上作业影响"),
            ("海上风暴潮定律", "风暴潮", "风暴潮灾害预警"),
            ("海上巨浪定律", "巨浪灾害", "海上巨浪灾害评估"),
            ("海雾灾害定律", "雾灾害", "海雾影响海上航行"),
            ("海上大风灾害定律", "风害", "海上大风灾害影响"),
            ("海上冰灾定律", "海冰灾害", "海冰灾害评估"),
            ("海上雷暴定律", "雷电灾害", "海上雷电灾害防护"),
            ("海上综合灾害定律", "复合灾害", "海上复合灾害评估"),
        ]
    }

    /// 航运气象规则
    pub fn shipping_meteorology_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("航线气象定律", "航线天气", "船舶航线气象保障"),
            ("港口气象定律", "港口天气", "港口气象预报服务"),
            ("航道气象定律", "航道天气", "航道气象保障方法"),
            ("锚地气象定律", "锚泊天气", "锚地气象预报分析"),
            ("船舶定线定律", "气象定线", "船舶气象定线技术"),
            ("船舶避风定律", "避风港口", "船舶避风决策方法"),
            ("船舶安全定律", "安全气象", "船舶安全气象保障"),
            ("航运效率定律", "气象优化", "气象优化航运效率"),
        ]
    }

    /// 渔业气象规则
    pub fn fishing_meteorology_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("渔场气象定律", "渔场天气", "渔场气象预报分析"),
            ("渔业安全定律", "安全保障", "渔业气象安全保障"),
            ("渔汛气象定律", "渔汛预报", "渔汛气象预报技术"),
            ("渔船气象定律", "渔船天气", "渔船气象保障方法"),
            ("养殖气象定律", "养殖天气", "水产养殖气象服务"),
            ("渔业灾害定律", "灾害预警", "渔业气象灾害预警"),
            ("渔期预报定律", "渔期预报", "渔期气象预报方法"),
            ("渔业资源定律", "资源分布", "气象与渔业资源关系"),
        ]
    }

    /// 海洋气象要素规则
    pub fn marine_elements_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("海面风定律", "海风分布", "海面风分布特征"),
            ("海面温度定律", "海温分布", "海表温度分布规律"),
            ("海面湿度定律", "湿度分布", "海面湿度分布特征"),
            ("海面气压定律", "气压分布", "海面气压分布规律"),
            ("海面降水定律", "降水分布", "海上降水分布"),
            ("海面云定律", "云分布", "海上云分布特征"),
            ("海面能见度定律", "能见度", "海上能见度变化"),
            ("海面波浪定律", "波浪特征", "海浪分布变化规律"),
        ]
    }

    /// 海洋气象监测规则
    pub fn marine_monitoring_rules(&self) -> Vec<&'static str> {
        vec![
            "海面风监测: 海面风速风向监测方法",
            "海温监测: 海表温度监测和分析",
            "海浪监测: 海浪高度和周期监测",
            "海雾监测: 海雾形成和消散监测",
            "海冰监测: 海冰范围和厚度监测",
            "台风监测: 海上台风路径监测",
            "风暴监测: 海上风暴系统监测",
            "综合监测: 海洋气象综合监测网",
        ]
    }

    /// 海洋气象服务规则
    pub fn service_areas(&self) -> Vec<&'static str> {
        vec![
            "航运服务: 航运气象预报和保障服务",
            "渔业服务: 渔业气象预报和服务",
            "港口服务: 港口气象预报服务",
            "海洋工程: 海洋工程气象保障",
            "海上救援: 海上搜救气象保障",
            "海洋环境: 海洋环境气象服务",
            "海洋科研: 海洋科学气象支撑",
            "海洋军事: 海军气象保障服务",
        ]
    }
}

impl Default for MarineMeteorologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MarineMeteorologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("marine_meteorology")
    }

    fn explain(&self) -> String {
        format!(
            "【海洋气象学规则】\n\n\
            海气相互作用规则:\n{}\n\n\
            海洋天气系统规则:\n{}\n\n\
            海洋气象观测规则:\n{}\n\n\
            海洋气象预报规则:\n{}\n\n\
            海洋气象灾害规则:\n{}\n\n\
            航运气象规则:\n{}\n\n\
            渔业气象规则:\n{}\n\n\
            海洋气象要素规则:\n{}\n\n\
            海洋气象监测规则:\n{}\n\n\
            海洋气象服务规则:\n{}",
            self.air_sea_interaction_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.marine_weather_systems_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.marine_observation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.marine_forecast_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.marine_disaster_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.shipping_meteorology_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.fishing_meteorology_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.marine_elements_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.marine_monitoring_rules()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n"),
            self.service_areas()
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
    fn test_marine_meteorology_rules() {
        let rules = MarineMeteorologyRules::new();
        assert_eq!(rules.air_sea_interaction_rules().len(), 8);
        assert_eq!(rules.marine_weather_systems_rules().len(), 8);
        assert_eq!(rules.marine_observation_rules().len(), 8);
        assert_eq!(rules.marine_forecast_rules().len(), 8);
        assert_eq!(rules.marine_disaster_rules().len(), 8);
        assert_eq!(rules.shipping_meteorology_rules().len(), 8);
        assert_eq!(rules.fishing_meteorology_rules().len(), 8);
        assert_eq!(rules.marine_elements_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_interaction_rules() {
        let rules = MarineMeteorologyRules::new();
        let laws = rules.air_sea_interaction_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("海")));
    }

    #[test]
    fn test_forecast_rules() {
        let rules = MarineMeteorologyRules::new();
        assert_eq!(rules.marine_forecast_rules().len(), 8);
    }

    #[test]
    fn test_monitoring_methods() {
        let rules = MarineMeteorologyRules::new();
        assert_eq!(rules.marine_monitoring_rules().len(), 8);
    }
}