/// Sample of what the new codegen may look like

#[java_oxide(
    assignable = crate::java::lang::Object,
    signature = c"net/fabricmc/fabric/api/attachment/v1/AttachmentRegistry",
)]
/// final class [AttachmentRegistry](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentRegistry.html)
pub enum AttachmentRegistry {}

impl AttachmentRegistry {
    /// [builder](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentRegistry.html#builder())
    #[deprecated]
    pub fn builder<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> Result<
        Option<::java_oxide::Local<'env, AttachmentRegistry_Builder>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        ::java_oxide::call_method!(
            type: static object,
            method: c"builder",
            sig: c"()Lnet/fabricmc/fabric/api/attachment/v1/AttachmentRegistry$Builder;"
        )
    }
}

#[java_oxide(
    assignable = crate::java::lang::Object,
    signature = c"net/fabricmc/fabric/api/attachment/v1/AttachmentTarget",
)]
/// interface [AttachmentTarget](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentTarget.html)
pub enum AttachmentTarget {}

impl AttachmentTarget {
    /// public static final [NBT_ATTACHMENT_KEY](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentTarget.html#NBT_ATTACHMENT_KEY)
    pub const NBT_ATTACHMENT_KEY: &'static str = "fabric:attachments";

    /// [getAttached](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentTarget.html#getAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType))
    pub fn getAttached<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<AttachmentType>,
    ) -> Result<
        Option<::java_oxide::Local<'env, crate::java::lang::Object>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        ::java_oxide::call_method!(
            type: object,
            method: c"getAttached",
            sig: c"(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;)Ljava/lang/Object;",
            arg0,
        )
    }

    /// [getAttachedOrSet](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentTarget.html#getAttachedOrSet(net.fabricmc.fabric.api.attachment.v1.AttachmentType.java.lang.Object))
    pub fn getAttachedOrSet<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<AttachmentType>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> Result<
        Option<::java_oxide::Local<'env, crate::java::lang::Object>>,
        ::java_oxide::Local<'env, crate::java::lang::Throwable>,
    > {
        ::java_oxide::call_method!(
            type: object,
            method: c"getAttachedOrSet",
            sig: c"(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;Ljava/lang/Object;)Ljava/lang/Object;",
            arg0, arg1,
        )
    }

    /// [hasAttached](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentTarget.html#hasAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType))
    pub fn hasAttached<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<AttachmentType>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
        ::java_oxide::call_method!(
            type: boolean,
            method: c"hasAttached",
            sig: c"(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;)Z",
            arg0,
        )
    }
}


///
///  What the code would previously look like
///


pub enum AttachmentRegistry {}
unsafe impl ::java_oxide::ReferenceType for AttachmentRegistry {}
unsafe impl ::java_oxide::JniType for AttachmentRegistry {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/fabric/api/attachment/v1/AttachmentRegistry")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for AttachmentRegistry {}
impl AttachmentRegistry {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_oxide::Local::from_raw(__jni_env, __jni_env.require_class(c"net/fabricmc/fabric/api/attachment/v1/AttachmentRegistry")).as_global()
            })
            .as_raw()
    }
    ///[builder](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentRegistry.html#builder())
    #[deprecated]
    pub fn builder<'env>(
        __jni_env: ::java_oxide::Env<'env>,
    ) -> ::std::result::Result<::std::option::Option<::java_oxide::Local<'env, AttachmentRegistry_Builder>>, ::java_oxide::Local<'env, crate::java::lang::Throwable>>
    {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> = ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_static_method(
                        __jni_class,
                        c"builder",
                        c"()Lnet/fabricmc/fabric/api/attachment/v1/AttachmentRegistry$Builder;",
                    ))
                })
                .as_raw();
            __jni_env.call_static_object_method_a(__jni_class, __jni_method, __jni_args.as_ptr())
        }
    }
}

///interface [AttachmentTarget](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentTarget.html)
pub enum AttachmentTarget {}
unsafe impl ::java_oxide::ReferenceType for AttachmentTarget {}
unsafe impl ::java_oxide::JniType for AttachmentTarget {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/fabric/api/attachment/v1/AttachmentTarget")
    }
}
unsafe impl ::java_oxide::AssignableTo<crate::java::lang::Object> for AttachmentTarget {}
impl AttachmentTarget {
    fn __class_global_ref(__jni_env: ::java_oxide::Env) -> ::java_oxide::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<::java_oxide::Global<crate::java::lang::Object>> = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe { ::java_oxide::Local::from_raw(__jni_env, __jni_env.require_class(c"net/fabricmc/fabric/api/attachment/v1/AttachmentTarget")).as_global() })
            .as_raw()
    }
    ///[getAttached](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentTarget.html#getAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType))
    pub fn getAttached<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<AttachmentType>,
    ) -> ::std::result::Result<::std::option::Option<::java_oxide::Local<'env, crate::java::lang::Object>>, ::java_oxide::Local<'env, crate::java::lang::Throwable>>
    {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> = ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getAttached",
                        c"(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;)Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[getAttachedOrSet](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentTarget.html#getAttachedOrSet(net.fabricmc.fabric.api.attachment.v1.AttachmentType.java.lang.Object))
    pub fn getAttachedOrSet<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<AttachmentType>,
        arg1: impl ::java_oxide::AsArg<crate::java::lang::Object>,
    ) -> ::std::result::Result<::std::option::Option<::java_oxide::Local<'env, crate::java::lang::Object>>, ::java_oxide::Local<'env, crate::java::lang::Throwable>>
    {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> = ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0), ::java_oxide::AsJValue::as_jvalue(&arg1)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getAttachedOrSet",
                        c"(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;Ljava/lang/Object;)Ljava/lang/Object;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///[hasAttached](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentTarget.html#hasAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType))
    pub fn hasAttached<'env>(
        self: &::java_oxide::Ref<'env, Self>,
        arg0: impl ::java_oxide::AsArg<AttachmentType>,
    ) -> ::std::result::Result<bool, ::java_oxide::Local<'env, crate::java::lang::Throwable>> {
        static __METHOD: ::std::sync::OnceLock<::java_oxide::JMethodID> = ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [::java_oxide::AsJValue::as_jvalue(&arg0)];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_oxide::JMethodID::from_raw(__jni_env.require_method(__jni_class, c"hasAttached", c"(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;)Z"))
                })
                .as_raw();
            __jni_env.call_boolean_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
    ///public static final [NBT_ATTACHMENT_KEY](https://maven.fabricmc.net/docs/fabric-api-0.138.3+1.21.10/net/fabricmc/fabric/api/attachment/v1/AttachmentTarget.html#NBT_ATTACHMENT_KEY)
    pub const NBT_ATTACHMENT_KEY: &'static str = "fabric:attachments";
}
