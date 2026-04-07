//! 天文学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 天文学规则
pub struct AstronomyRules {
    metadata: RuleMetadata,
}

impl AstronomyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "天文学定律",
                "天文学基本定律和理论"
            )
            .with_origin("天文学")
            .with_tags(vec!["科学".into(), "天文学".into()]),
        }
    }

    /// 开普勒定律
    pub fn kepler_laws(&self) -> Vec<&'static str> {
        vec![
            "第一定律: 行星轨道是椭圆，太阳在焦点上",
            "第二定律: 行星与太阳连线在相等时间扫过相等面积",
            "第三定律: 轨道周期平方与半长轴立方成正比",
        ]
    }

    /// 牛顿万有引力定律应用
    pub fn gravity_applications(&self) -> Vec<&'static str> {
        vec![
            "解释行星运动",
            "计算天体质量",
            "预测天体轨道",
            "解释潮汐现象",
        ]
    }

    /// 宇宙学原理
    pub fn cosmological_principles(&self) -> Vec<&'static str> {
        vec![
            "宇宙在大尺度上是均匀的",
            "宇宙在大尺度上是各向同性的",
            "哈勃定律: 星系远离速度与距离成正比",
            "大爆炸理论: 宇宙起源于一次大爆炸",
        ]
    }

    /// 恒星演化
    pub fn stellar_evolution(&self) -> Vec<&'static str> {
        vec![
            "恒星形成: 气体云坍缩",
            "主序阶段: 氢聚变",
            "红巨星阶段: 氦聚变",
            "恒星死亡: 白矮星/中子星/黑洞",
        ]
    }

    /// 太阳系数据
    pub fn solar_system_data(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("太阳", "占太阳系总质量99.86%"),
            ("水星", "最小行星，无大气"),
            ("金星", "最热行星，厚大气"),
            ("地球", "已知唯一有生命的行星"),
            ("火星", "红色行星，有极冠"),
            ("木星", "最大行星，气态巨星"),
            ("土星", "环最明显"),
            ("天王星", "侧躺着自转"),
            ("海王星", "最远行星，大风"),
        ]
    }

    /// 天文距离单位
    pub fn distance_units(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("天文单位(AU)", "地球到太阳的平均距离，约1.5亿公里"),
            ("光年(ly)", "光走一年的距离，约9.46万亿公里"),
            ("秒差距(pc)", "约3.26光年"),
        ]
    }
}

impl Default for AstronomyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AstronomyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("astronomy")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let planets = self.solar_system_data();
        let units = self.distance_units();
        format!(
            "【天文学定律】\n\n\
            开普勒定律:\n{}\n\n\
            宇宙学原理:\n{}\n\n\
            恒星演化:\n{}\n\n\
            太阳系:\n{}\n\n\
            距离单位:\n{}\n",
            self.kepler_laws().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.cosmological_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.stellar_evolution().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            planets.iter().map(|(n, d)| format!("  • {}: {}", n, d)).collect::<Vec<_>>().join("\n"),
            units.iter().map(|(n, d)| format!("  • {}: {}", n, d)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astronomy_rules() {
        let rules = AstronomyRules::new();
        assert_eq!(rules.kepler_laws().len(), 3);
    }
}