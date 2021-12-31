#[macro_export]
macro_rules! impl_msg {
    (pub struct $name:ident { $(pub $element: ident: $ty: ty),* $(,)? }) => {

        #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
        pub struct $name {
            $($element: $ty),*
        }

        impl Msg for $name {
            type Proto = proto::chainmain::nft::v1::$name;
        }

        impl TryFrom<proto::chainmain::nft::v1::$name> for $name {
            type Error = ErrorReport;

            fn try_from(proto: proto::chainmain::nft::v1::$name) -> Result<$name> {
                $name::try_from(&proto)
            }
        }

        impl TryFrom<&proto::chainmain::nft::v1::$name> for $name {
            type Error = ErrorReport;

            fn try_from(proto: &proto::chainmain::nft::v1::$name) -> Result<$name> {
                Ok($name {
                    $($element: proto.$element.parse()?),*
                })
            }
        }

        impl From<$name> for proto::chainmain::nft::v1::$name {
            fn from(denom: $name) -> proto::chainmain::nft::v1::$name {
                proto::chainmain::nft::v1::$name::from(&denom)
            }
        }

        impl From<&$name> for proto::chainmain::nft::v1::$name {
            fn from(msg: &$name) -> proto::chainmain::nft::v1::$name {
                proto::chainmain::nft::v1::$name {
                    $($element: msg.$element.to_string()),*
                }
            }
        }


    };
}
