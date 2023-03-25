use crate::editor_component_service::EditorComponentService;
use crate::editor_menu_service::MenuItem;
use crate::state::inspector::InspectorState;
use crate::ui::context::UIContext;
use crate::ui::elements::display::Popup;
use crate::ui::elements::display::Text;
use crate::ui::elements::input::Button;
use crate::ui::elements::input::Checkbox;
use crate::ui::elements::input::Input;
use crate::ui::elements::layout::Collapsible;
use crate::ui::elements::layout::Column;
use crate::ui::elements::layout::Empty;
use crate::ui::elements::layout::Row;
use crate::ui::elements::layout::RowItem;
use crate::ui::elements::layout::Scroll;
use crate::ui::elements::UIAlign;
use crate::ui::elements::UIElement;
use crate::ui::elements::UISize;
use crate::ui::elements::UIWidget;
use crate::ui::hooks::use_read_service;
use crate::ui::hooks::use_state;
use fruity_ecs::entity::entity_reference::EntityReference;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_game_engine::FruityResult;
use std::sync::Arc;

pub fn inspect_entity(
    ctx: &mut UIContext,
    entity: &mut EntityReference,
) -> FruityResult<UIElement> {
    let (component_search_text, set_component_search_text) = use_state(ctx, "".to_string());
    let (display_add_component_popup, set_display_add_component_popup) = use_state(ctx, false);
    let inspector_state = use_read_service::<InspectorState>(&ctx);
    let editor_component_service = use_read_service::<EditorComponentService>(&ctx);

    let entity_reader = entity.read()?;
    let entity_id = entity_reader.get_entity_id();
    let entity_2 = entity.clone();
    let entity_3 = entity.clone();
    let head = Column {
        children: vec![Row {
            children: vec![
                RowItem {
                    size: UISize::Units(50.0),
                    child: Checkbox {
                        label: "".to_string(),
                        value: entity_reader.is_enabled(),
                        on_change: Arc::new(move |_, value| {
                            let entity_writer = entity_2.write()?;
                            entity_writer.set_enabled(value);

                            Ok(())
                        }),
                    }
                    .elem(),
                },
                RowItem {
                    size: UISize::Fill,
                    child: Input {
                        value: entity_reader.get_name(),
                        placeholder: "Name ...".to_string(),
                        on_change: Arc::new(move |_, value: &str| {
                            let entity_writer = entity_3.write()?;
                            entity_writer.set_name(value);

                            Ok(())
                        }),
                        ..Default::default()
                    }
                    .elem(),
                },
            ],
            ..Default::default()
        }
        .elem()],
        align: UIAlign::default(),
    }
    .elem();

    let components = Column {
        children: entity
            .get_all_components()?
            .into_iter()
            .enumerate()
            .map(|(index, component)| {
                let class_name = {
                    let component_reader = component.read()?;
                    component_reader.get_class_name()?
                };

                FruityResult::Ok(
                    Collapsible {
                        key: format!("{}_{}", index, class_name),
                        title: class_name,
                        child: inspector_state.inspect_component(ctx, component)?,
                        secondary_actions: vec![MenuItem {
                            label: "Delete".to_string(),
                            action: Arc::new(move |ctx| {
                                // Get what we need
                                let entity_service = use_read_service::<EntityService>(&ctx);

                                // Remove the component
                                entity_service.remove_component(entity_id, index)?;

                                Ok(())
                            }),
                            options: Default::default(),
                        }],
                        ..Default::default()
                    }
                    .elem(),
                )
            })
            .try_collect::<Vec<_>>()?,
        align: UIAlign::Start,
    }
    .elem();

    let add_component = Column {
        children: vec![Button {
            label: "+".to_string(),
            on_click: Arc::new(move |_| {
                set_display_add_component_popup(!display_add_component_popup);
                Ok(())
            }),
            ..Default::default()
        }
        .elem()],
        align: UIAlign::Center,
    }
    .elem();

    let add_component_popup = if display_add_component_popup {
        Popup {
            content: Column {
                children: vec![
                    Row {
                        children: vec![
                            RowItem {
                                size: UISize::Units(40.0),
                                child: Text {
                                    text: "🔍".to_string(),
                                    ..Default::default()
                                }
                                .elem(),
                            },
                            RowItem {
                                size: UISize::Fill,
                                child: Input {
                                    value: component_search_text.clone(),
                                    placeholder: "Search ...".to_string(),
                                    on_edit: Arc::new(move |_, value| {
                                        set_component_search_text(value.to_string());
                                        Ok(())
                                    }),
                                    ..Default::default()
                                }
                                .elem(),
                            },
                        ],
                        ..Default::default()
                    }
                    .elem(),
                    Scroll {
                        child: Column {
                            children: editor_component_service
                                .search(&component_search_text)
                                .map(|component| {
                                    Button {
                                        label: component.clone(),
                                        on_click: Arc::new(move |ctx| {
                                            // Get what we need
                                            let entity_service =
                                                use_read_service::<EntityService>(&ctx);
                                            let editor_component_service =
                                                use_read_service::<EditorComponentService>(&ctx);

                                            // Add the component
                                            if let Some(components) =
                                                editor_component_service.instantiate(&component)?
                                            {
                                                entity_service
                                                    .add_components(entity_id, components)?;
                                            }

                                            Ok(())
                                        }),
                                        ..Default::default()
                                    }
                                    .elem()
                                })
                                .collect::<Vec<_>>(),
                            ..Default::default()
                        }
                        .elem(),
                        ..Default::default()
                    }
                    .elem(),
                ],
                align: UIAlign::Start,
            }
            .elem(),
        }
        .elem()
    } else {
        Empty {}.elem()
    };

    Ok(Scroll {
        child: Column {
            children: vec![head, components, add_component, add_component_popup],
            align: UIAlign::Start,
        }
        .elem(),
        ..Default::default()
    }
    .elem())
}
