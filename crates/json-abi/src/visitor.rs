use crate::{Constructor, Error, Event, EventParam, Function, Param};
use alloc::vec::Vec;

/// Trait allowing traversal of a JSON ABI.
///
/// Should be used in conjunction with the [Walk] trait.
pub trait Visitor<'a> {
    fn visit_abi(&mut self, _abi: &'a crate::JsonAbi) {}
    fn visit_constructor(&mut self, _constructor: &'a crate::Constructor) {}
    fn visit_function(&mut self, _function: &'a crate::Function) {}
    fn visit_event(&mut self, _event: &'a crate::Event) {}
    fn visit_error(&mut self, _error: &'a crate::Error) {}
    fn visit_param(&mut self, _param: &'a crate::Param) {}
    fn visit_event_param(&mut self, _param: &'a crate::EventParam) {}
}

/// Trait allowing traversal of a mutable JSON ABI.
///
/// Should be used in conjunction with the [WalkMut] trait.
pub trait VisitorMut {
    fn visit_abi(&mut self, _abi: &mut crate::JsonAbi) {}
    fn visit_constructor(&mut self, _constructor: &mut crate::Constructor) {}
    fn visit_function(&mut self, _function: &mut crate::Function) {}
    fn visit_event(&mut self, _event: &mut crate::Event) {}
    fn visit_error(&mut self, _error: &mut crate::Error) {}
    fn visit_param(&mut self, _param: &mut crate::Param) {}
    fn visit_event_param(&mut self, _param: &mut crate::EventParam) {}
}

pub trait Walk {
    fn walk<'a>(&'a self, visitor: &mut dyn Visitor<'a>);
}

pub trait WalkMut {
    fn walk_mut(&mut self, visitor: &mut dyn VisitorMut);
}

impl<W: Walk> Walk for Vec<W> {
    fn walk<'a>(&'a self, visitor: &mut dyn Visitor<'a>) {
        self.iter().for_each(|item| item.walk(visitor))
    }
}

impl<W: WalkMut> WalkMut for Vec<W> {
    fn walk_mut(&mut self, visitor: &mut dyn VisitorMut) {
        self.iter_mut().for_each(|item| item.walk_mut(visitor))
    }
}

macro_rules! impl_walk {
    ($ty:ty, $visitor_method:ident, $($walkable_part:ident),*) => {
        impl Walk for $ty {
            fn walk<'a>(&'a self, visitor: &mut dyn Visitor<'a>) {
                visitor.$visitor_method(self);
                $(
                    self.$walkable_part.walk(visitor);
                )*
            }
        }

        impl WalkMut for $ty {
            fn walk_mut(&mut self, visitor: &mut dyn VisitorMut) {
                visitor.$visitor_method(self);
                $(
                    self.$walkable_part.walk_mut(visitor);
                )*
            }
        }
    }
}

impl_walk!(Param, visit_param, components);
impl_walk!(EventParam, visit_event_param, components);
impl_walk!(Event, visit_event, inputs);
impl_walk!(Error, visit_error, inputs);
impl_walk!(Function, visit_function, inputs, outputs);
impl_walk!(Constructor, visit_constructor, inputs);

impl Walk for crate::JsonAbi {
    fn walk<'a>(&'a self, visitor: &mut dyn Visitor<'a>) {
        visitor.visit_abi(self);
        if let Some(constructor) = &self.constructor {
            constructor.walk(visitor);
        }
        for function in self.functions() {
            function.walk(visitor);
        }
        for error in self.errors() {
            error.walk(visitor);
        }
        for event in self.events() {
            event.walk(visitor);
        }
    }
}

impl WalkMut for crate::JsonAbi {
    fn walk_mut(&mut self, visitor: &mut dyn VisitorMut) {
        visitor.visit_abi(self);
        if let Some(constructor) = &mut self.constructor {
            constructor.walk_mut(visitor);
        }
        for function in self.functions_mut() {
            function.walk_mut(visitor);
        }
        for error in self.errors_mut() {
            error.walk_mut(visitor);
        }
        for event in self.events_mut() {
            event.walk_mut(visitor);
        }
    }
}
