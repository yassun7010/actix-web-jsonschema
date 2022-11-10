macro_rules! schema_new_type {
    (
        #[doc = $document:literal]
        #[derive(Debug, AsRef, Deref, DerefMut, From, FromRequest)]
        pub struct $Type:ident<T>(pub T);
    ) => {
        #[doc = $document]
        #[derive(Debug)]
        pub struct $Type<T>(pub T);

        impl<T> $Type<T> {
            /// Unwrap into inner `T` value.
            pub fn into_inner(self) -> T {
                self.0
            }
        }

        impl<T> AsRef<T> for $Type<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T> std::ops::Deref for $Type<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $Type<T> {
            fn deref_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }

        impl<T> From<T> for $Type<T> {
            fn from(data: T) -> Self {
                $Type(data)
            }
        }

        impl<T> actix_web::FromRequest for $Type<T>
        where
            T: crate::schema::SchemaDeserialize,
        {
            type Error = actix_web::Error;
            type Future = futures::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;

            #[inline]
            fn from_request(
                req: &actix_web::HttpRequest,
                payload: &mut actix_web::dev::Payload,
            ) -> Self::Future {
                actix_web::web::$Type::<serde_json::Value>::from_request(req, payload)
                    .map(|result| match result {
                        Ok(value) => Ok($Type(crate::schema::SchemaContext::from_value::<T>(
                            value.into_inner(),
                        )?)),
                        Err(err) => Err(err),
                    })
                    .boxed_local()
            }
        }
    };

    (
        #[doc = $document:literal]
        #[derive(Debug, AsRef, Deref, DerefMut, From, FromRequest, Responder)]
        pub struct $Type:ident<T>(pub T);
    ) => {
        crate::macros::schema_new_type! {
            #[doc = $document]
            #[derive(Debug, AsRef, Deref, DerefMut, From, FromRequest)]
            pub struct $Type<T>(pub T);
        }

        impl<T> actix_web::Responder for $Type<T>
        where
            T: crate::schema::SchemaSerialize,
        {
            type Body = actix_web::body::EitherBody<String>;

            fn respond_to(
                self,
                req: &actix_web::HttpRequest,
            ) -> actix_web::HttpResponse<Self::Body> {
                actix_web::web::$Type::respond_to(actix_web::web::$Type(self.0), req)
            }
        }
    };
}

pub(crate) use schema_new_type;
