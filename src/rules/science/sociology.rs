//! 社会学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 社会学定律集合
pub struct SociologyLaws {
    metadata: RuleMetadata,
}

impl SociologyLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("社会学定律", "社会学基本定律")
                .with_origin("社会科学")
                .with_tags(vec!["科学".into(), "社会".into()]),
        }
    }

    /// 社会结构定律
    pub fn structure_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("社会分层定律", "阶层分化", "社会阶层分化规律"),
            ("社会流动定律", "阶层流动", "社会阶层流动"),
            ("社会网络定律", "关系网络", "社会关系网络"),
            ("社会角色定律", "角色扮演", "社会角色与行为"),
            ("社会群体定律", "群体形成", "社会群体形成"),
            ("社会组织定律", "组织结构", "社会组织结构"),
            ("社会制度定律", "制度规范", "社会制度规范"),
        ]
    }

    /// 社会互动定律
    pub fn interaction_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("社会交换定律", "互惠交换", "社会互动基于交换"),
            ("符号互动定律", "意义建构", "通过符号互动"),
            ("冲突定律", "利益冲突", "社会利益冲突"),
            ("合作定律", "协同合作", "社会协同合作"),
            ("竞争定律", "资源竞争", "社会资源竞争"),
            ("模仿定律", "行为模仿", "模仿他人行为"),
            ("从众定律", "群体影响", "群体行为影响"),
        ]
    }

    /// 社会变迁定律
    pub fn change_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("社会进化定律", "渐进变化", "社会渐进演化"),
            ("社会革命定律", "剧烈变革", "社会剧烈变革"),
            ("现代化定律", "现代化进程", "社会现代化"),
            ("全球化定律", "全球整合", "全球社会整合"),
            ("城市化定律", "城市集聚", "人口城市集聚"),
            ("工业化定律", "工业转型", "社会工业化"),
            ("信息化定律", "信息变革", "社会信息化"),
        ]
    }

    /// 社会问题定律
    pub fn problem_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("贫困定律", "资源匮乏", "社会贫困问题"),
            ("不平等定律", "差距存在", "社会不平等"),
            ("犯罪定律", "社会越轨", "犯罪行为成因"),
            ("歧视定律", "偏见歧视", "社会歧视现象"),
            ("失业定律", "就业不足", "社会失业问题"),
            ("老龄化定律", "年龄结构", "社会老龄化"),
            ("人口定律", "人口变化", "人口变化规律"),
        ]
    }

    /// 社会学理论
    pub fn theories(&self) -> Vec<&'static str> {
        vec![
            "功能主义理论",
            "冲突理论",
            "符号互动理论",
            "交换理论",
            "结构主义理论",
            "后现代理论",
            "批判理论",
            "建构主义理论",
        ]
    }

    /// 社会研究方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "问卷调查",
            "访谈研究",
            "观察研究",
            "实验研究",
            "文献研究",
            "统计分析",
            "质性研究",
            "量化研究",
        ]
    }

    /// 社会心理学定律
    pub fn social_psychology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("社会认知定律", "认知偏差", "社会认知偏差规律"),
            ("态度形成定律", "态度改变", "态度形成与改变"),
            ("群体心理定律", "群体心理", "群体心理效应"),
            ("偏见定律", "偏见形成", "偏见产生与消除"),
            ("社会促进定律", "他人在场", "他人在场影响表现"),
            ("去个性化定律", "匿名效应", "群体中去个性化现象"),
            ("社会认同定律", "群体认同", "社会认同理论"),
            ("归因定律", "因果归因", "行为归因规律"),
        ]
    }

    /// 文化社会学定律
    pub fn cultural_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("文化再生产定律", "文化传递", "布迪厄文化再生产理论"),
            ("文化资本定律", "资本积累", "文化资本社会功能"),
            ("亚文化定律", "亚文化群", "亚文化群体形成"),
            ("文化霸权定律", "意识形态", "葛兰西文化霸权理论"),
            ("消费文化定律", "消费社会", "鲍德里亚消费社会理论"),
            ("文化全球化定律", "文化交融", "全球化下文化变迁"),
            ("文化记忆定律", "集体记忆", "集体文化记忆机制"),
            ("符号消费定律", "符号价值", "消费的符号意义"),
        ]
    }

    /// 城市社会学定律
    pub fn urban_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("城市生态定律", "人类生态学", "帕克城市生态学理论"),
            ("同心圆定律", "城市结构", "伯吉斯同心圆模型"),
            ("城市化定律", "城市扩张", "城市化发展规律"),
            ("郊区化定律", "郊区扩散", "城市郊区化趋势"),
            ("绅士化定律", "社区变迁", "社区绅士化过程"),
            ("城市隔离定律", "居住隔离", "城市居住空间隔离"),
            ("城市治理定律", "城市治理", "城市治理模式理论"),
            ("智慧城市定律", "数字治理", "智慧城市发展规律"),
        ]
    }
}

impl Default for SociologyLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SociologyLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("sociology")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【社会学定律】\n\n结构定律:\n{}\n\n互动定律:\n{}\n\n变迁定律:\n{}\n\n社会心理学定律:\n{}\n\n文化社会学定律:\n{}\n\n城市社会学定律:\n{}\n",
            self.structure_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.interaction_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.change_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.social_psychology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.urban_laws().iter()
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
    fn test_sociology_laws() {
        let laws = SociologyLaws::new();
        assert!(!laws.structure_laws().is_empty());
        assert!(!laws.interaction_laws().is_empty());
    }
}
