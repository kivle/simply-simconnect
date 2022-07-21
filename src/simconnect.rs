#![allow(unused)]

// Partially based on https://github.com/Sequal32/simconnect-rust/blob/master/src/lib.rs

use super::bindings::*;
use std::ptr;

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

pub struct SimConnect {
    handle: HANDLE,
}

impl SimConnect {
    pub fn new() -> Self {
        Self {
            handle: std::ptr::null_mut(),
        }
    }

    pub fn open(&mut self, program_name: &str) -> bool {
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

        !self.handle.is_null()
    }

    pub fn close(&mut self) -> bool {
        if self.opened() {
            unsafe {
                SimConnect_Close(*&mut self.handle);
            }
        }

        self.handle.is_null()
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
    ) -> bool {
        unsafe {
            return SimConnect_AICreateEnrouteATCAircraft(
                self.handle,
                as_c_string!(container_title),
                as_c_string!(tail_number),
                flight_number,
                as_c_string!(flight_plan_path),
                flight_plan_position,
                as_c_bool!(touch_and_go),
                request_id,
            ) == 0;
        }
    }

    pub fn ai_create_non_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        init_pos: SIMCONNECT_DATA_INITPOSITION,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_AICreateNonATCAircraft(
                self.handle,
                as_c_string!(container_title),
                as_c_string!(tail_number),
                init_pos,
                request_id,
            ) == 0
        }
    }

    pub fn ai_create_parked_atc_aircraft(
        &self,
        container_title: &str,
        tail_number: &str,
        airport_id: &str,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_AICreateParkedATCAircraft(
                self.handle,
                as_c_string!(container_title),
                as_c_string!(tail_number),
                as_c_string!(airport_id),
                request_id,
            ) == 0
        }
    }

    pub fn ai_create_simulated_object(
        &self,
        container_title: &str,
        init_pos: SIMCONNECT_DATA_INITPOSITION,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_AICreateSimulatedObject(
                self.handle,
                as_c_string!(container_title),
                init_pos,
                request_id,
            ) == 0
        }
    }

    pub fn ai_release_control(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_AIReleaseControl(self.handle, object_id, request_id) == 0 }
    }

    pub fn ai_remove_object(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_AIRemoveObject(self.handle, object_id, request_id) == 0 }
    }

    pub fn ai_set_aircraft_flight_plan(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        flight_plan_path: &str,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe {
            SimConnect_AISetAircraftFlightPlan(
                self.handle,
                object_id,
                as_c_string!(flight_plan_path),
                request_id,
            ) == 0
        }
    }

    pub fn add_client_event_to_notification_group(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        maskable: bool,
    ) -> bool {
        unsafe {
            SimConnect_AddClientEventToNotificationGroup(
                self.handle,
                group_id,
                event_id,
                as_c_bool!(maskable),
            ) == 0
        }
    }

    pub fn add_to_client_data_definition(
        &self,
        define_id: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
        offset: DWORD,
        size_or_type: DWORD,
        epsilon: f32,
        datum_id: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_AddToClientDataDefinition(
                self.handle,
                define_id,
                offset,
                size_or_type,
                epsilon,
                datum_id,
            ) == 0
        }
    }

    pub fn add_to_data_definition(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        datum_name: &str,
        units_name: &str,
        datum_type: SIMCONNECT_DATATYPE,
        epsilon: f32,
        datum_id: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_AddToDataDefinition(
                self.handle,
                define_id,
                as_c_string!(datum_name),
                as_c_string!(units_name),
                datum_type,
                epsilon,
                datum_id,
            ) == 0
        }
    }

    pub fn call_dispatch(
        &self,
        dispatch: DispatchProc,
        context: *mut ::std::os::raw::c_void,
    ) -> bool {
        unsafe { SimConnect_CallDispatch(self.handle, dispatch, context) == 0 }
    }

    pub fn camera_set_relative_6dof(
        &self,
        delta_x: f32,
        delta_y: f32,
        delta_z: f32,
        pitch_deg: f32,
        bank_deg: f32,
        heading_deg: f32,
    ) -> bool {
        unsafe {
            SimConnect_CameraSetRelative6DOF(
                self.handle,
                delta_x,
                delta_y,
                delta_z,
                pitch_deg,
                bank_deg,
                heading_deg,
            ) == 0
        }
    }

    pub fn clear_client_data_definition(
        &self,
        define_id: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
    ) -> bool {
        unsafe { SimConnect_ClearClientDataDefinition(self.handle, define_id) == 0 }
    }

    pub fn clear_data_definition(&self, define_id: SIMCONNECT_DATA_DEFINITION_ID) -> bool {
        unsafe { SimConnect_ClearDataDefinition(self.handle, define_id) == 0 }
    }

    pub fn clear_input_group(&self, group_id: SIMCONNECT_INPUT_GROUP_ID) -> bool {
        unsafe { SimConnect_ClearInputGroup(self.handle, group_id) == 0 }
    }

    pub fn clear_notification_group(&self, group_id: SIMCONNECT_NOTIFICATION_GROUP_ID) -> bool {
        unsafe { SimConnect_ClearNotificationGroup(self.handle, group_id) == 0 }
    }

    pub fn complete_custom_missing_action(&self, instance_id: GUID) -> bool {
        unsafe { SimConnect_CompleteCustomMissionAction(self.handle, instance_id) == 0 }
    }

    pub fn create_client_data(
        &self,
        client_data_id: SIMCONNECT_CLIENT_DATA_ID,
        size: DWORD,
        flags: SIMCONNECT_CREATE_CLIENT_DATA_FLAG,
    ) -> bool {
        unsafe { SimConnect_CreateClientData(self.handle, client_data_id, size, flags) == 0 }
    }

    pub fn execute_missing_action(&self, instance_id: GUID) -> bool {
        unsafe { SimConnect_ExecuteMissionAction(self.handle, instance_id) == 0 }
    }

    pub fn flight_load(&self, filename: &str) -> bool {
        unsafe { SimConnect_FlightLoad(self.handle, as_c_string!(filename)) == 0 }
    }

    pub fn flight_plan_load(&self, filename: &str) -> bool {
        unsafe { SimConnect_FlightPlanLoad(self.handle, as_c_string!(filename)) == 0 }
    }

    pub fn flight_save(
        &self,
        filename: &str,
        title: &str,
        description: &str,
        flags: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_FlightSave(
                self.handle,
                as_c_string!(filename),
                as_c_string!(title),
                as_c_string!(description),
                flags,
            ) == 0
        }
    }

    pub fn get_last_sent_packet_id(&self, error: &mut DWORD) -> bool {
        unsafe { SimConnect_GetLastSentPacketID(self.handle, error) == 0 }
    }

    pub fn get_next_dispatch(&self, data: *mut *mut SIMCONNECT_RECV, cb_data: *mut DWORD) -> bool {
        unsafe { SimConnect_GetNextDispatch(self.handle, data, cb_data) == 0 }
    }

    pub fn map_client_data_name_to_id(
        &self,
        client_data_name: &str,
        client_data_id: SIMCONNECT_CLIENT_DATA_ID,
    ) -> bool {
        unsafe {
            SimConnect_MapClientDataNameToID(
                self.handle,
                as_c_string!(client_data_name),
                client_data_id,
            ) == 0
        }
    }

    pub fn map_client_event_to_sim_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        event_name: &str,
    ) -> bool {
        unsafe {
            SimConnect_MapClientEventToSimEvent(self.handle, event_id, as_c_string!(event_name))
                == 0
        }
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
    ) -> bool {
        unsafe {
            SimConnect_MapInputEventToClientEvent(
                self.handle,
                group_id,
                as_c_string!(input_definition),
                down_event_id,
                down_value,
                up_event_id,
                up_value,
                as_c_bool!(maskable),
            ) == 0
        }
    }

    pub fn menu_add_item(
        &self,
        menu_item: &str,
        menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        data: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_MenuAddItem(self.handle, as_c_string!(menu_item), menu_event_id, data) == 0
        }
    }

    pub fn menu_add_sub_item(
        &self,
        menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        menu_item: &str,
        sub_menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        data: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_MenuAddSubItem(
                self.handle,
                menu_event_id,
                as_c_string!(menu_item),
                sub_menu_event_id,
                data,
            ) == 0
        }
    }

    pub fn menu_delete_item(&self, menu_event_id: SIMCONNECT_CLIENT_EVENT_ID) -> bool {
        unsafe { SimConnect_MenuDeleteItem(self.handle, menu_event_id) == 0 }
    }

    pub fn menu_delete_sub_item(
        &self,
        menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
        sub_menu_event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> bool {
        unsafe { SimConnect_MenuDeleteSubItem(self.handle, menu_event_id, sub_menu_event_id) == 0 }
    }

    pub fn remove_client_event(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
    ) -> bool {
        unsafe { SimConnect_RemoveClientEvent(self.handle, group_id, event_id) == 0 }
    }

    pub fn remove_input_event(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        input_definition: &str,
    ) -> bool {
        unsafe {
            SimConnect_RemoveInputEvent(self.handle, group_id, as_c_string!(input_definition)) == 0
        }
    }

    pub fn request_client_data(
        &self,
        client_data_id: SIMCONNECT_CLIENT_DATA_ID,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
        period: SIMCONNECT_CLIENT_DATA_PERIOD,
        flags: SIMCONNECT_CLIENT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestClientData(
                self.handle,
                client_data_id,
                request_id,
                define_id,
                period,
                flags,
                origin,
                interval,
                limit,
            ) == 0
        }
    }

    pub fn request_data_on_sim_object(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        period: SIMCONNECT_PERIOD,
        flags: SIMCONNECT_DATA_REQUEST_FLAG,
        origin: DWORD,
        interval: DWORD,
        limit: DWORD,
    ) -> bool {
        unsafe {
            SimConnect_RequestDataOnSimObject(
                self.handle,
                request_id,
                define_id,
                object_id,
                period,
                flags,
                origin,
                interval,
                limit,
            ) == 0
        }
    }

    pub fn request_data_on_sim_object_type(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        radius_meters: DWORD,
        type_: SIMCONNECT_SIMOBJECT_TYPE,
    ) -> bool {
        unsafe {
            SimConnect_RequestDataOnSimObjectType(
                self.handle,
                request_id,
                define_id,
                radius_meters,
                type_,
            ) == 0
        }
    }

    pub fn request_facilities_list(
        &self,
        type_: SIMCONNECT_FACILITY_LIST_TYPE,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_RequestFacilitiesList(self.handle, type_, request_id) == 0 }
    }

    pub fn request_notification_group(
        &self,
        group_id: SIMCONNECT_DATA_REQUEST_ID,
        reserved: DWORD,
        flags: DWORD,
    ) -> bool {
        unsafe { SimConnect_RequestNotificationGroup(self.handle, group_id, reserved, flags) == 0 }
    }

    pub fn request_reserved_key(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        key_choice_1: &str,
        key_choice_2: &str,
        key_choice_3: &str,
    ) -> bool {
        unsafe {
            SimConnect_RequestReservedKey(
                self.handle,
                event_id,
                as_c_string!(key_choice_1),
                as_c_string!(key_choice_2),
                as_c_string!(key_choice_3),
            ) == 0
        }
    }

    pub fn request_response_times(&self, count: DWORD, elapsed_seconds: *mut f32) -> bool {
        unsafe { SimConnect_RequestResponseTimes(self.handle, count, elapsed_seconds) == 0 }
    }

    pub fn request_system_state(
        &self,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
        state: &str,
    ) -> bool {
        unsafe { SimConnect_RequestSystemState(self.handle, request_id, as_c_string!(state)) == 0 }
    }

    pub fn set_client_data(
        &self,
        client_id: SIMCONNECT_CLIENT_DATA_ID,
        define_id: SIMCONNECT_CLIENT_DATA_DEFINITION_ID,
        flags: SIMCONNECT_CLIENT_DATA_SET_FLAG,
        reserved: DWORD,
        cb_unit_size: DWORD,
        data_set: *mut ::std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_SetClientData(
                self.handle,
                client_id,
                define_id,
                flags,
                reserved,
                cb_unit_size,
                data_set,
            ) == 0
        }
    }

    pub fn set_data_on_sim_object(
        &self,
        define_id: SIMCONNECT_DATA_DEFINITION_ID,
        object_id: SIMCONNECT_OBJECT_ID,
        flags: SIMCONNECT_DATA_SET_FLAG,
        array_count: DWORD,
        cb_unit_size: DWORD,
        data_set: *mut ::std::os::raw::c_void,
    ) -> bool {
        unsafe {
            SimConnect_SetDataOnSimObject(
                self.handle,
                define_id,
                object_id,
                flags,
                array_count,
                cb_unit_size,
                data_set,
            ) == 0
        }
    }

    pub fn set_input_group_priority(
        &self,
        group_id: SIMCONNECT_INPUT_GROUP_ID,
        priority: DWORD,
    ) -> bool {
        unsafe { SimConnect_SetInputGroupPriority(self.handle, group_id, priority) == 0 }
    }

    pub fn set_input_group_state(&self, group_id: SIMCONNECT_INPUT_GROUP_ID, state: DWORD) -> bool {
        unsafe { SimConnect_SetInputGroupState(self.handle, group_id, state) == 0 }
    }

    pub fn set_notification_group_priority(
        &self,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        priority: DWORD,
    ) -> bool {
        unsafe { SimConnect_SetNotificationGroupPriority(self.handle, group_id, priority) == 0 }
    }

    pub fn set_system_event_state(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        state: SIMCONNECT_STATE,
    ) -> bool {
        unsafe { SimConnect_SetSystemEventState(self.handle, event_id, state) == 0 }
    }

    pub fn set_system_state(&self, state: &str, integer: DWORD, float: f32, string: &str) -> bool {
        unsafe {
            SimConnect_SetSystemState(
                self.handle,
                as_c_string!(state),
                integer,
                float,
                as_c_string!(string),
            ) == 0
        }
    }

    pub fn subscribe_to_facilities(
        &self,
        type_: SIMCONNECT_FACILITY_LIST_TYPE,
        request_id: SIMCONNECT_DATA_REQUEST_ID,
    ) -> bool {
        unsafe { SimConnect_SubscribeToFacilities(self.handle, type_, request_id) == 0 }
    }

    pub fn subscribe_to_system_event(
        &self,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        system_event_name: &str,
    ) -> bool {
        unsafe {
            SimConnect_SubscribeToSystemEvent(
                self.handle,
                event_id,
                as_c_string!(system_event_name),
            ) == 0
        }
    }

    pub fn transmit_client_event(
        &self,
        object_id: SIMCONNECT_OBJECT_ID,
        event_id: SIMCONNECT_CLIENT_EVENT_ID,
        data: DWORD,
        group_id: SIMCONNECT_NOTIFICATION_GROUP_ID,
        flags: SIMCONNECT_EVENT_FLAG,
    ) -> bool {
        unsafe {
            SimConnect_TransmitClientEvent(self.handle, object_id, event_id, data, group_id, flags)
                == 0
        }
    }

    pub fn unsubscribe_from_system_event(&self, event_id: SIMCONNECT_CLIENT_EVENT_ID) -> bool {
        unsafe { SimConnect_UnsubscribeFromSystemEvent(self.handle, event_id) == 0 }
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
