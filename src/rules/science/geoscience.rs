//! 地球科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 地球科学定律集合
pub struct GeoscienceLaws {
    metadata: RuleMetadata,
}

impl GeoscienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "地球科学定律",
                "地球科学基本定律"
            )
            .with_origin("地球科学")
            .with_tags(vec!["科学".into(), "地球".into()]),
        }
    }

    /// 地质学定律
    pub fn geology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地层叠加定律", "新地层覆盖老地层", "沉积岩层顺序排列规律"),
            ("地层连续定律", "地层水平延伸", "原始沉积层水平连续"),
            ("化石定年定律", "化石确定年代", "化石组合反映地层年代"),
            ("地质均匀性定律", "过去→现在", "过去地质过程与现在相同"),
            ("板块构造理论", "大陆漂移", "地球板块运动形成地貌"),
            ("地震波定律", "波传播规律", "地震波在地球内部传播"),
            ("岩浆形成定律", "部分熔融", "岩石部分熔融形成岩浆"),
            ("变质定律", "压力温度", "岩石在高温高压下变质"),
        ]
    }

    /// 气象学定律
    pub fn meteorology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("气压高度定律", "P = P₀e⁻h/H", "气压随高度降低"),
            ("温度递减定律", "每升高1km降6.5℃", "对流层温度递减"),
            ("科里奥利效应", "地球自转偏转", "大气和洋流受地球自转影响"),
            ("伯努利方程", "P + ½ρv² + ρgh = 常数", "流体动力学原理"),
            ("理想气体定律", "PV = nRT", "大气状态方程"),
            ("风压定律", "P = ½ρv²", "风速与风压关系"),
            ("大气环流定律", "三圈环流", "全球大气环流模式"),
            ("水汽定律", "饱和水汽压", "水汽凝结条件"),
        ]
    }

    /// 海洋学定律
    pub fn oceanography_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("盐度定律", "海水盐度35‰", "大洋平均盐度"),
            ("洋流定律", "风驱动+密度", "洋流驱动机制"),
            ("潮汐定律", "引力作用", "月球和太阳引力产生潮汐"),
            ("海水密度定律", "ρ取决于T,S,P", "海水密度与温度盐度压力"),
            ("波浪定律", "风生波浪", "风速风程决定波浪"),
            ("海冰定律", "冻结温度-1.9℃", "海水冻结温度"),
        ]
    }

    /// 地球现象
    pub fn phenomena(&self) -> Vec<&'static str> {
        vec![
            "地震",
            "火山喷发",
            "海啸",
            "台风",
            "龙卷风",
            "洪水",
            "干旱",
            "气候变化",
            "厄尔尼诺",
            "拉尼娜",
            "极光",
            "季风",
        ]
    }
}

impl Default for GeoscienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GeoscienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("geoscience")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【地球科学定律】\n\n地质学定律:\n{}\n\n气象学定律:\n{}\n\n海洋学定律:\n{}\n",
            self.geology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.meteorology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.oceanography_laws().iter()
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
    fn test_geoscience_laws() {
        let laws = GeoscienceLaws::new();
        assert!(!laws.geology_laws().is_empty());
    }
}