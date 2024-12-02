use std::{
    marker::PhantomData,
    sync::{Arc, RwLock, Weak},
};

// * Guard
/// Guard to make sure [SymbolBox] can't be dropped when it's accessed via [SymbolRef] (or the other way around)
pub struct Guard<'a, T: ?Sized>(Arc<RwLock<T>>, PhantomData<&'a T>);
impl<T: ?Sized> std::ops::Deref for Guard<'_, T> {
    type Target = RwLock<T>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

// * SymbolBox
/// Smart pointer for your symbols, so they can be referenced using [SymbolRef]
pub struct SymbolBox<T, H: ?Sized> {
    object: Arc<RwLock<T>>,
    references: Vec<Weak<RwLock<H>>>,
}

impl<T, H: ?Sized> SymbolBox<T, H> {
    /// Create a new [SymbolBox] from it's contents
    pub fn new(object: T) -> Self {
        Self {
            object: Arc::new(RwLock::new(object)),
            references: Vec::new(),
        }
    }

    /// Access contents of this [SymbolBox]
    #[inline]
    pub fn object(&self) -> &RwLock<T> {
        &self.object
    }

    /// Get a list of references to this [SymbolBox] and return their [SymbolRefHandler]s
    pub fn references(&mut self) -> Vec<Guard<H>> {
        let mut references = Vec::with_capacity(self.references.len());
        self.references.retain(|reference| {
            if let Some(reference) = reference.upgrade() {
                references.push(Guard(reference, PhantomData));
                true
            } else {
                false
            }
        });

        references
    }

    /// Create a new [SymbolRef] referencing this [SymbolBox]
    #[inline]
    pub fn new_ref(&mut self, handler: H) -> SymbolRef<T, H> {
        SymbolRef::new(self, handler)
    }

    /// Create a new [SymbolRef] referencing this [SymbolBox]
    #[inline]
    pub fn new_ref_unsize<U: ?Sized>(
        &mut self,
        handler: impl SymbolRefHandler + 'static,
    ) -> SymbolRef<U>
    where
        T: std::marker::Unsize<U>,
    {
        SymbolRef::new_unsize(self, handler)
    }
}

// * SymbolRef
/// SymbolRefHandler is an object that is stored in every [SymbolRef] and can also be accessed via [`SymbolBox::references`]
/// It provides methods to act on events like LSP rename, go to references, etc.
pub trait SymbolRefHandler {
    /// Get the name of the symbol
    fn name(&self) -> std::borrow::Cow<str>;
}

impl SymbolRefHandler for () {
    fn name(&self) -> std::borrow::Cow<str> {
        "<???>".into()
    }
}

/// Reference to [SymbolBox], invalidates, if SymbolBox drops
pub struct SymbolRef<T: ?Sized, H: ?Sized> {
    object: Option<Weak<RwLock<T>>>,
    handler: Arc<RwLock<H>>,
}

impl<H, T: ?Sized> SymbolRef<H, T> {
    /// Create a new SymbolRef from [SymbolBoxAccess]
    pub fn new<BH>(symbol_box: &mut SymbolBox<T, BH>, handler: H) -> Self
    where
        T: Sized,
        BH: std::marker::Unsize<H>,
    {
        let handler = Arc::new(RwLock::new(handler));
        symbol_box.references.push(Arc::downgrade(&handler));

        Self {
            object: Some(Arc::downgrade(
                &(symbol_box.object.clone() as Arc<RwLock<T>>),
            )),
            handler,
        }
    }

    /// Create a new SymbolRef from [SymbolBoxAccess]
    pub fn new_unsize(
        symbol_box: &mut SymbolBox<impl std::marker::Unsize<T>>,
        handler: impl std::marker::Unsize<H>,
    ) -> Self {
        let handler = Arc::new(RwLock::new(handler));
        symbol_box.references.push(Arc::downgrade(&handler));

        Self {
            object: Some(Arc::downgrade(
                &(symbol_box.object.clone() as Arc<RwLock<T>>),
            )),
            handler,
        }
    }

    /// Create an unbound [SymbolRef], [`Self::object`] returns None until it's bound using [`Self::bind`]
    pub fn unbound(handler: impl std::marker::Unsize<H>) -> Self {
        let handler = Arc::new(RwLock::new(handler));
        Self {
            object: None,
            handler,
        }
    }

    /// Access contents of the [SymbolBox]
    pub fn object(&self) -> Option<Guard<T>> {
        self.object
            .as_ref()
            .and_then(|object| object.upgrade().map(|object| Guard(object, PhantomData)))
    }

    /// Get the [SymbolRefHandler] associated with this [SymbolRef]
    pub fn handler(&self) -> &RwLock<dyn SymbolRefHandler> {
        &self.handler
    }
}

#[test]
fn test() {
    use assert2::*;
    let mut symbol_box = SymbolBox::new(42);
    *symbol_box.object().try_write().unwrap() = 69;
    check!(*symbol_box.object().try_read().unwrap() == 69);

    let symbol_ref = symbol_box.new_ref(());
    *symbol_box.object().try_write().unwrap() += 1;
    check!(*symbol_box.object().try_read().unwrap() == 70);
    check!(*symbol_ref.object().unwrap().try_read().unwrap() == 70);
}

#[test]
fn test_safety_drop_ref() {
    use assert2::*;
    let mut symbol_box = SymbolBox::<i32>::new(42);
    check!(symbol_box.references().len() == 0);
    check!(Arc::strong_count(&symbol_box.object) == 1);
    check!(Arc::weak_count(&symbol_box.object) == 0);
    let symbol_ref = symbol_box.new_ref(());
    check!(symbol_box.references().len() == 1);
    check!(Arc::strong_count(&symbol_box.object) == 1);
    check!(Arc::weak_count(&symbol_box.object) == 1);
    drop(symbol_ref);
    check!(symbol_box.references().len() == 0);
    check!(Arc::strong_count(&symbol_box.object) == 1);
    check!(Arc::weak_count(&symbol_box.object) == 0);
}

#[test]
fn test_safety_drop_box() {
    use assert2::*;
    let mut symbol_box = SymbolBox::new(42);
    let symbol_ref = symbol_box.new_ref(());
    check!(Arc::strong_count(&symbol_ref.handler) == 1);
    check!(Arc::weak_count(&symbol_ref.handler) == 1);
    drop(symbol_box);
    check!(Arc::strong_count(&symbol_ref.handler) == 1);
    check!(Arc::weak_count(&symbol_ref.handler) == 0);
    check!(symbol_ref.object().is_none());
}
