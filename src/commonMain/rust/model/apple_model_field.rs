use stremio_core::runtime::Env;
use stremio_core_protobuf::ToProtobuf;

use crate::model::AppleModelField;
use crate::protobuf::stremio::core::runtime::Field;

impl From<Field> for AppleModelField {
    fn from(field: Field) -> Self {
        match field {
            Field::Ctx => AppleModelField::Ctx,
            Field::AuthLink => AppleModelField::AuthLink,
            Field::DataExport => AppleModelField::DataExport,
            Field::ContinueWatchingPreview => AppleModelField::ContinueWatchingPreview,
            Field::Board => AppleModelField::Board,
            Field::Discover => AppleModelField::Discover,
            Field::Library => AppleModelField::Library,
            Field::LibraryByType => AppleModelField::LibraryByType,
            Field::ContinueWatching => AppleModelField::ContinueWatching,
            Field::Search => AppleModelField::Search,
            Field::LocalSearch => AppleModelField::LocalSearch,
            Field::Addons => AppleModelField::Addons,
            Field::MetaDetails => AppleModelField::MetaDetails,
            Field::AddonDetails => AppleModelField::AddonDetails,
            Field::StreamingServer => AppleModelField::StreamingServer,
            Field::Player => AppleModelField::Player,
        }
    }
}

impl ToProtobuf<Field, ()> for AppleModelField {
    fn to_protobuf<E: Env + 'static>(&self, _args: &()) -> Field {
        match self {
            AppleModelField::Ctx => Field::Ctx,
            AppleModelField::AuthLink => Field::AuthLink,
            AppleModelField::DataExport => Field::DataExport,
            AppleModelField::ContinueWatchingPreview => Field::ContinueWatchingPreview,
            AppleModelField::Board => Field::Board,
            AppleModelField::Discover => Field::Discover,
            AppleModelField::Library => Field::Library,
            AppleModelField::LibraryByType => Field::LibraryByType,
            AppleModelField::ContinueWatching => Field::ContinueWatching,
            AppleModelField::Search => Field::Search,
            AppleModelField::LocalSearch => Field::LocalSearch,
            AppleModelField::Addons => Field::Addons,
            AppleModelField::MetaDetails => Field::MetaDetails,
            AppleModelField::AddonDetails => Field::AddonDetails,
            AppleModelField::StreamingServer => Field::StreamingServer,
            AppleModelField::Player => Field::Player,
        }
    }
}
