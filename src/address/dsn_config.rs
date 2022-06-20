use crate::transport::smtp::extension::{NotifyOn, NotifyParameter, RetParameter};

#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DsnConfig {
    envelope_id: String,
    notify_success: bool,
    notify_failure: bool,
    notify_delay: bool,
    ret: Ret,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Action {
    Notify,
    DoNotIssue,
}

impl DsnConfig {
    pub fn builder(envelope_id: String) -> Builder {
        Builder {
            envelope_id,
            notify_failure: Action::Notify,
            notify_success: Action::DoNotIssue,
            notify_delay: Action::DoNotIssue,
            ret: Ret::HeadersOnly,
        }
    }
    pub(crate) fn notify_parameter(&self) -> NotifyParameter {
        let mut notification_parameters = vec![];
        if self.notify_success {
            notification_parameters.push(NotifyOn::Success);
        }
        if self.notify_failure {
            notification_parameters.push(NotifyOn::Failure);
        }
        if self.notify_delay {
            notification_parameters.push(NotifyOn::Delay);
        }
        if notification_parameters.is_empty() {
            NotifyParameter::Never
        } else {
            NotifyParameter::List(notification_parameters)
        }
    }

    pub fn ret(&self) -> Ret {
        self.ret
    }


    pub fn envelope_id(&self) -> &str {
        &self.envelope_id
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Ret {
    Full,
    HeadersOnly,
}

impl Into<RetParameter> for Ret {
    fn into(self) -> RetParameter {
        match self {
            Ret::Full => RetParameter::Full,
            Ret::HeadersOnly => RetParameter::Headers,
        }
    }
}

pub struct Builder {
    envelope_id: String,
    notify_success: Action,
    notify_failure: Action,
    notify_delay: Action,
    ret: Ret,
}

impl Builder {
    pub fn on_success(mut self, action: Action) -> Self {
        self.notify_success = action;
        self
    }
    pub fn on_failure(mut self, action: Action) -> Self {
        self.notify_failure = action;
        self
    }
    pub fn on_relay(mut self, action: Action) -> Self {
        self.notify_failure = action;
        self
    }
    pub fn return_message(mut self, ret: Ret) -> Self {
        self.ret = ret;
        self
    }
    pub fn build(self) -> DsnConfig {
        DsnConfig {
            envelope_id: self.envelope_id,
            notify_success: self.notify_success == Action::Notify,
            notify_failure: self.notify_failure == Action::Notify,
            notify_delay: self.notify_delay == Action::Notify,
            ret: self.ret,
        }
    }
}