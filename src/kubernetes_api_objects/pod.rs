// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::api_resource::*;
use crate::kubernetes_api_objects::common::*;
use crate::kubernetes_api_objects::container::*;
use crate::kubernetes_api_objects::dynamic::*;
use crate::kubernetes_api_objects::error::ParseDynamicObjectError;
use crate::kubernetes_api_objects::marshal::*;
use crate::kubernetes_api_objects::object_meta::*;
use crate::kubernetes_api_objects::resource::*;
use crate::kubernetes_api_objects::resource_requirements::*;
use crate::kubernetes_api_objects::volume::*;
use crate::pervasive_ext::string_view::*;
use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::string::*;

verus! {

/// Pod is a type of API object used for grouping one or more containers that share storage and network resources.
/// This is the smallest deployable unit in Kubernetes.
///
/// You can specify the Container(s), including the images and commands, and the Volume(s),
/// such as a ConfigMap or a Secret, in the specification of a Pod (i.e., PodSpec).
///
/// This definition is a wrapper of Pod defined at
/// https://github.com/Arnavion/k8s-openapi/blob/v0.17.0/src/v1_26/api/core/v1/pod.rs.
/// It is supposed to be used in exec controller code.
///
/// More detailed information: https://kubernetes.io/docs/concepts/workloads/pods/.

#[verifier(external_body)]
pub struct Pod {
    inner: deps_hack::k8s_openapi::api::core::v1::Pod,
}

impl Pod {
    pub spec fn view(&self) -> PodView;

    #[verifier(external_body)]
    pub fn default() -> (pod: Pod)
        ensures
            pod@ == PodView::default(),
    {
        Pod {
            inner: deps_hack::k8s_openapi::api::core::v1::Pod::default(),
        }
    }

    #[verifier(external_body)]
    pub fn metadata(&self) -> (metadata: ObjectMeta)
        ensures
            metadata@ == self@.metadata,
    {
        ObjectMeta::from_kube(self.inner.metadata.clone())
    }

    #[verifier(external_body)]
    pub fn spec(&self) -> (spec: Option<PodSpec>)
        ensures
            self@.spec.is_Some() == spec.is_Some(),
            spec.is_Some() ==> spec.get_Some_0()@ == self@.spec.get_Some_0(),
    {
        todo!()
    }

    #[verifier(external_body)]
    pub fn set_metadata(&mut self, metadata: ObjectMeta)
        ensures
            self@ == old(self)@.set_metadata(metadata@),
    {
        self.inner.metadata = metadata.into_kube();
    }

    #[verifier(external_body)]
    pub fn set_spec(&mut self, spec: PodSpec)
        ensures
            self@ == old(self)@.set_spec(spec@),
    {
        self.inner.spec = Some(spec.into_kube());
    }

    #[verifier(external)]
    pub fn into_kube(self) -> deps_hack::k8s_openapi::api::core::v1::Pod {
        self.inner
    }

    #[verifier(external_body)]
    pub fn api_resource() -> (res: ApiResource)
        ensures
            res@.kind == PodView::kind(),
    {
        ApiResource::from_kube(deps_hack::kube::api::ApiResource::erase::<deps_hack::k8s_openapi::api::core::v1::Pod>(&()))
    }

    // NOTE: This function assumes serde_json::to_string won't fail!
    #[verifier(external_body)]
    pub fn to_dynamic_object(self) -> (obj: DynamicObject)
        ensures
            obj@ == self@.to_dynamic_object(),
    {
        DynamicObject::from_kube(
            deps_hack::k8s_openapi::serde_json::from_str(&deps_hack::k8s_openapi::serde_json::to_string(&self.inner).unwrap()).unwrap()
        )
    }

    /// Convert a DynamicObject to a Pod
    #[verifier(external_body)]
    pub fn from_dynamic_object(obj: DynamicObject) -> (res: Result<Pod, ParseDynamicObjectError>)
        ensures
            res.is_Ok() == PodView::from_dynamic_object(obj@).is_Ok(),
            res.is_Ok() ==> res.get_Ok_0()@ == PodView::from_dynamic_object(obj@).get_Ok_0(),
    {
        let parse_result = obj.into_kube().try_parse::<deps_hack::k8s_openapi::api::core::v1::Pod>();
        if parse_result.is_ok() {
            let res = Pod { inner: parse_result.unwrap() };
            Ok(res)
        } else {
            Err(ParseDynamicObjectError::ExecError)
        }
    }
}

#[verifier(external_body)]
pub struct PodSpec {
    inner: deps_hack::k8s_openapi::api::core::v1::PodSpec,
}

impl PodSpec {
    pub spec fn view(&self) -> PodSpecView;

    #[verifier(external_body)]
    pub fn default() -> (pod_spec: PodSpec)
        ensures
            pod_spec@ == PodSpecView::default(),
    {
        PodSpec {
            inner: deps_hack::k8s_openapi::api::core::v1::PodSpec::default(),
        }
    }

    #[verifier(external_body)]
    pub fn set_containers(&mut self, containers: Vec<Container>)
        ensures
            self@ == old(self)@.set_containers(containers@.map_values(|container: Container| container@)),
    {
        self.inner.containers = containers.into_iter().map(|container: Container| container.into_kube()).collect()
    }

    #[verifier(external_body)]
    pub fn set_volumes(&mut self, volumes: Vec<Volume>)
        ensures
            self@ == old(self)@.set_volumes(volumes@.map_values(|vol: Volume| vol@)),
    {
        self.inner.volumes = Some(volumes.into_iter().map(|vol: Volume| vol.into_kube()).collect())
    }

    #[verifier(external_body)]
    pub fn set_init_containers(&mut self, init_containers: Vec<Container>)
        ensures
            self@ == old(self)@.set_init_containers(init_containers@.map_values(|container: Container| container@)),
    {
        self.inner.init_containers = Some(init_containers.into_iter().map(|container: Container| container.into_kube()).collect())
    }

    #[verifier(external_body)]
    pub fn set_service_account_name(&mut self, service_account: String)
        ensures
            self@ == old(self)@.set_service_account_name(service_account@),
    {
        self.inner.service_account_name = Some(service_account.into_rust_string())
    }

    #[verifier(external_body)]
    pub fn set_tolerations(&mut self, tolerations: Vec<Toleration>)
        ensures
            self@ == old(self)@,
    {
        self.inner.tolerations = Some(
            tolerations.into_iter().map(|t: Toleration| t.into_kube()).collect()
        )
    }

    #[verifier(external)]
    pub fn into_kube(self) -> deps_hack::k8s_openapi::api::core::v1::PodSpec {
        self.inner
    }
}

#[verifier(external_body)]
pub struct Toleration {
    inner: deps_hack::k8s_openapi::api::core::v1::Toleration,
}

impl Toleration {
    #[verifier(external)]
    pub fn from_kube(inner: deps_hack::k8s_openapi::api::core::v1::Toleration) -> Toleration {
        Toleration { inner: inner }
    }

    #[verifier(external)]
    pub fn into_kube(self) -> deps_hack::k8s_openapi::api::core::v1::Toleration {
        self.inner
    }
}

/// PodView is the ghost type of Pod.
/// It is supposed to be used in spec and proof code.

pub struct PodView {
    pub metadata: ObjectMetaView,
    pub spec: Option<PodSpecView>,
}

impl PodView {
    pub open spec fn default() -> PodView {
        PodView {
            metadata: ObjectMetaView::default(),
            spec: None,
        }
    }

    pub open spec fn set_metadata(self, metadata: ObjectMetaView) -> PodView {
        PodView {
            metadata: metadata,
            ..self
        }
    }

    pub open spec fn set_spec(self, spec: PodSpecView) -> PodView {
        PodView {
            spec: Some(spec),
            ..self
        }
    }
}

impl ResourceView for PodView {
    type Spec = Option<PodSpecView>;

    open spec fn metadata(self) -> ObjectMetaView {
        self.metadata
    }

    open spec fn kind() -> Kind {
        Kind::PodKind
    }

    open spec fn object_ref(self) -> ObjectRef {
        ObjectRef {
            kind: Self::kind(),
            name: self.metadata.name.get_Some_0(),
            namespace: self.metadata.namespace.get_Some_0(),
        }
    }

    proof fn object_ref_is_well_formed() {}

    open spec fn spec(self) -> Option<PodSpecView> {
        self.spec
    }

    open spec fn to_dynamic_object(self) -> DynamicObjectView {
        DynamicObjectView {
            kind: Self::kind(),
            metadata: self.metadata,
            spec: PodView::marshal_spec(self.spec),
        }
    }

    open spec fn from_dynamic_object(obj: DynamicObjectView) -> Result<PodView, ParseDynamicObjectError> {
        if obj.kind != Self::kind() {
            Err(ParseDynamicObjectError::UnmarshalError)
        } else if !PodView::unmarshal_spec(obj.spec).is_Ok() {
            Err(ParseDynamicObjectError::UnmarshalError)
        } else {
            Ok(PodView {
                metadata: obj.metadata,
                spec: PodView::unmarshal_spec(obj.spec).get_Ok_0(),
            })
        }
    }

    proof fn to_dynamic_preserves_integrity() {
        PodView::spec_integrity_is_preserved_by_marshal();
    }

    proof fn from_dynamic_preserves_metadata() {}

    proof fn from_dynamic_preserves_kind() {}

    closed spec fn marshal_spec(s: Option<PodSpecView>) -> Value;

    closed spec fn unmarshal_spec(v: Value) -> Result<Option<PodSpecView>, ParseDynamicObjectError>;

    #[verifier(external_body)]
    proof fn spec_integrity_is_preserved_by_marshal(){}

    proof fn from_dynamic_object_result_determined_by_unmarshal() {}

    open spec fn rule(obj: PodView) -> bool {
        true
    }

    open spec fn transition_rule(new_obj: PodView, old_obj: PodView) -> bool {
        true
    }
}

pub struct PodSpecView {
    pub containers: Seq<ContainerView>,
    pub volumes: Option<Seq<VolumeView>>,
    pub init_containers: Option<Seq<ContainerView>>,
    pub service_account_name: Option<StringView>,
}

impl PodSpecView {
    pub open spec fn default() -> PodSpecView {
        PodSpecView {
            containers: Seq::empty(),
            volumes: None,
            init_containers: None,
            service_account_name: None,
        }
    }

    pub open spec fn set_containers(self, containers: Seq<ContainerView>) -> PodSpecView {
        PodSpecView {
            containers: containers,
            ..self
        }
    }

    pub open spec fn set_volumes(self, volumes: Seq<VolumeView>) -> PodSpecView {
        PodSpecView {
            volumes: Some(volumes),
            ..self
        }
    }

    pub open spec fn set_init_containers(self, init_containers: Seq<ContainerView>) -> PodSpecView {
        PodSpecView {
            init_containers: Some(init_containers),
            ..self
        }
    }

    pub open spec fn set_service_account_name(self, service_account_name: StringView) -> PodSpecView {
        PodSpecView {
            service_account_name: Some(service_account_name),
            ..self
        }
    }
}

impl Marshalable for PodSpecView {
    spec fn marshal(self) -> Value;

    spec fn unmarshal(value: Value) -> Result<Self, ParseDynamicObjectError>;

    #[verifier(external_body)]
    proof fn marshal_returns_non_null() {}

    #[verifier(external_body)]
    proof fn marshal_preserves_integrity() {}
}

}
