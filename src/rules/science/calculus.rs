//! 微积分定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CalculusRules, name: "微积分定律", desc: "微积分定律", origin: "国际", tags: ["科学", "数学"] }
impl CalculusRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "导数定义:f'(x)=lim[h->0](f(x+h)-f(x))/h",
            "基本导数:(x^n)’=n*x^(n-1), (sin x)’=cos x, (e^x)’=e^x",
            "链式法则:(f(g(x)))’=f'(g(x))*g’(x)",
            "乘积法则:(fg)’=f’g+fg'",
            "商法则:(f/g)’=(f’g-fg’)/g^2",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "不定积分:F(x)=int f(x)dx 表示F’(x)=f(x)",
            "基本积分:int x^n dx=x^(n+1)/(n+1)+C",
            "定积分:int_a^b f(x)dx=F(b)-F(a) 牛顿-莱布尼茨公式",
            "分部积分:int u dv=uv-int v du",
            "换元积分:int f(g(x))g’(x)dx=int f(u)du",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "极值:导数为零的点可能是极大或极小值",
            "二阶导数:f”>0极小值f”<0极大值",
            "曲线下面积:定积分的几何意义",
            "体积:旋转体体积=pi*int f(x)^2 dx",
            "弧长:int sqrt(1+(f’(x))^2) dx",
        ]
    }
}
impl Rule for CalculusRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("calculus")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "微积分定律",
            &[
                ("微分", &self.section_0()),
                ("积分", &self.section_1()),
                ("应用", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CalculusRules::new();
        assert!(!r.explain().is_empty());
    }
}
