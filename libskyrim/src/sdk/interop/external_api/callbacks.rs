use core::fmt;

/// ABI used by flat exported callback-registration symbols.
///
/// This matches the common "register returns token/id" pattern used by several
/// SKSE plugin APIs that expose callback registration through exported symbols.
pub type RegisterCallbackFn<Callback, Id> = unsafe extern "system" fn(Callback) -> Id;

/// ABI used by flat exported callback-unregistration symbols.
///
/// The `Id` must be the same token returned by the matching
/// [`RegisterCallbackFn`].
pub type UnregisterCallbackFn<Id> = unsafe extern "system" fn(Id);

/// ABI used by one flat exported subscriber-style symbol.
///
/// This pattern is common when another plugin exports one "add subscriber"
/// function instead of a full `RequestPluginAPI` table.
pub type ExportedSubscriberFn<Arg> = unsafe extern "system" fn(Arg);

/// One typed wrapper around a flat exported subscriber-style function.
///
/// Use this when another plugin exposes a single exported function such as
/// `AddSubscriber(...)` and there is no matching unregister step.
#[derive(Clone, Copy)]
pub struct ExportedSubscriber<Arg> {
    subscribe: ExportedSubscriberFn<Arg>,
}

impl<Arg> ExportedSubscriber<Arg> {
    #[inline(always)]
    pub const fn new(subscribe: ExportedSubscriberFn<Arg>) -> Self {
        Self { subscribe }
    }

    #[inline(always)]
    pub const fn subscriber_fn(&self) -> ExportedSubscriberFn<Arg> {
        self.subscribe
    }

    /// # Safety
    /// The caller must ensure that `arg` satisfies the exported symbol's ABI,
    /// ownership, and lifetime contract.
    #[inline(always)]
    pub unsafe fn subscribe(&self, arg: Arg) {
        unsafe { (self.subscribe)(arg) }
    }
}

impl<Arg> fmt::Debug for ExportedSubscriber<Arg> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExportedSubscriber")
            .field(
                "subscribe",
                &format_args!("{:p}", self.subscribe as *const ()),
            )
            .finish()
    }
}

/// One typed pair of exported callback registration symbols.
///
/// This wraps the common exported-symbol pair:
///
/// - `RegisterSomethingCallback(...) -> Id`
/// - `UnregisterSomethingCallback(Id)`
///
/// and returns an [`ExportedCallbackRegistration`] token that unregisters on
/// drop.
#[derive(Clone, Copy)]
pub struct ExportedCallbackRegistrar<Callback, Id> {
    register: RegisterCallbackFn<Callback, Id>,
    unregister: UnregisterCallbackFn<Id>,
}

impl<Callback, Id> ExportedCallbackRegistrar<Callback, Id> {
    #[inline(always)]
    pub const fn new(
        register: RegisterCallbackFn<Callback, Id>,
        unregister: UnregisterCallbackFn<Id>,
    ) -> Self {
        Self {
            register,
            unregister,
        }
    }

    #[inline(always)]
    pub const fn register_fn(&self) -> RegisterCallbackFn<Callback, Id> {
        self.register
    }

    #[inline(always)]
    pub const fn unregister_fn(&self) -> UnregisterCallbackFn<Id> {
        self.unregister
    }

    /// # Safety
    /// The caller must ensure that the callback value has the ABI and lifetime
    /// contract expected by the target plugin.
    #[inline(always)]
    pub unsafe fn register(&self, callback: Callback) -> ExportedCallbackRegistration<Id> {
        let id = unsafe { (self.register)(callback) };
        ExportedCallbackRegistration::new(id, self.unregister)
    }
}

impl<Callback, Id> fmt::Debug for ExportedCallbackRegistrar<Callback, Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExportedCallbackRegistrar")
            .field(
                "register",
                &format_args!("{:p}", self.register as *const ()),
            )
            .field(
                "unregister",
                &format_args!("{:p}", self.unregister as *const ()),
            )
            .finish()
    }
}

/// RAII token that unregisters one flat exported callback when dropped.
///
/// This is intentionally tiny and ownership-oriented:
///
/// - if the token is dropped, unregister runs
/// - if the caller needs to keep the raw registration id, [`forget`](Self::forget)
///   disables the auto-unregister behavior and returns that id
pub struct ExportedCallbackRegistration<Id> {
    id: Option<Id>,
    unregister: UnregisterCallbackFn<Id>,
}

impl<Id> ExportedCallbackRegistration<Id> {
    #[inline(always)]
    pub const fn new(id: Id, unregister: UnregisterCallbackFn<Id>) -> Self {
        Self {
            id: Some(id),
            unregister,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> Option<&Id> {
        self.id.as_ref()
    }

    #[inline(always)]
    pub const fn unregister_fn(&self) -> UnregisterCallbackFn<Id> {
        self.unregister
    }

    /// Prevent auto-unregister on drop and return the raw registration id.
    #[inline(always)]
    pub fn forget(mut self) -> Option<Id> {
        self.id.take()
    }
}

impl<Id: fmt::Debug> fmt::Debug for ExportedCallbackRegistration<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExportedCallbackRegistration")
            .field("id", &self.id)
            .field(
                "unregister",
                &format_args!("{:p}", self.unregister as *const ()),
            )
            .finish()
    }
}

impl<Id> Drop for ExportedCallbackRegistration<Id> {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            unsafe {
                (self.unregister)(id);
            }
        }
    }
}
