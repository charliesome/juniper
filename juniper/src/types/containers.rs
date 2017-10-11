use ast::{FromInputValue, InputValue, Selection, ToInputValue};
use value::Value;
use schema::meta::MetaType;

use executor::{Async, Executor, Registry};
use types::base::GraphQLType;

impl<T, CtxT> GraphQLType for Option<T>
where
    T: GraphQLType<Context = CtxT>,
{
    type Context = CtxT;
    type TypeInfo = T::TypeInfo;

    fn name(_: &T::TypeInfo) -> Option<&str> {
        None
    }

    fn meta<'r>(info: &T::TypeInfo, registry: &mut Registry<'r>) -> MetaType<'r> {
        registry.build_nullable_type::<T>(info).into_meta()
    }

    fn resolve(
        &self,
        info: &T::TypeInfo,
        _: Option<&[Selection]>,
        executor: &Executor<CtxT>,
    ) -> Async<Value> {
        match *self {
            Some(ref obj) => executor.resolve_into_value(info, obj),
            None => Value::null().into(),
        }
    }
}

impl<T> FromInputValue for Option<T>
where
    T: FromInputValue,
{
    fn from_input_value(v: &InputValue) -> Option<Option<T>> {
        match v {
            &InputValue::Null => Some(None),
            v => match v.convert() {
                Some(x) => Some(Some(x)),
                None => None,
            },
        }
    }
}

impl<T> ToInputValue for Option<T>
where
    T: ToInputValue,
{
    fn to_input_value(&self) -> InputValue {
        match *self {
            Some(ref v) => v.to_input_value(),
            None => InputValue::null(),
        }
    }
}

impl<T, CtxT> GraphQLType for Vec<T>
where
    T: GraphQLType<Context = CtxT>,
{
    type Context = CtxT;
    type TypeInfo = T::TypeInfo;

    fn name(_: &T::TypeInfo) -> Option<&str> {
        None
    }

    fn meta<'r>(info: &T::TypeInfo, registry: &mut Registry<'r>) -> MetaType<'r> {
        registry.build_list_type::<T>(info).into_meta()
    }

    fn resolve(
        &self,
        info: &T::TypeInfo,
        _: Option<&[Selection]>,
        executor: &Executor<CtxT>,
    ) -> Async<Value> {
        Async::all(
            self.iter()
                .map(|e| executor.resolve_into_value(info, e))
        ).map(Value::list)
    }
}

impl<T> FromInputValue for Vec<T>
where
    T: FromInputValue,
{
    fn from_input_value(v: &InputValue) -> Option<Vec<T>> {
        match *v {
            InputValue::List(ref ls) => {
                let v: Vec<_> = ls.iter().filter_map(|i| i.item.convert()).collect();

                if v.len() == ls.len() {
                    Some(v)
                } else {
                    None
                }
            }
            ref other => if let Some(e) = other.convert() {
                Some(vec![e])
            } else {
                None
            },
        }
    }
}

impl<T> ToInputValue for Vec<T>
where
    T: ToInputValue,
{
    fn to_input_value(&self) -> InputValue {
        InputValue::list(self.iter().map(|v| v.to_input_value()).collect())
    }
}

impl<'a, T, CtxT> GraphQLType for &'a [T]
where
    T: GraphQLType<Context = CtxT>,
{
    type Context = CtxT;
    type TypeInfo = T::TypeInfo;

    fn name(_: &T::TypeInfo) -> Option<&str> {
        None
    }

    fn meta<'r>(info: &T::TypeInfo, registry: &mut Registry<'r>) -> MetaType<'r> {
        registry.build_list_type::<T>(info).into_meta()
    }

    fn resolve(
        &self,
        info: &T::TypeInfo,
        _: Option<&[Selection]>,
        executor: &Executor<CtxT>,
    ) -> Async<Value> {
        let iter = self.iter().map(|e|
            executor.resolve_into_value(info, e));

        Async::all(iter).map(Value::list)
    }
}

impl<'a, T> ToInputValue for &'a [T]
where
    T: ToInputValue,
{
    fn to_input_value(&self) -> InputValue {
        InputValue::list(self.iter().map(|v| v.to_input_value()).collect())
    }
}
