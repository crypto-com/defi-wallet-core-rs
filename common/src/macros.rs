#[macro_export]
macro_rules! msg_wrapper {
    ($(#[$outer:meta])*
     $path:path =>

     pub struct $name:ident {$($(#[$inner:meta])* pub $element: ident: $ty: ty),* $(,)? }) => {
        $(#[$outer])*
        #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
        pub struct $name {
            $($(#[$inner])* pub $element: $ty),*
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
