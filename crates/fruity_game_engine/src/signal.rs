use crate::any::FruityAny;
use crate::introspect::IntrospectFields;
use crate::introspect::IntrospectMethods;
use crate::lazy_static;
use crate::script_value::convert::TryFromScriptValue;
use crate::script_value::convert::TryIntoScriptValue;
use crate::script_value::ScriptObject;
use crate::script_value::ScriptValue;
use crate::typescript;
use crate::utils::introspect::ArgumentCaster;
use crate::FruityResult;
use crate::Mutex;
use crate::RwLock;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

struct IdGenerator {
    incrementer: usize,
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator { incrementer: 0 }
    }

    pub fn generate_id(&mut self) -> usize {
        self.incrementer += 1;
        self.incrementer
    }
}

lazy_static! {
    static ref ID_GENERATOR: Mutex<IdGenerator> = Mutex::new(IdGenerator::new());
}

/// An identifier for a signal observer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObserverIdentifier(usize);

#[derive(FruityAny)]
struct InternSignal<T: 'static> {
    observers: Vec<(
        ObserverIdentifier,
        Arc<dyn Sync + Send + Fn(&T) -> FruityResult<()>>,
    )>,
}

/// An observer pattern
#[derive(FruityAny)]
#[typescript(
    "interface Signal<T> {
  notify(event: T);
  addObserver(callback: (value: T) => void);
}"
)]
pub struct Signal<T: 'static> {
    intern: Arc<RwLock<InternSignal<T>>>,
}

impl<T: 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            intern: self.intern.clone(),
        }
    }
}

impl<T> Signal<T> {
    /// Returns a Signal
    pub fn new() -> Signal<T> {
        Signal {
            intern: Arc::new(RwLock::new(InternSignal {
                observers: Vec::new(),
            })),
        }
    }

    /// Add an observer to the signal
    /// An observer is a closure that will be called when the signal will be sent
    /// Returns an handler, the handler can be duplicated but when the last observer is dropped,
    /// the observer is unregistered
    pub fn add_observer<F: Sync + Send + Fn(&T) -> FruityResult<()> + 'static>(
        &self,
        observer: F,
    ) -> ObserverHandler<T> {
        let mut intern_writer = self.intern.write();

        let mut id_generator = ID_GENERATOR.lock();
        let observer_id = ObserverIdentifier(id_generator.generate_id());
        intern_writer
            .observers
            .push((observer_id, Arc::new(observer)));

        ObserverHandler {
            observer_id,
            intern: self.intern.clone(),
        }
    }

    /// Add an observer to the signal that can dispose itself
    /// An observer is a closure that will be called when the signal will be sent
    pub fn add_self_dispose_observer<
        F: Sync + Send + Fn(&T, &ObserverHandler<T>) -> FruityResult<()> + 'static,
    >(
        &self,
        observer: F,
    ) {
        let mut intern_writer = self.intern.write();

        let mut id_generator = ID_GENERATOR.lock();
        let observer_id = ObserverIdentifier(id_generator.generate_id());

        let handler = ObserverHandler {
            observer_id,
            intern: self.intern.clone(),
        };

        intern_writer
            .observers
            .push((observer_id, Arc::new(move |data| observer(data, &handler))));
    }

    /// Notify that the event happened
    /// This will launch all the observers that are registered for this signal
    pub fn notify(&self, event: T) -> FruityResult<()> {
        let observers = {
            let intern = self.intern.read();
            intern.observers.clone()
        };

        observers
            .iter()
            .try_for_each(|(_, observer)| observer(&event))
    }
}

impl<T> Default for Signal<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Debug for Signal<T> {
    fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<T> IntrospectFields for Signal<T>
where
    T: TryFromScriptValue + TryIntoScriptValue,
{
    fn get_class_name(&self) -> FruityResult<String> {
        Ok("Signal".to_string())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn set_field_value(&mut self, _name: &str, _value: ScriptValue) -> FruityResult<()> {
        unreachable!()
    }

    fn get_field_value(&self, _name: &str) -> FruityResult<ScriptValue> {
        unreachable!()
    }
}

impl<T> IntrospectMethods for Signal<T>
where
    T: TryFromScriptValue + TryIntoScriptValue + Clone,
{
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec!["notify".to_string()])
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        match name {
            "notify" => {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<T>()?;

                let handle = self.notify(arg1);

                handle.into_script_value()
            }
            _ => unreachable!(),
        }
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec!["add_observer".to_string()])
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        match name {
            "add_observer" => {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster
                    .cast_next::<Arc<dyn Send + Sync + Fn(T) -> FruityResult<ScriptValue>>>()?;

                let handle = self.add_observer(move |arg| {
                    arg1(arg.clone())?;

                    Ok(())
                });

                handle.into_script_value()
            }
            _ => unreachable!(),
        }
    }
}

impl IntrospectMethods for Signal<Box<dyn ScriptObject>> {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec!["notify".to_string()])
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        match name {
            "notify" => {
                let mut caster = ArgumentCaster::new(args);
                let arg1 = caster.cast_next::<Box<dyn ScriptObject>>()?;

                let handle = self.notify(arg1);

                handle.into_script_value()
            }
            _ => unreachable!(),
        }
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec!["add_observer".to_string()])
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        match name {
            "add_observer" => {
                let mut caster = ArgumentCaster::new(args);
                let arg1 =
                    caster.cast_next::<Arc<
                        dyn Send + Sync + Fn(Box<dyn ScriptObject>) -> FruityResult<ScriptValue>,
                    >>()?;

                let handle = self.add_observer(move |arg| {
                    arg1(arg.duplicate())?;

                    Ok(())
                });

                handle.into_script_value()
            }
            _ => unreachable!(),
        }
    }
}

/// A write guard over a signal property, when it's dropped, the update signal is sent
pub struct SignalWriteGuard<'a, T: Send + Sync + Clone + 'static> {
    target: &'a mut SignalProperty<T>,
}

impl<'a, T: Send + Sync + Clone> Drop for SignalWriteGuard<'a, T> {
    fn drop(&mut self) {
        self.target
            .on_updated
            .notify(self.target.value.clone())
            .unwrap();
    }
}

impl<'a, T: Send + Sync + Clone> Deref for SignalWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.target.value
    }
}

impl<'a, T: Send + Sync + Clone> DerefMut for SignalWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.target.value
    }
}

/// A variable with a signal that is notified on update
#[derive(Clone, FruityAny)]
#[typescript(
    "interface SignalProperty<T> {
  value: T;
  onUpdated: Signal<T>;
}"
)]
pub struct SignalProperty<T: Send + Sync + Clone + 'static> {
    value: T,

    /// A signal sent when the property is updated
    pub on_updated: Signal<T>,
}

impl<T: Send + Sync + Clone + Default> Default for SignalProperty<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Send + Sync + Clone> SignalProperty<T> {
    /// Returns a SignalProperty
    pub fn new(value: T) -> Self {
        Self {
            value,
            on_updated: Signal::new(),
        }
    }

    /// Returns a SignalProperty
    pub fn write(&mut self) -> SignalWriteGuard<T> {
        SignalWriteGuard::<T> { target: self }
    }
}

impl<T: Send + Sync + Clone> Deref for SignalProperty<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Send + Sync + Clone + Debug> Debug for SignalProperty<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.value.fmt(formatter)
    }
}

impl<T> IntrospectFields for SignalProperty<T>
where
    T: TryIntoScriptValue + TryFromScriptValue + Send + Sync + Clone + Debug,
{
    fn get_class_name(&self) -> FruityResult<String> {
        Ok("SignalProperty".to_string())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec!["value".to_string(), "on_updated".to_string()])
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        match name {
            "value" => self.value = T::from_script_value(value)?,
            _ => unreachable!(),
        };

        FruityResult::Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        match name {
            "value" => self.value.clone().into_script_value(),
            "on_updated" => self.on_updated.clone().into_script_value(),
            _ => unreachable!(),
        }
    }
}

impl<T> IntrospectMethods for SignalProperty<T>
where
    T: TryIntoScriptValue + TryFromScriptValue + Send + Sync + Clone + Debug,
{
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_const_method(&self, _name: &str, _args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        unreachable!()
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_mut_method(
        &mut self,
        _name: &str,
        _args: Vec<ScriptValue>,
    ) -> FruityResult<ScriptValue> {
        unreachable!()
    }
}

/// A signal subscription handler, can be used to unsubscribe the signal
#[derive(FruityAny)]
#[typescript(
    "interface ObserverHandler {
  dispose();
}"
)]
pub struct ObserverHandler<T: 'static> {
    observer_id: ObserverIdentifier,
    intern: Arc<RwLock<InternSignal<T>>>,
}

impl<T> ObserverHandler<T> {
    /// Remove an observer from the signal
    pub fn dispose(self) {
        let mut intern = self.intern.write();
        let observer_index = intern
            .observers
            .iter()
            .enumerate()
            .find(|(_index, elem)| elem.0 == self.observer_id)
            .map(|elem| elem.0);

        if let Some(observer_index) = observer_index {
            let _ = intern.observers.remove(observer_index);
        }
    }

    /// Remove an observer from the signal
    pub fn dispose_by_ref(&self) {
        let mut intern = self.intern.write();
        let observer_index = intern
            .observers
            .iter()
            .enumerate()
            .find(|(_index, elem)| elem.0 == self.observer_id)
            .map(|elem| elem.0);

        if let Some(observer_index) = observer_index {
            let _ = intern.observers.remove(observer_index);
        }
    }
}

impl<T> IntrospectFields for ObserverHandler<T>
where
    T: TryFromScriptValue + TryIntoScriptValue,
{
    fn get_class_name(&self) -> FruityResult<String> {
        Ok("ObserverHandler".to_string())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn set_field_value(&mut self, _name: &str, _value: ScriptValue) -> FruityResult<()> {
        unreachable!()
    }

    fn get_field_value(&self, _name: &str) -> FruityResult<ScriptValue> {
        unreachable!()
    }
}

impl<T> IntrospectMethods for ObserverHandler<T>
where
    T: TryFromScriptValue + TryIntoScriptValue,
{
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec!["dispose".to_string()])
    }

    fn call_const_method(&self, name: &str, _args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        match name {
            "dispose" => self.dispose_by_ref().into_script_value(),
            _ => unreachable!(),
        }
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_mut_method(
        &mut self,
        _name: &str,
        _args: Vec<ScriptValue>,
    ) -> FruityResult<ScriptValue> {
        unreachable!()
    }
}

impl<T> Clone for ObserverHandler<T> {
    fn clone(&self) -> Self {
        Self {
            observer_id: self.observer_id.clone(),
            intern: self.intern.clone(),
        }
    }
}

impl<T> Debug for ObserverHandler<T> {
    fn fmt(&self, _: &mut Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
