//! 分析化学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 分析化学定律集合
pub struct AnalyticalChemistryLaws {
    metadata: RuleMetadata,
}

impl AnalyticalChemistryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("分析化学定律", "分析化学基本定律")
                .with_origin("化学")
                .with_tags(vec!["科学".into(), "化学".into(), "分析".into()]),
        }
    }

    /// 定量分析定律
    pub fn quantitative_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("定量定律", "准确测量", "定量分析方法"),
            ("摩尔定律", "摩尔计算", "摩尔定量关系"),
            ("浓度定律", "浓度计算", "溶液浓度计算"),
            ("滴定律", "滴定分析", "滴定定量方法"),
            ("重量定律", "重量分析", "重量分析方法"),
            ("容量定律", "容量分析", "容量分析方法"),
            ("标准定律", "标准物质", "标准物质使用"),
            ("误差定律", "误差分析", "分析误差控制"),
        ]
    }

    /// 定性分析定律
    pub fn qualitative_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("定性定律", "物质鉴定", "定性分析方法"),
            ("鉴别定律", "特征鉴别", "物质特征鉴别"),
            ("分离定律", "分离鉴定", "分离后鉴定"),
            ("检测定律", "检测反应", "检测反应特征"),
            ("验证定律", "验证确认", "定性验证"),
            ("鉴定定律", "鉴定方法", "物质鉴定方法"),
        ]
    }

    /// 仪器分析定律
    pub fn instrumental_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("光谱定律", "光谱分析", "光谱分析方法"),
            ("色谱定律", "色谱分离", "色谱分析方法"),
            ("质谱定律", "质量分析", "质谱分析方法"),
            ("电化学定律", "电分析方法", "电化学分析"),
            ("热分析定律", "热分析法", "热分析方法"),
            ("核磁定律", "NMR分析", "核磁共振分析"),
            ("色谱联用定律", "联用技术", "色谱联用分析"),
            ("光学定律", "光学分析", "光学分析方法"),
        ]
    }

    /// 分离定律
    pub fn separation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("萃取定律", "溶剂萃取", "溶剂萃取分离"),
            ("蒸馏定律", "蒸馏分离", "蒸馏分离方法"),
            ("结晶定律", "结晶分离", "结晶分离方法"),
            ("吸附定律", "吸附分离", "吸附分离方法"),
            ("离子交换定律", "交换分离", "离子交换分离"),
            ("膜分离定律", "膜分离", "膜分离技术"),
            ("色谱分离定律", "色谱法", "色谱分离技术"),
        ]
    }

    /// 分析方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "滴定分析",
            "重量分析",
            "光谱分析",
            "色谱分析",
            "质谱分析",
            "电化学分析",
            "热分析",
            "核磁共振",
        ]
    }

    /// 分析仪器
    pub fn instruments(&self) -> Vec<&'static str> {
        vec![
            "分光光度计",
            "色谱仪",
            "质谱仪",
            "电化学分析仪",
            "热分析仪",
            "核磁共振仪",
            "X射线衍射仪",
            "红外光谱仪",
        ]
    }

    /// 光谱分析定律
    pub fn spectroscopy_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("紫外可见光谱定律", "UV-Vis", "分子电子跃迁吸收规律"),
            ("红外光谱定律", "IR吸收", "分子振动转动吸收"),
            ("拉曼光谱定律", "拉曼散射", "非弹性散射光谱分析"),
            ("原子吸收定律", "AAS", "原子对特征波长吸收"),
            ("原子发射定律", "AES", "原子激发态发射光谱"),
            ("荧光光谱定律", "荧光发射", "分子荧光发射规律"),
            ("X射线荧光定律", "XRF", "X射线激发元素荧光"),
            ("圆二色定律", "CD光谱", "手性分子圆二色性"),
        ]
    }

    /// 电分析化学定律
    pub fn electroanalytical_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("能斯特方程定律", "E = E°+RT/nF·ln(a)", "电极电位与浓度关系"),
            ("法拉第电解定律", "m = MIt/(nF)", "电解产物质量计算"),
            ("极谱分析定律", "极谱波", "极谱分析扩散电流规律"),
            ("库仑分析定律", "电量测量", "库仑滴定定量分析"),
            ("电位滴定定律", "电位突跃", "电位滴定终点判断"),
            ("伏安分析定律", "伏安曲线", "循环伏安法分析规律"),
            ("离子选择电极定律", "电位响应", "离子选择性电极响应"),
            ("电导分析定律", "电导率", "溶液电导率分析规律"),
        ]
    }

    /// 色谱分析定律
    pub fn chromatography_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("塔板理论定律", "理论塔板", "色谱柱效率评价"),
            ("速率理论定律", "van Deemter", "色谱峰展宽因素分析"),
            ("保留时间定律", "保留值", "色谱保留时间规律"),
            ("分离度定律", "分辨率", "色谱峰分离度计算"),
            ("选择性定律", "选择性因子", "色谱分离选择性优化"),
            ("气相色谱定律", "GC", "气相色谱分离规律"),
            ("液相色谱定律", "HPLC", "高效液相色谱规律"),
            ("离子色谱定律", "IC", "离子色谱分离规律"),
        ]
    }

    /// 光谱分析
    pub fn spectroscopy_methods(&self) -> Vec<&'static str> {
        vec![
            "紫外可见光谱: 分子电子跃迁产生的吸收光谱",
            "红外光谱: 分子振动和转动能级跃迁的特征吸收",
            "核磁共振: 原子核在磁场中的共振吸收",
            "质谱法: 将分子离子化按质荷比分离检测",
            "原子吸收光谱: 基态原子对特征波长光的吸收",
            "荧光光谱: 分子受激发后发射的荧光分析",
        ]
    }

    /// 分离分析
    pub fn separation_methods(&self) -> Vec<&'static str> {
        vec![
            "气相色谱: 挥发性组分在气相和固定相间分配分离",
            "液相色谱: 组分在液相和固定相间分配分离",
            "离子色谱: 分离检测溶液中的离子",
            "凝胶色谱: 按分子大小分离的体积排阻色谱",
            "电泳: 带电粒子在电场中按迁移率分离",
            "萃取: 利用物质在两种溶剂中分配系数不同分离",
        ]
    }
}

impl Default for AnalyticalChemistryLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AnalyticalChemistryLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("analytical_chemistry")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【分析化学定律】\n\n定量定律:\n{}\n\n定性定律:\n{}\n\n仪器定律:\n{}\n\n光谱分析定律:\n{}\n\n电分析化学定律:\n{}\n\n色谱分析定律:\n{}\n",
            self.quantitative_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.qualitative_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.instrumental_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.spectroscopy_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.electroanalytical_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.chromatography_laws().iter()
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
    fn test_analytical_chemistry_laws() {
        let laws = AnalyticalChemistryLaws::new();
        assert!(!laws.quantitative_laws().is_empty());
        assert!(!laws.instrumental_laws().is_empty());
    }
}
