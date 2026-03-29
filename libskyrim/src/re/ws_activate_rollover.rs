// C++ `RE::WSActivateRollover` minimal menu marker.
//
// TODO: SOURCE - add RTTI/VTABLE once `generate_offsets.py` exposes the
// `WSActivateRollover` offset constants; the current stub preserves only
// `MENU_NAME` for menu lookup APIs.
core_util::abstract_type! { pub type WSActivateRollover; }

impl WSActivateRollover {
    pub const MENU_NAME: &'static str = "WSActivateRollover";
}
