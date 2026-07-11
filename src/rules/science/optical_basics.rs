//! 光学基础规则
//!
//! 光学基础研究光的产生、传播、反射、折射和干涉等现象。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: OpticalBasicsRules,
    name: "光学基础规则",
    desc: "光学现象与原理方法",
    origin: "电磁学",
    tags: ["科学", "物理", "电磁", "光学"]
}

impl OpticalBasicsRules {
    /// 光的本质
    pub fn nature_of_light(&self) -> Vec<&'static str> {
        vec![
            "电磁波: 光是电磁波",
            "波粒二象性: 光具有波动性和粒子性",
            "光子: 光的能量量子",
            "光子能量: E = hf",
            "光子动量: p = h/λ",
            "普朗克常数: h = 6.626×10⁻³⁴ J·s",
            "光速: c = 3×10⁸ m/s（真空中）",
            "可见光波长: 380-780 nm",
        ]
    }

    /// 光的反射
    pub fn reflection(&self) -> Vec<&'static str> {
        vec![
            "反射定律: θ₁ = θ₂，入射角等于反射角",
            "镜面反射: 平面镜反射",
            "漫反射: 粗糙表面反射",
            "全反射: 光密到光疏介质全反射",
            "临界角: θc = arcsin(n₂/n₁)",
            "反射率: R = (n₁-n₂)²/(n₁+n₂)²",
            "反射应用: 镜子、激光反射镜",
            "金属反射: 金属表面高反射率",
        ]
    }

    /// 光的折射
    pub fn refraction(&self) -> Vec<&'static str> {
        vec![
            "折射定律: n₁sinθ₁ = n₂sinθ₂",
            "折射率: n = c/v",
            "空气折射率: n ≈ 1.0003",
            "玻璃折射率: n ≈ 1.5",
            "水的折射率: n ≈ 1.33",
            "色散: 不同波长折射率不同",
            "棱镜: 利用色散分光",
            "折射应用: 透镜、光纤",
        ]
    }

    /// 光的干涉
    pub fn interference(&self) -> Vec<&'static str> {
        vec![
            "干涉条件: 相干光叠加",
            "相干光: 频率相同、相位差固定",
            "相长干涉: Δ = 2kπ，光强增加",
            "相消干涉: Δ = (2k+1)π，光强减弱",
            "光程差: Δ = nλ/2",
            "双缝干涉: 杨氏双缝干涉",
            "干涉条纹: 明暗相间条纹",
            "干涉应用: 干涉仪、薄膜干涉",
        ]
    }

    /// 光的衍射
    pub fn diffraction(&self) -> Vec<&'static str> {
        vec![
            "衍射现象: 光绕过障碍物传播",
            "单缝衍射: 光通过单缝的衍射",
            "圆孔衍射: 光通过圆孔的衍射",
            "衍射极限: 光学系统的分辨率限制",
            "衍射公式: d sinθ = mλ",
            "瑞利判据: θ = 1.22λ/D",
            "衍射应用: 衍射光栅、X射线衍射",
            "衍射限制: 影响成像分辨率",
        ]
    }

    /// 光的偏振
    pub fn polarization(&self) -> Vec<&'static str> {
        vec![
            "自然光: 各方向偏振均匀",
            "线偏振: E矢量振动方向固定",
            "偏振片: 只允许特定方向偏振通过",
            "马吕斯定律: I = I₀cos²θ",
            "偏振产生: 反射、散射、双折射",
            "布儒斯特角: 全偏振反射",
            "偏振应用: 偏振显微镜、3D电影",
            "圆偏振: E矢量旋转",
        ]
    }

    /// 光学器件
    pub fn optical_devices(&self) -> Vec<&'static str> {
        vec![
            "凸透镜: 会聚光线",
            "凹透镜: 发散光线",
            "透镜公式: 1/f = 1/u + 1/v",
            "透镜成像: 成像位置和大小",
            "棱镜: 分光和偏转光线",
            "反射镜: 反射光线",
            "光纤: 传输光信号",
            "光学系统: 多器件组合",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "光学成像: 相机、望远镜",
            "光学测量: 干涉测量、光谱分析",
            "光学通信: 光纤通信",
            "激光技术: 激光器应用",
            "光学显示: 显示器、投影仪",
            "光学存储: 光盘存储",
            "光学传感: 光学传感器",
            "医学光学: 内窥镜、眼科",
        ]
    }
}

impl Rule for OpticalBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("optical_basics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "光学基础规则",
            &[
                ("光的本质", &self.nature_of_light()),
                ("光的反射", &self.reflection()),
                ("光的折射", &self.refraction()),
                ("光的干涉", &self.interference()),
                ("光的衍射", &self.diffraction()),
                ("光的偏振", &self.polarization()),
                ("光学器件", &self.optical_devices()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optical_basics_rules() {
        let rules = OpticalBasicsRules::new();
        assert_eq!(rules.metadata().name, "光学基础规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.nature_of_light().is_empty());
        assert!(!rules.reflection().is_empty());
    }
}
