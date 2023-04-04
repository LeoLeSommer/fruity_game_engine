use crate::ui::context::UIContext;
use crate::ui::elements::display::Text;
use crate::ui::elements::input::Button;
use crate::ui::elements::layout::Empty;
use crate::ui::elements::layout::Row;
use crate::ui::elements::layout::RowItem;
use crate::ui::elements::UIElement;
use crate::ui::elements::UISize;
use crate::ui::elements::UIWidget;
use fruity_game_engine::introspect::IntrospectFields;
use fruity_game_engine::introspect::IntrospectMethods;
use fruity_game_engine::resource::resource_reference::AnyResourceReference;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::script_value::convert::TryFromScriptValue;
use fruity_game_engine::script_value::ScriptObject;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::Arc;
use fruity_game_engine::FruityResult;

pub fn draw_editor_resource_reference<T: IntrospectFields + IntrospectMethods + ?Sized>(
    name: &str,
    value: Box<dyn ScriptObject>,
    on_update: Box<
        dyn Fn(&UIContext, Box<dyn ScriptObject>) -> FruityResult<()> + Send + Sync + 'static,
    >,
) -> FruityResult<UIElement> {
    let value =
        if let Ok(value) = ResourceReference::<T>::from_script_value(ScriptValue::Object(value)) {
            value
        } else {
            return Ok(Empty {}.elem());
        };

    Ok(Row {
        children: vec![
            RowItem {
                size: UISize::Units(40.0),
                child: Text {
                    text: name.to_string(),
                    ..Default::default()
                }
                .elem(),
            },
            RowItem {
                size: UISize::Fill,
                child: Button {
                    label: value.get_name(),
                    on_click: Arc::new(|_| Ok(())),
                    accept_drag: Some(Arc::new(|_, item| {
                        Ok(
                            if let Some(resource) = item.downcast_ref::<AnyResourceReference>() {
                                resource.downcast::<T>().is_some()
                            } else {
                                item.downcast_ref::<ResourceReference<T>>().is_some()
                            },
                        )
                    })),
                    on_drag: Some(Arc::new(move |ctx, resource| {
                        let resource = if let Some(resource) =
                            resource.downcast_ref::<AnyResourceReference>()
                        {
                            resource.downcast::<T>()
                        } else {
                            resource
                                .downcast_ref::<ResourceReference<T>>()
                                .map(|resource| resource.clone())
                        };

                        if let Some(resource) = resource {
                            on_update(ctx, Box::new(resource))?;
                        }

                        Ok(())
                    })),
                    ..Default::default()
                }
                .elem(),
                ..Default::default()
            },
        ],
        ..Default::default()
    }
    .elem())
}
