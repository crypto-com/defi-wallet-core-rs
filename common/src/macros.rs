#[macro_export]
macro_rules! msg_wrapper {
    ($path:path => pub struct $name:ident { $(pub $element: ident: $ty: ty),* $(,)? }) => {

        #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
        pub struct $name {
            $($element: $ty),*
        }

        impl Msg for $name {
            type Proto = $path;
        }

        impl TryFrom<$path> for $name {
            type Error = ErrorReport;

            fn try_from(proto: $path) -> Result<$name> {
                $name::try_from(&proto)
            }
        }

        impl TryFrom<&$path> for $name {
            type Error = ErrorReport;

            fn try_from(proto: &$path) -> Result<$name> {
                Ok($name {
                    $($element: proto.$element.parse()?),*
                })
            }
        }

        impl From<$name> for $path {
            fn from(denom: $name) -> $path {
                <$path>::from(&denom)
            }
        }

        impl From<&$name> for $path {
            fn from(msg: &$name) -> $path {
                $path {
                    $($element: msg.$element.to_string()),*
                }
            }
        }


    };
}
