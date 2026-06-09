//! 相对论定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 相对论定律集合
pub struct RelativityLaws {
    metadata: RuleMetadata,
}

impl RelativityLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("相对论定律", "相对论基本定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "相对论".into()]),
        }
    }

    /// 狭义相对论定律
    pub fn special_relativity(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("爱因斯坦质量能量方程", "E = mc²", "质量与能量的等价关系"),
            ("时间膨胀", "t' = t/√(1-v²/c²)", "运动物体的时间变慢"),
            ("长度收缩", "L' = L√(1-v²/c²)", "运动物体在运动方向上缩短"),
            ("相对论质量", "m' = m/√(1-v²/c²)", "运动物体的质量增加"),
            ("相对论动量", "p = mv/√(1-v²/c²)", "相对论修正的动量公式"),
            ("洛伦兹变换", "x' = γ(x-vt)", "不同参考系间的坐标变换"),
            (
                "相对论速度叠加",
                "u' = (u+v)/(1+uv/c²)",
                "相对论修正的速度叠加",
            ),
            (
                "光速不变原理",
                "c = 299792458 m/s",
                "光速在所有惯性参考系中相同",
            ),
        ]
    }

    /// 广义相对论定律
    pub fn general_relativity(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            (
                "爱因斯坦场方程",
                "Gμν = 8πGTμν/c⁴",
                "时空弯曲与物质能量的关系",
            ),
            ("等效原理", "引力等效加速度", "引力场与加速度效应等效"),
            ("引力时间膨胀", "t' = t√(1-2GM/rc²)", "引力场中时间变慢"),
            (
                "引力红移",
                "光在引力场中频率改变",
                "光从引力场强处向弱处传播频率降低",
            ),
            ("引力透镜效应", "光线弯曲", "光线经过大质量物体时弯曲"),
            (
                "黑洞理论",
                "史瓦西半径 r = 2GM/c²",
                "引力足够强时空坍缩形成黑洞",
            ),
            ("引力波", "时空涟漪", "大质量物体运动产生的时空扰动"),
            ("宇宙膨胀", "哈勃定律 v = H₀d", "宇宙在加速膨胀"),
        ]
    }

    /// 相对论效应
    pub fn effects(&self) -> Vec<&'static str> {
        vec![
            "双生子效应",
            "参考系效应",
            "时空曲率",
            "引力波探测",
            "黑洞事件视界",
            "奇点理论",
            "虫洞理论",
            "时空旅行可能性",
        ]
    }

    /// 相对论常数
    pub fn constants(&self) -> Vec<(&'static str, f64, &'static str)> {
        vec![
            ("光速 c", 2.998e8, "m/s"),
            ("引力常数 G", 6.674e-11, "m³/(kg·s²)"),
            ("哈勃常数 H₀", 70.0, "km/s/Mpc"),
        ]
    }

    /// 狭义相对论
    pub fn special_relativity_laws(&self) -> Vec<&'static str> {
        vec![
            "光速不变原理: 真空中光速对所有惯性参考系相同",
            "时间膨胀: 运动的时钟走得慢Δt=γΔt₀",
            "长度收缩: 运动方向上物体长度缩短L=L₀/γ",
            "质能等价: E=mc²质量和能量可以相互转化",
            "相对论动量: p=γmv速度接近光速时动量趋于无穷",
            "同时性相对性: 不同惯性系对两个事件是否同时判断不同",
            "洛伦兹变换: 不同惯性系之间时空坐标的变换关系",
        ]
    }

    /// 广义相对论
    pub fn general_relativity_laws(&self) -> Vec<&'static str> {
        vec![
            "等效原理: 引力效应与加速度效应局部不可区分",
            "时空弯曲: 质量和能量导致四维时空几何发生弯曲",
            "爱因斯坦场方程: Gμν+Λgμν=8πG/c⁴·Tμν",
            "引力红移: 光从强引力场中传出频率降低波长变长",
            "引力时间膨胀: 强引力场中时间流逝更慢",
            "引力波: 加速质量产生的时空曲率涟漪以光速传播",
            "黑洞: 时空弯曲到连光都无法逃逸的极端天体区域",
        ]
    }
}

impl Default for RelativityLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RelativityLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("relativity")
    }

    fn explain(&self) -> String {
        format!(
            "【相对论定律】\n\n狭义相对论:\n{}\n\n广义相对论:\n{}\n",
            self.special_relativity()
                .iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.general_relativity()
                .iter()
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
    fn test_relativity_laws() {
        let laws = RelativityLaws::new();
        assert!(!laws.special_relativity().is_empty());
        assert!(!laws.general_relativity().is_empty());
    }
}
