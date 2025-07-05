use median::{
    // attr::{AttrBuilder, AttrType},
    builder::MaxWrappedBuilder,
    class::Class,
    // max_sys::t_atom_long,
    object::MaxObj,
    // symbol::SymbolRef,
    post,
    wrapper::{
        //attr_get_tramp, attr_set_tramp,
        MaxObjWrapped,
        MaxObjWrapper,
    },
};

mod find;
mod template;

//you need to wrap your external in this macro to get the system to register your object and
//automatically generate trampolines and what not.
median::external_no_main! {
    #[name="af.path.template"]
    pub struct PathTemplateExtn {
        // template: template::PathTemplate,
        // input: Option<template::PathInput>,
    }

    //implement the max object wrapper
    impl MaxObjWrapped<PathTemplateExtn> for PathTemplateExtn {
        //create an instance of your object
        //setup inlets/outlets and clocks
        fn new(_builder: &mut dyn MaxWrappedBuilder<Self>) -> Self {
            // let _ = builder.add_outlet(median::outlet::Outlet::new(
            //     "path",
            //     median::outlet::OutletType::Symbol,
            //     Self::path_tramp,
            // ));

            Self {
                // template: template::PathTemplate::default(),
                // input: None,
            }
        }

        // Register any methods you need for your class
        fn class_setup(c: &mut Class<MaxObjWrapper<Self>>) {
            // c.add_attribute(
            //     AttrBuilder::new_accessors(
            //         "template",
            //         AttrType::Symbol,
            //         Self::template_tramp,
            //         Self::set_template_tramp,
            //     )
            //     .build()
            //     .unwrap(),
            // )
            // .expect("failed to add template attribute");

            // c.add_attribute(
            //     AttrBuilder::new_accessors(
            //         "extension",
            //         AttrType::Symbol,
            //         Self::extension_tramp,
            //         Self::set_extension_tramp,
            //     )
            //     .build()
            //     .unwrap(),
            // )
            // .expect("failed to add extension attribute");

            // c.add_attribute(
            //     AttrBuilder::new_accessors(
            //         "suffix",
            //         AttrType::Symbol,
            //         Self::suffix_tramp,
            //         Self::set_suffix_tramp,
            //     )
            //     .build()
            //     .unwrap(),
            // )
            // .expect("failed to add suffix attribute");

            // c.add_attribute(
            //     AttrBuilder::new_accessors(
            //         "prefix",
            //         AttrType::Symbol,
            //         Self::prefix_tramp,
            //         Self::set_prefix_tramp,
            //     )
            //     .build()
            //     .unwrap(),
            // )
            // .expect("failed to add prefix attribute");
        }
    }

    //implement any methods you might want for your object that aren't part of the wrapper
    impl PathTemplateExtn {
        //create a "bang" method and automatically register it.
        //the name of the method can be anything you want
        #[bang]
        pub fn bang(&self) {
            let i = median::inlet::Proxy::get_inlet(self.max_obj());
            median::object_post!(self.max_obj(), "HERE bang inlet {}", i);
        }

        // #[attr_get_tramp]
        // pub fn path(&self) -> {
        //     // FIXME: handle potential failure
        //     SymbolRef::try_from("test").unwrap().into(),
        // }

        // #[attr_get_tramp]
        // pub fn template(&self) -> String {
        //     self.template.template.clone()
        // }

        // #[attr_set_tramp]
        // pub fn set_template(&mut self, v: String) {
        //     self.template = path_template::PathTemplate::new(&v);
        // }
    }
}

median::external_no_main! {
    #[name="af.path.find"]
    pub struct PathFindExtn {}

    impl MaxObjWrapped<PathFindExtn> for PathFindExtn {
        fn new(_builder: &mut dyn MaxWrappedBuilder<Self>) -> Self {
            post!("created af.path.find");
            Self {}
        }
    }
}

median::ext_main! {
    PathTemplateExtn::register();
    PathFindExtn::register();
}
