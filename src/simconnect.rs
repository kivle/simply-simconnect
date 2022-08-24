#![allow(unused)]

// Partially based on https://github.com/Sequal32/simconnect-rust/blob/master/src/lib.rs

use super::bindings::*;
use std::{error::Error, fmt::Display, ptr};

#[derive(Debug, Clone)]
pub struct SimConnectError {
    message: String,
    result: Option<HRESULT>,
}

impl SimConnectError {
    fn new(msg: &str, result: Option<HRESULT>) -> Self {
        Self {
            message: msg.to_string(),
            result,
        }
    }
}

type SimConnectResult<T> = Result<T, SimConnectError>;

macro_rules! as_c_string {
    ($target:expr) => {
        std::ffi::CString::new($target).unwrap().as_ptr()
    };
}

macro_rules! as_c_bool {
    ($target:expr) => {{
        if $target {
            1
        } else {
            0
        }
    }};
}

macro_rules! simconnect_call {
    ($call: expr, $msg: expr) => {
        unsafe {
            match $call {
                0 => Ok(()),
                r => Err(SimConnectError::new($msg, Some(r))),
            }
        }
    };
}
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
    fn as_c_struct(&self) -> SIMCONNECT_DATA_INITPOSITION {
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

pub struct SimConnect {
    handle: HANDLE,
}

impl SimConnect {
    pub fn new() -> Self {
        Self {
            handle: std::ptr::null_mut(),
        }
    }

    pub fn open(&mut self, program_name: &str) -> SimConnectResult<()> {
        if !self.opened() {
            unsafe {
                SimConnect_Open(
                    &mut self.handle,
                    as_c_string!(program_name),
                    ptr::null_mut(),
                    0,
                    ptr::null_mut(),
                    0,
                );
            }
        }

        match self.handle.is_null() {
            true => Err(SimConnectError::new("Failed to open connection", None)),
            false => Ok(()),
        }
    }

    pub fn close(&mut self) -> SimConnectResult<()> {
        if self.opened() {
            unsafe {
                SimConnect_Close(*&mut self.handle);
            }
        } else {
            return Err(SimConnectError::new("Connection already closed", None));
        }

        match self.handle.is_null() {
            true => Ok(()),
            false => Err(SimConnectError::new("Failed to close connection", None)),
        }
    }

    pub fn opened(&self) -> bool {
        !self.handle.is_null()
    }

    pub fn ai_create_enroute_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        flight_number: i32,
        flight_plan_path: &str,
        flight_plan_position: f64,
        touch_and_go: bool,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_AICreateEnrouteATCAircraft(
                self.handle,
                as_c_string!(container_title),
                as_c_string!(tail_number),
                flight_number,
                as_c_string!(flight_plan_path),
                flight_plan_position,
                as_c_bool!(touch_and_go),
                request_id,
            ),
            "Failed to create enroute atc aircraft"
        )
    }

    pub fn ai_create_non_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        init_pos: InitPosition,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_AICreateNonATCAircraft(
                self.handle,
                as_c_string!(container_title),
                as_c_string!(tail_number),
                init_pos.as_c_struct(),
                request_id,
            ),
            "Failed to create non-atc aircraft"
        )
    }

    pub fn ai_create_parked_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        airport_id: &str,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_AICreateParkedATCAircraft(
                self.handle,
                as_c_string!(container_title),
                as_c_string!(tail_number),
                as_c_string!(airport_id),
                request_id,
            ),
            "Failed to create parked atc aircraft"
        )
    }

    pub fn ai_create_simulated_object(
        &self,
        container_title: &str,
        init_pos: InitPosition,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_AICreateSimulatedObject(
                self.handle,
                as_c_string!(container_title),
                init_pos.as_c_struct(),
                request_id,
            ),
            "Failed to create simulated object"
        )
    }

    pub fn ai_release_control(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_AIReleaseControl(self.handle, object_id, request_id),
            "Failed to release control"
        )
    }

    pub fn ai_remove_object(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_AIRemoveObject(self.handle, object_id, request_id),
            "Failed to remove object"
        )
    }

    pub fn ai_set_aircraft_flight_plan(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        flight_plan_path: &str,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_AISetAircraftFlightPlan(
                self.handle,
                object_id,
                as_c_string!(flight_plan_path),
                request_id,
            ),
            "Failed to set aircraft flight plan"
        )
    }

    pub fn add_client_event_to_notification_group(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        maskable: bool,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_AddClientEventToNotificationGroup(
                self.handle,
                group_id,
                event_id,
                as_c_bool!(maskable),
            ),
            "Failed to add client event to notification group"
        )
    }

    pub fn add_to_client_data_definition(
        &self,
        define_id: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
        offset: DWORD,
        size_or_type: DWORD,
        epsilon: f32,
        datum_id: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_AddToClientDataDefinition(
                self.handle,
                define_id,
                offset,
                size_or_type,
                epsilon,
                datum_id,
            ),
            "Failed to add client data definition"
        )
    }

    pub fn add_to_data_definition(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        datum_name: &str,
        units_name: &str,
        datum_type: DataType,
        epsilon: f32,
        datum_id: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_AddToDataDefinition(
                self.handle,
                define_id,
                as_c_string!(datum_name),
                as_c_string!(units_name),
                datum_type as SIMCONNECT_DATATYPE,
                epsilon,
                datum_id,
            ),
            "Failed to add to data definition"
        )
    }

    pub fn call_dispatch(
        &self,
        dispatch: DispatchProc,
        context: *mut ::std::os::raw::c_void,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_CallDispatch(self.handle, dispatch, context),
            "Failed to set call dispatch"
        )
    }

    pub fn camera_set_relative_6dof(
        &self,
        delta_x: f32,
        delta_y: f32,
        delta_z: f32,
        pitch_deg: f32,
        bank_deg: f32,
        heading_deg: f32,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_CameraSetRelative6DOF(
                self.handle,
                delta_x,
                delta_y,
                delta_z,
                pitch_deg,
                bank_deg,
                heading_deg,
            ),
            "Failed to change relative camera position"
        )
    }

    pub fn clear_client_data_definition(
        &self,
        define_id: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_ClearClientDataDefinition(self.handle, define_id),
            "Failed to clear client data definition"
        )
    }

    pub fn clear_data_definition(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_ClearDataDefinition(self.handle, define_id),
            "Failed to clear data definition"
        )
    }

    pub fn clear_input_group(&self, group_id: SIMCONNECT_INPUT_GROUP_ID) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_ClearInputGroup(self.handle, group_id),
            "Failed to clear input group"
        )
    }

    pub fn clear_notification_group(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_ClearNotificationGroup(self.handle, group_id),
            "Failed to clear notification group"
        )
    }

    pub fn complete_custom_missing_action(&self, instance_id: GUID) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_CompleteCustomMissionAction(self.handle, instance_id),
            "Failed to complete custom missing action"
        )
    }

    pub fn create_client_data(
        &self,
        client_data_id: SIMCONNECT_CLIENT_DATA_ID,
        size: DWORD,
        flags: CreateClientDataFlag,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_CreateClientData(
                self.handle,
                client_data_id,
                size,
                flags as SIMCONNECT_CREATE_CLIENT_DATA_FLAG
            ),
            "Failed to create client data"
        )
    }

    pub fn execute_missing_action(&self, instance_id: GUID) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_ExecuteMissionAction(self.handle, instance_id),
            "Failed to execute missing action"
        )
    }

    pub fn flight_load(&self, filename: &str) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_FlightLoad(self.handle, as_c_string!(filename)),
            "Failed to load flight"
        )
    }

    pub fn flight_plan_load(&self, filename: &str) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_FlightPlanLoad(self.handle, as_c_string!(filename)),
            "Failed to load flight plan"
        )
    }

    pub fn flight_save(
        &self,
        filename: &str,
        title: &str,
        description: &str,
        flags: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_FlightSave(
                self.handle,
                as_c_string!(filename),
                as_c_string!(title),
                as_c_string!(description),
                flags,
            ),
            "Failed to save flight"
        )
    }

    pub fn get_last_sent_packet_id(&self) -> SimConnectResult<DWORD> {
        unsafe {
            let error: &mut DWORD = &mut 0;
            match SimConnect_GetLastSentPacketID(self.handle, error) {
                0 => Ok(*error),
                r => Err(SimConnectError::new(
                    "Failed to get last sent package id",
                    Some(r),
                )),
            }
        }
    }

    pub fn get_next_dispatch(
        &self,
        data: *mut *mut SIMCONNECT_RECV,
        cb_data: *mut DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_GetNextDispatch(self.handle, data, cb_data),
            "Failed to get next dispatch"
        )
    }

    pub fn map_client_data_name_to_id(
        &self,
        client_data_name: &str,
        client_data_id: SIMCONNECT_CLIENT_DATA_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_MapClientDataNameToID(
                self.handle,
                as_c_string!(client_data_name),
                client_data_id,
            ),
            "Failed to map client data name to id"
        )
    }

    pub fn map_client_event_to_sim_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        event_name: &str,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_MapClientEventToSimEvent(self.handle, event_id, as_c_string!(event_name)),
            "Failed to map client event to sim event"
        )
    }

    pub fn map_input_event_to_client_event(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        input_definition: &str,
        down_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        down_value: DWORD,
        up_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        up_value: DWORD,
        maskable: bool,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_MapInputEventToClientEvent(
                self.handle,
                group_id,
                as_c_string!(input_definition),
                down_event_id,
                down_value,
                up_event_id,
                up_value,
                as_c_bool!(maskable),
            ),
            "Failed to map input event to client event"
        )
    }

    pub fn menu_add_item(
        &self,
        menu_item: &str,
        menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        data: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_MenuAddItem(self.handle, as_c_string!(menu_item), menu_event_id, data),
            "Failed to add menu item"
        )
    }

    pub fn menu_add_sub_item(
        &self,
        menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        menu_item: &str,
        sub_menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        data: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_MenuAddSubItem(
                self.handle,
                menu_event_id,
                as_c_string!(menu_item),
                sub_menu_event_id,
                data,
            ),
            "Failed to add sub menu item"
        )
    }

    pub fn menu_delete_item(
        &self,
        menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_MenuDeleteItem(self.handle, menu_event_id),
            "Failed to delete menu item"
        )
    }

    pub fn menu_delete_sub_item(
        &self,
        menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        sub_menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_MenuDeleteSubItem(self.handle, menu_event_id, sub_menu_event_id),
            "Failed to delete sub menu item"
        )
    }

    pub fn remove_client_event(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_RemoveClientEvent(self.handle, group_id, event_id),
            "Failed to remove client event"
        )
    }

    pub fn remove_input_event(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        input_definition: &str,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_RemoveInputEvent(self.handle, group_id, as_c_string!(input_definition)),
            "Failed to remove input event"
        )
    }

    pub fn request_client_data(
        &self,
        client_data_id: SIMCONNECT_CLIENT_DATA_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
        period: ClientDataPeriod,
        flags: ClientDataRequestFlag,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_RequestClientData(
                self.handle,
                client_data_id,
                request_id,
                define_id,
                period as SIMCONNECT_CLIENT_DATA_PERIOD,
                flags as SIMCONNECT_CLIENT_DATA_REQUEST_FLAG,
                origin,
                interval,
                limit,
            ),
            "Failed to request client data"
        )
    }

    pub fn request_data_on_sim_object(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        period: Period,
        flags: DataRequestFlag,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_RequestDataOnSimObject(
                self.handle,
                request_id,
                define_id,
                object_id,
                period as SIMCONNECT_PERIOD,
                flags as SIMCONNECT_DATA_REQUEST_FLAG,
                origin,
                interval,
                limit,
            ),
            "Failed to request data on sim object"
        )
    }

    pub fn request_data_on_sim_object_type(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        radius_meters: DWORD,
        type_: SimObjectType,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_RequestDataOnSimObjectType(
                self.handle,
                request_id,
                define_id,
                radius_meters,
                type_ as SIMCONNECT_SIMOBJECT_TYPE,
            ),
            "Failed to request data for sim object type"
        )
    }

    pub fn request_facilities_list(
        &self,
        type_: FacilityListType,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_RequestFacilitiesList(
                self.handle,
                type_ as SIMCONNECT_FACILITY_LIST_TYPE,
                request_id
            ),
            "Failed to request facilities list"
        )
    }

    pub fn request_notification_group(
        &self,
        group_id: SIMCONNECT_DATA_REQUEST_ID,
        reserved: DWORD,
        flags: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_RequestNotificationGroup(self.handle, group_id, reserved, flags),
            "Failed to request notification group"
        )
    }

    pub fn request_reserved_key(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        key_choice_1: &str,
        key_choice_2: &str,
        key_choice_3: &str,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_RequestReservedKey(
                self.handle,
                event_id,
                as_c_string!(key_choice_1),
                as_c_string!(key_choice_2),
                as_c_string!(key_choice_3),
            ),
            "Failed to request reserved key"
        )
    }

    pub fn request_response_times(&self, count: DWORD) -> SimConnectResult<f32> {
        unsafe {
            let elapsed_seconds: &mut f32 = &mut 0.0;
            match SimConnect_RequestResponseTimes(self.handle, count, elapsed_seconds) {
                0 => Ok(*elapsed_seconds),
                r => Err(SimConnectError::new(
                    "Failed to request response times",
                    Some(r),
                )),
            }
        }
    }

    pub fn request_system_state(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        state: &str,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_RequestSystemState(self.handle, request_id, as_c_string!(state)),
            "Failed to request system state"
        )
    }

    pub fn set_client_data(
        &self,
        client_id: SIMCONNECT_CLIENT_DATA_ID,
        define_id: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
        flags: ClientDataSetFlag,
        reserved: DWORD,
        cb_unit_size: DWORD,
        data_set: *mut ::std::os::raw::c_void,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_SetClientData(
                self.handle,
                client_id,
                define_id,
                flags as SIMCONNECT_CLIENT_DATA_SET_FLAG,
                reserved,
                cb_unit_size,
                data_set,
            ),
            "Failed to set client data"
        )
    }

    pub fn set_data_on_sim_object(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        flags: DataSetFlag,
        array_count: DWORD,
        cb_unit_size: DWORD,
        data_set: *mut ::std::os::raw::c_void,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_SetDataOnSimObject(
                self.handle,
                define_id,
                object_id,
                flags as SIMCONNECT_DATA_SET_FLAG,
                array_count,
                cb_unit_size,
                data_set,
            ),
            "Failed to set data on sim object"
        )
    }

    pub fn set_input_group_priority(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        priority: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_SetInputGroupPriority(self.handle, group_id, priority),
            "Failed to set input group priority"
        )
    }

    pub fn set_input_group_state(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        state: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_SetInputGroupState(self.handle, group_id, state),
            "Failed to set input group state"
        )
    }

    pub fn set_notification_group_priority(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        priority: DWORD,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_SetNotificationGroupPriority(self.handle, group_id, priority),
            "Failed to set notification group priority"
        )
    }

    pub fn set_system_event_state(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        state: State,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_SetSystemEventState(self.handle, event_id, state as SIMCONNECT_STATE),
            "Failed to set system event state"
        )
    }

    pub fn set_system_state(
        &self,
        state: &str,
        integer: DWORD,
        float: f32,
        string: &str,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_SetSystemState(
                self.handle,
                as_c_string!(state),
                integer,
                float,
                as_c_string!(string),
            ),
            "Failed to set system state"
        )
    }

    pub fn subscribe_to_facilities(
        &self,
        type_: FacilityListType,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_SubscribeToFacilities(
                self.handle,
                type_ as SIMCONNECT_FACILITY_LIST_TYPE,
                request_id
            ),
            "Failed to subscribe to facilities"
        )
    }

    pub fn subscribe_to_system_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        system_event_name: &str,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_SubscribeToSystemEvent(
                self.handle,
                event_id,
                as_c_string!(system_event_name),
            ),
            "Failed to subscribe to system event"
        )
    }

    pub fn transmit_client_event(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        data: DWORD,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        flags: EventFlag,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_TransmitClientEvent(
                self.handle,
                object_id,
                event_id,
                data,
                group_id,
                flags as SIMCONNECT_EVENT_FLAG
            ),
            "Failed to transmit client event"
        )
    }

    pub fn unsubscribe_from_system_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> SimConnectResult<()> {
        simconnect_call!(
            SimConnect_UnsubscribeFromSystemEvent(self.handle, event_id),
            "Failed to unsubscribe from system event"
        )
    }

    // TODO: SimConnect_InsertString ??
    // TODO: SimConnect_RetrieveString ??
    // TODO: SimConnect_Text ??
    // TODO: SimConnect_Weather* ??
}

impl Default for SimConnect {
    fn default() -> Self {
        SimConnect::new()
    }
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        if self.opened() {
            self.close();
        }
    }
}
