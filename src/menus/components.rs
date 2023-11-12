use bevy::prelude::Component;

use crate::AppState;

#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct InGameMenu {}

#[derive(Component)]
pub struct HostPage {}

#[derive(Component)]
pub struct JoinPage {}

#[derive(Component)]
pub struct ControlsPage {}

#[derive(Component)]
pub struct CreditsPage {}

#[derive(Component)]
pub struct Popup;

#[derive(Component)]
pub struct Switch{
    pub host_port: bool,
    pub port: bool,
    pub ip: bool,
    pub host: bool,
    pub num_camps: bool,
    pub num_chests: bool,
    pub enemy_per_camp: bool,
    pub map_seed: bool,
    pub EID_percentage: bool,
}

pub trait InputType: Component {
    fn push_char(&mut self, ch: char);
    fn pop_char(&mut self);
    fn is_empty(&self) -> bool;
    fn is_active(switch: &Switch) -> bool;
    fn is_valid(active: bool) -> bool;
}

impl InputType for HostPortInput {
    fn push_char(&mut self, ch: char) {
        self.port.push(ch);
    }

    fn pop_char(&mut self) {
        self.port.pop();
    }

    fn is_empty(&self) -> bool {
        self.port.is_empty()
    }

    fn is_active(switch: &Switch) -> bool {
        switch.host
    }

    fn is_valid(active: bool) -> bool {
        active
    }
}

impl InputType for NumCampsInput {
    fn push_char(&mut self, ch: char) {
        self.value.push(ch);
    }

    fn pop_char(&mut self) {
        self.value.pop();
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn is_active(switch: &Switch) -> bool {
        switch.num_camps
    }

    fn is_valid(active: bool) -> bool {
        active
    }
}

impl InputType for NumChestsInput {
    fn push_char(&mut self, ch: char) {
        self.value.push(ch);
    }

    fn pop_char(&mut self) {
        self.value.pop();
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn is_active(switch: &Switch) -> bool {
        switch.num_chests
    }

    fn is_valid(active: bool) -> bool {
        active
    }
}

impl InputType for EnemyPerCampInput {
    fn push_char(&mut self, ch: char) {
        self.value.push(ch);
    }

    fn pop_char(&mut self) {
        self.value.pop();
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn is_active(switch: &Switch) -> bool {
        switch.enemy_per_camp
    }

    fn is_valid(active: bool) -> bool {
        active
    }
}

impl InputType for MapSeedInput {
    fn push_char(&mut self, ch: char) {
        self.value.push(ch);
    }

    fn pop_char(&mut self) {
        self.value.pop();
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn is_active(switch: &Switch) -> bool {
        switch.map_seed
    }

    fn is_valid(active: bool) -> bool {
        active
    }
}

impl InputType for EIDPercentageInput {
    fn push_char(&mut self, ch: char) {
        self.value.push(ch);
    }

    fn pop_char(&mut self) {
        self.value.pop();
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn is_active(switch: &Switch) -> bool {
        switch.EID_percentage
    }

    fn is_valid(active: bool) -> bool {
        active
    }
}

impl InputType for JoinHostPortInput {
    fn push_char(&mut self, ch: char) {
        self.port.push(ch);
    }

    fn pop_char(&mut self) {
        self.port.pop();
    }

    fn is_empty(&self) -> bool {
        self.port.is_empty()
    }

    fn is_active(switch: &Switch) -> bool {
        switch.host_port
    }

    fn is_valid(active: bool) -> bool {
        active
    }
}

impl InputType for JoinPortInput {
    fn push_char(&mut self, ch: char) {
        self.port.push(ch);
    }

    fn pop_char(&mut self) {
        self.port.pop();
    }

    fn is_empty(&self) -> bool {
        self.port.is_empty()
    }

    fn is_active(switch: &Switch) -> bool {
        switch.port
    }

    fn is_valid(active: bool) -> bool {
        active
    }
}

impl InputType for JoinIPInput {
    fn push_char(&mut self, ch: char) {
        self.ip.push(ch);
    }

    fn pop_char(&mut self) {
        self.ip.pop();
    }

    fn is_empty(&self) -> bool {
        self.ip.is_empty()
    }

    fn is_active(switch: &Switch) -> bool {
        switch.ip
    }

    fn is_valid(active: bool) -> bool {
        active
    }
}

pub trait ButtonTypeTrait {
    type Marker: Component;
    fn app_state() -> AppState;
}

pub struct HostButtonType;
impl ButtonTypeTrait for HostButtonType {
    type Marker = HostButton;
    fn app_state() -> AppState {
        AppState::Hosting
    }
}

pub struct JoinButtonType;
impl ButtonTypeTrait for JoinButtonType {
    type Marker = JoinButton;
    fn app_state() -> AppState {
        AppState::Joining
    }
}

pub struct BackButtonType;
impl ButtonTypeTrait for BackButtonType {
    type Marker = BackToMainMenu;
    fn app_state() -> AppState {
        AppState::MainMenu
    }
}

pub struct ControlsButtonType;
impl ButtonTypeTrait for ControlsButtonType {
    type Marker = ControlsButton;
    fn app_state() -> AppState {
        AppState::Controls
    }
}

pub struct CreditsButtonType;
impl ButtonTypeTrait for CreditsButtonType {
    type Marker = CreditsButton;
    fn app_state() -> AppState {
        AppState::Credits
    }
}



#[derive(Component)]
pub struct HostButton {}//host button to go to the host page

#[derive(Component)]
pub struct JoinButton {}//join button to go to the join page

#[derive(Component)]
pub struct ControlsButton {}

#[derive(Component)]
pub struct CreditsButton {}

#[derive(Component)]
pub struct BackToMainMenu {}// back to main menu button

#[derive(Component)]
pub struct GameOver {}

#[derive(Component)]
pub struct HostPortInput {
    pub port: String,
} //host port input field

#[derive(Component)]
pub struct HostPortSaveBut {}//host port save button to save what the user typed in into the network address field

#[derive(Component)]
pub struct HostPortBut {}

#[derive(Component)]
pub struct NumCampsBut {}

#[derive(Component)]
pub struct NumChestsBut {}

#[derive(Component)]
pub struct EnemyPerCampBut {}

#[derive(Component)]
pub struct MapSeedBut {}

#[derive(Component)]
pub struct EIDPercentageBut {}

#[derive(Component)]
pub struct JoinHostPortBut {}

#[derive(Component)]
pub struct NumCampsInput {
    pub value: String,
}

#[derive(Component)]
pub struct NumChestsInput {
    pub value: String,
}

#[derive(Component)]
pub struct EnemyPerCampInput {
    pub value: String,
}

#[derive(Component)]
pub struct  MapSeedInput {
    pub value: String,
}

#[derive(Component)]
pub struct  EIDPercentageInput {
    pub value: String,
}

#[derive(Component)]
pub struct JoinHostPortInput {
    pub port: String,
}

#[derive(Component)]
pub struct JoinPortBut {}

#[derive(Component)]
pub struct JoinIpBut {}

#[derive(Component)]
pub struct JoinPortInput {
    pub port: String,
} //joining port input field

#[derive(Component)]
pub struct JoinIPInput {
    pub ip: String,
} //joining IP input field

#[derive(Component)]
pub struct JoinSaveBut {}//Joining port save button to save what the user typed in into the network address field

#[derive(Component)]
pub struct Initialized{}