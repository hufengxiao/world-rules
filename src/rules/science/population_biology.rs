//! 种群生物学定律
//!
//! 种群生物学研究种群的结构、动态、遗传和进化，
//! 包括种群增长、种群调节、种群遗传结构和种群生态学。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 种群生物学定律集合
pub struct PopulationBiologyLaws {
    metadata: RuleMetadata,
}

impl PopulationBiologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("种群生物学定律", "种群生物学基本定律和种群动态机制")
                .with_origin("生物学")
                .with_tags(vec!["科学".into(), "生物".into(), "种群".into()]),
        }
    }

    /// 种群增长定律
    pub fn population_growth_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("指数增长定律", "J型曲线", "无限资源下指数增长"),
            ("逻辑斯蒂增长定律", "S型曲线", "有限资源下S型增长"),
            ("环境容纳量定律", "K值", "环境最大容纳量"),
            ("增长率定律", "r值", "内禀增长率"),
            ("种群爆发定律", "快速增长", "种群短期快速增长"),
            ("种群崩溃定律", "快速下降", "种群短期快速下降"),
            ("周期波动定律", "周期变化", "种群数量周期波动"),
            ("稳定平衡定律", "平衡状态", "种群稳定在平衡点"),
        ]
    }

    /// 种群调节定律
    pub fn population_regulation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("密度制约定律", "密度依赖", "种群密度影响增长率"),
            ("非密度制约定律", "密度无关", "环境因素影响种群"),
            ("竞争调节定律", "资源竞争", "种内竞争调节种群"),
            ("捕食调节定律", "捕食压力", "捕食者调节被捕食者"),
            ("疾病调节定律", "病原影响", "疾病影响种群密度"),
            ("食物调节定律", "食物限制", "食物供应限制种群"),
            ("空间调节定律", "空间限制", "栖息地空间限制"),
            ("行为调节定律", "行为影响", "行为模式调节种群"),
        ]
    }

    /// 种群结构定律
    pub fn population_structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("年龄结构定律", "年龄分布", "种群年龄组成"),
            ("性别结构定律", "性别比例", "种群性别比例"),
            ("空间结构定律", "空间分布", "种群空间分布"),
            ("遗传结构定律", "基因组成", "种群基因频率"),
            ("社会结构定律", "社会等级", "种群社会组织"),
            ("繁殖结构定律", "繁殖状态", "繁殖个体比例"),
            ("大小结构定律", "个体大小", "个体大小分布"),
            ("健康状况定律", "健康状态", "种群健康水平"),
        ]
    }

    /// 种群遗传定律
    pub fn population_genetics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("哈迪-温伯格定律", "基因频率", "理想群体基因频率恒定"),
            ("基因频率定律", "p+q=1", "等位基因频率"),
            ("基因型频率定律", "p²+2pq+q²=1", "基因型频率分布"),
            ("基因漂变定律", "随机变化", "小种群基因随机变化"),
            ("自然选择定律", "频率改变", "选择改变基因频率"),
            ("基因流动定律", "种群交流", "基因在种群间流动"),
            ("突变压力定律", "新等位基因", "突变产生新基因"),
            ("遗传负荷定律", "遗传负担", "有害基因的负荷"),
        ]
    }

    /// 种群动态类型
    pub fn dynamics_types(&self) -> Vec<&'static str> {
        vec![
            "r策略者: 高繁殖率、快速增长、短期生存",
            "K策略者: 低繁殖率、稳定种群、长期生存",
            "周期种群: 数量周期性波动",
            "不规则波动: 数量不规则变化",
            "稳定种群: 数量相对稳定",
            "爆发种群: 周期性爆发增长",
            "衰退种群: 数量持续下降",
            "灭绝种群: 最终走向灭绝",
        ]
    }

    /// 种群影响因素
    pub fn influencing_factors(&self) -> Vec<&'static str> {
        vec![
            "出生率: 新个体加入种群的速度",
            "死亡率: 个体死亡离开种群的速度",
            "迁入率: 外来个体加入种群",
            "迁出率: 种群个体离开",
            "资源供应: 食物、水、空间等资源",
            "环境条件: 温度、湿度、气候等",
            "捕食压力: 捕食者对种群的影响",
            "种间竞争: 与其他物种竞争资源",
        ]
    }

    /// 种群统计方法
    pub fn census_methods(&self) -> Vec<&'static str> {
        vec![
            "绝对计数: 直接计数所有个体",
            "样方法: 样方内计数推算总数",
            "标记重捕法: 标记释放后重捕估算",
            "去除采样法: 逐次去除估算密度",
            "距离采样法: 距离测量估算密度",
            "间接计数法: 通过痕迹间接估算",
            "遥感监测: 利用遥感技术监测",
            "模型预测: 数学模型预测数量",
        ]
    }
}

impl Default for PopulationBiologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PopulationBiologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("population_biology")
    }

    fn explain(&self) -> String {
        format!(
            "【种群生物学定律】\n\n\
            种群增长定律:\n{}\n\n\
            种群调节定律:\n{}\n\n\
            种群结构定律:\n{}\n\n\
            种群遗传定律:\n{}\n\n\
            种群动态类型:\n{}\n\n\
            种群影响因素:\n{}\n",
            self.population_growth_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.population_regulation_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.population_structure_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.population_genetics_laws()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dynamics_types()
                .iter()
                .map(|d| format!("  • {}", d))
                .collect::<Vec<_>>()
                .join("\n"),
            self.influencing_factors()
                .iter()
                .map(|f| format!("  • {}", f))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population_biology_laws() {
        let rules = PopulationBiologyLaws::new();
        assert_eq!(rules.population_growth_laws().len(), 8);
        assert_eq!(rules.population_regulation_laws().len(), 8);
        assert_eq!(rules.population_structure_laws().len(), 8);
        assert_eq!(rules.population_genetics_laws().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_hardy_weinberg_law() {
        let rules = PopulationBiologyLaws::new();
        let laws = rules.population_genetics_laws();
        assert!(laws.iter().any(|(n, _, _)| n.contains("哈迪-温伯格")));
    }

    #[test]
    fn test_dynamics_types() {
        let rules = PopulationBiologyLaws::new();
        assert_eq!(rules.dynamics_types().len(), 8);
    }
}