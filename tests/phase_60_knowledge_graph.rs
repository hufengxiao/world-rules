//! Phase 60 测试 - 规则知识图谱

use world_rules::knowledge_graph::{
    Entity, EntityExtractor, EntityType, GraphEdge, GraphNode, GraphStats, InferenceEngine,
    InferenceResult, InferenceRule, KnowledgeGraph, Relation, RelationExtractor, RelationType,
};

#[test]
fn test_entity_type_properties() {
    assert_eq!(EntityType::Concept.to_string(), "概念");
    assert_eq!(EntityType::Action.to_string(), "动作");
    assert!(EntityType::Action.is_action());
    assert!(EntityType::Condition.is_conditional());
    assert!(EntityType::Result.is_conditional());
    assert!(!EntityType::Concept.is_conditional());
}

#[test]
fn test_entity_creation() {
    let entity = Entity::new("球员", EntityType::Subject)
        .with_confidence(0.95)
        .with_source("足球规则")
        .with_attribute("数量", "11人")
        .with_synonyms(vec!["运动员", "选手"]);

    assert_eq!(entity.name, "球员");
    assert_eq!(entity.entity_type, EntityType::Subject);
    assert_eq!(entity.confidence, 0.95);
    assert!(entity.matches("球员"));
    assert!(entity.matches("运动员"));
    assert!(!entity.matches("裁判"));
}

#[test]
fn test_entity_extractor_basic() {
    let extractor = EntityExtractor::new();
    let entities = extractor.extract("球员在比赛中犯规会被出示黄牌");

    assert!(!entities.is_empty());

    // 应该检测到主体
    let has_subject = entities
        .iter()
        .any(|e| e.entity_type == EntityType::Subject);
    assert!(has_subject);

    // 应该检测到动作
    let has_action = entities.iter().any(|e| e.entity_type == EntityType::Action);
    assert!(has_action);
}

#[test]
fn test_entity_extractor_quantities() {
    let extractor = EntityExtractor::new();
    let entities = extractor.extract("比赛时间为90分钟，每队11人");

    let quantities: Vec<_> = entities
        .iter()
        .filter(|e| e.entity_type == EntityType::Quantity)
        .collect();

    assert!(!quantities.is_empty());
}

#[test]
fn test_relation_type_properties() {
    assert_eq!(RelationType::Causes.to_string(), "导致");
    assert!(RelationType::Causes.is_causal());
    assert!(RelationType::Requires.is_causal());
    assert!(RelationType::Implies.is_causal());
    assert!(!RelationType::Permits.is_causal());

    assert!(RelationType::Contradicts.is_conflict());
    assert!(RelationType::Prohibits.is_conflict());
    assert!(!RelationType::Causes.is_conflict());

    assert!(RelationType::Permits.is_positive());
    assert!(RelationType::Equivalent.is_positive());
    assert!(!RelationType::Prohibits.is_positive());
}

#[test]
fn test_relation_creation() {
    let relation = Relation::new("entity1", "entity2", RelationType::Causes)
        .with_confidence(0.85)
        .with_source("测试规则");

    assert_eq!(relation.source_entity, "entity1");
    assert_eq!(relation.target_entity, "entity2");
    assert_eq!(relation.relation_type, RelationType::Causes);
    assert!(relation.involves("entity1"));
    assert!(relation.involves("entity2"));
    assert!(!relation.involves("entity3"));
}

#[test]
fn test_relation_extractor() {
    let entity_extractor = EntityExtractor::new();
    let entities = entity_extractor.extract("球员在比赛中犯规会被出示黄牌");

    let relation_extractor = RelationExtractor::new();
    let relations = relation_extractor.extract(&entities, "球员在比赛中犯规会被出示黄牌");

    assert!(!relations.is_empty());

    // 应该有参与关系
    let has_participates = relations
        .iter()
        .any(|r| r.relation_type == RelationType::Participates);
    assert!(has_participates);
}

#[test]
fn test_knowledge_graph_basic() {
    let mut graph = KnowledgeGraph::new("体育规则");

    assert_eq!(graph.name, "体育规则");
    assert_eq!(graph.node_count(), 0);
    assert_eq!(graph.edge_count(), 0);

    graph.add_entity(Entity::new("球员", EntityType::Subject));
    assert_eq!(graph.node_count(), 1);

    graph.add_entities(vec![
        Entity::new("犯规", EntityType::Action),
        Entity::new("黄牌", EntityType::Result),
    ]);
    assert_eq!(graph.node_count(), 3);
}

#[test]
fn test_knowledge_graph_relations() {
    let mut graph = KnowledgeGraph::new("体育规则");

    graph.add_entity(Entity::new("球员", EntityType::Subject));
    graph.add_entity(Entity::new("犯规", EntityType::Action));

    graph.add_relation(Relation::new(
        "主体_球员",
        "动作_犯规",
        RelationType::Participates,
    ));

    assert_eq!(graph.edge_count(), 1);

    // 检查节点度数更新
    let source_node = graph.get_node("主体_球员").unwrap();
    assert_eq!(source_node.out_degree, 1);

    let target_node = graph.get_node("动作_犯规").unwrap();
    assert_eq!(target_node.in_degree, 1);
}

#[test]
fn test_graph_node_properties() {
    let entity = Entity::new("测试", EntityType::Subject);
    let node = GraphNode::from_entity(entity);

    assert_eq!(node.degree(), 0);
    assert!(node.is_leaf());
    assert!(node.is_root());
}

#[test]
fn test_knowledge_graph_filtering() {
    let mut graph = KnowledgeGraph::new("测试图谱");

    graph.add_entities(vec![
        Entity::new("球员", EntityType::Subject),
        Entity::new("教练", EntityType::Subject),
        Entity::new("犯规", EntityType::Action),
    ]);

    let subjects = graph.filter_nodes_by_type(EntityType::Subject);
    assert_eq!(subjects.len(), 2);

    let actions = graph.filter_nodes_by_type(EntityType::Action);
    assert_eq!(actions.len(), 1);
}

#[test]
fn test_knowledge_graph_path_finding() {
    let mut graph = KnowledgeGraph::new("测试图谱");

    graph.add_entities(vec![
        Entity::new("A", EntityType::Subject),
        Entity::new("B", EntityType::Object),
        Entity::new("C", EntityType::Result),
    ]);

    graph.add_relation(Relation::new("主体_A", "客体_B", RelationType::Causes));
    graph.add_relation(Relation::new("客体_B", "结果_C", RelationType::Causes));

    let path = graph.find_path("主体_A", "结果_C");
    assert!(path.is_some());

    let path = path.unwrap();
    assert_eq!(path.len(), 3);
    assert_eq!(path[0], "主体_A");
    assert_eq!(path[2], "结果_C");
}

#[test]
fn test_knowledge_graph_stats() {
    let mut graph = KnowledgeGraph::new("测试图谱");

    graph.add_entities(vec![
        Entity::new("球员", EntityType::Subject),
        Entity::new("犯规", EntityType::Action),
    ]);

    graph.add_relation(Relation::new(
        "主体_球员",
        "动作_犯规",
        RelationType::Participates,
    ));

    let stats = graph.get_stats();

    assert_eq!(stats.node_count, 2);
    assert_eq!(stats.edge_count, 1);
    assert!(stats.avg_degree > 0.0);
}

#[test]
fn test_knowledge_graph_dot_export() {
    let mut graph = KnowledgeGraph::new("测试图谱");

    graph.add_entity(Entity::new("A", EntityType::Subject));
    graph.add_entity(Entity::new("B", EntityType::Action));
    graph.add_relation(Relation::new("主体_A", "动作_B", RelationType::Causes));

    let dot = graph.to_dot();
    assert!(dot.contains("digraph"));
    assert!(dot.contains("主体_A"));
    assert!(dot.contains("动作_B"));
}

#[test]
fn test_knowledge_graph_json() {
    let mut graph = KnowledgeGraph::new("测试图谱");
    graph.add_entity(Entity::new("测试", EntityType::Subject));

    let json = graph.to_json().unwrap();
    assert!(json.contains("测试图谱"));
    assert!(json.contains("测试"));

    let parsed = KnowledgeGraph::from_json(&json).unwrap();
    assert_eq!(parsed.name, "测试图谱");
    assert_eq!(parsed.node_count(), 1);
}

#[test]
fn test_inference_engine_creation() {
    let graph = KnowledgeGraph::new("测试图谱");
    let engine = InferenceEngine::new(graph);

    assert_eq!(engine.get_graph().name, "测试图谱");
}

#[test]
fn test_inference_engine_query() {
    let mut graph = KnowledgeGraph::new("测试图谱");
    graph.add_entity(Entity::new("球员", EntityType::Subject));

    let engine = InferenceEngine::new(graph);

    let result = engine.query("球员");
    assert!(result.is_success());

    let result = engine.query("不存在");
    assert!(!result.is_success());
}

#[test]
fn test_inference_engine_consistency() {
    let graph = KnowledgeGraph::new("测试图谱");
    let engine = InferenceEngine::new(graph);

    let result = engine.validate_consistency();
    assert!(result.is_success());
}

#[test]
fn test_inference_rule_creation() {
    let rule = InferenceRule::new("测试规则", "这是一个测试规则").with_confidence(0.9);

    assert_eq!(rule.name, "测试规则");
    assert_eq!(rule.description, "这是一个测试规则");
    assert_eq!(rule.confidence, 0.9);
}

#[test]
fn test_inference_result() {
    let result = InferenceResult::success("推理成功").with_confidence(0.85);

    assert!(result.is_success());
    assert_eq!(result.confidence, 0.85);

    let failure = InferenceResult::failure("推理失败");
    assert!(!failure.is_success());
}

#[test]
fn test_complete_workflow() {
    // 1. 创建实体抽取器并抽取实体
    let entity_extractor = EntityExtractor::new();
    let entities = entity_extractor.extract("球员犯规会被出示黄牌");

    // 2. 创建知识图谱并添加实体
    let mut graph = KnowledgeGraph::new("足球规则");
    graph.add_entities(entities);

    // 3. 创建关系抽取器并抽取关系
    let relation_extractor = RelationExtractor::new();
    let relations = relation_extractor.extract(
        graph
            .get_all_nodes()
            .iter()
            .map(|n| Entity::new(&n.name, n.entity_type))
            .collect::<Vec<_>>()
            .as_slice(),
        "球员犯规会被出示黄牌",
    );
    graph.add_relations(relations);

    // 4. 获取统计信息
    let stats = graph.get_stats();
    assert!(stats.node_count > 0);

    // 5. 创建推理引擎
    let engine = InferenceEngine::new(graph);
    let consistency = engine.validate_consistency();
    assert!(consistency.is_success());
}
