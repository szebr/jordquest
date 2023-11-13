use bevy::prelude::Component;

use crate::AppState;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct InGameUi;

#[derive(Component)]
pub struct HostPage;

#[derive(Component)]
pub struct JoinPage;

#[derive(Component)]
pub struct ControlsPage;

#[derive(Component)]
pub struct CreditsPage;

#[derive(Component)]
pub struct ConnectingPage;

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
    pub eid_percentage: bool,
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

impl InputType for EnemiesPerCampInput {
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

impl InputType for EidPercentageInput {
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
        switch.eid_percentage
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
pub struct HostButton;

#[derive(Component)]
pub struct JoinButton;

#[derive(Component)]
pub struct ControlsButton;

#[derive(Component)]
pub struct CreditsButton;

#[derive(Component)]
pub struct BackToMainMenu;

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct HostPortInput {
    pub port: String,
}

#[derive(Component)]
pub struct HostPortSaveButton;

#[derive(Component)]
pub struct HostPortButton;

#[derive(Component)]
pub struct NumCampsButton;

#[derive(Component)]
pub struct NumChestsButton;

#[derive(Component)]
pub struct EnemiesPerCampButton;

#[derive(Component)]
pub struct MapSeedButton;

#[derive(Component)]
pub struct EidPercentageButton;

#[derive(Component)]
pub struct JoinHostPortButton;

#[derive(Component)]
pub struct NumCampsInput {
    pub value: String,
}

#[derive(Component)]
pub struct NumChestsInput {
    pub value: String,
}

#[derive(Component)]
pub struct EnemiesPerCampInput {
    pub value: String,
}

#[derive(Component)]
pub struct MapSeedInput {
    pub value: String,
}

#[derive(Component)]
pub struct EidPercentageInput {
    pub value: String,
}

#[derive(Component)]
pub struct JoinHostPortInput {
    pub port: String,
}

#[derive(Component)]
pub struct JoinPortButton;

#[derive(Component)]
pub struct JoinIpButton;

#[derive(Component)]
pub struct JoinPortInput {
    pub port: String,
}

#[derive(Component)]
pub struct JoinIPInput {
    pub ip: String,
}

#[derive(Component)]
pub struct JoinSaveButton;

#[derive(Component)]
pub struct Initialized;