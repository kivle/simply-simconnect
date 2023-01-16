use super::bindings::*;

macro_rules! as_c_string {
    ($target:expr) => {
        std::ffi::CString::new($target).unwrap().as_ptr()
    };
}

pub(crate) use as_c_string;

macro_rules! as_c_bool {
    ($target:expr) => {{
        if $target {
            1
        } else {
            0
        }
    }};
}

pub(crate) use as_c_bool;

#[derive(Debug, Clone)]
pub struct SimConnectError {
    message: String,
    result: Option<HRESULT>,
}

impl SimConnectError {
    pub fn new(msg: &str, result: Option<HRESULT>) -> Self {
        Self {
            message: msg.to_string(),
            result,
        }
    }
}

pub type SimConnectResult<T> = Result<T, SimConnectError>;

pub struct InitPosition {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub pitch: f64,
    pub bank: f64,
    pub heading: f64,
    pub on_ground: bool,
    pub airspeed: u32,
}

impl InitPosition {
    pub fn as_c_struct(&self) -> SIMCONNECT_DATA_INITPOSITION {
        SIMCONNECT_DATA_INITPOSITION {
            Latitude: self.latitude,
            Longitude: self.longitude,
            Altitude: self.altitude,
            Pitch: self.pitch,
            Bank: self.bank,
            Heading: self.heading,
            OnGround: as_c_bool!(self.on_ground),
            Airspeed: self.airspeed,
        }
    }
}

pub enum ClientDataType {
    Int8 = SIMCONNECT_CLIENTDATATYPE_INT8 as isize,
    Int16 = SIMCONNECT_CLIENTDATATYPE_INT16 as isize,
    Int32 = SIMCONNECT_CLIENTDATATYPE_INT32 as isize,
    Int64 = SIMCONNECT_CLIENTDATATYPE_INT64 as isize,
    Float32 = SIMCONNECT_CLIENTDATATYPE_FLOAT32 as isize,
    Float64 = SIMCONNECT_CLIENTDATATYPE_FLOAT64 as isize,
}

pub enum DataType {
    Invalid = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INVALID as isize,
    Int32 = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INT32 as isize,
    Int64 = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INT64 as isize,
    Float32 = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT32 as isize,
    Float64 = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64 as isize,
    String8 = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING8 as isize,
    String32 = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING32 as isize,
    String64 = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING64 as isize,
    String128 = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING128 as isize,
    String256 = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING256 as isize,
    String260 = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING260 as isize,
    StringV = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRINGV as isize,
    InitPosition = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INITPOSITION as isize,
    MarkerState = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_MARKERSTATE as isize,
    Waypoint = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_WAYPOINT as isize,
    LatLonAlt = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_LATLONALT as isize,
    Xyz = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_XYZ as isize,
    Max = SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_MAX as isize,
}

pub enum CreateClientDataFlag {
    Default = SIMCONNECT_CREATE_CLIENT_DATA_FLAG_DEFAULT as isize,
    ReadOnly = SIMCONNECT_CREATE_CLIENT_DATA_FLAG_READ_ONLY as isize,
}

pub enum Period {
    Never = SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_NEVER as isize,
    Once = SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_ONCE as isize,
    VisualFrame = SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_VISUAL_FRAME as isize,
    SimFrame = SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME as isize,
    Second = SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SECOND as isize,
}

pub enum DataRequestFlag {
    Default = SIMCONNECT_DATA_REQUEST_FLAG_DEFAULT as isize,
    Changed = SIMCONNECT_DATA_REQUEST_FLAG_CHANGED as isize,
    Tagged = SIMCONNECT_DATA_REQUEST_FLAG_TAGGED as isize,
}

pub enum SimObjectType {
    User = SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_USER as isize,
    All = SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_ALL as isize,
    Aircraft = SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_AIRCRAFT as isize,
    Helicopter = SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_HELICOPTER as isize,
    Boat = SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_BOAT as isize,
    Ground = SIMCONNECT_SIMOBJECT_TYPE_SIMCONNECT_SIMOBJECT_TYPE_GROUND as isize,
}

pub enum FacilityListType {
    Airport = SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_AIRPORT as isize,
    Waypoint = SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_WAYPOINT as isize,
    Ndb = SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_NDB as isize,
    Vor = SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_VOR as isize,
    Count = SIMCONNECT_FACILITY_LIST_TYPE_SIMCONNECT_FACILITY_LIST_TYPE_COUNT as isize,
}

pub enum ClientDataSetFlag {
    Default = SIMCONNECT_CLIENT_DATA_SET_FLAG_DEFAULT as isize,
    Tagged = SIMCONNECT_CLIENT_DATA_SET_FLAG_TAGGED as isize,
}

pub enum DataSetFlag {
    Default = SIMCONNECT_DATA_SET_FLAG_DEFAULT as isize,
    Tagged = SIMCONNECT_DATA_SET_FLAG_TAGGED as isize,
}

pub enum State {
    Off = SIMCONNECT_STATE_SIMCONNECT_STATE_OFF as isize,
    On = SIMCONNECT_STATE_SIMCONNECT_STATE_ON as isize,
}

pub enum EventFlag {
    Default = SIMCONNECT_EVENT_FLAG_DEFAULT as isize,
    FastRepeatTimer = SIMCONNECT_EVENT_FLAG_FAST_REPEAT_TIMER as isize,
    SlowRepeatTimer = SIMCONNECT_EVENT_FLAG_SLOW_REPEAT_TIMER as isize,
    GroupIdIsPriority = SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY as isize,
}

pub enum ClientDataPeriod {
    Never = SIMCONNECT_CLIENT_DATA_PERIOD_SIMCONNECT_CLIENT_DATA_PERIOD_NEVER as isize,
    Once = SIMCONNECT_CLIENT_DATA_PERIOD_SIMCONNECT_CLIENT_DATA_PERIOD_ONCE as isize,
    VisualFrame = SIMCONNECT_CLIENT_DATA_PERIOD_SIMCONNECT_CLIENT_DATA_PERIOD_VISUAL_FRAME as isize,
    OnSet = SIMCONNECT_CLIENT_DATA_PERIOD_SIMCONNECT_CLIENT_DATA_PERIOD_ON_SET as isize,
    Second = SIMCONNECT_CLIENT_DATA_PERIOD_SIMCONNECT_CLIENT_DATA_PERIOD_SECOND as isize,
}

pub enum ClientDataRequestFlag {
    Default = SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_DEFAULT as isize,
    Changed = SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED as isize,
    Tagged = SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_TAGGED as isize,
}
