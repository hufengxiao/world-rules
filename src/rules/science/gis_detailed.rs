//! GIS地理详细规则
//!
//! GIS地理学研究地理信息系统的原理和应用，
//! 包括GIS数据、GIS分析、GIS制图和GIS应用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// GIS地理详细规则集合
pub struct GISDetailedRules {
    metadata: RuleMetadata,
}

impl GISDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("GIS地理详细规则", "地理信息系统原理和应用规则")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地理".into(), "GIS".into()]),
        }
    }

    /// GIS数据规则
    pub fn data_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("矢量数据定律", "矢量模型", "矢量数据模型结构"),
            ("栅格数据定律", "栅格模型", "栅格数据模型结构"),
            ("空间数据定律", "空间数据", "空间数据类型特征"),
            ("属性数据定律", "属性信息", "属性数据关联管理"),
            ("数据采集定律", "数据获取", "GIS数据采集方法"),
            ("数据转换定律", "格式转换", "GIS数据格式转换"),
            ("数据质量定律", "质量控制", "GIS数据质量控制"),
            ("数据更新定律", "数据维护", "GIS数据更新维护"),
        ]
    }

    /// GIS分析规则
    pub fn analysis_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("缓冲区分析定律", "缓冲区", "空间缓冲区分析方法"),
            ("叠加分析定律", "图层叠加", "空间图层叠加分析"),
            ("网络分析定律", "网络分析", "GIS网络分析方法"),
            ("空间统计定律", "空间统计", "空间统计分析方法"),
            ("地形分析定律", "地形分析", "DEM地形分析方法"),
            ("空间插值定律", "插值方法", "空间数据插值方法"),
            ("空间聚类定律", "聚类分析", "空间聚类分析方法"),
            ("空间关联定律", "空间关联", "空间关联分析方法"),
        ]
    }

    /// GIS制图规则
    pub fn mapping_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地图投影定律", "投影变换", "地图投影坐标变换"),
            ("地图符号定律", "符号设计", "地图符号设计原则"),
            ("地图配色定律", "色彩设计", "地图色彩设计原则"),
            ("地图注记定律", "注记布局", "地图注记布局方法"),
            ("地图比例定律", "比例尺", "地图比例尺选择原则"),
            ("地图类型定律", "地图分类", "地图类型划分特征"),
            ("电子地图定律", "电子地图", "电子地图设计制作"),
            ("三维地图定律", "三维制图", "三维地图制图方法"),
        ]
    }

    /// GIS查询规则
    pub fn query_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("空间查询定律", "空间检索", "空间位置查询方法"),
            ("属性查询定律", "属性检索", "属性条件查询方法"),
            ("组合查询定律", "综合查询", "空间属性组合查询"),
            ("模糊查询定律", "模糊检索", "模糊条件查询方法"),
            ("范围查询定律", "范围检索", "空间范围查询方法"),
            ("路径查询定律", "路径检索", "路径查询导航方法"),
            ("历史查询定律", "历史检索", "历史数据查询方法"),
            ("实时查询定律", "实时检索", "实时数据查询方法"),
        ]
    }

    /// GIS可视化规则
    pub fn visualization_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("二维可视化定律", "平面显示", "GIS二维可视化方法"),
            ("三维可视化定律", "立体显示", "GIS三维可视化方法"),
            ("动态可视化定律", "动态显示", "GIS动态可视化方法"),
            ("交互可视化定律", "交互操作", "GIS交互可视化方法"),
            ("专题可视化定律", "专题图", "GIS专题可视化方法"),
            ("统计可视化定律", "统计图", "GIS统计可视化方法"),
            ("场景可视化定律", "场景漫游", "GIS场景漫游方法"),
            ("增强可视化定律", "增强表达", "GIS增强可视化方法"),
        ]
    }

    /// GIS建模规则
    pub fn modeling_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("空间建模定律", "模型构建", "GIS空间模型构建"),
            ("过程建模定律", "过程模拟", "GIS过程模拟方法"),
            ("分析建模定律", "分析模型", "GIS分析模型构建"),
            ("预测建模定律", "预测模拟", "GIS预测模型方法"),
            ("优化建模定律", "优化分析", "GIS优化模型方法"),
            ("决策建模定律", "决策支持", "GIS决策支持模型"),
            ("仿真建模定律", "仿真模拟", "GIS仿真模拟方法"),
            ("集成建模定律", "模型集成", "GIS多模型集成方法"),
        ]
    }

    /// GIS应用规则
    pub fn application_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("城市规划GIS定律", "规划应用", "城市规划GIS应用"),
            ("土地管理GIS定律", "土地应用", "土地管理GIS应用"),
            ("交通GIS定律", "交通应用", "交通管理GIS应用"),
            ("环境GIS定律", "环境应用", "环境管理GIS应用"),
            ("灾害GIS定律", "灾害应用", "灾害管理GIS应用"),
            ("农业GIS定律", "农业应用", "农业生产GIS应用"),
            ("林业GIS定律", "林业应用", "森林管理GIS应用"),
            ("水利GIS定律", "水利应用", "水利工程GIS应用"),
        ]
    }

    /// GIS服务规则
    pub fn service_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("WebGIS定律", "网络服务", "WebGIS网络服务"),
            ("云GIS定律", "云计算", "云GIS云计算服务"),
            ("移动GIS定律", "移动应用", "移动GIS移动应用"),
            ("GIS共享定律", "数据共享", "GIS数据共享服务"),
            ("GIS门户定律", "门户网站", "GIS门户网站服务"),
            ("GISAPI定律", "接口服务", "GISAPI接口服务"),
            ("GIS平台定律", "平台服务", "GIS平台服务架构"),
            ("GIS集成定律", "系统集成", "GIS系统集成方法"),
        ]
    }

    /// 主要GIS类型
    pub fn major_gis_types(&self) -> Vec<&'static str> {
        vec![
            "桌面GIS: 桌面计算机GIS系统",
            "服务器GIS: 服务器端GIS系统",
            "WebGIS: 网络WebGIS系统",
            "移动GIS: 移动设备GIS系统",
            "云GIS: 云平台GIS系统",
            "组件GIS: GIS组件开发系统",
            "嵌入式GIS: 嵌入式GIS系统",
            "实时GIS: 实时GIS系统",
            "三维GIS: 三维GIS系统",
            "时态GIS: 时态GIS系统",
        ]
    }

    /// GIS发展趋势
    pub fn development_trends(&self) -> Vec<&'static str> {
        vec![
            "大数据GIS: GIS大数据处理技术",
            "人工智能GIS: AI与GIS集成应用",
            "云计算GIS: 云计算GIS平台发展",
            "移动GIS: 移动GIS应用扩展",
            "三维GIS: 三维GIS技术发展",
            "实时GIS: 实时GIS系统建设",
            "开放GIS: 开源GIS软件发展",
            "集成GIS: 多技术集成GIS应用",
        ]
    }
}

impl Default for GISDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GISDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("gis_detailed")
    }

    fn explain(&self) -> String {
        format!(
            "【GIS地理详细规则】\n\n\
            GIS数据规则:\n{}\n\n\
            GIS分析规则:\n{}\n\n\
            GIS制图规则:\n{}\n\n\
            GIS查询规则:\n{}\n\n\
            GIS可视化规则:\n{}\n\n\
            GIS建模规则:\n{}\n\n\
            GIS应用规则:\n{}\n\n\
            GIS服务规则:\n{}\n\n\
            主要GIS类型:\n{}\n\n\
            GIS发展趋势:\n{}",
            self.data_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.analysis_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.mapping_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.query_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.visualization_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.modeling_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.application_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.service_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.major_gis_types()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.development_trends()
                .iter()
                .map(|d| format!("  • {}", d))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gis_detailed_rules() {
        let rules = GISDetailedRules::new();
        assert_eq!(rules.data_rules().len(), 8);
        assert_eq!(rules.analysis_rules().len(), 8);
        assert_eq!(rules.mapping_rules().len(), 8);
        assert_eq!(rules.query_rules().len(), 8);
        assert_eq!(rules.visualization_rules().len(), 8);
        assert_eq!(rules.modeling_rules().len(), 8);
        assert_eq!(rules.application_rules().len(), 8);
        assert_eq!(rules.service_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_gis_types() {
        let rules = GISDetailedRules::new();
        assert_eq!(rules.major_gis_types().len(), 10);
    }

    #[test]
    fn test_analysis() {
        let rules = GISDetailedRules::new();
        let laws = rules.analysis_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("分析")));
    }

    #[test]
    fn test_trends() {
        let rules = GISDetailedRules::new();
        assert_eq!(rules.development_trends().len(), 8);
    }
}
