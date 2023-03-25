use fruity_ecs::entity::entity_reference::EntityReference;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_editor::editor_menu_service::MenuItem;
use fruity_editor::state::inspector::InspectorState;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::input::Button;
use fruity_editor::ui::elements::layout::Collapsible;
use fruity_editor::ui::elements::layout::Column;
use fruity_editor::ui::elements::layout::Scroll;
use fruity_editor::ui::elements::UIElement;
use fruity_editor::ui::elements::UIWidget;
use fruity_editor::ui::hooks::use_read_service;
use fruity_editor::ui::hooks::use_service;
use fruity_editor::ui::hooks::use_write_service;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::FruityResult;
use fruity_hierarchy::components::parent::Parent;
use std::ops::Deref;
use std::sync::Arc;

pub fn entity_list_component(ctx: &mut UIContext) -> FruityResult<UIElement> {
    let entity_service = use_service::<EntityService>(ctx);
    let entity_service_reader = entity_service.read();

    let all_entities = entity_service_reader
        .iter_all_entities()
        .collect::<Vec<_>>();

    let root_entities = all_entities
        .iter()
        .filter(|entity| {
            if let Some(parent) = entity
                .read()
                .unwrap()
                .get_component_by_type::<Parent>()
                .unwrap()
            {
                if let Some(_) = *parent.parent {
                    false
                } else {
                    true
                }
            } else {
                true
            }
        })
        .collect::<Vec<_>>();

    Ok(Scroll {
        child: Column {
            children: root_entities
                .iter()
                .map(|child| {
                    draw_entity_line(ctx, (*child).clone(), &all_entities, entity_service.clone())
                })
                .try_collect::<Vec<_>>()?,
            ..Default::default()
        }
        .elem(),
        ..Default::default()
    }
    .elem())
}

pub fn draw_entity_line(
    ctx: &mut UIContext,
    entity: EntityReference,
    all_entities: &Vec<EntityReference>,
    entity_service: ResourceReference<EntityService>,
) -> FruityResult<UIElement> {
    let entity_2 = entity.clone();
    let entity_3 = entity.clone();
    let entity_reader = entity.read()?;
    let entity_id = entity_reader.get_entity_id();

    let children = all_entities
        .iter()
        .filter(|entity| {
            if let Some(parent) = entity
                .read()
                .unwrap()
                .get_component_by_type::<Parent>()
                .unwrap()
            {
                if let Some(parent) = parent.parent.deref() {
                    parent.get_entity_id().unwrap() == entity_id
                } else {
                    false
                }
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    if children.len() > 0 {
        let entity_service_2 = entity_service.clone();
        Ok(Collapsible {
            key: entity_reader.get_name(),
            title: entity_reader.get_name(),
            on_click: Some(Arc::new(move |ctx| {
                let mut inspector_state = use_write_service::<InspectorState>(ctx);
                inspector_state.select(Box::new(entity_2.clone()))
            })),
            child: Column {
                children: children
                    .iter()
                    .map(|child| {
                        draw_entity_line(
                            ctx,
                            (*child).clone(),
                            all_entities,
                            entity_service_2.clone(),
                        )
                    })
                    .try_collect::<Vec<_>>()?,
                ..Default::default()
            }
            .elem(),
            secondary_actions: vec![MenuItem {
                label: "Delete".to_string(),
                action: Arc::new(move |ctx| {
                    let entity_service = use_read_service::<EntityService>(ctx);
                    entity_service.remove(entity_id)
                }),
                options: Default::default(),
            }],
            ..Default::default()
        }
        .elem())
    } else {
        Ok(Button {
            label: entity_reader.get_name(),
            on_click: Arc::new(move |ctx| {
                let mut inspector_state = use_write_service::<InspectorState>(ctx);
                inspector_state.select(Box::new(entity_3.clone()))
            }),
            secondary_actions: vec![MenuItem {
                label: "Delete".to_string(),
                action: Arc::new(move |ctx| {
                    let entity_service = use_read_service::<EntityService>(ctx);
                    entity_service.remove(entity_id)
                }),
                options: Default::default(),
            }],
            ..Default::default()
        }
        .elem())
    }
}
