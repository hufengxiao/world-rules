//! 水文学详细规则
//!
//! 水文学研究地球上水的分布、运动、循环和变化规律，
//! 包括水文循环、河流水文、地下水水文和水文预报。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 水文学详细规则集合
pub struct HydrologyDetailedRules {
    metadata: RuleMetadata,
}

impl HydrologyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("水文学详细规则", "水文学详细定律和水文循环")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "水文".into()]),
        }
    }

    /// 水文循环规则
    pub fn hydrological_cycle_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("蒸发定律", "水面蒸发", "水面蒸发速率影响因素"),
            ("蒸散发定律", "植被蒸腾", "植被蒸散发过程规律"),
            ("降水定律", "降水形成", "降水形成机制和分布"),
            ("入渗定律", "水分入渗", "水分入渗土壤过程"),
            ("径流定律", "地表径流", "径流形成和汇聚过程"),
            ("地下水补给定律", "补给过程", "地下水补给来源和途径"),
            ("地下水排泄定律", "排泄方式", "地下水排泄途径"),
            ("水量平衡定律", "水量守恒", "区域水量收支平衡"),
        ]
    }

    /// 河流水文规则
    pub fn river_hydrology_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("河流分级定律", "Strahler分级", "河流等级划分方法"),
            ("河网密度定律", "河网分布", "河网密度影响因素"),
            ("径流系数定律", "降水径流", "降水与径流关系"),
            ("洪水定律", "洪水形成", "洪水形成机制和特征"),
            ("枯水定律", "枯水期", "河流枯水期规律"),
            ("河流泥沙定律", "泥沙运动", "河流泥沙输运规律"),
            ("河床演变定律", "河床变化", "河床冲淤变化规律"),
            ("河流水温定律", "水温变化", "河流水温时空变化"),
        ]
    }

    /// 地下水水文规则
    pub fn groundwater_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("达西定律", "渗透流速", "地下水渗透流速定律"),
            ("含水层定律", "储水层", "含水层类型和特征"),
            ("地下水流定律", "流动方向", "地下水流运动规律"),
            ("地下水动力学定律", "动力学", "地下水运动动力学"),
            ("地下水污染定律", "污染运移", "地下水污染扩散规律"),
            ("地下水开采定律", "开采影响", "地下水开采影响"),
            ("地下水补给区定律", "补给区域", "地下水补给区特征"),
            ("地下水排泄区定律", "排泄区域", "地下水排泄区特征"),
        ]
    }

    /// 湖泊水文规则
    pub fn lake_hydrology_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("湖泊水量定律", "水量平衡", "湖泊水量收支平衡"),
            ("湖泊水位定律", "水位变化", "湖泊水位季节变化"),
            ("湖泊水温定律", "分层现象", "湖泊水温垂直分层"),
            ("湖泊冰情定律", "结冰融冰", "湖泊结冰融冰规律"),
            ("湖泊沉积定律", "沉积速率", "湖泊沉积速率和特征"),
            ("湖泊水质定律", "水质变化", "湖泊水质时空变化"),
            ("湖泊演化定律", "湖泊演变", "湖泊演化和消亡规律"),
            ("人工湖定律", "水库水文", "水库水文特性规律"),
        ]
    }

    /// 水文预报规则
    pub fn hydrological_forecast_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("洪水预报定律", "洪水预测", "洪水预报方法和模型"),
            ("径流预报定律", "径流预测", "径流预报模型和方法"),
            ("水位预报定律", "水位预测", "水位预报技术和方法"),
            ("水文模型定律", "数值模拟", "水文数值模型类型"),
            ("水文频率定律", "频率分析", "水文要素频率分析"),
            ("水文统计定律", "统计分析", "水文统计方法应用"),
            ("水文实时预报定律", "实时更新", "水文实时预报系统"),
            ("水文预警定律", "预警发布", "水文预警等级标准"),
        ]
    }

    /// 水文测量规则
    pub fn hydrological_measurement_rules(
        &self,
    ) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("流量测量定律", "流量测验", "河流流量测量方法"),
            ("水位测量定律", "水位观测", "水位观测站设置"),
            ("降水测量定律", "雨量观测", "降水量观测方法"),
            ("蒸发测量定律", "蒸发观测", "蒸发量观测方法"),
            ("泥沙测量定律", "泥沙测验", "泥沙含量测量方法"),
            ("水温测量定律", "水温观测", "水温观测仪器方法"),
            ("水质监测定律", "水质测定", "水质监测指标方法"),
            ("地下水监测定律", "地下水位", "地下水位监测方法"),
        ]
    }

    /// 水资源规则
    pub fn water_resources_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("水资源评价定律", "资源评估", "水资源评价方法"),
            ("水资源配置定律", "分配调度", "水资源优化配置"),
            ("水资源开发定律", "开发利用", "水资源开发利用"),
            ("水资源管理定律", "管理措施", "水资源管理制度"),
            ("水资源保护定律", "保护策略", "水资源保护措施"),
            ("水资源节约定律", "节约用水", "节水技术和措施"),
            ("水资源循环定律", "循环利用", "水资源循环利用"),
            ("水资源承载力定律", "承载能力", "水资源承载力分析"),
        ]
    }

    /// 水文地理规则
    pub fn hydrological_geography_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("水文分区定律", "区域划分", "水文特征区域划分"),
            ("流域划分定律", "流域边界", "流域边界划分方法"),
            ("水文特征定律", "特征指标", "水文特征统计指标"),
            ("水文相似定律", "相似流域", "水文相似流域判别"),
            ("水文地带定律", "地带分布", "水文地带分布规律"),
            ("水文高度定律", "垂直变化", "水文要素垂直变化"),
            ("水文经纬定律", "空间分布", "水文要素经纬分布"),
            ("水文季节定律", "季节变化", "水文要素季节变化"),
        ]
    }

    /// 水文研究方法
    pub fn research_methods(&self) -> Vec<&'static str> {
        vec![
            "水文观测: 水文站网观测水文要素",
            "水文实验: 室内外水文实验研究",
            "水文模拟: 数值模型模拟水文过程",
            "水文统计: 统计分析水文资料",
            "水文遥感: 遥感技术观测水文信息",
            "水文GIS: GIS分析水文空间数据",
            "水文同位素: 同位素技术水文研究",
            "水文大数据: 大数据技术水文应用",
        ]
    }

    /// 水文应用领域
    pub fn application_areas(&self) -> Vec<&'static str> {
        vec![
            "防洪减灾: 洪水预报预警防洪调度",
            "水资源管理: 水资源配置调度管理",
            "水利工程: 水利工程规划设计运行",
            "水环境保护: 水环境监测保护治理",
            "农业灌溉: 农业灌溉水资源调配",
            "城市水务: 城市供水排水水务管理",
            "生态水文: 生态系统水文过程研究",
            "干旱监测: 干旱监测预警评估",
        ]
    }
}

impl Default for HydrologyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HydrologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("hydrology_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【水文学详细规则】\n\n\
            水文循环规则:\n{}\n\n\
            河流水文规则:\n{}\n\n\
            地下水水文规则:\n{}\n\n\
            湖泊水文规则:\n{}\n\n\
            水文预报规则:\n{}\n\n\
            水文测量规则:\n{}\n\n\
            水资源规则:\n{}\n\n\
            水文地理规则:\n{}\n\n\
            水文研究方法:\n{}\n\n\
            水文应用领域:\n{}",
            self.hydrological_cycle_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.river_hydrology_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.groundwater_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.lake_hydrology_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hydrological_forecast_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hydrological_measurement_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.water_resources_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hydrological_geography_rules()
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
    fn test_hydrology_detailed_rules() {
        let rules = HydrologyDetailedRules::new();
        assert_eq!(rules.hydrological_cycle_rules().len(), 8);
        assert_eq!(rules.river_hydrology_rules().len(), 8);
        assert_eq!(rules.groundwater_rules().len(), 8);
        assert_eq!(rules.lake_hydrology_rules().len(), 8);
        assert_eq!(rules.hydrological_forecast_rules().len(), 8);
        assert_eq!(rules.hydrological_measurement_rules().len(), 8);
        assert_eq!(rules.water_resources_rules().len(), 8);
        assert_eq!(rules.hydrological_geography_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_cycle_rules() {
        let rules = HydrologyDetailedRules::new();
        let laws = rules.hydrological_cycle_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("蒸发")));
    }

    #[test]
    fn test_groundwater_rules() {
        let rules = HydrologyDetailedRules::new();
        let laws = rules.groundwater_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("达西")));
    }

    #[test]
    fn test_research_methods() {
        let rules = HydrologyDetailedRules::new();
        assert_eq!(rules.research_methods().len(), 8);
    }
}
