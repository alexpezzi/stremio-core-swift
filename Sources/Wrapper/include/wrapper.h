#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <Foundation/Foundation.h>

void initialize_rust(void);

/**
 * Loads the runtime from storage. `storage_suite_name` is the user defaults suite to store in,
 * such as an app group shared with app extensions, or null for the standard user defaults.
 *
 * It can be called again to reload the runtime from storage, for example in an app extension
 * whose app changed the shared storage. The device name of the first call is kept.
 */
NSObject *initializeNative(NSString *device_info, NSString *storage_suite_name);

void dispatchNative(NSObject *action_protobuf);

NSObject *getStateNative(int32_t field);

NSObject *decodeStreamDataNative(NSString *field);

void sendNextAnalyticsBatch(void);

NSString *getVersionNative(void);

void releaseObjectNative(NSObject *object);
