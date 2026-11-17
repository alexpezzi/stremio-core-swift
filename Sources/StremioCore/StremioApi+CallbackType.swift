//
//  File.swift
//
//
//  Created by Alvin on 17.06.24.
//

import Foundation
import SwiftProtobuf
import os.log

extension StremioApi {
    public class CallbackType {
        public static var error: Int = tag { $0.error = .init() }
        public static var addonUninstalled: Int = tag { $0.addonUninstalled = .init() }
        public static var addonInstalled: Int = tag { $0.addonInstalled = .init() }
        public static var addonUpgraded: Int = tag { $0.addonUpgraded = .init() }
        public static var settingsUpdated: Int = tag { $0.settingsUpdated = .init() }
        public static var userAuthenticated: Int = tag { $0.userAuthenticated = .init() }
        public static var userAccountDeleted: Int = tag { $0.userAccountDeleted = .init() }
        public static var userPulledFromApi: Int = tag { $0.userPulledFromApi = .init() }
        public static var metaItemRated: Int = tag { $0.metaItemRated = .init() }
        public static var libraryItemMarkedAsWatched: Int = tag { $0.libraryItemMarkedAsWatched = .init() }
        public static var streamingServerUrlsBucketChanged: Int = tag { $0.streamingServerUrlsBucketChanged = .init() }

        private static func tag(_ build: (inout Stremio_Core_Runtime_Event) -> Void) -> Int {
            var event = Stremio_Core_Runtime_Event()
            build(&event)
            return event.getMessageTag
        }
    }

    static internal func handleEvent<T>(callbackType: Int,
                        completionHandler: ((Result<T, Stremio_Core_Runtime_Event.Error>) -> Void)?) {
        if let completionHandler = completionHandler {
            Core.addEventListener(type: callbackType) { result in
                if case .error(_:) = result.type{
                    completionHandler(.failure(result.error))
                }
                else if let success = result.type?.get() as T?{
                    completionHandler(.success(success))
                }
                else {
                    os_log(.fault, log: StremioApi.oslog, "Casting failed for type: %@, result: %@", String(describing: T.self), String(describing: result))
                }
                Core.removeEventListener(type: callbackType)
            }
        }
    }
}

extension Stremio_Core_Runtime_Event.Error : Error{}
extension Stremio_Core_Runtime_Event.OneOf_Type {
    func get<T>() -> T? {
        Mirror(reflecting: self).children.first?.value as? T
    }
}
