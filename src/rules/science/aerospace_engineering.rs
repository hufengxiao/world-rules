//! 航空航天工程定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 航空航天工程定律集合
pub struct AerospaceEngineeringLaws {
    metadata: RuleMetadata,
}

impl AerospaceEngineeringLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "航空航天工程定律",
                "航空航天工程基本定律"
            )
            .with_origin("工程")
            .with_tags(vec!["科学".into(), "工程".into(), "航空".into()]),
        }
    }

    /// 空气动力学定律
    pub fn aerodynamics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("升力定律", "L = ½ρv²SCₗ", "飞行升力原理"),
            ("阻力定律", "D = ½ρv²SCₓ", "飞行阻力计算"),
            ("推力定律", "推进力", "发动机推力"),
            ("马赫数定律", "M = v/a", "飞行速度与音速比"),
            ("边界层定律", "气流附着", "边界层效应"),
            ("激波定律", "压缩激波", "超声速激波"),
            ("湍流定律", "湍流特性", "湍流流场"),
            ("气动力定律", "气动力矩", "气动力和力矩"),
        ]
    }

    /// 飞行力学定律
    pub fn flight_mechanics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("飞行定律", "飞行原理", "飞行基本原理"),
            ("平衡定律", "飞行平衡", "飞行平衡条件"),
            ("稳定定律", "稳定性分析", "飞行稳定性"),
            ("控制定律", "飞行控制", "飞行控制原理"),
            ("机动定律", "机动飞行", "机动飞行规律"),
            ("爬升定律", "爬升性能", "爬升性能计算"),
            ("巡航定律", "巡航性能", "巡航性能分析"),
            ("着陆定律", "着陆性能", "着陆性能要求"),
        ]
    }

    /// 航天定律
    pub fn space_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("轨道定律", "开普勒轨道", "航天轨道运动"),
            ("逃逸定律", "逃逸速度", "逃逸地球引力"),
            ("推进定律", "火箭推进", "火箭推进原理"),
            ("变轨定律", "轨道变换", "轨道机动方法"),
            ("返回定律", "返回着陆", "航天器返回"),
            ("对接定律", "空间对接", "航天器对接"),
            ("姿态定律", "姿态控制", "航天器姿态"),
            ("通信定律", "空间通信", "航天通信原理"),
        ]
    }

    /// 结构定律
    pub fn structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("轻量化定律", "重量控制", "航空航天结构轻量化"),
            ("疲劳定律", "疲劳设计", "飞行结构疲劳"),
            ("气动弹性定律", "气动弹性", "气动弹性效应"),
            ("热结构定律", "热应力", "高温结构设计"),
            ("复合材料定律", "复合材料", "复合材料应用"),
            ("连接定律", "结构连接", "航空结构连接"),
        ]
    }

    /// 航空航天器类型
    pub fn vehicle_types(&self) -> Vec<&'static str> {
        vec![
            "飞机",
            "直升机",
            "无人机",
            "火箭",
            "卫星",
            "飞船",
            "空间站",
            "导弹",
        ]
    }

    /// 系统类型
    pub fn systems(&self) -> Vec<&'static str> {
        vec![
            "动力系统",
            "导航系统",
            "通信系统",
            "控制系统",
            "结构系统",
            "环境系统",
            "推进系统",
            "电气系统",
        ]
    }
}

impl Default for AerospaceEngineeringLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AerospaceEngineeringLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("aerospace_engineering")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【航空航天工程定律】\n\n空气动力学定律:\n{}\n\n飞行力学定律:\n{}\n\n航天定律:\n{}\n",
            self.aerodynamics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.flight_mechanics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.space_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aerospace_engineering_laws() {
        let laws = AerospaceEngineeringLaws::new();
        assert!(!laws.aerodynamics_laws().is_empty());
        assert!(!laws.space_laws().is_empty());
    }
}