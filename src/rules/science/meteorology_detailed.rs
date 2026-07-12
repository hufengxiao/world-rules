//! 气象学详细规则
//!
//! 气象学研究大气现象、天气过程和天气预报，
//! 包括大气物理、天气预报、气象观测和气象灾害。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 气象学详细规则集合
pub struct MeteorologyDetailedRules {
    metadata: RuleMetadata,
}

impl MeteorologyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("气象学详细规则", "气象学详细定律和天气系统")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "气象".into(), "大气".into()]),
        }
    }

    /// 天气预报规则
    pub fn weather_forecast_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("短期预报定律", "1-3天", "利用数值模式进行短期天气预测"),
            ("中期预报定律", "3-10天", "中期天气预报方法和不确定性"),
            ("长期预报定律", "月季尺度", "月度和季节气候趋势预测"),
            ("临近预报定律", "0-6小时", "临近天气快速更新预报"),
            ("集合预报定律", "多成员", "多个预报成员的概率预报"),
            ("区域预报定律", "区域范围", "特定区域天气预报方法"),
            ("定点预报定律", "站点预报", "特定地点天气预报技术"),
            ("预报订正定律", "偏差修正", "预报误差订正方法"),
        ]
    }

    /// 大气物理规则
    pub fn atmospheric_physics_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("大气热力学定律", "能量转换", "大气能量转换和热力学过程"),
            ("大气辐射定律", "辐射传输", "太阳和地球辐射在大气中传输"),
            ("云物理学定律", "云形成", "云的形成发展和降水过程"),
            ("大气光学定律", "光学现象", "大气光学现象如彩虹和晕"),
            ("大气电学定律", "雷电过程", "大气电荷和雷电产生机制"),
            ("大气声学定律", "声波传播", "声波在大气中的传播特性"),
            ("大气化学定律", "化学反应", "大气中的化学反应过程"),
            ("气溶胶物理定律", "颗粒物", "气溶胶的形成和演变规律"),
        ]
    }

    /// 大气环流规则
    pub fn atmospheric_circulation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("哈德莱环流定律", "热带环流", "赤道到副热带的经向环流"),
            ("费雷尔环流定律", "中纬环流", "中纬度地区的间接环流"),
            ("极地环流定律", "极地环流", "极地地区的经向环流"),
            ("西风带定律", "西风急流", "中纬度西风带形成和变化"),
            ("信风带定律", "热带东风", "低纬度信风的形成和变化"),
            ("季风环流定律", "季风转换", "季风环流的季节转换"),
            ("Walker环流定律", "纬向环流", "赤道太平洋纬向环流"),
            ("急流定律", "高空急流", "高空急流的形成和演变"),
        ]
    }

    /// 云和降水规则
    pub fn cloud_precipitation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("云分类定律", "云型识别", "云的类型分类和识别方法"),
            ("积云发展定律", "对流云", "积状云的形成和发展过程"),
            ("层云形成定律", "层状云", "层状云的形成和维持机制"),
            ("卷云演变定律", "高云", "高云的形成和演变规律"),
            ("降水形成定律", "降水机制", "暖云和冷云降水形成过程"),
            ("雨量分布定律", "降水分布", "降水时空分布规律"),
            ("雪形成定律", "固态降水", "雪的形成和降雪过程"),
            ("冰雹形成定律", "强对流", "冰雹的形成条件和过程"),
        ]
    }

    /// 气象观测规则
    pub fn meteorological_observation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地面观测定律", "地面站", "地面气象站观测要素和方法"),
            ("高空探测定律", "探空观测", "高空气象探测技术和数据"),
            ("自动站观测定律", "自动观测", "自动气象站观测系统"),
            ("观测质量控制定律", "质量控制", "气象观测数据质量控制"),
            ("观测标准化定律", "观测标准", "气象观测标准化规范"),
            ("观测误差定律", "误差分析", "气象观测误差来源分析"),
            ("观测密度定律", "站点分布", "气象观测站网密度设计"),
            ("观测时次定律", "观测时制", "气象观测时次和时间制度"),
        ]
    }

    /// 数值天气预报规则
    pub fn numerical_weather_prediction_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("模式初始化定律", "初值化", "数值模式初值形成方法"),
            ("模式积分定律", "时间积分", "模式方程时间积分方案"),
            ("物理参数化定律", "参数方案", "次网格物理过程参数化"),
            ("资料同化定律", "数据同化", "观测数据同化技术方法"),
            ("集合预报定律", "集合方法", "集合数值预报技术"),
            ("模式后处理定律", "产品加工", "模式输出产品后处理"),
            ("模式评估定律", "预报检验", "数值模式预报检验方法"),
            ("模式改进定律", "模式发展", "数值模式发展改进方向"),
        ]
    }

    /// 气象灾害规则
    pub fn meteorological_disaster_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("台风灾害定律", "热带气旋", "台风路径强度和灾害影响"),
            ("暴雨灾害定律", "强降水", "暴雨形成和洪涝灾害"),
            ("大风灾害定律", "强风", "大风形成和风灾影响"),
            ("冰雹灾害定律", "强对流", "冰雹形成和灾害影响"),
            ("雷电灾害定律", "雷电", "雷电形成和灾害防护"),
            ("沙尘暴灾害定律", "风沙天气", "沙尘暴形成和影响"),
            ("寒潮灾害定律", "强冷空气", "寒潮路径和灾害影响"),
            ("高温灾害定律", "热浪", "高温热浪形成和影响"),
        ]
    }

    /// 大气边界层规则
    pub fn atmospheric_boundary_layer_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("边界层结构定律", "垂直分层", "大气边界层垂直结构特征"),
            ("湍流混合定律", "湍流输送", "边界层湍流混合过程"),
            ("边界层发展定律", "日变化", "边界层厚度日变化规律"),
            ("地表通量定律", "能量交换", "地表-大气能量通量交换"),
            ("边界层风定律", "风廓线", "边界层风垂直分布特征"),
            ("边界层温度定律", "温度分布", "边界层温度垂直分布"),
            ("夜间边界层定律", "稳定层结", "夜间稳定边界层特征"),
            ("对流边界层定律", "对流发展", "白天对流边界层发展"),
        ]
    }

    /// 气象雷达规则
    pub fn weather_radar_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("雷达探测定律", "电磁探测", "气象雷达探测原理和方法"),
            ("雷达反射率定律", "回波强度", "雷达反射率因子和降水"),
            ("多普勒雷达定律", "速度探测", "多普勒雷达速度测量"),
            ("双偏振雷达定律", "偏振参数", "双偏振雷达探测技术"),
            ("雷达定量降水定律", "QPE", "雷达定量降水估计方法"),
            ("雷达组网定律", "网状探测", "多部雷达组网探测技术"),
            ("雷达质量控制定律", "质量控制", "雷达数据质量控制方法"),
            ("雷达产品定律", "产品生成", "气象雷达产品生成和应用"),
        ]
    }

    /// 气象卫星规则
    pub fn weather_satellite_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("卫星探测定律", "遥感探测", "气象卫星探测原理和方法"),
            ("可见光通道定律", "反射探测", "可见光通道探测和应用"),
            ("红外通道定律", "温度探测", "红外通道探测和应用"),
            ("水汽通道定律", "水汽探测", "水汽通道探测和应用"),
            ("云图分析定律", "云图判读", "卫星云图分析和判读"),
            ("云导风定律", "风场反演", "卫星云导风反演方法"),
            ("卫星反演定律", "参数反演", "卫星反演大气参数方法"),
            ("卫星监测定律", "灾害监测", "卫星监测天气系统和灾害"),
        ]
    }

    /// 气象研究方法
    pub fn research_methods(&self) -> Vec<&'static str> {
        vec![
            "观测分析: 利用气象观测数据分析天气过程",
            "数值模拟: 利用数值模式模拟大气过程",
            "统计分析: 统计方法分析气象数据规律",
            "动力诊断: 动力学方法诊断天气过程",
            "实验研究: 实验室模拟大气物理过程",
            "理论分析: 理论模型解释大气现象",
            "资料同化: 同化多源观测数据改进分析",
            "机器学习: 机器学习方法应用于气象预报",
        ]
    }

    /// 气象应用领域
    pub fn application_areas(&self) -> Vec<&'static str> {
        vec![
            "天气预报: 公众天气预报和专业气象服务",
            "航空气象: 航空飞行气象保障服务",
            "海洋气象: 海洋气象预报和航海保障",
            "农业气象: 农业气象服务和作物气象",
            "交通气象: 交通气象保障和安全服务",
            "城市气象: 城市气候和城市气象服务",
            "环境气象: 环境气象和污染气象服务",
            "健康气象: 健康气象和医疗气象服务",
        ]
    }
}

impl Default for MeteorologyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MeteorologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("meteorology_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【气象学详细规则】\n\n\
            天气预报规则:\n{}\n\n\
            大气物理规则:\n{}\n\n\
            大气环流规则:\n{}\n\n\
            云和降水规则:\n{}\n\n\
            气象观测规则:\n{}\n\n\
            数值天气预报规则:\n{}\n\n\
            气象灾害规则:\n{}\n\n\
            大气边界层规则:\n{}\n\n\
            气象雷达规则:\n{}\n\n\
            气象卫星规则:\n{}\n\n\
            气象研究方法:\n{}\n\n\
            气象应用领域:\n{}",
            self.weather_forecast_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_physics_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_circulation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cloud_precipitation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.meteorological_observation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.numerical_weather_prediction_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.meteorological_disaster_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_boundary_layer_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weather_radar_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.weather_satellite_rules()
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
    fn test_meteorology_detailed_rules() {
        let rules = MeteorologyDetailedRules::new();
        assert_eq!(rules.weather_forecast_rules().len(), 8);
        assert_eq!(rules.atmospheric_physics_rules().len(), 8);
        assert_eq!(rules.atmospheric_circulation_rules().len(), 8);
        assert_eq!(rules.cloud_precipitation_rules().len(), 8);
        assert_eq!(rules.meteorological_observation_rules().len(), 8);
        assert_eq!(rules.numerical_weather_prediction_rules().len(), 8);
        assert_eq!(rules.meteorological_disaster_rules().len(), 8);
        assert_eq!(rules.atmospheric_boundary_layer_rules().len(), 8);
        assert_eq!(rules.weather_radar_rules().len(), 8);
        assert_eq!(rules.weather_satellite_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_forecast_rules() {
        let rules = MeteorologyDetailedRules::new();
        let laws = rules.weather_forecast_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("预报")));
    }

    #[test]
    fn test_disaster_rules() {
        let rules = MeteorologyDetailedRules::new();
        let laws = rules.meteorological_disaster_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("台风")));
    }

    #[test]
    fn test_radar_rules() {
        let rules = MeteorologyDetailedRules::new();
        assert_eq!(rules.weather_radar_rules().len(), 8);
    }

    #[test]
    fn test_satellite_rules() {
        let rules = MeteorologyDetailedRules::new();
        assert_eq!(rules.weather_satellite_rules().len(), 8);
    }

    #[test]
    fn test_research_methods() {
        let rules = MeteorologyDetailedRules::new();
        assert_eq!(rules.research_methods().len(), 8);
    }

    #[test]
    fn test_application_areas() {
        let rules = MeteorologyDetailedRules::new();
        assert_eq!(rules.application_areas().len(), 8);
    }
}