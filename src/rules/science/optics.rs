//! 光学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 光学定律集合
pub struct OpticsLaws {
    metadata: RuleMetadata,
}

impl OpticsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "光学定律",
                "光学基本定律"
            )
            .with_origin("物理学")
            .with_tags(vec!["科学".into(), "物理".into(), "光学".into()]),
        }
    }

    /// 光学定律
    pub fn all_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("反射定律", "θ入 = θ出", "入射角等于反射角"),
            ("折射定律(斯涅尔定律)", "n₁sinθ₁ = n₂sinθ₂", "光在不同介质界面折射"),
            ("光的干涉", "相干光叠加", "两束相干光产生干涉图样"),
            ("光的衍射", "惠更斯原理", "光绕过障碍物传播"),
            ("光的偏振", "横波特性", "光的振动方向选择性"),
            ("费马原理", "光程最短", "光沿光程极值路径传播"),
            ("马吕斯定律", "I = I₀cos²θ", "偏振光通过检偏器的强度"),
            ("布拉格定律", "2d sinθ = nλ", "晶体衍射条件"),
            ("瑞利判据", "分辨极限", "光学仪器的分辨率极限"),
            ("菲涅尔方程", "反射振幅", "光在界面的反射系数"),
            ("色散定律", "n随λ变化", "不同波长光折射率不同"),
            ("吸收定律(比尔定律)", "I = I₀e⁻αx", "光在介质中的吸收"),
        ]
    }

    /// 光学现象
    pub fn phenomena(&self) -> Vec<&'static str> {
        vec![
            "全反射",
            "临界角",
            "彩虹形成",
            "牛顿环",
            "单缝衍射",
            "双缝干涉",
            "光栅衍射",
            "薄膜干涉",
            "偏振干涉",
            "光散射",
            "光放大",
            "激光产生",
        ]
    }

    /// 光学仪器原理
    pub fn instruments(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("透镜成像", "1/f = 1/u + 1/v"),
            ("显微镜", "放大倍数 = M₁ × M₂"),
            ("望远镜", "角放大率 = f物/f目"),
            ("相机", "曝光 = 光圈 × 时间"),
            ("光谱仪", "波长分离原理"),
            ("干涉仪", "相位差测量"),
            ("光纤", "全反射传输"),
        ]
    }

    /// 光学常数
    pub fn constants(&self) -> Vec<(&'static str, f64, &'static str)> {
        vec![
            ("真空折射率", 1.0, "无量纲"),
            ("空气折射率", 1.0003, "近似值"),
            ("水折射率", 1.33, "近似值"),
            ("玻璃折射率", 1.5, "常见玻璃"),
            ("钻石折射率", 2.42, "高折射率"),
        ]
    }
}

impl Default for OpticsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for OpticsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("optics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【光学定律】\n\n{}\n",
            self.all_laws().iter()
                .map(|(name, formula, desc)| format!(
                    "▶ {}\n   公式/原理: {}\n   说明: {}\n",
                    name, formula, desc
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optics_laws() {
        let laws = OpticsLaws::new();
        assert!(!laws.all_laws().is_empty());
    }
}