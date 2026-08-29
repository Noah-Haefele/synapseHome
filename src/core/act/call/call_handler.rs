use crate::core::api::grpc_call_server::LiveSignalsService;

pub struct CallHandler {
    grpc_call_signal_api: LiveSignalsService,
    call_target_device_id: i32,
}

impl CallHandler {
    pub fn new(grpc_call_signal_api: LiveSignalsService) -> Self {
        Self {
            grpc_call_signal_api,
            call_target_device_id: -1,
        }
    }

    pub fn initiate_call(&mut self, device_id: i32) {
        self.call_target_device_id = device_id;
        self.grpc_call_signal_api
            .trigger_call_state_changed("CALLING")
    }

    pub fn accept_call(&self) {}

    pub fn end_call(&mut self) {
        self.call_target_device_id = -1;
        self.grpc_call_signal_api.trigger_call_state_changed("IDLE");
    }
}
