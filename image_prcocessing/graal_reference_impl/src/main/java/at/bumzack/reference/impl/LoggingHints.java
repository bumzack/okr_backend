//package at.bumzack.reference.impl;
//
// //https://github.com/spring-attic/spring-native/blob/main/spring-native-configuration/src/main/java/LoggingInitHints.java
//
//import org.springframework.nativex.hint.InitializationHint;
//import org.springframework.nativex.hint.InitializationTime;
//import org.springframework.nativex.type.NativeConfiguration;
//import org.springframework.nativex.hint.NativeHint;
//
//@NativeHint(
//        initialization = @InitializationHint(types = {
//                org.slf4j.spi.LocationAwareLogger.class,
//                org.slf4j.Logger.class,
//                org.slf4j.event.SubstituteLoggingEvent.class,
//                org.slf4j.event.EventRecodingLogger.class,
//                org.slf4j.helpers.FormattingTuple.class,
//                org.slf4j.helpers.MessageFormatter.class,
//                org.slf4j.helpers.SubstituteLogger.class,
//                org.slf4j.helpers.Util.class,
//                org.slf4j.helpers.NOPLogger.class,
//                org.slf4j.helpers.NOPLoggerFactory.class,
//                org.slf4j.helpers.SubstituteLoggerFactory.class,
//                org.slf4j.impl.StaticLoggerBinder.class,
//                org.slf4j.LoggerFactory.class,
//                org.slf4j.MDC.class,
//                org.apache.commons.logging.LogFactory.class,
//        }, typeNames = {
//                "org.apache.commons.logging.LogAdapter",
//                "org.apache.commons.logging.LogAdapter$1",
//                "org.apache.commons.logging.LogAdapter$Slf4jLocationAwareLog",
//                "org.apache.commons.logging.LogAdapter$Log4jLog"
//        }, packageNames = {
//                "ch.qos.logback.core",
//                "ch.qos.logback.classic",
//                "ch.qos.logback.classic.util",
//                "org.apache.logging.log4j",
//                "org.apache.logging.slf4j",
//                "org.jboss.logging"
//        }, initTime = InitializationTime.BUILD))
//public class LoggingInitHints implements NativeConfiguration {
//}