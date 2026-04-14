//! 机械工程定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 机械工程定律集合
pub struct MechanicalEngineeringLaws {
    metadata: RuleMetadata,
}

impl MechanicalEngineeringLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "机械工程定律",
                "机械工程基本定律"
            )
            .with_origin("工程")
            .with_tags(vec!["科学".into(), "工程".into(), "机械".into()]),
        }
    }

    /// 机械设计定律
    pub fn design_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("功能设计定律", "功能优先", "机械功能设计"),
            ("可靠性定律", "可靠设计", "可靠性设计原则"),
            ("强度定律", "强度计算", "机械强度设计"),
            ("刚度定律", "刚度要求", "刚度设计原则"),
            ("稳定性定律", "稳定条件", "稳定性设计"),
            ("寿命定律", "寿命预测", "使用寿命设计"),
            ("安全定律", "安全系数", "安全设计原则"),
            ("标准化定律", "标准设计", "标准化原则"),
        ]
    }

    /// 机械传动定律
    pub fn transmission_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("齿轮传动定律", "齿轮啮合", "齿轮传动原理"),
            ("带传动定律", "带轮传动", "带传动规律"),
            ("链传动定律", "链条传动", "链传动原理"),
            ("蜗杆传动定律", "蜗轮蜗杆", "蜗杆传动特点"),
            ("螺旋传动定律", "螺纹传动", "螺旋传动原理"),
            ("摩擦传动定律", "摩擦传递", "摩擦传动规律"),
            ("液压传动定律", "液压传递", "液压传动原理"),
            ("气压传动定律", "气压传递", "气压传动原理"),
        ]
    }

    /// 机械制造定律
    pub fn manufacturing_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("加工定律", "加工方法", "机械加工原理"),
            ("切削定律", "切削参数", "切削加工规律"),
            ("成形定律", "成形工艺", "成形加工方法"),
            ("装配定律", "装配精度", "装配工艺规律"),
            ("精度定律", "精度控制", "加工精度控制"),
            ("表面定律", "表面质量", "表面处理方法"),
            ("热处理定律", "热处理工艺", "热处理原理"),
        ]
    }

    /// 机械运动定律
    pub fn motion_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("运动定律", "运动分析", "机械运动分析"),
            ("动力学定律", "动力计算", "机械动力学"),
            ("平衡定律", "平衡设计", "机械平衡原理"),
            ("振动定律", "振动分析", "机械振动控制"),
            ("摩擦定律", "摩擦分析", "摩擦润滑原理"),
            ("磨损定律", "磨损规律", "磨损预测"),
            ("润滑定律", "润滑设计", "润滑系统设计"),
        ]
    }

    /// 机械类型
    pub fn machine_types(&self) -> Vec<&'static str> {
        vec![
            "动力机械",
            "传动机械",
            "工作机械",
            "运输机械",
            "加工机械",
            "测量机械",
            "控制机械",
            "辅助机械",
        ]
    }

    /// 机械零件
    pub fn components(&self) -> Vec<&'static str> {
        vec![
            "轴",
            "齿轮",
            "轴承",
            "联轴器",
            "弹簧",
            "螺栓",
            "键",
            "销",
        ]
    }
}

impl Default for MechanicalEngineeringLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MechanicalEngineeringLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("mechanical_engineering")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【机械工程定律】\n\n设计定律:\n{}\n\n传动定律:\n{}\n\n制造定律:\n{}\n",
            self.design_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.transmission_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.manufacturing_laws().iter()
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
    fn test_mechanical_engineering_laws() {
        let laws = MechanicalEngineeringLaws::new();
        assert!(!laws.design_laws().is_empty());
        assert!(!laws.transmission_laws().is_empty());
    }
}