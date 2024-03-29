/* DO NOT EDIT THIS FILE - it is machine generated */
#include <jni.h>
/* Header for class com_training_project_LocalVpn */

#ifndef _Included_com_training_project_LocalVpn
#define _Included_com_training_project_LocalVpn
#ifdef __cplusplus
extern "C" {
#endif
#undef com_training_project_LocalVpn_BIND_ABOVE_CLIENT
#define com_training_project_LocalVpn_BIND_ABOVE_CLIENT 8L
#undef com_training_project_LocalVpn_BIND_ADJUST_WITH_ACTIVITY
#define com_training_project_LocalVpn_BIND_ADJUST_WITH_ACTIVITY 128L
#undef com_training_project_LocalVpn_BIND_ALLOW_OOM_MANAGEMENT
#define com_training_project_LocalVpn_BIND_ALLOW_OOM_MANAGEMENT 16L
#undef com_training_project_LocalVpn_BIND_AUTO_CREATE
#define com_training_project_LocalVpn_BIND_AUTO_CREATE 1L
#undef com_training_project_LocalVpn_BIND_DEBUG_UNBIND
#define com_training_project_LocalVpn_BIND_DEBUG_UNBIND 2L
#undef com_training_project_LocalVpn_BIND_EXTERNAL_SERVICE
#define com_training_project_LocalVpn_BIND_EXTERNAL_SERVICE -2147483648L
#undef com_training_project_LocalVpn_BIND_IMPORTANT
#define com_training_project_LocalVpn_BIND_IMPORTANT 64L
#undef com_training_project_LocalVpn_BIND_INCLUDE_CAPABILITIES
#define com_training_project_LocalVpn_BIND_INCLUDE_CAPABILITIES 4096L
#undef com_training_project_LocalVpn_BIND_NOT_FOREGROUND
#define com_training_project_LocalVpn_BIND_NOT_FOREGROUND 4L
#undef com_training_project_LocalVpn_BIND_NOT_PERCEPTIBLE
#define com_training_project_LocalVpn_BIND_NOT_PERCEPTIBLE 256L
#undef com_training_project_LocalVpn_BIND_WAIVE_PRIORITY
#define com_training_project_LocalVpn_BIND_WAIVE_PRIORITY 32L
#undef com_training_project_LocalVpn_CONTEXT_IGNORE_SECURITY
#define com_training_project_LocalVpn_CONTEXT_IGNORE_SECURITY 2L
#undef com_training_project_LocalVpn_CONTEXT_INCLUDE_CODE
#define com_training_project_LocalVpn_CONTEXT_INCLUDE_CODE 1L
#undef com_training_project_LocalVpn_CONTEXT_RESTRICTED
#define com_training_project_LocalVpn_CONTEXT_RESTRICTED 4L
#undef com_training_project_LocalVpn_MODE_APPEND
#define com_training_project_LocalVpn_MODE_APPEND 32768L
#undef com_training_project_LocalVpn_MODE_ENABLE_WRITE_AHEAD_LOGGING
#define com_training_project_LocalVpn_MODE_ENABLE_WRITE_AHEAD_LOGGING 8L
#undef com_training_project_LocalVpn_MODE_MULTI_PROCESS
#define com_training_project_LocalVpn_MODE_MULTI_PROCESS 4L
#undef com_training_project_LocalVpn_MODE_NO_LOCALIZED_COLLATORS
#define com_training_project_LocalVpn_MODE_NO_LOCALIZED_COLLATORS 16L
#undef com_training_project_LocalVpn_MODE_PRIVATE
#define com_training_project_LocalVpn_MODE_PRIVATE 0L
#undef com_training_project_LocalVpn_MODE_WORLD_READABLE
#define com_training_project_LocalVpn_MODE_WORLD_READABLE 1L
#undef com_training_project_LocalVpn_MODE_WORLD_WRITEABLE
#define com_training_project_LocalVpn_MODE_WORLD_WRITEABLE 2L
#undef com_training_project_LocalVpn_RECEIVER_EXPORTED
#define com_training_project_LocalVpn_RECEIVER_EXPORTED 2L
#undef com_training_project_LocalVpn_RECEIVER_NOT_EXPORTED
#define com_training_project_LocalVpn_RECEIVER_NOT_EXPORTED 4L
#undef com_training_project_LocalVpn_RECEIVER_VISIBLE_TO_INSTANT_APPS
#define com_training_project_LocalVpn_RECEIVER_VISIBLE_TO_INSTANT_APPS 1L
#undef com_training_project_LocalVpn_START_CONTINUATION_MASK
#define com_training_project_LocalVpn_START_CONTINUATION_MASK 15L
#undef com_training_project_LocalVpn_START_FLAG_REDELIVERY
#define com_training_project_LocalVpn_START_FLAG_REDELIVERY 1L
#undef com_training_project_LocalVpn_START_FLAG_RETRY
#define com_training_project_LocalVpn_START_FLAG_RETRY 2L
#undef com_training_project_LocalVpn_START_NOT_STICKY
#define com_training_project_LocalVpn_START_NOT_STICKY 2L
#undef com_training_project_LocalVpn_START_REDELIVER_INTENT
#define com_training_project_LocalVpn_START_REDELIVER_INTENT 3L
#undef com_training_project_LocalVpn_START_STICKY
#define com_training_project_LocalVpn_START_STICKY 1L
#undef com_training_project_LocalVpn_START_STICKY_COMPATIBILITY
#define com_training_project_LocalVpn_START_STICKY_COMPATIBILITY 0L
#undef com_training_project_LocalVpn_STOP_FOREGROUND_DETACH
#define com_training_project_LocalVpn_STOP_FOREGROUND_DETACH 2L
#undef com_training_project_LocalVpn_STOP_FOREGROUND_LEGACY
#define com_training_project_LocalVpn_STOP_FOREGROUND_LEGACY 0L
#undef com_training_project_LocalVpn_STOP_FOREGROUND_REMOVE
#define com_training_project_LocalVpn_STOP_FOREGROUND_REMOVE 1L
/*
 * Class:     com_training_project_LocalVpn
 * Method:    create_native
 * Signature: (Landroid/net/VpnService;)V
 */
JNIEXPORT void JNICALL Java_com_training_project_LocalVpn_create_1native
  (JNIEnv *, jobject, jobject);

/*
 * Class:     com_training_project_LocalVpn
 * Method:    destroy_native
 * Signature: ()V
 */
JNIEXPORT void JNICALL Java_com_training_project_LocalVpn_destroy_1native
  (JNIEnv *, jobject);

/*
 * Class:     com_training_project_LocalVpn
 * Method:    start_vpn
 * Signature: (I)V
 */
JNIEXPORT void JNICALL Java_com_training_project_LocalVpn_start_1vpn
  (JNIEnv *, jobject, jint);

/*
 * Class:     com_training_project_LocalVpn
 * Method:    stop_vpn
 * Signature: ()V
 */
JNIEXPORT void JNICALL Java_com_training_project_LocalVpn_stop_1vpn
  (JNIEnv *, jobject);

#ifdef __cplusplus
}
#endif
#endif
