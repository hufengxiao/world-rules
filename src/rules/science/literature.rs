//! 文学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 文学定律集合
pub struct LiteratureLaws {
    metadata: RuleMetadata,
}

impl LiteratureLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "文学定律",
                "文学基本定律"
            )
            .with_origin("人文科学")
            .with_tags(vec!["科学".into(), "文学".into()]),
        }
    }

    /// 文学创作定律
    pub fn creation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("创作动机定律", "创作驱动", "创作动机来源"),
            ("创作过程定律", "创作步骤", "创作过程规律"),
            ("创作风格定律", "风格形成", "创作风格形成"),
            ("创作技巧定律", "技巧运用", "创作技巧规律"),
            ("创作灵感定律", "灵感来源", "创作灵感激发"),
            ("创作个性定律", "个性表达", "创作个性体现"),
            ("创作规律定律", "创作规律", "创作基本规律"),
        ]
    }

    /// 文学形式定律
    pub fn form_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("体裁定律", "体裁分类", "文学体裁分类"),
            ("结构定律", "结构组织", "作品结构组织"),
            ("语言定律", "语言运用", "文学语言运用"),
            ("修辞定律", "修辞技巧", "修辞手法运用"),
            ("叙事定律", "叙事方式", "叙事技巧规律"),
            ("意象定律", "意象构建", "意象创造规律"),
            ("节奏定律", "节奏控制", "作品节奏控制"),
        ]
    }

    /// 文学接受定律
    pub fn reception_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("阅读定律", "阅读过程", "阅读理解过程"),
            ("理解定律", "意义理解", "作品意义理解"),
            ("欣赏定律", "审美体验", "审美欣赏体验"),
            ("批评定律", "批评标准", "文学批评标准"),
            ("传播定律", "作品传播", "文学作品传播"),
            ("影响定律", "作品影响", "作品社会影响"),
            ("经典定律", "经典形成", "经典作品形成"),
        ]
    }

    /// 文学发展定律
    pub fn development_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("文学演变定律", "文学变化", "文学发展演变"),
            ("流派定律", "流派形成", "文学流派形成"),
            ("传统定律", "传统传承", "文学传统传承"),
            ("创新定律", "创新发展", "文学创新发展"),
            ("时代定律", "时代反映", "文学反映时代"),
            ("民族定律", "民族特色", "文学民族特色"),
            ("世界定律", "世界文学", "世界文学交流"),
        ]
    }

    /// 文学体裁
    pub fn genres(&self) -> Vec<&'static str> {
        vec![
            "诗歌",
            "散文",
            "小说",
            "戏剧",
            "报告文学",
            "传记",
            "寓言",
            "童话",
        ]
    }

    /// 文学理论
    pub fn theories(&self) -> Vec<&'static str> {
        vec![
            "现实主义理论",
            "浪漫主义理论",
            "象征主义理论",
            "形式主义理论",
            "结构主义理论",
            "后现代主义理论",
            "读者接受理论",
            "新批评理论",
        ]
    }

    /// 叙事学定律
    pub fn narratology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("叙事视角定律", "视角选择", "叙事视角选择规律"),
            ("叙事时间定律", "时间处理", "叙事时间变形规律"),
            ("叙事声音定律", "叙述者", "叙事声音与可靠性"),
            ("叙事聚焦定律", "聚焦方式", "叙事聚焦控制"),
            ("叙事距离定律", "距离调节", "叙事距离与反讽"),
            ("隐含作者定律", "隐含作者", "隐含作者与叙述者区分"),
            ("叙事悬念定律", "悬念构建", "悬念构建与释放"),
            ("叙事空白定律", "留白艺术", "叙事省略与空白"),
        ]
    }

    /// 文学社会学定律
    pub fn literary_sociology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("文学场定律", "文学场域", "布尔迪厄文学场理论"),
            ("文学资本定律", "文化资本", "文学资本积累与转化"),
            ("文学生产定律", "生产机制", "文学生产社会条件"),
            ("文学消费定律", "阅读消费", "文学消费社会学"),
            ("文学制度定律", "制度框架", "文学制度与规范"),
            ("文学审查定律", "审查机制", "文学审查与自我审查"),
            ("文学赞助定律", "赞助机制", "文学赞助制度影响"),
            ("文学生态定律", "生态平衡", "文学生态系统平衡"),
        ]
    }

    /// 比较文学定律
    pub fn comparative_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("影响研究定律", "影响关系", "文学跨国影响研究"),
            ("平行研究定律", "平行比较", "无直接影响的平行比较"),
            ("翻译文学定律", "翻译转换", "文学翻译转换规律"),
            ("世界文学定律", "全球流通", "文学作品世界性流通"),
            ("跨文化定律", "文化对话", "跨文化文学对话"),
            ("主题学定律", "主题流变", "文学主题跨国流变"),
            ("文类学定律", "体裁比较", "文学体裁跨国比较"),
            ("形象学定律", "异国形象", "文学中的异国形象"),
        ]
    }
}

impl Default for LiteratureLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LiteratureLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("literature")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【文学定律】\n\n创作定律:\n{}\n\n形式定律:\n{}\n\n接受定律:\n{}\n\n叙事学定律:\n{}\n\n文学社会学定律:\n{}\n\n比较文学定律:\n{}\n",
            self.creation_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.form_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.reception_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.narratology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.literary_sociology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.comparative_laws().iter()
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
    fn test_literature_laws() {
        let laws = LiteratureLaws::new();
        assert!(!laws.creation_laws().is_empty());
        assert!(!laws.form_laws().is_empty());
    }
}