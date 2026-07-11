//! 地球物理详细规则
//!
//! 地球物理学研究地球的物理性质和内部结构。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 地球物理详细规则集合
pub struct GeophysicsDetailedRules {
    metadata: RuleMetadata,
}

impl GeophysicsDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("地球物理详细规则", "地球物理学基本概念和定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "地球物理".into()]),
        }
    }

    /// 地震学基础
    pub fn seismology_basics(&self) -> Vec<&'static str> {
        vec![
            "地震定义: 地壳内部能量释放引起的振动",
            "地震波: 地震能量传播形成的波动",
            "体波: 在地球内部传播的波",
            "面波: 在地球表面传播的波",
            "纵波P波: 压缩振动传播最快的波",
            "横波S波: 剪切振动传播较慢的波",
            "地震震级: 地震能量大小的度量",
            "地震烈度: 地震对地面影响程度的度量",
        ]
    }

    /// 地震波传播规则
    pub fn seismic_wave_propagation(&self) -> Vec<&'static str> {
        vec![
            "地震波速度: P波约6-8km/s S波约3.5-4.5km/s",
            "波速随深度变化: 波速随深度增加",
            "波速间断面: 地球内部结构的分界面",
            "莫霍面: 地壳与地幔的分界面",
            "核幔边界: 地幔与地核的分界面",
            "外核内核边界: 外核与内核的分界面",
            "地震射线: 地震波传播路径",
            "地震射线弯曲: 波速变化导致路径弯曲",
        ]
    }

    /// 重力场规则
    pub fn gravity_field_rules(&self) -> Vec<&'static str> {
        vec![
            "地球重力场: 地球引力场分布",
            "重力加速度: g ≈ 9.8 m/s²",
            "重力异常: 实测重力与理论值的差异",
            "自由空气异常: 高度校正后的重力异常",
            "布格异常: 地形校正后的重力异常",
            "重力梯度: 重力随位置变化的梯度",
            "重力测量: 用重力仪测量重力值",
            "重力应用: 探测地下结构和资源",
        ]
    }

    /// 地磁场规则
    pub fn geomagnetic_field_rules(&self) -> Vec<&'static str> {
        vec![
            "地球磁场: 地球产生的磁场",
            "磁极: 地球磁场的南北磁极",
            "磁偏角: 磁北与地理北的偏差",
            "磁倾角: 磁场与水平面的夹角",
            "磁场强度: 磁场的大小",
            "磁场变化: 磁场随时间和空间变化",
            "地磁起源: 地球液态外核运动产生",
            "地磁翻转: 磁极位置周期性交换",
        ]
    }

    /// 地热学规则
    pub fn geothermal_rules(&self) -> Vec<&'static str> {
        vec![
            "地热定义: 地球内部的热量",
            "地温梯度: 地温随深度增加的速率",
            "平均地温梯度: 约25-30°C/km",
            "地热流: 地球内部向表面传递的热量",
            "平均地热流: 约60-80 mW/m²",
            "地热资源: 可利用的地热能量",
            "地热发电: 利用地热产生电力",
            "地热分布: 地热资源地理分布",
        ]
    }

    /// 地球内部结构规则
    pub fn earth_internal_structure(&self) -> Vec<&'static str> {
        vec![
            "地壳: 地球最外层固体层",
            "地壳厚度: 大陆约30-50km 海洋约5-10km",
            "地幔: 地壳以下到2900km深度",
            "上地幔: 含软流层可发生塑性流动",
            "下地幔: 固态主要成分为硅酸盐",
            "地核: 地球中心部分2900-6371km",
            "外核: 液态铁镍合金",
            "内核: 固态铁镍合金半径约1200km",
        ]
    }

    /// 板块构造规则
    pub fn plate_tectonics_rules(&self) -> Vec<&'static str> {
        vec![
            "板块定义: 地壳和上地幔组成的刚性板块",
            "板块运动: 板块在地幔上缓慢移动",
            "板块边界: 板块之间接触的边界",
            "发散边界: 板块分离形成新地壳",
            "收敛边界: 板块碰撞地壳消减",
            "转换边界: 板块水平滑动",
            "板块速度: 板块移动速度约1-10cm/年",
            "板块驱动: 地幔对流驱动板块运动",
        ]
    }

    /// 地球物理探测方法
    pub fn geophysical_methods(&self) -> Vec<&'static str> {
        vec![
            "地震勘探",
            "重力探测",
            "磁法探测",
            "电法探测",
            "地热测量",
            "放射性测量",
            "遥感技术",
            "地球物理测井",
        ]
    }

    /// 应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "地震监测",
            "资源勘探",
            "地质灾害预警",
            "工程建设",
            "环境监测",
            "考古探测",
            "地球内部研究",
            "行星科学",
        ]
    }
}

impl Default for GeophysicsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GeophysicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("geophysics_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "地球物理详细规则",
            &[
                ("地震学基础", &self.seismology_basics()),
                ("地震波传播", &self.seismic_wave_propagation()),
                ("重力场", &self.gravity_field_rules()),
                ("地磁场", &self.geomagnetic_field_rules()),
                ("地热学", &self.geothermal_rules()),
                ("地球内部结构", &self.earth_internal_structure()),
                ("板块构造", &self.plate_tectonics_rules()),
                ("探测方法", &self.geophysical_methods()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geophysics_detailed_rules() {
        let rules = GeophysicsDetailedRules::new();
        assert_eq!(rules.metadata().name, "地球物理详细规则");
        assert!(!rules.seismology_basics().is_empty());
        assert!(!rules.gravity_field_rules().is_empty());
        assert!(!rules.plate_tectonics_rules().is_empty());
        assert!(!rules.explain().is_empty());
    }
}
