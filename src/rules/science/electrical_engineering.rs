//! 电气工程定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 电气工程定律集合
pub struct ElectricalEngineeringLaws {
    metadata: RuleMetadata,
}

impl ElectricalEngineeringLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "电气工程定律",
                "电气工程基本定律"
            )
            .with_origin("工程")
            .with_tags(vec!["科学".into(), "工程".into(), "电气".into()]),
        }
    }

    /// 电路定律
    pub fn circuit_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("欧姆定律", "V = IR", "电压电流电阻关系"),
            ("基尔霍夫电流定律", "节点电流", "节点电流守恒"),
            ("基尔霍夫电压定律", "回路电压", "回路电压守恒"),
            ("功率定律", "P = VI", "电功率计算"),
            ("阻抗定律", "Z = R + jX", "阻抗计算"),
            ("谐振定律", "谐振频率", "电路谐振条件"),
            ("叠加定律", "叠加分析", "线性电路叠加"),
            ("戴维南定律", "等效电路", "等效电路简化"),
        ]
    }

    /// 电磁定律
    pub fn electromagnetic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("电磁感应定律", "磁通变化", "电磁感应原理"),
            ("楞次定律", "感应方向", "感应电流方向"),
            ("磁场定律", "磁场强度", "磁场计算"),
            ("磁路定律", "磁路计算", "磁路分析"),
            ("自感定律", "自感系数", "自感现象"),
            ("互感定律", "互感系数", "互感现象"),
            ("涡流定律", "涡流效应", "涡流现象"),
        ]
    }

    /// 电机定律
    pub fn motor_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("电机定律", "能量转换", "机电能量转换"),
            ("转矩定律", "转矩产生", "电机转矩"),
            ("转速定律", "转速关系", "电机转速控制"),
            ("效率定律", "效率计算", "电机效率"),
            ("励磁定律", "励磁方式", "电机励磁"),
            ("起动定律", "起动过程", "电机起动"),
            ("调速定律", "调速方法", "电机调速"),
        ]
    }

    /// 电力系统定律
    pub fn power_system_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("输电定律", "输电原理", "电力输配"),
            ("变压定律", "变压器原理", "电压变换"),
            ("配电定律", "配电网络", "电力配电"),
            ("保护定律", "保护系统", "电力保护"),
            ("稳定定律", "稳定运行", "系统稳定性"),
            ("负荷定律", "负荷特性", "负荷分析"),
            ("电能质量定律", "质量指标", "电能质量"),
        ]
    }

    /// 电气设备
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "发电机",
            "电动机",
            "变压器",
            "开关",
            "电缆",
            "断路器",
            "继电器",
            "仪表",
        ]
    }

    /// 电气参数
    pub fn parameters(&self) -> Vec<&'static str> {
        vec![
            "电压",
            "电流",
            "功率",
            "频率",
            "相位",
            "阻抗",
            "电容",
            "电感",
        ]
    }
}

impl Default for ElectricalEngineeringLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ElectricalEngineeringLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("electrical_engineering")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【电气工程定律】\n\n电路定律:\n{}\n\n电磁定律:\n{}\n\n电机定律:\n{}\n",
            self.circuit_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.electromagnetic_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.motor_laws().iter()
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
    fn test_electrical_engineering_laws() {
        let laws = ElectricalEngineeringLaws::new();
        assert!(!laws.circuit_laws().is_empty());
        assert!(!laws.motor_laws().is_empty());
    }
}