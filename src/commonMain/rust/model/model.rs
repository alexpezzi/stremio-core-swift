use prost::Message;

use stremio_core::{
    models::{
        addon_details::AddonDetails,
        catalog_with_filters::CatalogWithFilters,
        catalogs_with_extra::CatalogsWithExtra,
        continue_watching_preview::ContinueWatchingPreview,
        ctx::Ctx,
        data_export::DataExport,
        library_by_type::LibraryByType,
        library_with_filters::{ContinueWatchingFilter, LibraryWithFilters, NotRemovedFilter},
        link::Link,
        local_search::LocalSearch,
        meta_details::MetaDetails,
        player::Player,
        streaming_server::StreamingServer,
    },
    runtime::Effects,
    types::{
        api::LinkAuthKey, events::DismissedEventsBucket, library::LibraryBucket,
        notifications::NotificationsBucket, profile::Profile, remote_config::RemoteConfigBucket,
        resource::MetaItemPreview, search_history::SearchHistoryBucket,
        server_urls::ServerUrlsBucket, streams::StreamsBucket,
    },
    Model,
};
use stremio_core_protobuf::{model::AddonsWithFilters, ToProtobuf};

use crate::env::AppleEnv;

pub struct AppleModelBuckets {
    pub profile: Profile,
    pub library: LibraryBucket,
    pub streams: StreamsBucket,
    pub server_urls: ServerUrlsBucket,
    pub notifications: NotificationsBucket,
    pub search_history: SearchHistoryBucket,
    pub dismissed_events: DismissedEventsBucket,
    pub remote_config: Option<RemoteConfigBucket>,
}

#[derive(Model, Clone)]
#[model(AppleEnv)]
pub struct AppleModel {
    pub ctx: Ctx,
    pub auth_link: Link<LinkAuthKey>,
    pub data_export: DataExport,
    pub continue_watching_preview: ContinueWatchingPreview,
    pub board: CatalogsWithExtra,
    pub discover: CatalogWithFilters<MetaItemPreview>,
    pub library: LibraryWithFilters<NotRemovedFilter>,
    pub library_by_type: LibraryByType<NotRemovedFilter>,
    pub continue_watching: LibraryWithFilters<ContinueWatchingFilter>,
    pub search: CatalogsWithExtra,
    pub local_search: LocalSearch,
    pub addons: AddonsWithFilters,
    pub meta_details: MetaDetails,
    pub addon_details: AddonDetails,
    pub streaming_server: StreamingServer,
    pub player: Player,
}

impl AppleModel {
    pub fn new(buckets: AppleModelBuckets) -> (AppleModel, Effects) {
        let AppleModelBuckets {
            profile,
            library,
            streams,
            server_urls,
            notifications,
            search_history,
            dismissed_events,
            remote_config,
        } = buckets;

        let (continue_watching_preview, continue_watching_preview_effects) =
            ContinueWatchingPreview::new(&library, &notifications);
        let (discover, discover_effects) = CatalogWithFilters::<MetaItemPreview>::new(&profile);
        let (library_, library_effects) =
            LibraryWithFilters::<NotRemovedFilter>::new(&library, &notifications);
        let (library_by_type, library_by_type_effects) = LibraryByType::<NotRemovedFilter>::new();
        let (continue_watching, continue_watching_effects) =
            LibraryWithFilters::<ContinueWatchingFilter>::new(&library, &notifications);
        let (addons, addons_effects) = AddonsWithFilters::new(&profile);
        let (streaming_server, streaming_server_effects) =
            StreamingServer::new::<AppleEnv>(&profile);
        let (local_search, local_search_effects) = LocalSearch::new::<AppleEnv>();

        let model = AppleModel {
            ctx: Ctx::new(
                profile,
                library,
                streams,
                server_urls,
                notifications,
                search_history,
                dismissed_events,
            )
            .with_remote_config::<AppleEnv>(remote_config),
            auth_link: Default::default(),
            data_export: Default::default(),
            continue_watching_preview,
            board: Default::default(),
            discover,
            library: library_,
            library_by_type,
            continue_watching,
            search: Default::default(),
            local_search,
            addons,
            meta_details: Default::default(),
            addon_details: Default::default(),
            streaming_server,
            player: Default::default(),
        };
        (
            model,
            continue_watching_preview_effects
                .join(discover_effects)
                .join(library_effects)
                .join(library_by_type_effects)
                .join(continue_watching_effects)
                .join(addons_effects)
                .join(streaming_server_effects)
                .join(local_search_effects),
        )
    }

    pub fn get_state_binary(&self, field: &AppleModelField) -> Vec<u8> {
        match field {
            AppleModelField::Ctx => self.ctx.to_protobuf::<AppleEnv>(&()).encode_to_vec(),
            AppleModelField::AuthLink => self.auth_link.to_protobuf::<AppleEnv>(&()).encode_to_vec(),
            AppleModelField::ContinueWatchingPreview => self
                .continue_watching_preview
                .to_protobuf::<AppleEnv>(&self.ctx)
                .encode_to_vec(),
            AppleModelField::Board => self.board.to_protobuf::<AppleEnv>(&self.ctx).encode_to_vec(),
            AppleModelField::Discover => self
                .discover
                .to_protobuf::<AppleEnv>(&self.ctx)
                .encode_to_vec(),
            AppleModelField::Library => self
                .library
                .to_protobuf::<AppleEnv>(&self.ctx)
                .encode_to_vec(),
            AppleModelField::LibraryByType => self
                .library_by_type
                .to_protobuf::<AppleEnv>(&self.ctx)
                .encode_to_vec(),
            AppleModelField::ContinueWatching => self
                .continue_watching
                .to_protobuf::<AppleEnv>(&self.ctx)
                .encode_to_vec(),
            AppleModelField::Search => self.search.to_protobuf::<AppleEnv>(&self.ctx).encode_to_vec(),
            AppleModelField::Addons => self.addons.to_protobuf::<AppleEnv>(&self.ctx).encode_to_vec(),
            AppleModelField::MetaDetails => self
                .meta_details
                .to_protobuf::<AppleEnv>(&(&self.ctx, &self.streaming_server))
                .encode_to_vec(),
            AppleModelField::AddonDetails => self
                .addon_details
                .to_protobuf::<AppleEnv>(&self.ctx)
                .encode_to_vec(),
            AppleModelField::StreamingServer => self
                .streaming_server
                .to_protobuf::<AppleEnv>(&())
                .encode_to_vec(),
            AppleModelField::Player => self
                .player
                .to_protobuf::<AppleEnv>(&(&self.ctx, &self.streaming_server))
                .encode_to_vec(),
            // No protobuf representation exists for these models yet.
            AppleModelField::DataExport | AppleModelField::LocalSearch => vec![],
        }
    }
}
