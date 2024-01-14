//package at.bumzack.reference.impl;
//
//
//// https://github.com/spring-attic/spring-native/blob/main/spring-native-configuration/src/main/java/kotlin/KotlinHints.java
//
//import org.springframework.nativex.hint.TypeAccess;
//import org.springframework.nativex.hint.NativeHint;
//import org.springframework.nativex.hint.ResourceHint;
//import org.springframework.nativex.hint.TypeHint;
//import org.springframework.nativex.type.NativeConfiguration;
//
//
//@NativeHint(trigger = kotlin.Unit.class,
//        resources = {
//                @ResourceHint(patterns= {
//                        "META-INF/.*.kotlin_module$",
//                        ".*.kotlin_builtins",
//                        "META-INF/services/.*"
//                })
//        }, types = {
//        @TypeHint(types = kotlin.KotlinVersion.class, access = { TypeAccess.PUBLIC_METHODS, TypeAccess.DECLARED_FIELDS, TypeAccess.DECLARED_METHODS, TypeAccess.DECLARED_CONSTRUCTORS}),
//        @TypeHint(typeNames = "kotlin.KotlinVersion$Companion")
//})
//@NativeHint(trigger = kotlin.reflect.full.KClasses.class,
//        types = {
//                @TypeHint(types = kotlin.reflect.full.KClasses.class, access = {}),
//                @TypeHint(types = kotlin.Metadata.class, access = TypeAccess.DECLARED_METHODS),
//                @TypeHint(types = kotlin.reflect.jvm.internal.ReflectionFactoryImpl.class, access = TypeAccess.DECLARED_CONSTRUCTORS),
//                @TypeHint(types = kotlin.reflect.jvm.internal.impl.resolve.scopes.DescriptorKindFilter.class, access = TypeAccess.DECLARED_FIELDS)
//        })
//@NativeHint(trigger = kotlin.coroutines.Continuation.class,
//        types = @TypeHint(types = kotlin.coroutines.Continuation.class, access = { TypeAccess.DECLARED_CONSTRUCTORS, TypeAccess.PUBLIC_METHODS}))
//@NativeHint(trigger = com.fasterxml.jackson.module.kotlin.KotlinModule.class,
//        types = {
//                @TypeHint(types = com.fasterxml.jackson.module.kotlin.KotlinModule.class),
//                @TypeHint(typeNames = {
//                        "com.fasterxml.jackson.module.kotlin.KotlinModule$Builder",
//                        "com.fasterxml.jackson.module.kotlin.SingletonSupport",
//                        "java.lang.String"
//                }, access = {})
//        })
//@NativeHint(trigger = kotlinx.serialization.descriptors.SerialDescriptor.class, types =
//@TypeHint(types = { kotlinx.serialization.encoding.CompositeEncoder.class, kotlinx.serialization.descriptors.SerialDescriptor.class },
//        typeNames = "kotlinx.serialization.internal.SerializationConstructorMarker")
//)
//public class KotlinHints implements NativeConfiguration {
//}
